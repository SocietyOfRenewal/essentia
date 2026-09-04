# Essentia

Essentia is the monetary and civic research system derived from the Society of Renewal Founding Book.

This repository contains:

1. the [Essentia v0.9.0 Monetary and Civic Research Architecture](docs/whitepaper.md);
2. an [executable monetary research scaffold](research/monetary_dynamics.py);
3. a runnable v0.1.0 Rust ledger prototype.

The whitepaper defines the direction. The research code tests monetary mechanisms. The Rust workspace is a narrow prototype and does not implement the current economic architecture.

There is no public mainnet, issued Essent, issued Essential Unit, live Freedom Floor, production ballot system, or public benefit.

## Monetary hypothesis

Essentia tests whether a cryptographic network can deliberately create a positive-value currency and use it to provide a real Universal Basic Income.

The core flow is:

```text
𝒰 -> ℛ -> ℰ
```

- `𝒰`, Essential Unit: the indexed real entitlement and unit of account.
- `ℛ`, Essential Settlement Receivable: the short-lived bridge claim created when `𝒰` is activated.
- `ℰ`, Essent: the transferable money used for general exchange.

A person may activate `𝒰` for a provider payment or unrestricted conversion. `ℛ` preserves the real value while the transaction settles. Settlement creates or transfers enough `ℰ` to represent the same internal purchasing power at that time.

`ℰ` may include:

- outside civic money created through equal per-person issuance;
- inside money created through mutual and producer credit;
- treasury and market operations under public rules.

A conventional redemption liability is not required behind each unit of `ℰ`. Value is expected to emerge from coordinated acceptance, useful goods and services, network effects, predictable rules, equal issuance, and confidence that other people will continue to accept it.

The External Liquidity Bridge supports imports, legacy obligations, and optional outside exchange. It is not the sole source of value and does not need to back all `ℰ` one-for-one.

## Research first

The first executable model is documented in [docs/MONETARY_RESEARCH.md](docs/MONETARY_RESEARCH.md).

Run its self-tests:

```bash
python3 research/monetary_dynamics.py --self-test
```

Generate scenario results:

```bash
python3 research/monetary_dynamics.py \
  --self-test \
  --json \
  --output-dir ./research/output
```

The model demonstrates two narrow points:

- starting at `1 ℰ/𝒰` or `1,000,000 ℰ/𝒰` produces the same normalized path when real claims and capacity are identical;
- physical shortage, supply response, producer credit, adoption friction, and confidence can change the path.

It is an illustrative scaffold, not a forecast.

## Repository layout

- `docs/whitepaper.md`: v0.9.0 monetary and civic architecture.
- `docs/MONETARY_RESEARCH.md`: hypotheses, scenarios, limitations, and next models.
- `research/monetary_dynamics.py`: dependency-free monetary and adoption simulations.
- `docs/ARCHITECTURE.md`: what the v0.1.0 prototype implements and what should be built next.
- `crates/essentia-core`: prototype types, signing, and state machine.
- `crates/essentia-node`: prototype HTTP node.
- `crates/essentia-cli`: prototype CLI.
- `examples/bootstrap`: sample genesis, validators, and keys.
- `scripts/demo.sh`: end-to-end prototype flow.

## Essentia v0.1.0 prototype

The Rust workspace implements:

- signed DID-style account registration;
- multi-node HTTP replication;
- rotating-proposer signed blocks;
- deterministic state transitions;
- prototype `ℰ` and `𝒰` balances;
- epoch budgets, purposes, quests, claims, and review;
- public-signal proposals and votes;
- JSON persistence.

These mechanics are research artifacts. They do not implement:

- the indexed `𝒰` unit;
- `ℛ`;
- outside civic issuance;
- mutual or producer credit;
- internal `ℰ/𝒰` price discovery;
- standing provider offers;
- supply-coupled monetary control;
- external exchange or the Liquidity Bridge;
- strong proof of personhood;
- private binding elections;
- production consensus.

## Quick start for the prototype

Start three nodes in separate shells:

```bash
cargo run -p essentia-node -- --config examples/bootstrap/node1.json
cargo run -p essentia-node -- --config examples/bootstrap/node2.json
cargo run -p essentia-node -- --config examples/bootstrap/node3.json
```

Inspect state:

```bash
cargo run -p essentia-cli -- query status --node http://127.0.0.1:7001
cargo run -p essentia-cli -- query state --node http://127.0.0.1:7001
```

Run the demo:

```bash
bash scripts/demo.sh
```

## Next implementation work

1. Implement a double-entry core that distinguishes `𝒰`, `ℛ`, outside civic `ℰ`, mutual credit, producer credit, and treasury operations.
2. Build the basket and internal price-index engine.
3. Implement the `𝒰 -> ℛ -> ℰ` activation and settlement state machine.
4. Add standing provider offers and a founding-market graph.
5. Expand the simulation into stock-flow-consistent and agent-based models.
6. Implement aggregate `ℛ` aging, monetary, supply, and adoption metrics.
7. Add purpose-bound personhood credentials and recovery.
8. Add signed public event logs and independent verifiers.
9. Test external exchange and bridge operations after internal use works.
10. Add BFT consensus only when independent nodes need shared finality.

## License

Documentation © @CloneOfNone and contributors. See `LICENSE-DOCS`.
