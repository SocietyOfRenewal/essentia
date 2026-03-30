# Architecture

## 1. Scope

Essentia v0.1.0 is a runnable prototype for the civic-ledger core:

- nodes talk over HTTP,
- blocks are signed and replicated,
- the state machine enforces mint boundaries,
- the client can create keys, inspect state, and submit transactions.

It is deliberately narrower than the long-term chain.

## 2. Node model

Each node runs:

- an HTTP API,
- a local mempool,
- a full chain snapshot,
- deterministic block execution,
- background peer sync,
- optional auto-propose.

The proposer for height `h` is:

```text
validator_set[(h - 1) % validator_set.len()]
```

That gives a deterministic schedule without leader election code in v0.1.0.

## 3. State model

The chain is account/object based.

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

## 4. Mint safety

The prototype keeps the critical economic fix from the rewritten whitepaper.

### Old failure mode

Reviewer reputation or reviewer influence should not scale issuance.

### Current rule

A claim payout is bounded by three independent ceilings:

```text
raw = quest_reward_ceiling * rubric_weighted_median_score
payout = min(raw, remaining_purpose_budget, remaining_epoch_mint_cap)
```

Reviewer identity affects whether a review is accepted at all. It does not affect the payout multiplier.

## 5. Claim lifecycle

1. Quest exists under a funded purpose.
2. Claimant posts a claim with a bond.
3. Attestors submit rubric scores.
4. Members may challenge with a bond.
5. A steward or validator resolves challenges.
6. Finalization checks:
   - challenge window elapsed,
   - minimum review count reached,
   - no unresolved challenge,
   - epoch cap remaining,
   - purpose budget remaining.
7. Finalization mints liquid `ℰ` plus an audit-tail lock.
8. Claim bond unlocks only after the audit-tail horizon.

## 6. Asset rules

### `ℰ`

- transferable,
- used for operational balances and claim payouts,
- used for claim and challenge bonds,
- subject to audit-tail locking.

### `𝒰`

- issued through the Freedom Floor rail,
- stored as expiring lots,
- not generally transferable,
- can be spent from member to authorized vendor,
- vendor redemption converts pending `𝒰` settlement into `ℰ` from a separate epoch pool.

## 7. Governance in v0.1.0

Only public-signal proposals are implemented here.

- proposal creation,
- yes/no ballots,
- highest sequence ballot wins,
- tally after close.

Encrypted private ballots are not part of this release.

## 8. Persistence

Each node persists one JSON snapshot:

```text
<data_dir>/snapshot.json
```

That snapshot contains:

- the full chain,
- current state,
- mempool,
- seen transactions,
- known peers.

This is acceptable for a prototype and simple demos. It is not the intended production storage layer.

## 9. Key files

The CLI writes JSON key files with:

- algorithm,
- DID,
- public key,
- secret key.

The prototype uses Ed25519 for signing. The signing boundary lives in `essentia-core/src/crypto.rs` so the backend can be replaced without rewriting the state machine.

## 10. Known missing pieces

- HotStuff or any other real BFT finality path
- threshold-encrypted ballots
- committee sampling by conflict graph and randomness beacon
- post-finalization clawback path during audit tail
- append-only storage engine
- metrics, tracing spans, and operational dashboards
