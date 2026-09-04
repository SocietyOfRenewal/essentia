# Essentia Monetary Research Program

## Status

This document defines the first executable research scaffold for the monetary hypothesis in Part B of the Society of Renewal Founding Book.

It is not a forecast, valuation model, or claim that the proposed economy has already been demonstrated. The purpose is to make assumptions visible enough to test, change, or reject.

## Central hypothesis

Essentia can create usable purchasing power without making every unit of `ℰ` a conventional debt or promising permanent redemption into an outside currency.

The proposed mechanism is:

1. `𝒰` defines a real, cost-of-living-indexed entitlement.
2. Activating `𝒰` creates `ℛ`, a short-lived settlement claim that preserves the real value of the entitlement while a payment clears.
3. `ℛ` settles into enough `ℰ` to represent the same internal purchasing power at that time.
4. `ℰ` acquires value through coordinated acceptance, equal civic issuance, useful network services, reciprocal trade, producer credit, and expectations of future acceptance.
5. The External Liquidity Bridge supports imports and interoperability. It is not the sole source of `ℰ` value.

The open question is whether these mechanisms can coordinate on a durable positive-value equilibrium at useful scale.

## Why nominal scale is not the same as inflation

Suppose a pilot activates `A` units of real entitlement during a period. Let `P` be the number of `ℰ` required for one `𝒰`. The protocol settles:

```text
NewCivicE = A * P
```

If `P` changes from `1 ℰ/𝒰` to `1,000,000 ℰ/𝒰`, the nominal issue changes by a factor of one million. The real claim does not change. If every balance and price is expressed at the new scale, this is denomination, not additional real demand.

The dangerous variable is the relationship between activated real claims and deliverable real output.

A minimal model is:

```text
P_t / P_(t-1) ≈ ActivatedRealClaims_t / DeliverableOutput_t
```

This is not a complete inflation model. It is a stylized price-factor relationship under the assumption that the newly issued settlement flow is the only spending flow and turns over once. It isolates one mechanism. Desired money balances, velocity, imports, inventories, expectations, market power, debt contracts, exchange rates, and distribution must be added before the model can inform policy.

## Executable scaffold

Run:

```bash
python3 research/monetary_dynamics.py --self-test
```

Generate machine-readable scenario results and CSV paths:

```bash
python3 research/monetary_dynamics.py \
  --self-test \
  --json \
  --output-dir ./research/output
```

The script contains two deliberately small model families.

### Indexed-settlement model

It tests:

- balanced real claims and capacity;
- identical economies starting at `1 ℰ/𝒰` and `1,000,000 ℰ/𝒰`;
- a fixed physical shortage;
- a shortage with ordinary supply response;
- a shortage with supply response plus producer credit;
- a real entitlement larger than current capacity.

The default scenarios show:

| Scenario | Ending `ℰ/𝒰` relative to start | Cumulative shortage in `𝒰` | Interpretation |
| --- | ---: | ---: | --- |
| Balanced, scale 1 | 1.000 | 0 | Stable |
| Balanced, scale 1,000,000 | 1.000 | 0 | Same normalized path |
| Fixed shortage | 47.330 | 10,800 | Real shortage creates continuing price pressure |
| Supply response | 1.305 | 857 | Capacity catches up after transient pressure |
| Supply plus producer credit | 1.176 | 500 | Faster capacity response reduces the price path |
| Real overclaim | 1.988 | 3,324 | Entitlement above capacity remains inflationary until supply catches up |

These numbers are products of chosen parameters. Their value is in the comparison, not the forecast.

### Adoption model

The adoption model tests a coordination problem. Acceptance depends on direct utility, provider coverage, expected network acceptance, friction, and instability.

Under the default illustrative parameters:

| Scenario | Ending acceptance |
| --- | ---: |
| Isolated wallets | 0.145 |
| Founding market | 0.977 |
| Founding market with high friction | 0.229 |
| Founding market with instability | 0.189 |

The result is not evidence that these exact thresholds exist. It demonstrates why an Essent launch must coordinate consumers, providers, suppliers, interfaces, and settlement at the same time. Distributing wallets one by one can select the low-value equilibrium even when a high-value equilibrium is possible.

## Required next models

The next research implementations should add:

1. Heterogeneous households with different propensities to spend and save.
2. Providers with sector-specific capacity, costs, inventories, and external-currency obligations.
3. Explicit desired `ℰ` balances and endogenous velocity.
4. Standing offers quoted in `𝒰` and settled in `ℰ`.
5. Internal and external exchange markets with depth, spreads, and market impact.
6. Producer credit linked to observed `ℛ`, shortages, and expected demand.
7. Mutual credit and outside civic issuance in the same accounting system.
8. Speculative demand, hoarding, bank-run behavior, and confidence shocks.
9. Rent, healthcare, energy, and food sectors with different supply elasticity.
10. Node-level issuance, cross-node clearing, migration, and external trade.
11. Identity error and Sybil attacks against equal issuance.
12. Governance delays, oracle error, manipulation, and policy credibility.
13. Distributional accounting for seigniorage gains and inflation losses.
14. Ecological limits and real resource depletion.

At least one model should be stock-flow consistent. At least one should be agent-based. The same policy must be tested under competing behavioral assumptions.

## Research discipline

- Publish assumptions before interpreting results.
- Include nearby failure regions rather than one calibrated success path.
- Separate identities from behavioral equations.
- Report real values alongside nominal `ℰ` totals.
- Do not infer external cash-out capacity from internal purchasing power.
- Do not infer failure from a large denomination.
- Do not infer success from a stable price produced by a subsidy.
- Preserve negative results.
- Let pilot participants challenge what the model leaves out.

The purpose is not to make the hypothesis look safe. It is to make it buildable.
