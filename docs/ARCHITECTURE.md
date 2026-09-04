# Architecture

## Status

This document describes the runnable Essentia v0.1.0 prototype.

It does not describe the current target architecture. The target is the [v0.8.0 Research Architecture](whitepaper.md), which is derived from Part B of the Society of Renewal Founding Book. The prototype should be preserved as a test fixture and source of implementation lessons, not incrementally mistaken for the production design.

In particular, v0.1.0 implements direct Essential Unit settlement into Essent from an epoch pool. The governing v0.8.0 architecture instead requires Essential Settlement Receivables, coverage accounting, a provider settlement waterfall, mutual credit, reserves, procurement, and bounded conversion. The v0.1.0 implementation is a test harness, not a real Freedom Floor or economically sufficient UBI mechanism.

## 1. Scope

Essentia v0.1.0 is runnable research software for a narrow civic-ledger test harness:

- nodes talk over HTTP;
- blocks are signed and replicated;
- the state machine enforces prototype mint boundaries;
- the client can create keys, inspect state, and submit transactions.

It is deliberately narrow and not production infrastructure.

## 2. Node model

Each node runs:

- an HTTP API;
- a local mempool;
- a full chain snapshot;
- deterministic block execution;
- background peer sync;
- optional auto-propose.

The proposer for height `h` is:

```text
validator_set[(h - 1) % validator_set.len()]
```

This gives a deterministic schedule without real Byzantine-fault-tolerant consensus.

## 3. State model

The prototype chain is account/object based.

### Core registries

- DID registry
- personhood credentials
- role grants
- epoch budgets
- purposes
- quests
- claims
- proposals

### Value stores

- liquid `ℰ` balances
- locked `ℰ` balances
- expiring `𝒰` lots
- vendor settlement queues

These names correspond to prototype objects. They do not establish that the underlying economic claims are viable.

## 4. Mint safety inside the prototype

The prototype enforces one useful accounting constraint: reviewer reputation or influence does not scale issuance.

A claim payout is bounded by three ceilings:

```text
raw = quest_reward_ceiling * rubric_weighted_median_score
payout = min(raw, remaining_purpose_budget, remaining_epoch_mint_cap)
```

Reviewer identity affects whether a review is accepted. It does not affect the payout multiplier.

This protects an internal budget rule. It does not demonstrate that minted ℰ has stable purchasing power or that a contribution payout is economically funded.

## 5. Claim lifecycle

1. A quest exists under a funded prototype purpose.
2. A claimant posts a claim with a bond.
3. Attestors submit rubric scores.
4. Members may challenge with a bond.
5. A steward or validator resolves challenges.
6. Finalization checks:
   - challenge window elapsed;
   - minimum review count reached;
   - no unresolved challenge;
   - epoch cap remaining;
   - purpose budget remaining.
7. Finalization mints liquid ℰ plus an audit-tail lock.
8. The claim bond unlocks only after the audit-tail horizon.

The lifecycle remains useful for testing workflow and fraud surfaces. v0.8.0 does not assume that arbitrary outcome scores should authorize monetary issuance. Future experiments must distinguish accounting approval from funded settlement.

## 6. Prototype asset rules

### ℰ

- transferable;
- used for prototype operational balances and claim payouts;
- used for claim and challenge bonds;
- subject to audit-tail locking.

### 𝒰

- issued as an expiring prototype balance;
- not generally transferable;
- can be spent from a member to an authorized vendor;
- vendor redemption converts pending 𝒰 settlement into ℰ from a separate epoch pool.

The final rule above is only a prototype transaction path. It is not a real-value guarantee. Minting more `ℰ` cannot guarantee a provider's purchasing power when demand for `ℰ` or external liquidity is absent.

## 7. Governance in v0.1.0

Only public-signal proposals are implemented:

- proposal creation;
- yes/no ballots;
- highest-sequence ballot wins;
- tally after close.

These are test signals, not binding elections. Encrypted private ballots, eligibility assurance, coercion resistance, decision-class selection, appeals, and independent verification are not implemented.

## 8. Persistence

Each node persists one JSON snapshot:

```text
<data_dir>/snapshot.json
```

The snapshot contains:

- the full chain;
- current state;
- mempool;
- seen transactions;
- known peers.

This is acceptable for demonstrations. It is not the intended record, database, backup, recovery, or audit architecture.

## 9. Key files

The CLI writes JSON key files with:

- algorithm;
- DID;
- public key;
- secret key.

The prototype uses Ed25519. The signing boundary lives in `essentia-core/src/crypto.rs` so the backend can later be replaced.

A DID string in the prototype is an account identifier. It is not proof of unique personhood, membership, eligibility, or secure recovery.

## 10. What should happen next

The next work is not to bolt production consensus and post-quantum signatures onto the current economic model.

The next work should be:

1. implement the v0.8.0 deterministic economic simulator;
2. represent 𝒰, ℰ, ℛ, external reserves, provider capacity, credit limits, defaults, and loss allocation explicitly;
3. run shadow accounting with no real-value dependence;
4. publish verifier outputs, coverage reports, and stress scenarios;
5. test plural identity and recovery separately from monetary rights;
6. implement signed transparency logs and witnesses for public records;
7. compare centralized, federated, and BFT operation only after the trust model is concrete;
8. preserve v0.1.0 compatibility only where it serves a documented experiment.

## 11. Known missing pieces

### Economic

- essential basket and versioning
- Essential Settlement Receivables
- capacity and liquidity coverage
- provider settlement waterfall
- mutual-credit issuance and repayment
- issuer-specific credit risk
- external reserves and procurement
- executable market conversion
- circuit breakers and emergency modes
- default and dissolution accounting

### Institutional

- real provider contracts
- legal and regulatory structure
- independent economic audit
- oracle governance
- appeals and error correction
- node admission and resolution

### Technical

- signed Merkle transparency log
- independent witnesses and split-view detection
- durable event and state stores
- reproducible verifier artifacts
- privacy-preserving credentials and recovery
- end-to-end verifiable voting for appropriate decision classes
- cryptographic agility and migration
- BFT consensus only if federation research justifies it
- metrics, tracing, and operational dashboards

---

> Target architecture -> [Essentia v0.8.0 Research Architecture](whitepaper.md)
