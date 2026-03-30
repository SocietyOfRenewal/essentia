# Essentia Network

This repo hosts the evolving specs and reference materials for the Essentia civic ledger (aligned with the [Society of Renewal](https://www.societyofrenewal.org/)) and will become a full client/server application written in Rust.

- Whitepaper: [`docs/whitepaper.md`](docs/whitepaper.md)

## License

Documentation © @CloneOfNone and contributors. See `LICENSE-DOCS`.

# Essentia v0.1.0 prototype

Rust workspace for a minimal client/server blockchain prototype aligned to the revised Essentia design.

This is a narrow civic-ledger prototype, not a general smart-contract chain. It implements:

- signed DID-style account registration,
- a multi-node HTTP server with peer sync,
- rotating-proposer signed blocks,
- deterministic state transitions,
- two assets with distinct rules:
  - `ESSENT` (`ℰ`): transferable,
  - `ESSENTIAL_UNITS` (`𝒰`): non-transferable except member-to-vendor spend plus vendor redemption,
- epoch budgets,
- purpose and quest funding,
- proof-of-contribution claims,
- reviewer scoring with median aggregation,
- budget-capped mint finalization that does not use reviewer reputation in the payout formula,
- public-signal governance proposals and votes,
- JSON persistence and bootstrap configs.

## Workspace layout

- `crates/essentia-core`: shared protocol types, signing, state machine, minting logic.
- `crates/essentia-node`: HTTP node daemon.
- `crates/essentia-cli`: key generation, queries, and transaction submission.
- `examples/bootstrap`: sample genesis, three validator configs, and key files.
- `scripts/demo.sh`: end-to-end demo flow.
- `docs/ARCHITECTURE.md`: design notes and current limits.

## What is implemented

### Network

- Three-node bootstrap layout with deterministic proposer rotation by height.
- Block import validation across peers.
- Background peer sync.
- Optional auto-propose loop.

### Ledger

- DID registration with signed transactions.
- Personhood credential issuance.
- Role issuance for validators, attestors, stewards, treasury operators, and vendors.
- `ℰ` transfers.
- `𝒰` issuance, member spend to vendors, and vendor redemption into `ℰ` from a separate epoch pool.

### Contribution and minting

- Epoch-level global `ℰ` mint cap.
- Purpose-level budgets.
- Quest-level reward ceilings.
- Claim bonds, challenge bonds, audit-tail locks.
- Review aggregation by rubric-weighted median score.
- Payout formula:

```text
payout = min(quest_reward_ceiling * score, remaining_purpose_budget, remaining_epoch_mint_cap)
```

Reviewer identity and reviewer status do not appear in that payout formula.

## Prototype limits

This is intentionally smaller than the whitepaper target.

- Consensus is rotating proposer plus signed block replication, not HotStuff.
- Cryptography uses Ed25519 in the prototype implementation. The code keeps a clean signing boundary so the consensus-critical signature backend can be swapped later.
- Governance ballots are public-signal only in v0.1.0. Threshold-encrypted binding elections are not implemented here.
- There is no general contract VM.
- Persistence is JSON snapshots, not a database.

## Quick start

### 1. Start three nodes

In three shells from the repo root:

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

### 4. Run the demo flow

```bash
bash scripts/demo.sh
```

The demo:

1. registers Alice, three attestors, and a vendor,
2. issues personhood and roles,
3. seeds Alice with `ℰ`,
4. creates a purpose and low-risk quest,
5. submits a claim and three reviews,
6. finalizes a budget-capped mint,
7. issues `𝒰`, spends it to a vendor, and redeems the vendor settlement.

## CLI examples

Generate a key:

```bash
cargo run -p essentia-cli -- keygen --out ./alice.json
```

Register a DID:

```bash
cargo run -p essentia-cli -- tx register-did \
  --node http://127.0.0.1:7001 \
  --key ./alice.json
```

Transfer `ℰ`:

```bash
cargo run -p essentia-cli -- tx transfer-essent \
  --node http://127.0.0.1:7001 \
  --key examples/bootstrap/keys/validator1.json \
  --to did:essentia:... \
  --amount 5000
```

Create a quest:

```bash
cargo run -p essentia-cli -- tx create-quest \
  --node http://127.0.0.1:7001 \
  --key examples/bootstrap/keys/validator1.json \
  --quest-id community-garden-q1 \
  --purpose-id community-garden \
  --title "Seed and irrigation rollout" \
  --reward-ceiling 50000 \
  --challenge-window-blocks 2 \
  --audit-tail-bps 2000 \
  --audit-tail-blocks 3 \
  --risk-band low \
  --rubric impact=5000 \
  --rubric quality=3000 \
  --rubric documentation=2000
```

## Suggested next steps

- replace the prototype signer with the intended ledger signature backend,
- add threshold-encrypted ballots and verifier artifacts,
- replace proposer rotation with actual BFT finality,
- move persistence to append-only block and state stores,
- add committee sampling and conflict graph checks for reviewer assignment.
