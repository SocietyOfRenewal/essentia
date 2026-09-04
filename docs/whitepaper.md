# Essentia Monetary and Civic Research Architecture

## Version 0.9.0 - The Monetary Coordination Hypothesis

Status: research specification, not a production protocol  
Date: 2026-09-03  
Authority: Part A and Part B of the Society of Renewal Founding Book  
Scope: indexed universal income, monetary coordination, civic evidence, privacy, and federated governance

---

## 0. Status

Essentia is an attempt to build a cryptographic economy in which every person receives a real Universal Basic Income and no one must trade dignity for survival.

That is a hypothesis, not a feature claim.

The repository contains a runnable Rust prototype. The code demonstrates signed accounts, replicated nodes, budget objects, contribution claims, two asset types, and basic transaction flows. It does not demonstrate a viable currency, stable purchasing power, a Freedom Floor, unique global personhood, private binding elections, production consensus, or lawful global payments.

This specification follows the [Society of Renewal Founding Book](https://github.com/SocietyOfRenewal/societyofrenewal/tree/main/docs/founding-book). Part A states the moral commitments. Part B establishes the scientific and economic reasoning, names uncertainty, and defines the research program. The Charter and Essentia must be derived from those foundations.

The monetary hypothesis is:

> A cryptographic network can deliberately coordinate a positive-value monetary equilibrium by issuing an equal, cost-of-living-indexed claim to every person, converting that claim into a commonly accepted currency, and coupling the resulting demand to production, settlement, and transparent monetary feedback.

The protocol uses three symbols:

```text
𝒰 -> ℛ -> ℰ
```

- `𝒰`, the Essential Unit, is the indexed real entitlement and unit of account.
- `ℛ`, the Essential Settlement Receivable, preserves the real claim while settlement occurs.
- `ℰ`, Essent, is the transferable money used for general exchange.

The External Liquidity Bridge connects this economy to national currencies and outside suppliers. It is useful infrastructure. It is not the source of all `ℰ` value and it does not need to back every unit one-for-one.

The design deliberately allows `ℰ` to be outside money. Civic issuance need not create a conventional debtor or a promise to redeem each `ℰ` for another asset. `ℛ` is the explicit settlement claim. `ℰ` is valuable to the extent that people can use it, expect others to use it, and trust the rules that govern its issuance.

The system succeeds only when the network makes that expectation true often enough to become self-reinforcing.

---

## 1. What Essentia is trying to prove

The question is not whether software can create a number.

The question is whether software, institutions, and coordinated human behavior can create durable purchasing power.

Bitcoin established that a digital object with no commodity backing and no conventional redemption promise can acquire enormous market value when people coordinate around its scarcity, transfer rules, security, and expected future acceptance. Monetary experiments have also shown that intrinsically worthless tokens can support cooperation among strangers. Fiat monetary theory contains positive-value equilibria for irredeemable money as well as zero-value equilibria.

Essentia takes the next step. It asks whether a network can deliberately create the useful equilibrium while distributing new money universally and indexing the income claim to the actual cost of living.

This is more demanding than creating a speculative asset. A Universal Basic Income must work when a person needs groceries, rent, medicine, transportation, communication, or unrestricted personal choice. It must continue to work after novelty fades. It must not depend on new buyers forever. It must not hide shortages behind nominal balances.

The protocol therefore has to coordinate five systems at once:

1. **Common belief.** People expect `ℰ` to remain acceptable because they can observe a growing network of use.
2. **Real usefulness.** Providers offer goods, labor, services, financing, and infrastructure for `ℰ`.
3. **Monetary integrity.** Issuance is universal, predictable, inspectable, and not captured by insiders.
4. **Productive response.** New demand helps idle capacity become output and sends capital toward bottlenecks.
5. **Correction.** Prices, shortages, settlement delays, concentration, and confidence are measured early enough to change policy.

A blockchain can help with common records and counterfeit resistance. It cannot supply the other four by itself. Essentia is therefore a monetary institution implemented through software, not a token contract pretending to be an economy.

---

## 2. Constitutional invariants

Every implementation must preserve these properties.

### 2.1 Dignity

Access to income, food, shelter, safety, appeal, or personhood may not depend on humiliation, wealth, public reputation, political agreement, or technical competence.

### 2.2 Equal civic issuance

The universal monetary share belongs equally to every eligible person. Wealth, stake, office, contribution score, validator role, and early participation do not increase the recurring civic issue.

### 2.3 Real value before nominal count

A million `ℰ` may be worth less than one `ℰ`. The system reports real purchasing power, availability, and settlement performance beside nominal balances.

### 2.4 No founder monetary privilege

There is no hidden premine, privileged exchange rate, permanent founder allocation, or private mint authority. Compensation for work is public compensation, not a superior claim on future money.

### 2.5 Privacy for people, transparency for power

Institutional rules, issuance, budgets, market operations, reserve use, official models, and emergency actions are inspectable. Personal transactions, household conditions, ballots, health records, recovery secrets, and protected disputes are minimized and compartmentalized.

### 2.6 Explicit issuance and explicit claims

Every unit of `ℰ` records its issuance class and authority in aggregate audit data. Every `ℛ` identifies who owes settlement, to whom, in what real amount, by when, and under which loss rule.

This does not mean every `ℰ` is a debt. It means the public can distinguish outside civic money, inside credit money, treasury operations, market operations, and fees.

### 2.7 Political equality is not financial sameness

Every person has equal civic standing. Credit limits, underwriting, and provider exposure may differ because financial promises carry different risks. Those differences may not become a social caste, a voting hierarchy, or a condition of essential access.

### 2.8 Reversibility and continuity

Experimental parameters must be reversible. Emergency changes expire. People retain access to ordinary payment rails and essential support during protocol disputes or failures.

### 2.9 Verifier first

A binding distribution, issuance calculation, price index, election, settlement, or policy change is not ready until an independent implementation can reproduce it from authorized inputs.

### 2.10 No disguised certainty

Every public mechanism is labeled as specification, simulation, pilot, demonstrated system, or production dependency. Confidence grows through evidence rather than vocabulary.

---

## 3. The monetary ontology

Essentia separates the real claim, the bridge claim, and the circulating money because they answer different questions.

### 3.1 Essential Unit (`𝒰`)

`𝒰` is a real-value unit of account and a non-transferable entitlement balance.

It answers:

> What share of a dignified minimum belongs to this person during this period?

A `𝒰` is not defined as a fixed number of dollars, euros, or `ℰ`. It is defined through a published basket and index methodology. The amount of local currency or `ℰ` represented by one `𝒰` changes as prices and conditions change.

The design is related to indexed units of account such as Chile's Unidad de Fomento. The important separation is between the stable real reference and the payment instrument used at settlement.

### 3.2 Essential Settlement Receivable (`ℛ`)

`ℛ` is a short-lived, non-circulating settlement claim created whenever a person activates `𝒰`.

It answers:

> Who must receive the value represented by this activated entitlement, and what real value remains unsettled?

The beneficiary may be:

- a provider after a direct essential purchase;
- the member after an unrestricted `𝒰` conversion;
- a payment agent acting for the member or provider;
- a cooperative or public institution that delivered a covered service.

Each `ℛ` remains denominated in `𝒰` until settlement. This prevents a delay between activation and settlement from transferring currency risk to the beneficiary.

`ℛ` is not general money. It cannot be mined, traded as a speculative asset, or used to purchase political power. It exists to preserve and route a specific claim.

### 3.3 Essent (`ℰ`)

`ℰ` is the general medium of exchange.

It answers:

> What common monetary unit can carry purchasing power through the network?

`ℰ` is transferable, divisible, and usable for ordinary goods and services, savings, wages, contracts, grants, mutual credit, producer finance, and settlement. It floats against national currencies. Its internal purchasing power is measured continuously against `𝒰` and broad market baskets.

The protocol supports two economic sources of `ℰ` in one fungible currency:

1. **Outside civic money.** `ℰ` created by the monetary commons without a conventional debtor and distributed under equal civic issuance or used to settle activated `𝒰`.
2. **Inside credit money.** `ℰ` created as matched positive and negative positions through mutual credit, producer credit, or treasury credit.

The ledger records the source class for audit. Ordinary recipients do not receive different grades of `ℰ`.

### 3.4 External Liquidity Bridge

The External Liquidity Bridge connects `ℰ` and `ℛ` to institutions that require national currency or another outside asset.

It may include:

- market makers and auctions;
- national-currency reserves;
- committed credit facilities;
- cooperative revenue;
- grants, donations, and public funding;
- import and procurement contracts;
- regulated payment partners;
- cross-node settlement agreements;
- insurance and guarantee funds.

The bridge helps with imports, taxes, legacy rent contracts, utility bills, external payroll, and optional cash-out. It is a catalyst and shock absorber, not the metaphysical backing of the currency.

---

## 4. The real unit and the Freedom Floor

### 4.1 Regional basket

For region `r`, household class `h`, category `k`, and period `t`, define:

```text
BasketCost(r, h, k, t)
```

The complete monthly floor is:

```text
FloorCost(r, h, t) = sum_k BasketCost(r, h, k, t)
```

The basket includes at least:

- food and household necessities;
- safe shelter;
- energy and water;
- basic healthcare and medication;
- communication;
- ordinary transportation;
- clothing and personal care;
- disability and accessibility costs;
- childcare or dependent care where relevant;
- an unrestricted personal component.

A purely restricted benefit is not a full income. The unrestricted component is part of the target from the beginning even when early pilots test only a smaller subset.

### 4.2 Basket construction

The index combines:

1. a normative basket defined through rights and public deliberation;
2. observed transactions and posted provider offers;
3. official price statistics;
4. availability and wait-time measurements;
5. quality and substitution rules;
6. household, disability, climate, and geographic adjustments;
7. independent sampling and challenge procedures.

The methodology, inputs, exclusions, revisions, and uncertainty bands are public. Personal purchase histories are not required to publish an accurate aggregate index.

### 4.3 Unit definition

One `𝒰` is a fixed fraction of the ratified local monthly Freedom Floor. A practical default is:

```text
1,000 𝒰 = one full monthly individual Freedom Floor
```

A person's periodic entitlement is:

```text
EntitlementU(i, r, t)
  = 1,000
    * HouseholdAdjustment(i, r, t)
    * CoverageFraction(r, t)
```

At full operation, `CoverageFraction = 1`. A pilot may use a smaller fraction, but it must state the fraction plainly.

### 4.4 Accrual and activation

`𝒰` accrues to an eligible person on a predictable cadence. Accrual itself does not immediately create circulating `ℰ`.

The person may activate `𝒰` through:

- direct payment to an essential provider;
- unrestricted conversion to `ℰ`;
- scheduled payment of rent, utilities, care, or another recurring obligation;
- transfer to a dependent or household pool under consent and guardianship rules;
- voluntary saving or carry-forward under published limits.

Activation creates `ℛ`. Settlement of `ℛ` creates or transfers `ℰ`, external money, netted obligations, or an agreed combination.

This spend-triggered structure avoids minting every possible claim in advance while preserving the person's right to use it.

---

## 5. The core conversion

### 5.1 Internal conversion rate

For region `r` and time `t`, define:

```text
Q_int(r, t) = ℰ required to obtain 1 𝒰 of purchasing power inside the network
```

`Q_int` is computed from actual accepted prices, standing offers, completed transactions, availability, and market depth. It is not copied from the external exchange price of `ℰ`.

At genesis, the numerical scale is chosen rather than discovered. A node may define `Q_int = 1 ℰ/𝒰` and recruit a founding set of providers willing to honor offers at that denomination. This does not prove that `ℰ` has value. The offers and reciprocal uses create the first observable value.

After launch, the estimator must avoid measuring its own mechanically generated settlements as independent market evidence. It should use:

- ordinary `ℰ` transactions not funded by a simultaneous `𝒰` activation;
- provider and supplier standing offers with real quantity limits;
- completed purchases and repeat acceptance;
- order-book depth and the price impact of practical transaction sizes;
- provider ability to reuse received `ℰ`;
- shortages, wait times, substitutions, refunds, and failed sales;
- robust medians across goods, providers, and time;
- a published confidence interval and stale-data rule.

For good `g`, a basic observation is:

```text
q_g(r, t) = PriceE(g, r, t) / ReferencePriceU(g, r, t)
```

`Q_int` is a robust aggregation of these ratios, not a vote by monetary stewards. If reliable observations are too sparse, Essentia reports uncertainty and uses the most recent valid reference window rather than fabricating precision.

A person activating `u` units creates:

```text
ReceivableU = u
SettlementE = u * Q_int(r, t)
```

The protocol records:

```text
𝒰 burned or locked
ℛ created for u 𝒰
ℰ transferred or issued for u * Q_int
ℛ retired after settlement
```

This is the central mechanism. It is not an optional feature reserved for a distant mature phase.

### 5.2 Direct provider payment

When a member pays a provider in `𝒰`:

1. the member authorizes the exact `𝒰` amount;
2. the provider delivers or contractually commits the good or service;
3. the protocol creates `ℛ` for the provider;
4. `ℛ` settles in `ℰ`, external currency, netted obligations, in-kind value, or a contractually accepted mix;
5. the provider can verify the calculation and settlement priority.

The member does not need to understand provider settlement to use the entitlement.

### 5.3 Unrestricted conversion

When a member converts `𝒰` for general use:

1. the member activates `u` units;
2. `ℛ` is created with the member or authorized payment agent as beneficiary;
3. the monetary commons settles `ℛ` into `u * Q_int` units of `ℰ`;
4. the member can spend, save, transfer, or exchange the `ℰ` without category restrictions.

This is what makes the Freedom Floor an income rather than only a service voucher.

External currency cash-out is separate. Internal purchasing power can be real even when external market liquidity is shallow. The interface must show both rather than collapsing them into one price.

### 5.4 External exchange rate

For an external asset `a` and trade size `x`, define:

```text
Q_ext(a, x, t) = units of asset a obtainable for x ℰ after fees and market impact
```

`Q_ext` may differ sharply from `Q_int` during early adoption, capital controls, external panic, or thin markets.

Essentia never claims that one thousand `𝒰` can always be cashed out into the national-currency cost of the entire basket. It claims that one thousand `𝒰` should activate the internal purchasing power of the ratified floor. External cash-out grows with markets, reserves, exports, and partners.

### 5.5 Very large nominal settlement

If `Q_int = 1,000,000 ℰ/𝒰`, activating one `𝒰` creates one million `ℰ`.

That number is not automatically a problem.

If every price, wage, balance, and contract uses the same scale, the real relationship can be unchanged. A neutral redenomination can divide all `ℰ` balances, prices, credit limits, and future issuance by a common factor without changing wealth or purchasing power.

The protocol distinguishes:

1. **Redenomination.** A common scaling of all nominal quantities.
2. **Indexed settlement.** More `ℰ` issued because the same `𝒰` claim requires more nominal units.
3. **Additional real entitlement.** More `𝒰` activated than before.
4. **External redemption.** Selling `ℰ` for a currency or asset outside the network.

Only the third necessarily increases the real claim being exercised. The fourth can move an external market. The first is neutral. The second must be evaluated through the real economy rather than the number of zeros.

---

## 6. Why adaptive issuance need not mechanically cause inflation

Minting is an accounting event. Inflation is a persistent change in the price relationship between money and goods.

A minimal identity illustrates the distinction.

Let:

- `A_t` be activated `𝒰` during period `t`, measured in real basket units;
- `Y_t` be deliverable output for those claims;
- `P_t` be `ℰ` per `𝒰`;
- new civic issue be `A_t * P_(t-1)`.

Under the deliberately simple assumption that the new issue is the only spending flow and turns over once, the normalized price path is approximately:

```text
P_t / P_(t-1) ≈ A_t / Y_t
```

If `A_t = Y_t`, the price level can remain stable whether `P` is `1`, `1,000`, or `1,000,000`. The nominal amount created scales with the denomination. Real demand does not.

If `A_t > Y_t`, some combination of higher prices, queues, imports, inventories, rationing, or unmet need must absorb the difference.

If unused capacity exists and production responds, new spending can increase output rather than prices. If the currency is saved, replaces another payment instrument, or settles obligations that already existed, its immediate price effect may also be small.

A production model must add:

- desired money balances and velocity;
- spending and saving differences;
- inventories and imports;
- wages and debt contracts;
- expectations and confidence;
- exchange-rate pass-through;
- sector-specific supply elasticity;
- rents, monopoly power, and strategic pricing;
- provider balance sheets;
- external obligations;
- demographic and ecological limits.

Essentia therefore does not encode a dogma that money growth is harmless. It also does not encode the opposite dogma that each new unit mechanically becomes inflation.

---

## 7. Issuance of Essent

### 7.1 Issuance classes

Every `ℰ` issue belongs to one auditable class.

#### Civic settlement issuance

Outside `ℰ` issued to settle activated `𝒰` through `ℛ`.

This issue has no conventional debtor and no redemption promise attached to the individual `ℰ`. Its economic cost can appear through dilution, price change, exchange-rate pressure, or resource competition. Those effects are measured directly.

#### Equal civic dividend

Outside `ℰ` issued in equal per-person amounts in addition to `𝒰` settlement when governance authorizes a general monetary dividend.

A direct `𝒰` entitlement is the primary real target. A separate equal `ℰ` dividend may help distribute seigniorage and seed universal participation.

#### Mutual credit

Inside `ℰ` created as matched positive and negative balances.

```text
buyer balance  -= x ℰ
seller balance += x ℰ
```

The buyer's negative position is a liability subject to a limit and repayment terms. The seller receives ordinary fungible `ℰ`.

#### Producer credit

Inside `ℰ` issued against credible capacity expansion, purchase orders, inventories, receivables, or standing demand revealed by `ℛ`.

Producer credit is central to stability because it helps supply respond to the demand created by the Freedom Floor.

#### Treasury issuance

Outside or inside `ℰ` used for public procurement, operations, grants, research, and infrastructure under a ratified budget and policy rule.

The record states whether the issue is permanent outside money or expected to be retired through revenue.

#### Market operations

Temporary `ℰ` issuance or retirement through auctions, swaps, savings instruments, collateralized lending, or bridge operations.

### 7.2 Source tagging without monetary caste

Source classes are visible in aggregate and to auditors. They do not create separate spendable tokens. A provider should not need to ask whether a received `ℰ` came from civic issuance or a bank-like credit line.

Risk remains attached to the institution and balance sheet that created an inside-money position. The common settlement layer and guarantee rules preserve monetary singleness while keeping the underlying exposure inspectable.

### 7.3 No fixed-supply religion

A fixed supply is one possible coordination rule. It is not a universal condition of value.

`ℰ` supply must be elastic enough to support adoption, universal income, trade, and productive investment. The relevant limits are not an arbitrary percentage of yesterday's token supply. They are:

- demand for real `ℰ` balances;
- activated `𝒰`;
- deliverable output;
- provider acceptance;
- price stability;
- settlement performance;
- productive capacity;
- external exchange pressure;
- concentration and distribution;
- confidence in future policy.

### 7.4 Seigniorage

Outside civic issuance creates purchasing power for the recipients without creating an equal conventional debt. Its counterpart is seigniorage.

Seigniorage is not free in every state of the economy. Existing holders may bear dilution if demand for balances does not rise. Providers may raise prices if claims exceed supply. The external exchange rate may fall. The policy can also mobilize idle output, distribute the gains from adoption, and reduce dependence on interest-bearing debt.

The distributional rule is constitutional:

- recurring civic issuance is equal per eligible person;
- infrastructure budgets are separate and public;
- founders receive no multiplier;
- validators and operators are paid for work through disclosed budgets;
- no private actor can purchase a larger share of future civic issuance.

### 7.5 Issuance and membership growth

Each new participant adds recurring issue. They can also add demand, labor, production, relationships, and network usefulness.

The protocol measures both sides. Population-proportional issuance is sustainable only where the marginal participant adds or is supported by enough acceptance and capacity. This does not mean a person must earn dignity through production. It means expansion planning must increase supply and solidarity rather than pretending enrollment has no resource effect.

---

## 8. The founding market

A monetary network can fail because everyone waits for everyone else.

Consumers do not value money that stores will not accept. Stores do not accept money that suppliers and workers reject. Suppliers do not accept money with no downstream uses. A slow wallet-by-wallet launch can coordinate rational people on the zero-value equilibrium.

Essentia therefore launches a **founding market**, not merely a token.

### 8.1 Founding commitments

Before real-value issuance begins, a node assembles:

- members ready to receive and use `𝒰` and `ℰ`;
- essential providers quoting standing offers in `𝒰`;
- general merchants accepting defined shares of payment in `ℰ`;
- workers and cooperatives willing to receive some income in `ℰ`;
- suppliers that accept `ℰ` from those providers;
- producer-credit facilities;
- public or cooperative services priced in `𝒰` or `ℰ`;
- exchange and bridge partners;
- conflict, refund, and failure procedures.

The commitments are specific. A grocery cooperative may offer ten thousand `𝒰` of food per month and accept settlement as 70 percent `ℰ`, 20 percent netted supplier obligations, and 10 percent national currency. A repair service may accept 100 percent `ℰ` because most of its costs are internal.

### 8.2 Minimum viable monetary area

A node does not need every industry. It needs enough closed trade loops that recipients can repeatedly use `ℰ` without converting all of it outward.

The protocol maps:

- who buys from whom;
- which suppliers require outside money;
- which categories have idle capacity;
- which obligations can be netted;
- where `ℰ` accumulates without a use;
- which additional provider would close the largest loop.

Launch thresholds are hypotheses and must be tested. The protocol should not pretend one global percentage is known in advance.

### 8.3 Adoption support

Temporary subsidies can move a network across an adoption threshold. They are legitimate when transparent and time-limited.

Examples include:

- bridge guarantees for early providers;
- onboarding and point-of-sale support;
- discounts funded by a public launch budget;
- producer credit for scarce categories;
- liquidity provision on early exchange markets;
- procurement commitments;
- transaction-fee subsidies;
- direct grants for accessibility and training.

A subsidy is not evidence that the unsubsidized system works. The exit schedule and outcome measures are published from the beginning.

### 8.4 Standing offers as real monetary support

The strongest support for `ℰ` is not a reserve statement. It is a dense set of credible offers:

```text
provider p offers quantity q of good g
at price u 𝒰
settled at Q_int * u ℰ
through time t
```

These offers make acceptance observable. They also let Essentia compare activated claims with actual deliverable capacity before the shortage reaches a cash register.

---

## 9. Essential Settlement Receivables

### 9.1 Creation

An `ℛ` record contains:

- receivable identifier;
- beneficiary credential or private commitment;
- activated `𝒰` amount;
- region and index version;
- creation time;
- settlement deadline;
- permitted settlement methods;
- priority class;
- dispute state;
- settlement events;
- final retirement proof.

Protected transaction detail remains off the public ledger. Aggregate issue and aging are public.

### 9.2 Immediate settlement

The default is immediate retirement of `ℛ` through `ℰ` settlement:

```text
SettlementE = ReceivableU * Q_int
```

This may transfer existing `ℰ` or create new civic `ℰ`.

### 9.3 Mixed settlement

A beneficiary may voluntarily select:

- all `ℰ`;
- part `ℰ`, part external currency;
- netting against obligations;
- in-kind inputs;
- a time deposit or savings instrument;
- deferred settlement with explicit compensation.

No provider or member is silently converted into the involuntary lender of last resort.

### 9.4 Receivable aging

Unsettled `ℛ` is a direct warning signal.

The system publishes:

- total outstanding `ℛ`;
- age buckets;
- settlement method;
- category and node concentration;
- disputed amount;
- external-currency share;
- average and tail settlement delay;
- impairment or restructuring.

A stable `ℰ` price does not excuse a growing backlog of `ℛ`.

### 9.5 Receivables as supply signals

Aggregated, privacy-preserving `ℛ` data describes demand that providers have actually served or members are attempting to activate.

Producer-credit allocation can use this signal to finance:

- inventory replacement;
- new equipment;
- housing construction and repair;
- training and hiring;
- energy generation and storage;
- logistics;
- clinic and care capacity;
- import substitution;
- cooperative formation.

This closes a central loop: universal demand becomes information and finance for increased supply.

---

## 10. Monetary observation and control

Essentia does not need one omniscient algorithm. It needs a transparent control system that can distinguish different failures.

### 10.1 Required observations

At minimum:

- internal `ℰ/𝒰` price by region and category;
- external exchange prices with executable depth;
- activated and unactivated `𝒰`;
- `ℛ` creation, settlement, aging, and default;
- provider acceptance and exit;
- goods availability, inventories, wait times, and substitution;
- output, employment, capacity use, and investment;
- `ℰ` balances, concentration, velocity, and dormant holdings;
- mutual-credit exposure and default;
- imports, exports, bridge flows, and currency mismatch;
- rents and monopoly concentration;
- expectations and willingness to accept `ℰ`;
- ecological limits.

### 10.2 No single inflation number

The monetary system publishes at least:

1. a broad `ℰ` price index;
2. the `𝒰` essential-basket index;
3. category shortages and wait times;
4. external exchange rates;
5. distribution-specific cost changes;
6. asset prices separately from consumption prices.

Housing, insulin, food, and a speculative asset cannot be compressed into one sufficient statistic.

### 10.3 Response order

When `ℰ` loses purchasing power or essential prices rise, the first task is diagnosis.

Possible responses include:

1. correct a broken or manipulated index;
2. identify physical shortages and supplier exits;
3. enforce competition and anti-capture rules where prices rose without cost or scarcity;
4. direct producer credit and procurement toward the bottleneck;
5. use inventories, substitutes, imports, and cross-node supply;
6. offer voluntary savings instruments to absorb balances temporarily;
7. change the mix of civic issue, credit issue, and bridge settlement;
8. slow optional large conversions when they destabilize external markets;
9. alter activation cadence while preserving accrued entitlement;
10. use transparent need-based allocation during true physical shortage;
11. revise issuance parameters if real claims persistently exceed the economy's ability to respond.

Cutting the Freedom Floor is not the automatic first response to a price increase. Sometimes the correct response is to build the thing whose price rose.

### 10.4 Monetary policy rule

No single formula is constitutional. A production system should support competing published controllers and shadow-test them against the same data.

A controller may target a vector such as:

```text
Target = {
  internal price stability,
  essential availability,
  ℛ settlement latency,
  broad acceptance,
  sustainable capacity growth,
  bounded external pressure,
  equitable distribution
}
```

Policy actions and model forecasts are recorded before outcomes are known. Emergency discretion is time-limited and reviewable.

### 10.5 Neutral rebase

A neutral rebase may keep nominal `ℰ` amounts readable.

If the system applies factor `z`, it multiplies or divides all:

- balances;
- posted prices;
- wages;
- debts and credit limits;
- contract amounts;
- future issuance parameters;
- index conversion values.

A rebase does not stabilize real value and must never be reported as doing so. It only changes the unit scale.

---

## 11. External exchange and the Liquidity Bridge

### 11.1 External conversion is a market service

A holder may offer `ℰ` for dollars, euros, bitcoin, another node's money, or another asset. The obtainable price depends on counterparties, depth, fees, regulation, and confidence.

The bridge may improve this market, but it does not promise infinite conversion.

### 11.2 Executable external price

The external price for size `x` uses:

- executed trades;
- quoted depth;
- time-weighted prices;
- venue diversity;
- related-party and wash-trade filters;
- fees and slippage;
- withdrawal reliability;
- legal access;
- currency risk;
- fallback auctions.

A last-trade number from a thin venue is not purchasing power.

### 11.3 Catalytic reserve

A reserve can:

- reassure early providers;
- settle imports;
- make markets during launch;
- insure narrow failure modes;
- finance procurement;
- smooth temporary external imbalance.

It does not need to equal all `ℰ` or all future `𝒰`. Full reserve backing would convert Essentia into a wrapper around money raised elsewhere and would abandon the monetary hypothesis before testing it.

Reserve use is public, bounded, and designed to decline relative to internal trade if the hypothesis succeeds.

### 11.4 Conversion corridor

A node may quote a bounded external bid and offer:

- published size limits;
- visible reserves and inventory;
- wider spreads under stress;
- auctions for large orders;
- automatic pause before insolvency;
- no claim that the corridor is the internal `𝒰` guarantee.

### 11.5 External imbalance

Persistent net imports require some combination of:

- exports;
- grants or transfers;
- investment;
- borrowing;
- reserve drawdown;
- import substitution;
- lower external consumption;
- negotiated cross-node support.

New `ℰ` alone cannot make an outside seller accept it. It can help build the production and network that gives the seller a reason to accept it later.

---

## 12. Failure states

### 12.1 Denomination growth

`Q_int` rises while availability, settlement, acceptance, and real claims remain stable.

This may be a nominal-scale problem. Verify the index and consider a neutral rebase.

### 12.2 Demand exceeds supply

Activated `𝒰` persistently exceeds deliverable output.

Symptoms include essential price pressure, shortages, queues, provider overload, import growth, or falling quality. Respond with supply finance, procurement, anti-capture policy, substitution, and explicit allocation where unavoidable. Reconsider activation or issue only when the gap cannot close.

### 12.3 Acceptance collapse

Providers and members expect others to stop accepting `ℰ`, so they attempt to spend or sell it immediately. Their response makes the expectation true.

Responses may include:

- credible standing offers;
- coordinated provider recommitment;
- bridge liquidity;
- temporary savings incentives;
- correction of governance failure;
- restoration of useful services priced in `ℰ`;
- bounded issuance changes;
- a new unit or node migration if trust in the issuer cannot be repaired.

### 12.4 Internal and external divergence

`ℰ` retains internal purchasing power but trades cheaply outside.

Do not mint according to the external rate for internal UBI. Diagnose thin liquidity, capital controls, external fear, trade imbalance, and market manipulation separately.

### 12.5 Receivable crisis

`ℛ` accumulates faster than settlement.

This is a direct failure even if wallets still display balances. Pause new commitments in the affected category where necessary, protect existing beneficiaries, finance providers, and publish the loss allocation.

### 12.6 Credit crisis

Mutual or producer-credit defaults impair `ℰ` holders or clearing institutions.

Use declared loss order:

1. borrower collateral and receivables;
2. borrower equity or loss share;
3. underwriting reserve;
4. guarantee pool;
5. node capital;
6. federation loss facility within a cap;
7. transparent impairment or restructuring.

Civic voting rights and future `𝒰` do not disappear because of insolvency.

### 12.7 Oracle failure

Price or availability data becomes stale, manipulated, or unavailable.

Freeze the affected calculation, fall back to independent sources or auctions, preserve accrued entitlements, and publish the uncertainty. Do not invent a precise rate.

### 12.8 Identity attack

Duplicate or synthetic identities capture equal issuance.

Quarantine the disputed issuance, preserve essential continuity for real people, investigate with privacy-preserving evidence, and provide appeal. Do not turn fraud prevention into permanent surveillance.

---

## 13. Identity, eligibility, and recovery

### 13.1 Purpose-bound credentials

Essentia does not require one public identifier for every part of life.

A person may use different unlinkable credentials for:

- recurring `𝒰` issuance;
- governance eligibility;
- provider authorization;
- professional qualification;
- age or residency claims;
- recovery;
- research participation.

The issuer or verifier learns only what the function requires.

### 13.2 Personhood

Universal per-person issuance requires strong resistance to duplicate enrollment. No known mechanism solves global proof of unique living personhood without tradeoffs.

The architecture supports combinations of:

- in-person community verification;
- document evidence;
- device and key continuity;
- existing trusted institutions;
- web-of-trust attestations;
- privacy-preserving uniqueness proofs;
- anomaly detection;
- challenge and appeal;
- periodic renewal appropriate to risk.

Biometrics are never the sole path. Raw biometric templates and government document images do not belong on a public chain.

### 13.3 One-use nullifiers

A personhood credential can derive a context-specific nullifier proving that the same eligible credential has not claimed twice during a period without exposing the person's general identity.

Each program uses a separate context so proofs cannot become a universal tracking identifier.

### 13.4 Recovery

Lost keys must not mean lost personhood or lost income.

Recovery supports:

- multiple devices;
- passkeys for ordinary sessions;
- offline recovery material;
- guardian quorums;
- institutional assistance;
- time delays and alerts;
- protected emergency continuity;
- appeal against hostile recovery.

No single guardian can seize identity or funds.

### 13.5 Separation from reputation

Eligibility proves a limited fact. It does not create a public score of human worth.

Provider reliability, credit performance, and professional authorization remain contextual. They do not alter equal civic issue or baseline voting rights.

---

## 14. Governance and monetary authority

### 14.1 Founding Book and Charter

Part B defines the current scientific and economic constraints. The Charter states the rights, institutions, and amendment rules derived from those constraints. Essentia implements ratified mechanisms.

The protocol does not become constitutional merely because code exists.

### 14.2 Monetary constitution

The Charter must define:

- who receives `𝒰`;
- the equality rule;
- basket rights and revision process;
- issuance classes;
- limits on founder and operator power;
- public data and privacy boundaries;
- monetary roles and conflicts;
- ordinary parameter changes;
- emergency authority and expiry;
- independent audit;
- appeal and remedy;
- node withdrawal and resolution;
- amendment requirements.

### 14.3 Monetary Council

A node may maintain a Monetary Council with bounded authority to execute ratified policy.

Membership should combine:

- elected or sortition-selected residents;
- providers and workers;
- recipients;
- monetary and production specialists;
- independent auditors;
- accessibility and rights advocates;
- non-voting model and data stewards.

Terms are limited. Conflicts are public. Removal and appeal exist. No council member can mint unilaterally.

### 14.4 Algorithms advise; accountable institutions decide

Models can calculate rates, forecast scenarios, detect anomalies, and recommend actions. Binding policy follows an authorized rule or recorded human decision.

The system publishes:

- model version;
- inputs;
- uncertainty;
- recommendation;
- decision;
- dissent;
- later outcome.

### 14.5 Decision methods by decision type

No single voting method governs everything.

- constitutional changes require broad, high-threshold ratification;
- technical parameters may use delegated expertise with review;
- budgets may use participatory allocation;
- emergency action may use narrow temporary authority;
- local operations remain local where external effects are limited;
- affected groups receive standing and voice.

Token wealth never becomes voting power.

---

## 15. Ledger architecture

### 15.1 Start with the trust problem

Essentia needs shared state where independent nodes can issue money, verify uniqueness, settle claims, and prevent double use. At global scale, a Byzantine fault tolerant ledger may be justified.

The research sequence still begins with simpler components so economic failure is not hidden by consensus engineering.

### 15.2 Stage 1: signed transparency log

The first implementation uses:

- double-entry accounting;
- signed events;
- append-only Merkle commitments;
- independent witnesses;
- reproducible index and issuance calculations;
- explicit corrections and reversals;
- ordinary databases for private operational data.

### 15.3 Stage 2: federated replicated log

When multiple independent nodes share issuance and clearing authority, they replicate canonical public events and cross-sign checkpoints. Conflicting histories are visible.

### 15.4 Stage 3: Byzantine fault tolerant finality

When the validator set and threat model require automated finality despite malicious or unavailable operators, the network adopts an audited BFT protocol.

Requirements include:

- deterministic state transitions;
- explicit validator admission and rotation;
- no wealth-weighted consensus;
- public fault evidence;
- bounded emergency pause;
- reproducible builds;
- independent clients;
- migration and recovery procedures.

### 15.5 State separation

Public consensus stores only what requires global agreement:

- issuance totals and commitments;
- spent-nullifier commitments;
- policy and model hashes;
- index commitments;
- aggregate `ℛ` state;
- node and validator state;
- governance outcomes;
- cross-node settlement;
- audit and release commitments.

Personal transactions, identities, invoices, ballots, health data, and protected cases remain encrypted, local, or selectively disclosed.

### 15.6 No arbitrary contract platform at launch

The monetary core uses audited native modules or narrowly scoped programs. An unrestricted contract environment would enlarge the attack surface and enable financial structures that can threaten the Freedom Floor.

### 15.7 Cryptographic agility

Algorithms are versioned. Keys and proofs have migration paths. No permanent monetary right depends on one cryptographic primitive remaining safe forever.

---

## 16. Oracles and market truth

### 16.1 Observation record

Every price or availability observation includes:

- source;
- item and quality specification;
- location;
- time;
- quantity and trade size;
- posted or executed status;
- currency;
- fees and conditions;
- signer;
- confidence and dispute state.

### 16.2 Aggregation

Aggregation uses published robust methods such as medians, trimmed estimators, venue weighting, stale-data rejection, and uncertainty bands.

No single merchant, exchange, government feed, or machine model controls the rate.

### 16.3 Actual availability

A posted price for an unavailable good is not a useful price. The index incorporates stock, delivery time, wait lists, substitution quality, and quantity limits.

### 16.4 Challenge

Members and providers can challenge observations or methodology. Material disputes trigger independent sampling or a temporary fallback calculation.

### 16.5 Cryptography cannot prove the world

A signature proves who submitted an observation. A commitment proves it was not changed. Neither proves that the shelf contained the item or that the transaction was genuine.

Oracle governance remains social, adversarial, and accountable.

---

## 17. Privacy and data governance

### 17.1 Data minimization

Collect the minimum data necessary for each function. Delete or expire data when the function no longer requires it.

### 17.2 Institutional observability

The public can inspect:

- monetary rules;
- issuance by class;
- aggregate distribution;
- price-index methodology;
- model versions;
- `ℛ` aging and settlement;
- reserves and bridge use;
- provider concentration;
- emergency actions;
- audits and incidents.

### 17.3 Personal boundaries

The public does not receive:

- named purchase histories;
- household composition;
- health or disability detail;
- precise location trails;
- private ballots;
- recovery relationships;
- raw identity evidence;
- protected conflict records.

### 17.4 Research access

Research datasets use de-identification, secure enclaves, differential privacy where appropriate, purpose-bound access, publication review, and penalties for re-identification.

Participants retain rights to explanation, correction, withdrawal where feasible, and remedy.

---

## 18. Research and rollout

No phase is automatic. Each phase exists to answer a different question.

### Phase 0: formal specification and simulation

Build:

- stock-flow-consistent models;
- agent-based economies;
- network-adoption models;
- provider and household balance sheets;
- internal and external markets;
- identity attack models;
- controller comparisons;
- adversarial scenarios.

The initial executable scaffold is [`research/monetary_dynamics.py`](../research/monetary_dynamics.py). It demonstrates denomination invariance, real shortage pressure, supply response, producer credit, and adoption thresholds. It is illustrative, not predictive.

### Phase 1: founding market and shadow ledger

Recruit a real network of members, providers, suppliers, workers, and bridge partners. Publish standing offers in `𝒰`. Run `𝒰 -> ℛ -> ℰ` calculations without relying on the balances for survival.

Question: Is there enough reciprocal structure to support actual use?

### Phase 2: outside `ℰ` monetary experiment

Issue equal outside `ℰ` to verified participants without attaching a conventional repayment liability. Enable real voluntary purchases under strict exposure limits.

Question: Can coordinated acceptance, usefulness, and transparent issuance create positive internal purchasing power?

### Phase 3: indexed Freedom Floor pilot

Issue a partial recurring `𝒰` entitlement. Allow both provider payment and unrestricted conversion through `ℛ`. Settle primarily in `ℰ`, with the bridge available but not silently maintaining all value.

Question: Does the adaptive conversion preserve real access without creating an unstable price loop?

### Phase 4: supply-coupled expansion

Use `ℛ`, shortages, standing offers, and provider data to issue producer credit and finance capacity.

Question: Does additional demand become additional output quickly enough to support a larger entitlement?

### Phase 5: external exchange

Open bounded exchange markets and bridge facilities. Measure internal/external divergence, reserve dependence, exports, imports, and conversion demand.

Question: Can `ℰ` gain outside value without making external redemption its only reason to exist?

### Phase 6: recurring full-floor pilot

Operate the complete monthly `𝒰` target for a bounded population and sufficiently covered region, with independent evaluation and legal protections.

Question: Can the system deliver a real UBI continuously?

### Phase 7: federation

Connect independent nodes with cross-node `ℰ` acceptance, multilateral clearing, exposure limits, migration, and resolution.

Question: Can local positive-value equilibria become a resilient global economy without one center owning every identity or monetary decision?

---

## 19. Required metrics

### 19.1 Real outcome

- percent of the ratified floor actually obtainable;
- unmet need by category;
- recipient autonomy and reported usefulness;
- housing, food, care, energy, and transport availability;
- time and humiliation cost of access;
- distribution across income, disability, household, and geography.

### 19.2 Monetary

- `Q_int` and its volatility;
- broad and essential inflation in `ℰ`;
- internal/external exchange divergence;
- desired and actual `ℰ` balances;
- velocity and concentration;
- equal issue per person;
- seigniorage distribution;
- mutual and producer-credit exposure;
- default and impairment;
- provider acceptance and exit.

### 19.3 Settlement

- `ℛ` created, settled, aged, disputed, and impaired;
- median and tail settlement time;
- settlement mix;
- bridge use;
- external-currency leakage;
- obligation netting;
- provider satisfaction and solvency.

### 19.4 Production

- capacity use;
- output response;
- investment;
- inventory;
- bottleneck duration;
- imports and exports;
- new provider formation;
- market concentration;
- ecological throughput.

### 19.5 Adoption and trust

- active users and providers;
- repeat use;
- network reachability;
- standing offer coverage;
- willingness to accept `ℰ` at different shares;
- expected future acceptance;
- reasons for exit;
- technical friction and accessibility.

### 19.6 Governance and technical integrity

- issuance-rule compliance;
- index reproducibility;
- unexplained overrides;
- appeal latency;
- privacy incidents;
- identity false rejection and duplicate enrollment;
- validator or witness faults;
- independent verifier agreement;
- emergency-power duration.

---

## 20. Falsification and redesign conditions

The research program does not need a single dramatic test that permanently declares success or failure. It needs identifiable mechanisms.

A stage has failed in its current form when, after reasonable correction:

- people hold `ℰ` only to sell it to subsidized buyers;
- provider acceptance does not become reciprocal;
- `ℛ` cannot settle on terms providers knowingly accept;
- internal purchasing power falls faster as indexed civic issuance rises;
- real supply does not respond and shortages become persistent;
- external conversion consumes the bridge faster than internal use grows;
- duplicate identity overwhelms equal issuance;
- monetary governance is captured;
- the system depends on surveillance or humiliation;
- losses are hidden in providers, members, or future issuance;
- another architecture delivers the same rights more safely.

Failure of one mechanism does not prove that trust-based money is impossible. It identifies what did not create or preserve the positive-value equilibrium.

The response may be to change:

- launch sequence;
- provider commitments;
- issuance composition;
- index design;
- supply finance;
- governance;
- identity;
- bridge design;
- monetary controller;
- node scale;
- or the role of `ℛ`.

The hypothesis becomes stronger by surviving attempts to break it, not by being protected from them.

---

## 21. Implementation priorities

The next code should implement the questions that matter most.

1. A double-entry accounting core that distinguishes civic outside issue, mutual credit, producer credit, `𝒰`, and `ℛ`.
2. A reproducible `𝒰` basket and index engine with uncertainty and availability.
3. The `𝒰 -> ℛ -> ℰ` activation and settlement state machine.
4. Standing provider offers priced in `𝒰`.
5. Internal `Q_int` calculation from actual offers and transactions.
6. Aggregate `ℛ` aging and settlement reports.
7. Producer-credit experiments linked to demand and capacity.
8. A founding-market graph that identifies closed loops and external leakage.
9. Simulation and replay before any real issuance.
10. Purpose-bound personhood credentials and one-use nullifiers.
11. Signed public event logs and independent verifiers.
12. Governance records, parameter versioning, and emergency expiry.
13. External exchange and reserve modules only after internal accounting works.
14. BFT consensus only after independent nodes need shared finality.

The v0.1.0 prototype is useful as code to inspect and test. Its two-asset state machine does not implement this architecture and should not define the research question.

---

## 22. Open questions

1. What founding-market coverage is enough to cross the acceptance threshold?
2. How much real `ℰ` balance demand emerges from equal civic issuance?
3. Which universal issue cadence produces stable use rather than immediate exit?
4. How should `Q_int` combine posted offers, completed trades, shortages, and quality?
5. Does a `𝒰` unit of account reduce money illusion and bargaining friction?
6. How much unrestricted conversion can occur before essential categories lose supply?
7. How quickly can producer credit turn `ℛ` demand into output?
8. Which sectors absorb demand through output, imports, price, queues, or rent capture?
9. Can internal value remain stable during external exchange volatility?
10. How much bridge liquidity is catalytic, and when does it become hidden backing?
11. Can a single fungible `ℰ` preserve source-specific credit risk without fragmenting use?
12. Which savings instruments stabilize velocity without favoring wealth?
13. How should neutral rebases treat offline devices and long-duration contracts?
14. Which identity design resists duplicates without creating a global surveillance key?
15. Can local issuance federate while preserving equal per-person shares?
16. What loss rules preserve trust without socializing every private risk?
17. What governance structure makes adaptive policy credible without becoming technocratic?
18. What evidence would show that a different monetary architecture is better?

---

## 23. Conclusion

Essentia does not begin from the premise that value must already exist somewhere else before a community can create money.

It begins from the fact that money is one of humanity's coordination technologies. A unit has value because people can use it and expect one another to keep using it. Institutions, production, law, trust, habit, memory, and network effects make that expectation durable.

Software can help create those conditions. It can give every person the same recurring monetary claim. It can make counterfeit issuance difficult. It can make the rules common knowledge. It can let providers quote stable real prices while settlement money floats. It can turn completed demand into visible receivables and finance. It can find trade loops, net obligations, expose bottlenecks, coordinate simultaneous adoption, and show when the promise is breaking.

That can create purchasing power.

It does not make physical limits disappear. It makes the relationship between money, claims, production, and trust visible enough to govern deliberately.

The target is not a token that resembles existing money.

The target is a monetary commons in which:

- every person receives a real Freedom Floor;
- `𝒰` keeps the promise stable;
- `ℛ` keeps settlement honest;
- `ℰ` carries value through a growing network;
- production responds to human need;
- seigniorage belongs equally to people;
- and failure becomes information rather than abandonment.

Whether this works is not settled by confidence or rejection.

It is settled by building the smallest honest version, measuring what happens, learning faster than the failure compounds, and continuing until the mechanism works or a better one is found.

---

## Research basis

- [Part B: The Science of Renewal](https://github.com/SocietyOfRenewal/societyofrenewal/blob/main/docs/founding-book/Part%20B.md)
- European Central Bank. ["The role of trust in money and monetary institutions."](https://www.ecb.europa.eu/press/key/date/2000/html/sp001026_2.en.html) (2000).
- Philip R. Lane. ["The digital euro: maintaining the autonomy of the monetary system."](https://www.ecb.europa.eu/press/key/date/2025/html/ecb.sp250320_1~41c9459722.en.html) European Central Bank (2025).
- Willem H. Buiter. [*Helicopter Money: Irredeemable Fiat Money and the Liquidity Trap*](https://doi.org/10.3386/w10163) (2003).
- Narayana R. Kocherlakota. ["Money Is Memory."](https://doi.org/10.1006/jeth.1997.2357) (1998).
- Roger E. A. Farmer. [*Money in a Heterogeneous Agent Model*](https://doi.org/10.3386/w32836) (2024).
- Jesús Fernández-Villaverde and Daniel Sanches. ["Can Currency Competition Work?"](https://doi.org/10.3386/w22157) (2016; published 2019).
- Gabriele Camera and Marco Casari. ["The Coordination Value of Monetary Exchange: Experimental Evidence."](https://doi.org/10.1257/mic.6.1.290) (2014).
- Satoshi Nakamoto. [*Bitcoin: A Peer-to-Peer Electronic Cash System*](https://bitcoin.org/bitcoin.pdf) (2008).
- Lin William Cong, Ye Li, and Neng Wang. [*Tokenomics: Dynamic Adoption and Valuation*](https://doi.org/10.3386/w27222) (2020).
- Fernando E. Alvarez et al. [*Strategic Complementarities in a Dynamic Model of Technology Adoption: P2P Digital Payments*](https://doi.org/10.3386/w31280) (2023).
- Dennis Egger et al. ["General Equilibrium Effects of Cash Transfers: Experimental Evidence from Kenya."](https://doi.org/10.3982/ECTA17945) (2022).
- Helge Berger, Sune Karlsson, and Pär Österholm. [*A Note of Caution on the Relation Between Money Growth and Inflation*](https://doi.org/10.5089/9798400244834.001) (2023).
- Claudio Borio, Boris Hofmann, and Egon Zakrajšek. ["Does money growth help explain the recent inflation surge?"](https://www.bis.org/publ/bisbull67.htm) (2023).
- Robert J. Shiller. [*Indexed Units of Account: Theory and Assessment of Historical Experience*](https://doi.org/10.3386/w6356) (1998).
- Robert J. Shiller. [*Designing Indexed Units of Account*](https://doi.org/10.3386/w7160) (1999).
- Alessandro Longo et al. [*Impact of a Blockchain-based Universal Basic Income Pilot: The Case of Circles UBI Currency*](https://arxiv.org/abs/2504.02714) (2025, revised 2026).
- [Duniter and Ğ1 documentation](https://duniter.org/g1/).
- [The Encointer Book](https://book.encointer.org/).
- James Stodder. ["Complementary credit networks and macroeconomic stability: Switzerland's Wirtschaftsring."](https://doi.org/10.1016/j.jebo.2009.06.002) (2009).
- Tomaž Fleischman, Paolo Dini, and Giuseppe Littera. ["Liquidity-Saving through Obligation-Clearing and Mutual Credit."](https://doi.org/10.3390/jrfm13120295) (2020).
