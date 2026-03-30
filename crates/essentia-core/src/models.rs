use std::collections::{BTreeMap, BTreeSet};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

pub type Amount = u64;
pub type BlockHeight = u64;
pub type Epoch = u64;
pub type BasisPoints = u16;
pub type Did = String;

pub const BASIS_POINTS_SCALE: u64 = 10_000;
pub const ESSENT_TICKER: &str = "ESSENT";
pub const ESSENT_SYMBOL: &str = "ℰ";
pub const ESSENTIAL_UNITS_TICKER: &str = "ESSENTIAL_UNITS";
pub const ESSENTIAL_UNITS_SYMBOL: &str = "𝒰";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyFile {
    pub algorithm: String,
    pub did: Did,
    pub public_key: String,
    pub secret_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DidRecord {
    pub did: Did,
    pub public_key: String,
    pub registered_at: BlockHeight,
    pub metadata: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonhoodCredential {
    pub did: Did,
    pub eligible: bool,
    pub nullifier_commitment: String,
    pub issued_at: BlockHeight,
    pub revoked_at: Option<BlockHeight>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Validator,
    Attestor,
    Steward,
    TreasuryOperator,
    Vendor,
}

impl FromStr for Role {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "validator" => Ok(Self::Validator),
            "attestor" => Ok(Self::Attestor),
            "steward" => Ok(Self::Steward),
            "treasury_operator" | "treasury-operator" | "treasuryoperator" => Ok(Self::TreasuryOperator),
            "vendor" => Ok(Self::Vendor),
            other => Err(format!("unknown role: {other}")),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RiskBand {
    Low,
    Medium,
    High,
}

impl FromStr for RiskBand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            other => Err(format!("unknown risk band: {other}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpochBudget {
    pub epoch: Epoch,
    pub global_essent_mint_cap: Amount,
    pub freedom_floor_units_pool: Amount,
    pub vendor_redemption_pool_e: Amount,
    pub civic_gas_pool_e: Amount,
    pub created_by: Did,
    pub created_at: BlockHeight,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EpochAccounting {
    pub contribution_minted_e: Amount,
    pub freedom_floor_issued_u: Amount,
    pub vendor_redemptions_e: Amount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Purpose {
    pub purpose_id: String,
    pub epoch: Epoch,
    pub name: String,
    pub description_hash: String,
    pub essent_budget: Amount,
    pub spent_essent: Amount,
    pub created_by: Did,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricDimension {
    pub name: String,
    pub weight_bps: BasisPoints,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quest {
    pub quest_id: String,
    pub purpose_id: String,
    pub title: String,
    pub reward_ceiling: Amount,
    pub challenge_window_blocks: BlockHeight,
    pub audit_tail_bps: BasisPoints,
    pub audit_tail_blocks: BlockHeight,
    pub risk_band: RiskBand,
    pub rubric: Vec<RubricDimension>,
    pub created_by: Did,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewScore {
    pub dimension: String,
    pub score_bps: BasisPoints,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub reviewer: Did,
    pub submitted_at: BlockHeight,
    pub summary_hash: String,
    pub scores: Vec<ReviewScore>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Challenge {
    pub challenger: Did,
    pub submitted_at: BlockHeight,
    pub challenge_hash: String,
    pub bond: Amount,
    pub resolved: bool,
    pub accepted: Option<bool>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ClaimStatus {
    PendingReview,
    Challenged,
    Finalized,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claim {
    pub claim_id: String,
    pub quest_id: String,
    pub claimant: Did,
    pub evidence_root: String,
    pub metadata_hash: String,
    pub claimant_bond: Amount,
    pub submitted_at: BlockHeight,
    pub status: ClaimStatus,
    pub reviews: BTreeMap<Did, Review>,
    pub challenges: Vec<Challenge>,
    pub payout_essent: Option<Amount>,
    pub finalized_at: Option<BlockHeight>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProposalKind {
    PublicSignal,
    BudgetSignal,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProposalStatus {
    Open,
    Tallied,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ballot {
    pub voter: Did,
    pub seq: u64,
    pub yes: bool,
    pub submitted_at: BlockHeight,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalResult {
    pub yes: u64,
    pub no: u64,
    pub turnout: u64,
    pub passed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub proposal_id: String,
    pub title: String,
    pub body_hash: String,
    pub kind: ProposalKind,
    pub opens_at: BlockHeight,
    pub closes_at: BlockHeight,
    pub created_by: Did,
    pub status: ProposalStatus,
    pub ballots: BTreeMap<Did, Ballot>,
    pub result: Option<ProposalResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EssentialUnitLot {
    pub amount: Amount,
    pub expires_at: BlockHeight,
    pub source_epoch: Epoch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum LockReason {
    ClaimBond { claim_id: String },
    ChallengeBond { claim_id: String },
    AuditTail { claim_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockedBalance {
    pub owner: Did,
    pub amount: Amount,
    pub unlock_height: Option<BlockHeight>,
    pub reason: LockReason,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VendorSettlement {
    pub vendor: Did,
    pub pending_units: Amount,
    pub redeemed_essent: Amount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerState {
    pub chain_id: String,
    pub height: BlockHeight,
    pub dids: BTreeMap<Did, DidRecord>,
    pub personhood: BTreeMap<Did, PersonhoodCredential>,
    pub roles: BTreeMap<Did, BTreeSet<Role>>,
    pub validator_set: Vec<Did>,
    pub nonces: BTreeMap<Did, u64>,
    pub essent_balances: BTreeMap<Did, Amount>,
    pub locked_essent: Vec<LockedBalance>,
    pub essential_units: BTreeMap<Did, Vec<EssentialUnitLot>>,
    pub vendor_settlements: BTreeMap<Did, VendorSettlement>,
    pub budgets: BTreeMap<Epoch, EpochBudget>,
    pub epoch_accounting: BTreeMap<Epoch, EpochAccounting>,
    pub purposes: BTreeMap<String, Purpose>,
    pub quests: BTreeMap<String, Quest>,
    pub claims: BTreeMap<String, Claim>,
    pub proposals: BTreeMap<String, Proposal>,
    pub used_evidence_roots: BTreeSet<String>,
}

impl LedgerState {
    pub fn liquid_essent(&self, did: &str) -> Amount {
        self.essent_balances.get(did).copied().unwrap_or_default()
    }

    pub fn locked_essent_total(&self, did: &str) -> Amount {
        self.locked_essent
            .iter()
            .filter(|lock| lock.owner == did)
            .map(|lock| lock.amount)
            .sum()
    }

    pub fn total_essential_units(&self, did: &str) -> Amount {
        self.essential_units
            .get(did)
            .map(|lots| lots.iter().map(|lot| lot.amount).sum())
            .unwrap_or_default()
    }

    pub fn has_role(&self, did: &str, role: Role) -> bool {
        self.roles
            .get(did)
            .map(|roles| roles.contains(&role))
            .unwrap_or(false)
    }

    pub fn is_validator(&self, did: &str) -> bool {
        self.validator_set.iter().any(|entry| entry == did)
    }

    pub fn has_personhood(&self, did: &str) -> bool {
        self.personhood
            .get(did)
            .map(|credential| credential.eligible && credential.revoked_at.is_none())
            .unwrap_or(false)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorGenesis {
    pub did: Did,
    pub public_key: String,
    pub moniker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisBalance {
    pub did: Did,
    pub essent: Amount,
    pub essential_units: Vec<EssentialUnitLot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisRoleGrant {
    pub did: Did,
    pub roles: Vec<Role>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisPersonhood {
    pub did: Did,
    pub eligible: bool,
    pub nullifier_commitment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisConfig {
    pub chain_id: String,
    pub bootstrap_admin: Did,
    pub initial_validators: Vec<ValidatorGenesis>,
    pub initial_roles: Vec<GenesisRoleGrant>,
    pub initial_balances: Vec<GenesisBalance>,
    pub initial_personhood: Vec<GenesisPersonhood>,
    pub initial_epoch_budget: EpochBudget,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub chain_id: String,
    pub nonce: u64,
    pub created_at_unix_ms: i64,
    pub kind: TransactionKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TransactionKind {
    RegisterDid {
        metadata: Option<String>,
    },
    IssuePersonhood {
        did: Did,
        nullifier_commitment: String,
        eligible: bool,
    },
    IssueRole {
        did: Did,
        role: Role,
    },
    CreateEpochBudget {
        epoch: Epoch,
        global_essent_mint_cap: Amount,
        freedom_floor_units_pool: Amount,
        vendor_redemption_pool_e: Amount,
        civic_gas_pool_e: Amount,
    },
    CreatePurpose {
        purpose_id: String,
        epoch: Epoch,
        name: String,
        description_hash: String,
        essent_budget: Amount,
    },
    CreateQuest {
        quest_id: String,
        purpose_id: String,
        title: String,
        reward_ceiling: Amount,
        challenge_window_blocks: BlockHeight,
        audit_tail_bps: BasisPoints,
        audit_tail_blocks: BlockHeight,
        risk_band: RiskBand,
        rubric: Vec<RubricDimension>,
    },
    SubmitClaim {
        claim_id: String,
        quest_id: String,
        evidence_root: String,
        metadata_hash: String,
        claimant_bond: Amount,
    },
    SubmitReview {
        claim_id: String,
        summary_hash: String,
        scores: Vec<ReviewScore>,
    },
    ChallengeClaim {
        claim_id: String,
        challenge_hash: String,
        bond: Amount,
    },
    ResolveChallenge {
        claim_id: String,
        accepted: bool,
        note_hash: String,
    },
    FinalizeClaim {
        claim_id: String,
    },
    TransferEssent {
        to: Did,
        amount: Amount,
        memo: Option<String>,
    },
    IssueFreedomFloor {
        to: Did,
        epoch: Epoch,
        amount: Amount,
        expires_at: BlockHeight,
    },
    SpendEssentialUnits {
        vendor: Did,
        amount: Amount,
        memo: Option<String>,
    },
    RedeemVendorSettlement {
        epoch: Epoch,
        amount: Amount,
    },
    CreateProposal {
        proposal_id: String,
        title: String,
        body_hash: String,
        kind: ProposalKind,
        opens_at: BlockHeight,
        closes_at: BlockHeight,
    },
    CastVote {
        proposal_id: String,
        seq: u64,
        yes: bool,
    },
    TallyProposal {
        proposal_id: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedTransaction {
    pub tx: Transaction,
    pub signer: Did,
    pub public_key: String,
    pub signature: String,
    pub tx_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsignedBlock {
    pub height: BlockHeight,
    pub chain_id: String,
    pub prev_hash: String,
    pub proposer: Did,
    pub timestamp_unix_ms: i64,
    pub txs: Vec<SignedTransaction>,
    pub state_root: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub height: BlockHeight,
    pub chain_id: String,
    pub prev_hash: String,
    pub proposer: Did,
    pub timestamp_unix_ms: i64,
    pub txs: Vec<SignedTransaction>,
    pub state_root: String,
    pub block_hash: String,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chain {
    pub genesis: GenesisConfig,
    pub state: LedgerState,
    pub blocks: Vec<Block>,
    pub mempool: BTreeMap<String, SignedTransaction>,
    pub seen_transactions: BTreeSet<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateDump {
    pub chain_id: String,
    pub height: BlockHeight,
    pub latest_hash: String,
    pub state: LedgerState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountView {
    pub did: Did,
    pub confirmed_nonce: u64,
    pub next_nonce: u64,
    pub liquid_essent: Amount,
    pub locked_essent: Amount,
    pub total_essential_units: Amount,
    pub pending_vendor_units: Amount,
    pub roles: Vec<Role>,
    pub personhood_eligible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeStatus {
    pub node_id: String,
    pub public_url: String,
    pub chain_id: String,
    pub height: BlockHeight,
    pub latest_hash: String,
    pub validator_count: usize,
    pub mempool_size: usize,
    pub peers: Vec<String>,
    pub expected_next_proposer: Option<Did>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitTxRequest {
    pub tx: SignedTransaction,
    pub propagate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitTxResponse {
    pub accepted: bool,
    pub tx_hash: String,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportBlockRequest {
    pub block: Block,
    pub propagate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposeBlockResponse {
    pub proposed: bool,
    pub height: Option<BlockHeight>,
    pub block_hash: Option<String>,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlocksQuery {
    pub from: Option<BlockHeight>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerRegistration {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoProposeConfig {
    pub enabled: bool,
    pub interval_ms: u64,
    pub max_txs: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    pub node_id: String,
    pub listen_addr: String,
    pub public_url: String,
    pub data_dir: String,
    pub genesis_file: String,
    pub validator_key_file: Option<String>,
    pub peers: Vec<String>,
    pub auto_propose: Option<AutoProposeConfig>,
}
