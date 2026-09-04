# Essentia v0.1.0 Prototype Architecture

## Status

This document describes the runnable Rust prototype.

The current research target is the [v0.9.0 Monetary and Civic Research Architecture](whitepaper.md), derived from Part B of the Society of Renewal Founding Book. The prototype is useful code, but it does not define the monetary system.

In particular, v0.1.0 has prototype `𝒰` and `ℰ` balance paths. The target architecture requires:

```text
𝒰 -> ℛ -> ℰ
```

- `𝒰` as an indexed real entitlement;
- `ℛ` as the bridge claim created on activation;
- `ℰ` as outside civic money and inside credit money;
- internal purchasing-power measurement;
- a founding market;
- supply-coupled issuance;
- external exchange separated from internal value.

## 1. Scope

The prototype provides a test harness for:

- signed transactions;
- account registration;
- replicated nodes;
- deterministic state transitions;
- budget and claim workflows;
- two balance types;
- simple public-signal governance.

It does not provide production consensus, privacy, personhood, monetary stability, a UBI, or a lawful payment network.

## 2. Node model

Each node runs:

- an HTTP API;
- a local mempool;
- a full chain snapshot;
- deterministic block execution;
- background peer synchronization;
- optional automatic proposal.

The proposer for height `h` is:

```text
validator_set[(h - 1) % validator_set.len()]
```

This is deterministic rotation, not Byzantine fault tolerant consensus.

## 3. Prototype state

### Registries

- DID-style accounts
- administrator-issued personhood objects
- role grants
- epoch budgets
- purposes
- quests
- claims
- proposals

### Balances

- liquid `ℰ`
- locked `ℰ`
- expiring `𝒰` lots
- vendor settlement queues

The names do not imply that the prototype satisfies the v0.9.0 semantics.

## 4. Claim payout experiment

A prototype claim payout is:

```text
raw = quest_reward_ceiling * rubric_weighted_median_score
payout = min(raw, remaining_purpose_budget, remaining_epoch_mint_cap)
```

Reviewer identity does not directly multiply payout.

This is useful for testing workflow and authorization. It does not establish the purchasing power of the issued `ℰ`. Production work should distinguish contract approval from monetary issuance and settlement.

## 5. Prototype `𝒰` path

The prototype:

1. creates an expiring `𝒰` balance;
2. lets a member spend `𝒰` to an authorized vendor;
3. lets the vendor redeem against a configured epoch pool.

The v0.9.0 target instead requires an indexed unit, `ℛ`, direct and unrestricted conversion, internal price discovery, outside civic issuance, provider settlement choices, and visible failure states.

## 6. Governance

The prototype supports:

- proposal creation;
- public yes/no signals;
- highest-sequence ballot replacement;
- tally after close.

These are not binding private elections. Eligibility assurance, secret ballots, coercion resistance, method selection, appeals, and independent verification remain research and implementation work.

## 7. Persistence

Each node stores one JSON snapshot:

```text
<data_dir>/snapshot.json
```

The snapshot contains the chain, state, mempool, seen transactions, and known peers. This is acceptable for demonstrations and unacceptable for production recovery, auditing, or scale.

## 8. Cryptography

The prototype uses Ed25519 and JSON key files. The signing boundary can be replaced later.

A prototype DID string proves control of one key. It does not prove unique personhood, eligibility, membership, or recovery safety.

## 9. Current research model

The first monetary model is [`research/monetary_dynamics.py`](../research/monetary_dynamics.py), documented in [MONETARY_RESEARCH.md](MONETARY_RESEARCH.md).

It tests:

- nominal-scale invariance;
- activated real claims versus deliverable output;
- capacity response;
- producer credit;
- provider coverage and adoption thresholds;
- friction and confidence shocks.

The model is deliberately incomplete. It is the beginning of the required research stack, not an economic forecast.

## 10. Implementation sequence

1. Double-entry representations for `𝒰`, `ℛ`, civic `ℰ`, mutual credit, producer credit, treasury issue, and external assets.
2. Basket, availability, and `Q_int` calculation.
3. `𝒰 -> ℛ -> ℰ` activation and settlement.
4. Standing offers and founding-market graph analysis.
5. Monetary and adoption simulation with reproducible scenarios.
6. Aggregate `ℛ` aging and settlement telemetry.
7. Purpose-bound identity, uniqueness, and recovery.
8. Signed append-only public evidence log with independent witnesses.
9. External exchange, reserves, and bridge experiments.
10. Federated shared state.
11. BFT finality if the trust model requires it.

## 11. Missing components

### Monetary and economic

- indexed `𝒰` basket;
- `ℛ` lifecycle;
- equal outside civic issuance;
- mutual and producer credit;
- internal purchasing-power index;
- neutral rebase;
- standing offers;
- supply response;
- external markets and bridge;
- seigniorage accounting;
- defaults and loss allocation.

### Identity and governance

- privacy-preserving unique personhood;
- purpose-bound credentials;
- recovery and appeals;
- private verifiable ballots;
- bounded monetary authority;
- model and decision records.

### Distributed systems

- durable event storage;
- Merkle transparency log;
- independent witnesses;
- reproducible verifiers;
- privacy-preserving data separation;
- federated clearing;
- production consensus;
- cryptographic migration;
- operations and incident response.

---

> Research target -> [Essentia v0.9.0 Monetary and Civic Research Architecture](whitepaper.md)
