# Essentia Network

This repository contains two different things:

1. a runnable v0.1.0 Rust research prototype; and
2. the [Essentia Research Architecture v0.8.0](docs/whitepaper.md), derived from Part B of the [Society of Renewal Founding Book](https://github.com/SocietyOfRenewal/societyofrenewal/blob/main/docs/founding-book/Part%20B.md).

The distinction is structural. The prototype demonstrates transaction and replication ideas. The whitepaper defines the research direction. The prototype must not be treated as an implementation of a live currency, government, Freedom Floor, identity system, or production ledger.

There is no public mainnet, issued Essent, issued Essential Unit, production ballot system, or live Society benefit.

## Economic model

The current research architecture is derived from Part B of the Founding Book. A quoted conversion rate can calculate a large quantity of `ℰ` when `ℰ` loses value, but it cannot guarantee that the resulting balance will buy the promised essentials. The design therefore separates:

- **𝒰:** a person-held entitlement measured against a regional essential basket;
- **ℰ:** a transferable mutual-credit and settlement instrument;
- **ℛ:** the receivable a provider earns after delivering an 𝒰-funded good or service;
- **External Liquidity Bridge:** reserves, revenues, credit lines, contracts, and payment partners that connect internal claims to the outside economy;
- **Public Evidence Plane:** signed, independently verifiable records of rules, liabilities, coverage, budgets, and institutional actions.

𝒰 value is attached to delivered essentials and funded settlement capacity. Conversion into ℰ is a later, bounded service. It is not the source of the entitlement's value.

The first production experiment should be double-entry accounting plus an append-only transparency log, not a new blockchain mainnet.

## License

Documentation © @CloneOfNone and contributors. See `LICENSE-DOCS`.

## Essentia v0.1.0 prototype

The current Rust workspace implements a deliberately narrow civic-ledger experiment:

- signed DID-style account registration;
- a multi-node HTTP server with peer synchronization;
- rotating-proposer signed blocks;
- deterministic state transitions;
- two prototype assets:
  - `ESSENT` (`ℰ`), transferable;
  - `ESSENTIAL_UNITS` (`𝒰`), non-transferable except member-to-vendor spend and vendor redemption;
- epoch budgets;
- purpose and quest funding;
- proof-of-contribution claims;
- reviewer scoring with median aggregation;
- budget-capped mint finalization;
- public-signal governance proposals and votes;
- JSON persistence and bootstrap configuration.

These mechanics are research artifacts, not the target economic architecture. Prototype `𝒰` redemption, generic `ℰ` minting, one-DID assumptions, and public-signal voting do not satisfy the requirements for a real pilot.

## Workspace layout

- `crates/essentia-core`: protocol types, signing, state machine, and prototype minting logic.
- `crates/essentia-node`: HTTP node daemon.
- `crates/essentia-cli`: key generation, queries, and transaction submission.
- `examples/bootstrap`: sample genesis, validator configurations, and key files.
- `scripts/demo.sh`: end-to-end prototype flow.
- `docs/ARCHITECTURE.md`: current prototype design and limitations.
- `docs/whitepaper.md`: v0.8.0 research architecture and stage-gate program.

## Prototype limits

- Consensus is rotating proposer plus signed block replication, not Byzantine fault tolerant consensus.
- Cryptography uses Ed25519.
- Governance ballots are public signals only.
- Persistence is JSON snapshots.
- The personhood credential is an administrator-issued prototype object, not proof of unique personhood.
- 𝒰 redemption uses a pre-funded prototype pool and does not implement real basket measurement, provider receivables, reserve accounting, executable-price estimation, or the settlement waterfall.
- The code does not implement mutual-credit ℰ, double-entry node exposure, external settlement, or loss resolution.
- No general contract virtual machine exists.

## Quick start

### 1. Start three nodes

In three shells from the repository root:

```bash
cargo run -p essentia-node -- --config examples/bootstrap/node1.json
cargo run -p essentia-node -- --config examples/bootstrap/node2.json
cargo run -p essentia-node -- --config examples/bootstrap/node3.json
```

### 2. Inspect bootstrap IDs

```bash
cat examples/bootstrap/ids.json
```

### 3. Check the network

```bash
cargo run -p essentia-cli -- query status --node http://127.0.0.1:7001
cargo run -p essentia-cli -- query state --node http://127.0.0.1:7001
```

### 4. Run the demo

```bash
bash scripts/demo.sh
```

The demo:

1. registers Alice, three reviewers, and a vendor;
2. issues prototype personhood and role credentials;
3. seeds Alice with ℰ;
4. creates a purpose and quest;
5. submits a claim and reviews;
6. finalizes a budget-capped payout;
7. issues 𝒰, spends it to a vendor, and redeems against a prototype pool.

That is a software demonstration, not an economic validation.

## Current payout experiment

The prototype uses:

```text
payout = min(
    quest_reward_ceiling * score,
    remaining_purpose_budget,
    remaining_epoch_mint_cap
)
```

Reviewer identity and reviewer status do not directly increase the payout.

Version 0.8.0 goes further: a real payout must reference a funded budget, authorized credit position, and settlement capacity. Review can establish whether work met a contract. Review cannot create purchasing power by scoring the work more highly.

## Next implementation work

Do not begin with a consensus rewrite.

The research sequence is:

1. specify double-entry accounts for ℰ, 𝒰 liabilities, ℛ claims, reserves, guarantees, and node exposure;
2. build economic simulations and failure injection;
3. implement a signed append-only transparency log with independent witnesses;
4. run a shadow ledger beside conventional settlement;
5. pilot closed business mutual credit without 𝒰;
6. pilot externally funded 𝒰 direct spending;
7. add voluntary mixed ℰ settlement and obligation netting;
8. test bounded conversion only after executable liquidity exists;
9. federate independent clearing nodes only after resolution and exposure rules work;
10. evaluate whether Byzantine fault tolerant consensus is actually necessary.

See [the whitepaper](docs/whitepaper.md) for acceptance gates, stop conditions, coverage equations, identity boundaries, and the replacement economic model.
