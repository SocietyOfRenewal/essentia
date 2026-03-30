use std::collections::{BTreeMap, BTreeSet};

use anyhow::Result;

use crate::crypto::{hash_json, sign_block, verify_block_signature, verify_signed_transaction};
use crate::error::ChainError;
use crate::models::{
    AccountView, Amount, BasisPoints, Block, BlockHeight, Chain, Challenge, Claim, ClaimStatus,
    DidRecord, Epoch, EpochAccounting, EpochBudget, EssentialUnitLot, GenesisConfig, KeyFile,
    LedgerState, LockReason, LockedBalance, NodeStatus, PersonhoodCredential, Proposal, ProposalKind,
    ProposalResult, ProposalStatus, Quest, Review, ReviewScore, RiskBand, Role, SignedTransaction,
    StateDump, TransactionKind, UnsignedBlock, VendorSettlement,
};

impl LedgerState {
    pub fn prepare_for_block(&mut self, next_height: BlockHeight) {
        let mut unlocked = Vec::new();
        self.locked_essent.retain(|lock| {
            if let Some(unlock_height) = lock.unlock_height {
                if unlock_height <= next_height {
                    unlocked.push((lock.owner.clone(), lock.amount));
                    return false;
                }
            }
            true
        });

        for (owner, amount) in unlocked {
            *self.essent_balances.entry(owner).or_default() += amount;
        }

        for lots in self.essential_units.values_mut() {
            lots.retain(|lot| lot.amount > 0 && lot.expires_at > next_height);
        }
    }
}

impl Chain {
    pub fn from_genesis(genesis: GenesisConfig) -> Self {
        let mut dids = BTreeMap::new();
        let mut roles: BTreeMap<String, BTreeSet<Role>> = BTreeMap::new();
        let mut validator_set = Vec::new();
        let mut essent_balances = BTreeMap::new();
        let mut essential_units = BTreeMap::new();
        let mut personhood = BTreeMap::new();

        for validator in &genesis.initial_validators {
            dids.insert(
                validator.did.clone(),
                DidRecord {
                    did: validator.did.clone(),
                    public_key: validator.public_key.clone(),
                    registered_at: 0,
                    metadata: Some(validator.moniker.clone()),
                },
            );
            roles
                .entry(validator.did.clone())
                .or_default()
                .insert(Role::Validator);
            validator_set.push(validator.did.clone());
        }

        for grant in &genesis.initial_roles {
            let entry = roles.entry(grant.did.clone()).or_default();
            for role in &grant.roles {
                entry.insert(*role);
            }
        }

        for balance in &genesis.initial_balances {
            essent_balances.insert(balance.did.clone(), balance.essent);
            essential_units.insert(balance.did.clone(), balance.essential_units.clone());
        }

        for member in &genesis.initial_personhood {
            personhood.insert(
                member.did.clone(),
                PersonhoodCredential {
                    did: member.did.clone(),
                    eligible: member.eligible,
                    nullifier_commitment: member.nullifier_commitment.clone(),
                    issued_at: 0,
                    revoked_at: (!member.eligible).then_some(0),
                },
            );
        }

        let mut budgets = BTreeMap::new();
        let mut epoch_accounting = BTreeMap::new();
        budgets.insert(genesis.initial_epoch_budget.epoch, genesis.initial_epoch_budget.clone());
        epoch_accounting.insert(genesis.initial_epoch_budget.epoch, EpochAccounting::default());

        let state = LedgerState {
            chain_id: genesis.chain_id.clone(),
            height: 0,
            dids,
            personhood,
            roles,
            validator_set,
            nonces: BTreeMap::new(),
            essent_balances,
            locked_essent: Vec::new(),
            essential_units,
            vendor_settlements: BTreeMap::new(),
            budgets,
            epoch_accounting,
            purposes: BTreeMap::new(),
            quests: BTreeMap::new(),
            claims: BTreeMap::new(),
            proposals: BTreeMap::new(),
            used_evidence_roots: BTreeSet::new(),
        };

        Self {
            genesis,
            state,
            blocks: Vec::new(),
            mempool: BTreeMap::new(),
            seen_transactions: BTreeSet::new(),
        }
    }

    pub fn latest_hash(&self) -> Result<String> {
        if let Some(block) = self.blocks.last() {
            Ok(block.block_hash.clone())
        } else {
            hash_json(&self.genesis)
        }
    }

    pub fn next_height(&self) -> BlockHeight {
        self.state.height + 1
    }

    pub fn expected_proposer(&self, height: BlockHeight) -> Option<String> {
        if self.state.validator_set.is_empty() {
            return None;
        }
        let index = ((height - 1) as usize) % self.state.validator_set.len();
        self.state.validator_set.get(index).cloned()
    }

    pub fn expected_nonce_for(&self, signer: &str) -> u64 {
        let confirmed = self.state.nonces.get(signer).copied().unwrap_or_default();
        let pending = self
            .mempool
            .values()
            .filter(|candidate| candidate.signer == signer)
            .map(|candidate| candidate.tx.nonce)
            .max()
            .unwrap_or(confirmed);
        pending + 1
    }

    pub fn state_dump(&self) -> Result<StateDump> {
        Ok(StateDump {
            chain_id: self.state.chain_id.clone(),
            height: self.state.height,
            latest_hash: self.latest_hash()?,
            state: self.state.clone(),
        })
    }

    pub fn account_view(&self, did: &str) -> AccountView {
        let roles = self
            .state
            .roles
            .get(did)
            .map(|roles| roles.iter().copied().collect())
            .unwrap_or_else(Vec::new);
        let vendor_pending = self
            .state
            .vendor_settlements
            .get(did)
            .map(|settlement| settlement.pending_units)
            .unwrap_or_default();
        AccountView {
            did: did.to_string(),
            confirmed_nonce: self.state.nonces.get(did).copied().unwrap_or_default(),
            next_nonce: self.expected_nonce_for(did),
            liquid_essent: self.state.liquid_essent(did),
            locked_essent: self.state.locked_essent_total(did),
            total_essential_units: self.state.total_essential_units(did),
            pending_vendor_units: vendor_pending,
            roles,
            personhood_eligible: self.state.has_personhood(did),
        }
    }

    pub fn submit_transaction(&mut self, tx: SignedTransaction) -> Result<()> {
        verify_signed_transaction(&tx)?;
        if tx.tx.chain_id != self.state.chain_id {
            return Err(ChainError::InvalidTransaction(format!(
                "transaction chain_id {} does not match {}",
                tx.tx.chain_id, self.state.chain_id
            ))
            .into());
        }
        if self.seen_transactions.contains(&tx.tx_hash) || self.mempool.contains_key(&tx.tx_hash) {
            return Err(ChainError::AlreadyExists(format!("transaction {}", tx.tx_hash)).into());
        }
        let expected_nonce = self.expected_nonce_for(&tx.signer);
        if tx.tx.nonce != expected_nonce {
            return Err(ChainError::NonceMismatch {
                signer: tx.signer.clone(),
                expected: expected_nonce,
                got: tx.tx.nonce,
            }
            .into());
        }
        self.validate_registry_binding(&self.state, &tx)?;
        let mut candidate = self.state.clone();
        candidate.prepare_for_block(self.next_height());
        self.apply_transaction(&mut candidate, &tx, self.next_height())?;
        self.mempool.insert(tx.tx_hash.clone(), tx);
        Ok(())
    }

    pub fn propose_block(&mut self, proposer_key: &KeyFile, max_txs: usize) -> Result<Block> {
        let height = self.next_height();
        let expected = self
            .expected_proposer(height)
            .ok_or_else(|| ChainError::Validation("validator set is empty".to_string()))?;
        if proposer_key.did != expected {
            return Err(ChainError::Unauthorized(format!(
                "{} is not the scheduled proposer for height {} (expected {})",
                proposer_key.did, height, expected
            ))
            .into());
        }

        let prev_hash = self.latest_hash()?;
        let mut candidate_state = self.state.clone();
        candidate_state.prepare_for_block(height);

        let mut ordered: Vec<_> = self.mempool.values().cloned().collect();
        ordered.sort_by(|left, right| {
            left.signer
                .cmp(&right.signer)
                .then(left.tx.nonce.cmp(&right.tx.nonce))
                .then(left.tx_hash.cmp(&right.tx_hash))
        });

        let mut accepted = Vec::new();
        let mut evict_hashes = Vec::new();

        for tx in ordered {
            if accepted.len() >= max_txs {
                break;
            }
            let result = self.apply_transaction(&mut candidate_state, &tx, height);
            match result {
                Ok(()) => {
                    evict_hashes.push(tx.tx_hash.clone());
                    accepted.push(tx);
                }
                Err(_) => {
                    evict_hashes.push(tx.tx_hash.clone());
                }
            }
        }

        candidate_state.height = height;
        let state_root = hash_json(&candidate_state)?;
        let unsigned = UnsignedBlock {
            height,
            chain_id: self.state.chain_id.clone(),
            prev_hash,
            proposer: proposer_key.did.clone(),
            timestamp_unix_ms: chrono::Utc::now().timestamp_millis(),
            txs: accepted.clone(),
            state_root,
        };
        let block = sign_block(proposer_key, unsigned)?;

        self.commit_block(block.clone(), candidate_state, &evict_hashes)?;
        Ok(block)
    }

    pub fn import_block(&mut self, block: Block) -> Result<()> {
        let expected_height = self.next_height();
        if block.height != expected_height {
            return Err(ChainError::Validation(format!(
                "block height mismatch: expected {}, got {}",
                expected_height, block.height
            ))
            .into());
        }
        if block.chain_id != self.state.chain_id {
            return Err(ChainError::Validation(format!(
                "block chain_id mismatch: expected {}, got {}",
                self.state.chain_id, block.chain_id
            ))
            .into());
        }
        let latest_hash = self.latest_hash()?;
        if block.prev_hash != latest_hash {
            return Err(ChainError::Validation(format!(
                "prev_hash mismatch: expected {}, got {}",
                latest_hash, block.prev_hash
            ))
            .into());
        }
        let expected_proposer = self
            .expected_proposer(block.height)
            .ok_or_else(|| ChainError::Validation("validator set is empty".to_string()))?;
        if block.proposer != expected_proposer {
            return Err(ChainError::Validation(format!(
                "unexpected proposer {} for height {}; expected {}",
                block.proposer, block.height, expected_proposer
            ))
            .into());
        }
        let proposer_record = self
            .state
            .dids
            .get(&block.proposer)
            .ok_or_else(|| ChainError::NotFound(format!("validator did {}", block.proposer)))?;
        verify_block_signature(&block, &proposer_record.public_key)?;

        let mut candidate_state = self.state.clone();
        candidate_state.prepare_for_block(block.height);
        for tx in &block.txs {
            verify_signed_transaction(tx)?;
            self.validate_registry_binding(&candidate_state, tx)?;
            self.apply_transaction(&mut candidate_state, tx, block.height)?;
        }
        candidate_state.height = block.height;
        let expected_root = hash_json(&candidate_state)?;
        if expected_root != block.state_root {
            return Err(ChainError::Validation(format!(
                "state root mismatch: expected {}, got {}",
                expected_root, block.state_root
            ))
            .into());
        }

        let evict_hashes: Vec<_> = block.txs.iter().map(|tx| tx.tx_hash.clone()).collect();
        self.commit_block(block, candidate_state, &evict_hashes)?;
        Ok(())
    }

    fn commit_block(
        &mut self,
        block: Block,
        candidate_state: LedgerState,
        evict_hashes: &[String],
    ) -> Result<()> {
        self.state = candidate_state;
        for tx_hash in evict_hashes {
            self.mempool.remove(tx_hash);
            self.seen_transactions.insert(tx_hash.clone());
        }
        self.blocks.push(block);
        Ok(())
    }

    fn validate_registry_binding(&self, state: &LedgerState, tx: &SignedTransaction) -> Result<()> {
        match &tx.tx.kind {
            TransactionKind::RegisterDid { .. } => {
                if state.dids.contains_key(&tx.signer) {
                    return Err(ChainError::AlreadyExists(format!("did {}", tx.signer)).into());
                }
            }
            _ => {
                let did = state
                    .dids
                    .get(&tx.signer)
                    .ok_or_else(|| ChainError::NotFound(format!("did {}", tx.signer)))?;
                if did.public_key != tx.public_key {
                    return Err(ChainError::Unauthorized(format!(
                        "public key mismatch for {}",
                        tx.signer
                    ))
                    .into());
                }
            }
        }
        Ok(())
    }

    fn apply_transaction(
        &self,
        state: &mut LedgerState,
        signed_tx: &SignedTransaction,
        block_height: BlockHeight,
    ) -> Result<()> {
        let signer = signed_tx.signer.as_str();
        let expected_nonce = state.nonces.get(signer).copied().unwrap_or_default() + 1;
        if signed_tx.tx.nonce != expected_nonce {
            return Err(ChainError::NonceMismatch {
                signer: signer.to_string(),
                expected: expected_nonce,
                got: signed_tx.tx.nonce,
            }
            .into());
        }

        match &signed_tx.tx.kind {
            TransactionKind::RegisterDid { metadata } => {
                if state.dids.contains_key(signer) {
                    return Err(ChainError::AlreadyExists(format!("did {signer}")).into());
                }
                state.dids.insert(
                    signer.to_string(),
                    DidRecord {
                        did: signer.to_string(),
                        public_key: signed_tx.public_key.clone(),
                        registered_at: block_height,
                        metadata: metadata.clone(),
                    },
                );
            }
            TransactionKind::IssuePersonhood {
                did,
                nullifier_commitment,
                eligible,
            } => {
                require_any_role(state, signer, &[Role::Validator, Role::Steward])?;
                require_registered_did(state, did)?;
                state.personhood.insert(
                    did.clone(),
                    PersonhoodCredential {
                        did: did.clone(),
                        eligible: *eligible,
                        nullifier_commitment: nullifier_commitment.clone(),
                        issued_at: block_height,
                        revoked_at: (!eligible).then_some(block_height),
                    },
                );
            }
            TransactionKind::IssueRole { did, role } => {
                require_any_role(
                    state,
                    signer,
                    &[Role::Validator, Role::Steward, Role::TreasuryOperator],
                )?;
                require_registered_did(state, did)?;
                state.roles.entry(did.clone()).or_default().insert(*role);
                if *role == Role::Validator && !state.validator_set.iter().any(|entry| entry == did) {
                    state.validator_set.push(did.clone());
                    state.validator_set.sort();
                }
            }
            TransactionKind::CreateEpochBudget {
                epoch,
                global_essent_mint_cap,
                freedom_floor_units_pool,
                vendor_redemption_pool_e,
                civic_gas_pool_e,
            } => {
                require_any_role(state, signer, &[Role::Validator, Role::TreasuryOperator])?;
                if state.budgets.contains_key(epoch) {
                    return Err(ChainError::AlreadyExists(format!("epoch budget {epoch}")).into());
                }
                state.budgets.insert(
                    *epoch,
                    EpochBudget {
                        epoch: *epoch,
                        global_essent_mint_cap: *global_essent_mint_cap,
                        freedom_floor_units_pool: *freedom_floor_units_pool,
                        vendor_redemption_pool_e: *vendor_redemption_pool_e,
                        civic_gas_pool_e: *civic_gas_pool_e,
                        created_by: signer.to_string(),
                        created_at: block_height,
                    },
                );
                state.epoch_accounting.insert(*epoch, EpochAccounting::default());
            }
            TransactionKind::CreatePurpose {
                purpose_id,
                epoch,
                name,
                description_hash,
                essent_budget,
            } => {
                require_any_role(state, signer, &[Role::Validator, Role::Steward, Role::TreasuryOperator])?;
                if state.purposes.contains_key(purpose_id) {
                    return Err(ChainError::AlreadyExists(format!("purpose {purpose_id}")).into());
                }
                let budget = state
                    .budgets
                    .get(epoch)
                    .ok_or_else(|| ChainError::NotFound(format!("epoch budget {epoch}")))?;
                let already_allocated: Amount = state
                    .purposes
                    .values()
                    .filter(|purpose| purpose.epoch == *epoch)
                    .map(|purpose| purpose.essent_budget)
                    .sum();
                if already_allocated.saturating_add(*essent_budget) > budget.global_essent_mint_cap {
                    return Err(ChainError::Validation(format!(
                        "purpose budget {} exceeds remaining epoch cap {}",
                        essent_budget,
                        budget.global_essent_mint_cap.saturating_sub(already_allocated)
                    ))
                    .into());
                }
                state.purposes.insert(
                    purpose_id.clone(),
                    crate::models::Purpose {
                        purpose_id: purpose_id.clone(),
                        epoch: *epoch,
                        name: name.clone(),
                        description_hash: description_hash.clone(),
                        essent_budget: *essent_budget,
                        spent_essent: 0,
                        created_by: signer.to_string(),
                        active: true,
                    },
                );
            }
            TransactionKind::CreateQuest {
                quest_id,
                purpose_id,
                title,
                reward_ceiling,
                challenge_window_blocks,
                audit_tail_bps,
                audit_tail_blocks,
                risk_band,
                rubric,
            } => {
                require_any_role(state, signer, &[Role::Validator, Role::Steward])?;
                if state.quests.contains_key(quest_id) {
                    return Err(ChainError::AlreadyExists(format!("quest {quest_id}")).into());
                }
                let purpose = state
                    .purposes
                    .get(purpose_id)
                    .ok_or_else(|| ChainError::NotFound(format!("purpose {purpose_id}")))?;
                if !purpose.active {
                    return Err(ChainError::Validation(format!("purpose {purpose_id} is inactive")).into());
                }
                if *reward_ceiling == 0 {
                    return Err(ChainError::Validation("reward_ceiling must be > 0".to_string()).into());
                }
                if *reward_ceiling > purpose.essent_budget {
                    return Err(ChainError::Validation(format!(
                        "quest reward ceiling {} exceeds purpose budget {}",
                        reward_ceiling, purpose.essent_budget
                    ))
                    .into());
                }
                validate_rubric(rubric)?;
                state.quests.insert(
                    quest_id.clone(),
                    Quest {
                        quest_id: quest_id.clone(),
                        purpose_id: purpose_id.clone(),
                        title: title.clone(),
                        reward_ceiling: *reward_ceiling,
                        challenge_window_blocks: *challenge_window_blocks,
                        audit_tail_bps: *audit_tail_bps,
                        audit_tail_blocks: *audit_tail_blocks,
                        risk_band: *risk_band,
                        rubric: rubric.clone(),
                        created_by: signer.to_string(),
                        active: true,
                    },
                );
            }
            TransactionKind::SubmitClaim {
                claim_id,
                quest_id,
                evidence_root,
                metadata_hash,
                claimant_bond,
            } => {
                require_registered_did(state, signer)?;
                require_personhood(state, signer)?;
                if state.claims.contains_key(claim_id) {
                    return Err(ChainError::AlreadyExists(format!("claim {claim_id}")).into());
                }
                let quest = state
                    .quests
                    .get(quest_id)
                    .ok_or_else(|| ChainError::NotFound(format!("quest {quest_id}")))?;
                if !quest.active {
                    return Err(ChainError::Validation(format!("quest {quest_id} is inactive")).into());
                }
                if state.used_evidence_roots.contains(evidence_root) {
                    return Err(ChainError::Validation(format!(
                        "evidence root {evidence_root} already used"
                    ))
                    .into());
                }
                deduct_liquid_essent(state, signer, *claimant_bond)?;
                state.locked_essent.push(LockedBalance {
                    owner: signer.to_string(),
                    amount: *claimant_bond,
                    unlock_height: None,
                    reason: LockReason::ClaimBond {
                        claim_id: claim_id.clone(),
                    },
                });
                state.used_evidence_roots.insert(evidence_root.clone());
                state.claims.insert(
                    claim_id.clone(),
                    Claim {
                        claim_id: claim_id.clone(),
                        quest_id: quest_id.clone(),
                        claimant: signer.to_string(),
                        evidence_root: evidence_root.clone(),
                        metadata_hash: metadata_hash.clone(),
                        claimant_bond: *claimant_bond,
                        submitted_at: block_height,
                        status: ClaimStatus::PendingReview,
                        reviews: BTreeMap::new(),
                        challenges: Vec::new(),
                        payout_essent: None,
                        finalized_at: None,
                    },
                );
            }
            TransactionKind::SubmitReview {
                claim_id,
                summary_hash,
                scores,
            } => {
                require_any_role(state, signer, &[Role::Attestor, Role::Validator])?;
                let (claimant, claim_status, quest_id) = {
                    let claim = state
                        .claims
                        .get(claim_id)
                        .ok_or_else(|| ChainError::NotFound(format!("claim {claim_id}")))?;
                    (claim.claimant.clone(), claim.status, claim.quest_id.clone())
                };
                if claimant == signer {
                    return Err(ChainError::Unauthorized("claimants cannot review their own claims".to_string()).into());
                }
                if claim_status == ClaimStatus::Rejected || claim_status == ClaimStatus::Finalized {
                    return Err(ChainError::Validation(format!(
                        "claim {claim_id} is not reviewable in status {:?}",
                        claim_status
                    ))
                    .into());
                }
                let quest = state
                    .quests
                    .get(&quest_id)
                    .ok_or_else(|| ChainError::NotFound(format!("quest {quest_id}")))?;
                validate_review_scores(scores, &quest.rubric)?;
                let claim = state
                    .claims
                    .get_mut(claim_id)
                    .ok_or_else(|| ChainError::NotFound(format!("claim {claim_id}")))?;
                claim.reviews.insert(
                    signer.to_string(),
                    Review {
                        reviewer: signer.to_string(),
                        submitted_at: block_height,
                        summary_hash: summary_hash.clone(),
                        scores: scores.clone(),
                    },
                );
            }
            TransactionKind::ChallengeClaim {
                claim_id,
                challenge_hash,
                bond,
            } => {
                require_personhood(state, signer)?;
                let claim_status = {
                    let claim = state
                        .claims
                        .get(claim_id)
                        .ok_or_else(|| ChainError::NotFound(format!("claim {claim_id}")))?;
                    claim.status
                };
                if claim_status == ClaimStatus::Rejected || claim_status == ClaimStatus::Finalized {
                    return Err(ChainError::Validation(format!(
                        "claim {claim_id} cannot be challenged in status {:?}",
                        claim_status
                    ))
                    .into());
                }
                deduct_liquid_essent(state, signer, *bond)?;
                state.locked_essent.push(LockedBalance {
                    owner: signer.to_string(),
                    amount: *bond,
                    unlock_height: None,
                    reason: LockReason::ChallengeBond {
                        claim_id: claim_id.clone(),
                    },
                });
                let claim = state
                    .claims
                    .get_mut(claim_id)
                    .ok_or_else(|| ChainError::NotFound(format!("claim {claim_id}")))?;
                claim.status = ClaimStatus::Challenged;
                claim.challenges.push(Challenge {
                    challenger: signer.to_string(),
                    submitted_at: block_height,
                    challenge_hash: challenge_hash.clone(),
                    bond: *bond,
                    resolved: false,
                    accepted: None,
                });
            }
            TransactionKind::ResolveChallenge {
                claim_id,
                accepted,
                note_hash: _,
            } => {
                require_any_role(state, signer, &[Role::Validator, Role::Steward])?;
                let had_unresolved = {
                    let claim = state
                        .claims
                        .get_mut(claim_id)
                        .ok_or_else(|| ChainError::NotFound(format!("claim {claim_id}")))?;
                    let mut had_unresolved = false;
                    for challenge in &mut claim.challenges {
                        if !challenge.resolved {
                            challenge.resolved = true;
                            challenge.accepted = Some(*accepted);
                            had_unresolved = true;
                        }
                    }
                    had_unresolved
                };
                if !had_unresolved {
                    return Err(ChainError::Validation(format!(
                        "claim {claim_id} has no unresolved challenges"
                    ))
                    .into());
                }
                if *accepted {
                    slash_claim_bond(state, claim_id)?;
                    refund_challenge_bonds(state, claim_id)?;
                    let claim = state
                        .claims
                        .get_mut(claim_id)
                        .ok_or_else(|| ChainError::NotFound(format!("claim {claim_id}")))?;
                    claim.status = ClaimStatus::Rejected;
                } else {
                    slash_challenge_bonds(state, claim_id)?;
                    let claim = state
                        .claims
                        .get_mut(claim_id)
                        .ok_or_else(|| ChainError::NotFound(format!("claim {claim_id}")))?;
                    claim.status = ClaimStatus::PendingReview;
                }
            }
            TransactionKind::FinalizeClaim { claim_id } => {
                let (claimant, quest_id, submitted_at, status, review_count, unresolved_challenges) = {
                    let claim = state
                        .claims
                        .get(claim_id)
                        .ok_or_else(|| ChainError::NotFound(format!("claim {claim_id}")))?;
                    (
                        claim.claimant.clone(),
                        claim.quest_id.clone(),
                        claim.submitted_at,
                        claim.status,
                        claim.reviews.len(),
                        claim.challenges.iter().any(|challenge| !challenge.resolved),
                    )
                };
                if status == ClaimStatus::Rejected || status == ClaimStatus::Finalized {
                    return Err(ChainError::Validation(format!(
                        "claim {claim_id} is not finalizable in status {:?}",
                        status
                    ))
                    .into());
                }
                if unresolved_challenges {
                    return Err(ChainError::Validation(format!(
                        "claim {claim_id} has unresolved challenges"
                    ))
                    .into());
                }
                let quest = state
                    .quests
                    .get(&quest_id)
                    .ok_or_else(|| ChainError::NotFound(format!("quest {quest_id}")))?
                    .clone();
                if block_height < submitted_at.saturating_add(quest.challenge_window_blocks) {
                    return Err(ChainError::Validation(format!(
                        "claim {claim_id} challenge window is still open"
                    ))
                    .into());
                }
                let minimum_reviews = minimum_reviews(quest.risk_band);
                if review_count < minimum_reviews {
                    return Err(ChainError::Validation(format!(
                        "claim {claim_id} has {} review(s); {} required",
                        review_count, minimum_reviews
                    ))
                    .into());
                }
                let score_bps = compute_claim_score_bps(state, claim_id, &quest)?;
                let purpose = state
                    .purposes
                    .get(&quest.purpose_id)
                    .ok_or_else(|| ChainError::NotFound(format!("purpose {}", quest.purpose_id)))?
                    .clone();
                let budget = state
                    .budgets
                    .get(&purpose.epoch)
                    .ok_or_else(|| ChainError::NotFound(format!("epoch budget {}", purpose.epoch)))?
                    .clone();
                let accounting = state
                    .epoch_accounting
                    .get(&purpose.epoch)
                    .cloned()
                    .unwrap_or_default();
                let raw_payout = mul_bps(quest.reward_ceiling, score_bps)?;
                let purpose_remaining = purpose.essent_budget.saturating_sub(purpose.spent_essent);
                let epoch_remaining = budget
                    .global_essent_mint_cap
                    .saturating_sub(accounting.contribution_minted_e);
                let payout = raw_payout.min(purpose_remaining).min(epoch_remaining);
                let audit_tail_amount = mul_bps(payout, quest.audit_tail_bps as u64)?;
                let liquid_amount = payout.saturating_sub(audit_tail_amount);
                let unlock_height = block_height.saturating_add(quest.audit_tail_blocks);

                *state.essent_balances.entry(claimant.clone()).or_default() += liquid_amount;
                if audit_tail_amount > 0 {
                    state.locked_essent.push(LockedBalance {
                        owner: claimant.clone(),
                        amount: audit_tail_amount,
                        unlock_height: Some(unlock_height),
                        reason: LockReason::AuditTail {
                            claim_id: claim_id.clone(),
                        },
                    });
                }
                unlock_claim_bond_at(state, claim_id, unlock_height)?;

                state
                    .epoch_accounting
                    .entry(purpose.epoch)
                    .or_default()
                    .contribution_minted_e = accounting
                    .contribution_minted_e
                    .saturating_add(payout);
                if let Some(purpose_entry) = state.purposes.get_mut(&purpose.purpose_id) {
                    purpose_entry.spent_essent = purpose_entry.spent_essent.saturating_add(payout);
                }
                if let Some(claim) = state.claims.get_mut(claim_id) {
                    claim.status = ClaimStatus::Finalized;
                    claim.payout_essent = Some(payout);
                    claim.finalized_at = Some(block_height);
                }
            }
            TransactionKind::TransferEssent { to, amount, memo: _ } => {
                require_registered_did(state, signer)?;
                require_registered_did(state, to)?;
                deduct_liquid_essent(state, signer, *amount)?;
                *state.essent_balances.entry(to.clone()).or_default() += *amount;
            }
            TransactionKind::IssueFreedomFloor {
                to,
                epoch,
                amount,
                expires_at,
            } => {
                require_any_role(state, signer, &[Role::Validator, Role::TreasuryOperator])?;
                require_registered_did(state, to)?;
                let budget = state
                    .budgets
                    .get(epoch)
                    .ok_or_else(|| ChainError::NotFound(format!("epoch budget {epoch}")))?
                    .clone();
                let accounting = state
                    .epoch_accounting
                    .get(epoch)
                    .cloned()
                    .unwrap_or_default();
                let remaining = budget
                    .freedom_floor_units_pool
                    .saturating_sub(accounting.freedom_floor_issued_u);
                if *amount > remaining {
                    return Err(ChainError::Validation(format!(
                        "requested {} {} exceeds remaining epoch pool {}",
                        amount,
                        crate::models::ESSENTIAL_UNITS_SYMBOL,
                        remaining
                    ))
                    .into());
                }
                state
                    .essential_units
                    .entry(to.clone())
                    .or_default()
                    .push(EssentialUnitLot {
                        amount: *amount,
                        expires_at: *expires_at,
                        source_epoch: *epoch,
                    });
                state
                    .epoch_accounting
                    .entry(*epoch)
                    .or_default()
                    .freedom_floor_issued_u = accounting.freedom_floor_issued_u.saturating_add(*amount);
            }
            TransactionKind::SpendEssentialUnits { vendor, amount, memo: _ } => {
                require_registered_did(state, signer)?;
                require_registered_did(state, vendor)?;
                if !state.has_role(vendor, Role::Vendor) {
                    return Err(ChainError::Unauthorized(format!("{vendor} is not a vendor"))
                        .into());
                }
                spend_essential_units(state, signer, *amount)?;
                let settlement = state
                    .vendor_settlements
                    .entry(vendor.clone())
                    .or_insert_with(|| VendorSettlement {
                        vendor: vendor.clone(),
                        pending_units: 0,
                        redeemed_essent: 0,
                    });
                settlement.pending_units = settlement.pending_units.saturating_add(*amount);
            }
            TransactionKind::RedeemVendorSettlement { epoch, amount } => {
                require_registered_did(state, signer)?;
                if !state.has_role(signer, Role::Vendor) {
                    return Err(ChainError::Unauthorized(format!("{signer} is not a vendor")).into());
                }
                let budget = state
                    .budgets
                    .get(epoch)
                    .ok_or_else(|| ChainError::NotFound(format!("epoch budget {epoch}")))?
                    .clone();
                let accounting = state
                    .epoch_accounting
                    .get(epoch)
                    .cloned()
                    .unwrap_or_default();
                let settlement = state
                    .vendor_settlements
                    .entry(signer.to_string())
                    .or_insert_with(|| VendorSettlement {
                        vendor: signer.to_string(),
                        pending_units: 0,
                        redeemed_essent: 0,
                    });
                if *amount > settlement.pending_units {
                    return Err(ChainError::Validation(format!(
                        "requested redemption {} exceeds pending units {}",
                        amount, settlement.pending_units
                    ))
                    .into());
                }
                let remaining_pool = budget
                    .vendor_redemption_pool_e
                    .saturating_sub(accounting.vendor_redemptions_e);
                if *amount > remaining_pool {
                    return Err(ChainError::Validation(format!(
                        "requested redemption {} exceeds remaining vendor pool {}",
                        amount, remaining_pool
                    ))
                    .into());
                }
                settlement.pending_units = settlement.pending_units.saturating_sub(*amount);
                settlement.redeemed_essent = settlement.redeemed_essent.saturating_add(*amount);
                *state.essent_balances.entry(signer.to_string()).or_default() += *amount;
                state
                    .epoch_accounting
                    .entry(*epoch)
                    .or_default()
                    .vendor_redemptions_e = accounting.vendor_redemptions_e.saturating_add(*amount);
            }
            TransactionKind::CreateProposal {
                proposal_id,
                title,
                body_hash,
                kind,
                opens_at,
                closes_at,
            } => {
                require_personhood(state, signer)?;
                if state.proposals.contains_key(proposal_id) {
                    return Err(ChainError::AlreadyExists(format!("proposal {proposal_id}")).into());
                }
                if *closes_at <= *opens_at {
                    return Err(ChainError::Validation(
                        "proposal closes_at must be greater than opens_at".to_string(),
                    )
                    .into());
                }
                state.proposals.insert(
                    proposal_id.clone(),
                    Proposal {
                        proposal_id: proposal_id.clone(),
                        title: title.clone(),
                        body_hash: body_hash.clone(),
                        kind: *kind,
                        opens_at: *opens_at,
                        closes_at: *closes_at,
                        created_by: signer.to_string(),
                        status: ProposalStatus::Open,
                        ballots: BTreeMap::new(),
                        result: None,
                    },
                );
            }
            TransactionKind::CastVote {
                proposal_id,
                seq,
                yes,
            } => {
                require_personhood(state, signer)?;
                let proposal = state
                    .proposals
                    .get_mut(proposal_id)
                    .ok_or_else(|| ChainError::NotFound(format!("proposal {proposal_id}")))?;
                if proposal.status != ProposalStatus::Open {
                    return Err(ChainError::Validation(format!(
                        "proposal {proposal_id} is not open"
                    ))
                    .into());
                }
                if block_height < proposal.opens_at || block_height > proposal.closes_at {
                    return Err(ChainError::Validation(format!(
                        "proposal {proposal_id} is not accepting votes at height {block_height}"
                    ))
                    .into());
                }
                if let Some(existing) = proposal.ballots.get(signer) {
                    if *seq <= existing.seq {
                        return Err(ChainError::Validation(format!(
                            "vote seq {} must be greater than existing seq {}",
                            seq, existing.seq
                        ))
                        .into());
                    }
                }
                proposal.ballots.insert(
                    signer.to_string(),
                    crate::models::Ballot {
                        voter: signer.to_string(),
                        seq: *seq,
                        yes: *yes,
                        submitted_at: block_height,
                    },
                );
            }
            TransactionKind::TallyProposal { proposal_id } => {
                let proposal = state
                    .proposals
                    .get_mut(proposal_id)
                    .ok_or_else(|| ChainError::NotFound(format!("proposal {proposal_id}")))?;
                if proposal.status == ProposalStatus::Tallied {
                    return Err(ChainError::AlreadyExists(format!("proposal result {proposal_id}"))
                        .into());
                }
                if block_height <= proposal.closes_at {
                    return Err(ChainError::Validation(format!(
                        "proposal {proposal_id} cannot be tallied before close"
                    ))
                    .into());
                }
                let yes_votes = proposal.ballots.values().filter(|ballot| ballot.yes).count() as u64;
                let no_votes = proposal.ballots.len() as u64 - yes_votes;
                let turnout = proposal.ballots.len() as u64;
                proposal.result = Some(ProposalResult {
                    yes: yes_votes,
                    no: no_votes,
                    turnout,
                    passed: yes_votes > no_votes,
                });
                proposal.status = ProposalStatus::Tallied;
            }
        }

        state.nonces.insert(signer.to_string(), signed_tx.tx.nonce);
        Ok(())
    }
}

fn require_registered_did(state: &LedgerState, did: &str) -> Result<()> {
    if state.dids.contains_key(did) {
        Ok(())
    } else {
        Err(ChainError::NotFound(format!("did {did}")).into())
    }
}

fn require_personhood(state: &LedgerState, did: &str) -> Result<()> {
    if state.has_personhood(did) {
        Ok(())
    } else {
        Err(ChainError::Unauthorized(format!(
            "{did} does not have an eligible personhood credential"
        ))
        .into())
    }
}

fn require_any_role(state: &LedgerState, did: &str, roles: &[Role]) -> Result<()> {
    if roles.iter().any(|role| state.has_role(did, *role)) {
        Ok(())
    } else {
        Err(ChainError::Unauthorized(format!("{did} lacks required role"))
            .into())
    }
}

fn deduct_liquid_essent(state: &mut LedgerState, did: &str, amount: Amount) -> Result<()> {
    let available = state.liquid_essent(did);
    if available < amount {
        return Err(ChainError::InsufficientBalance {
            owner: did.to_string(),
            asset: crate::models::ESSENT_TICKER.to_string(),
            available,
            required: amount,
        }
        .into());
    }
    let entry = state.essent_balances.entry(did.to_string()).or_default();
    *entry = entry.saturating_sub(amount);
    Ok(())
}

fn spend_essential_units(state: &mut LedgerState, did: &str, amount: Amount) -> Result<()> {
    let total = state.total_essential_units(did);
    if total < amount {
        return Err(ChainError::InsufficientBalance {
            owner: did.to_string(),
            asset: crate::models::ESSENTIAL_UNITS_TICKER.to_string(),
            available: total,
            required: amount,
        }
        .into());
    }
    let lots = state.essential_units.entry(did.to_string()).or_default();
    let mut remaining = amount;
    lots.sort_by_key(|lot| lot.expires_at);
    for lot in lots.iter_mut() {
        if remaining == 0 {
            break;
        }
        let take = remaining.min(lot.amount);
        lot.amount = lot.amount.saturating_sub(take);
        remaining = remaining.saturating_sub(take);
    }
    lots.retain(|lot| lot.amount > 0);
    Ok(())
}

fn validate_rubric(rubric: &[crate::models::RubricDimension]) -> Result<()> {
    if rubric.is_empty() {
        return Err(ChainError::Validation("rubric must not be empty".to_string()).into());
    }
    let mut names = BTreeSet::new();
    let mut total = 0u64;
    for dimension in rubric {
        if !names.insert(dimension.name.clone()) {
            return Err(ChainError::Validation(format!(
                "duplicate rubric dimension {}",
                dimension.name
            ))
            .into());
        }
        total = total.saturating_add(dimension.weight_bps as u64);
    }
    if total != crate::models::BASIS_POINTS_SCALE {
        return Err(ChainError::Validation(format!(
            "rubric weights must sum to {}; got {}",
            crate::models::BASIS_POINTS_SCALE,
            total
        ))
        .into());
    }
    Ok(())
}

fn validate_review_scores(scores: &[ReviewScore], rubric: &[crate::models::RubricDimension]) -> Result<()> {
    let rubric_names: BTreeSet<_> = rubric.iter().map(|dimension| dimension.name.clone()).collect();
    let score_names: BTreeSet<_> = scores.iter().map(|score| score.dimension.clone()).collect();
    if rubric_names != score_names {
        return Err(ChainError::Validation(
            "review scores must exactly match rubric dimensions".to_string(),
        )
        .into());
    }
    for score in scores {
        if score.score_bps as u64 > crate::models::BASIS_POINTS_SCALE {
            return Err(ChainError::Validation(format!(
                "score {} for {} exceeds basis point scale",
                score.score_bps, score.dimension
            ))
            .into());
        }
    }
    Ok(())
}

fn minimum_reviews(risk_band: RiskBand) -> usize {
    match risk_band {
        RiskBand::Low => 3,
        RiskBand::Medium => 5,
        RiskBand::High => 7,
    }
}

fn compute_claim_score_bps(state: &LedgerState, claim_id: &str, quest: &Quest) -> Result<u64> {
    let claim = state
        .claims
        .get(claim_id)
        .ok_or_else(|| ChainError::NotFound(format!("claim {claim_id}")))?;
    let mut total = 0u64;
    for dimension in &quest.rubric {
        let mut values = Vec::new();
        for review in claim.reviews.values() {
            let score = review
                .scores
                .iter()
                .find(|score| score.dimension == dimension.name)
                .ok_or_else(|| {
                    ChainError::Validation(format!(
                        "review {} missing dimension {}",
                        review.reviewer, dimension.name
                    ))
                })?;
            values.push(score.score_bps as u64);
        }
        values.sort_unstable();
        let median = median(&values)?;
        total = total.saturating_add((dimension.weight_bps as u64).saturating_mul(median) / crate::models::BASIS_POINTS_SCALE);
    }
    Ok(total.min(crate::models::BASIS_POINTS_SCALE))
}

fn median(values: &[u64]) -> Result<u64> {
    if values.is_empty() {
        return Err(ChainError::Validation("median of empty slice".to_string()).into());
    }
    let mid = values.len() / 2;
    if values.len() % 2 == 1 {
        Ok(values[mid])
    } else {
        Ok((values[mid - 1] + values[mid]) / 2)
    }
}

fn mul_bps(amount: Amount, bps: u64) -> Result<Amount> {
    let widened = (amount as u128)
        .saturating_mul(bps as u128)
        .checked_div(crate::models::BASIS_POINTS_SCALE as u128)
        .ok_or_else(|| ChainError::Validation("division by zero in mul_bps".to_string()))?;
    Ok(widened as Amount)
}

fn unlock_claim_bond_at(state: &mut LedgerState, claim_id: &str, unlock_height: BlockHeight) -> Result<()> {
    let mut found = false;
    for lock in &mut state.locked_essent {
        if let LockReason::ClaimBond { claim_id: current } = &lock.reason {
            if current == claim_id {
                lock.unlock_height = Some(unlock_height);
                found = true;
            }
        }
    }
    if found {
        Ok(())
    } else {
        Err(ChainError::NotFound(format!("claim bond lock {claim_id}")).into())
    }
}

fn slash_claim_bond(state: &mut LedgerState, claim_id: &str) -> Result<()> {
    let before = state.locked_essent.len();
    state.locked_essent.retain(|lock| {
        !matches!(&lock.reason, LockReason::ClaimBond { claim_id: current } if current == claim_id)
    });
    if state.locked_essent.len() == before {
        return Err(ChainError::NotFound(format!("claim bond lock {claim_id}")).into());
    }
    Ok(())
}

fn refund_challenge_bonds(state: &mut LedgerState, claim_id: &str) -> Result<()> {
    let mut refunded = Vec::new();
    state.locked_essent.retain(|lock| {
        if matches!(&lock.reason, LockReason::ChallengeBond { claim_id: current } if current == claim_id) {
            refunded.push((lock.owner.clone(), lock.amount));
            false
        } else {
            true
        }
    });
    if refunded.is_empty() {
        return Err(ChainError::NotFound(format!("challenge bond lock {claim_id}")).into());
    }
    for (owner, amount) in refunded {
        *state.essent_balances.entry(owner).or_default() += amount;
    }
    Ok(())
}

fn slash_challenge_bonds(state: &mut LedgerState, claim_id: &str) -> Result<()> {
    let before = state.locked_essent.len();
    state.locked_essent.retain(|lock| {
        !matches!(&lock.reason, LockReason::ChallengeBond { claim_id: current } if current == claim_id)
    });
    if state.locked_essent.len() == before {
        return Err(ChainError::NotFound(format!("challenge bond lock {claim_id}")).into());
    }
    Ok(())
}

pub fn build_status(
    chain: &Chain,
    node_id: &str,
    public_url: &str,
    peers: &[String],
) -> Result<NodeStatus> {
    Ok(NodeStatus {
        node_id: node_id.to_string(),
        public_url: public_url.to_string(),
        chain_id: chain.state.chain_id.clone(),
        height: chain.state.height,
        latest_hash: chain.latest_hash()?,
        validator_count: chain.state.validator_set.len(),
        mempool_size: chain.mempool.len(),
        peers: peers.to_vec(),
        expected_next_proposer: chain.expected_proposer(chain.next_height()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ReviewScore, RubricDimension};

    #[test]
    fn median_even_rounds_down() {
        let values = vec![1000, 3000, 7000, 9000];
        assert_eq!(median(&values).unwrap(), 5000);
    }

    #[test]
    fn mul_bps_scales_amount() {
        assert_eq!(mul_bps(50_000, 2_000).unwrap(), 10_000);
    }

    #[test]
    fn rubric_validation_requires_exact_scale() {
        let rubric = vec![
            RubricDimension {
                name: "impact".to_string(),
                weight_bps: 6_000,
            },
            RubricDimension {
                name: "quality".to_string(),
                weight_bps: 3_000,
            },
            RubricDimension {
                name: "docs".to_string(),
                weight_bps: 1_000,
            },
        ];
        assert!(validate_rubric(&rubric).is_ok());
        let invalid = vec![
            ReviewScore {
                dimension: "impact".to_string(),
                score_bps: 9_000,
            },
            ReviewScore {
                dimension: "quality".to_string(),
                score_bps: 8_500,
            },
        ];
        assert!(validate_review_scores(&invalid, &rubric).is_err());
    }
}
