# Essentia Research Architecture

## Version 0.8.0 - Derived from Part B

Status: research specification, not a production protocol
Date: 2026-09-03
Scope: civic evidence, entitlement, clearing, and accountable coordination for the Society of Renewal

---

## 0. Status

Essentia does not yet exist as a public economic system.

The repository contains a runnable v0.1.0 Rust prototype. That prototype demonstrates signed accounts, replicated nodes, budget objects, contribution claims, two asset types, and basic transaction flows. It does not demonstrate a viable currency, a Freedom Floor, proof of unique personhood, private binding elections, production consensus, or a lawful global payment network.

This architecture is derived from [Part B of the Founding Book](https://github.com/SocietyOfRenewal/societyofrenewal/blob/main/docs/founding-book/Part%20B.md).

Its monetary core separates three claims that must never be confused:

- `𝒰`, the Essential Unit, represents a person's indexed entitlement to essential purchasing power;
- `ℛ`, the Essential Settlement Receivable, records what a provider is owed after delivering an essential good or service;
- `ℰ`, Essent, is a transferable mutual-credit and settlement instrument.

A conversion formula can calculate how many units of `ℰ` correspond to a target amount at a quoted price. It cannot guarantee that the resulting `ℰ` will buy the promised essentials. If `ℰ` loses most of its usable market value, issuing a larger quantity may increase sell pressure and accelerate the collapse. The guarantee therefore attaches to delivered essentials, provider capacity, and funded settlement rather than an unlimited token conversion.

The central rule is:

> Software may enforce a claim. A society must make the claim true.

No balance displayed in a wallet counts as a Freedom Floor until a person can exchange it for real food, shelter, healthcare, communication, transport, energy, safety, or flexible spending power at the promised level.

---

## 1. Purpose

Essentia is a proposed protocol suite for six kinds of coordination:

1. publishing rules, evidence, budgets, and institutional actions in tamper-evident form;
2. proving limited facts about identity and eligibility without exposing a whole life;
3. recording democratic decisions without making one voting method universal;
4. clearing reciprocal obligations and mutual credit;
5. administering a real-need entitlement called Essential Units;
6. measuring whether the Society is keeping its promises.

Essentia is not one blockchain that must contain everything.

It may eventually include replicated Byzantine fault tolerant ledgers where multiple mutually distrustful operators need shared write authority. Early versions should use the simplest architecture that can produce independently verifiable records, correct errors, protect private data, and survive operator failure.

Essentia is not:

- a speculative general-purpose chain;
- an algorithmic stablecoin;
- a promise that token appreciation will finance universal income;
- a proof-of-stake government;
- a public reputation system;
- a universal identity number;
- an autonomous decision-maker;
- a substitute for contracts, reserves, providers, institutions, law, or productive capacity.

The protocol serves the Society. The protocol is replaceable.

---

## 2. Constitutional invariants

The following properties constrain every implementation.

### 2.1 Dignity

No protocol function may make access to food, shelter, safety, appeal, or personhood depend on public humiliation, wealth, reputation, political agreement, or technical competence.

### 2.2 Reality before denomination

A monetary claim is measured by what it obtains, not by the number of units issued.

### 2.3 Privacy for people, transparency for power

Institutional rules, budgets, issuance, settlement exposures, oracle inputs, and official actions must be inspectable. Personal data, ballots, recovery secrets, health information, household circumstances, and restorative records must be minimized and compartmentalized.

### 2.4 Explicit liabilities

Every issued monetary or entitlement claim must identify the institution that owes performance, the resources available for settlement, and the loss process if performance fails.

### 2.5 No wealth-weighted civic rights

ℰ balances, stake, contribution scores, vendor volume, validator bonds, and office may not increase baseline voting rights.

### 2.6 Reversibility

Pilots must be bounded. Monetary parameters must have circuit breakers. Emergency powers must expire. Affected people must have appeal and remedy.

### 2.7 Verifier first

A binding rule, tally, distribution, or settlement calculation is not ready until an independent implementation can reproduce it from authorized inputs.

---

## 3. The system model

Essentia separates functions whose trust, privacy, accounting, and failure requirements differ.

### 3.1 Public evidence plane

The public evidence plane records:

- canonical policy texts and revisions;
- proposal and decision identifiers;
- public budgets and appropriations;
- issuance totals by authority and purpose;
- aggregate entitlement liabilities;
- reserve and coverage attestations;
- provider credentials and status commitments;
- oracle inputs that can safely be public;
- software release hashes;
- audit reports;
- emergency actions and expiration times;
- aggregate performance and failure metrics.

The first implementation should be a signed append-only Merkle log with independent witnesses. [RFC 9162](https://www.rfc-editor.org/rfc/rfc9162) demonstrates the relevant pattern: clients can verify inclusion and consistency without treating the log operator as incapable of lying.

### 3.2 Private operational plane

The private operational plane contains:

- account balances;
- household and eligibility records;
- provider invoices;
- detailed transaction data;
- protected evidence;
- ballot ciphertexts or local ballot records;
- recovery information;
- risk and underwriting files.

These records do not belong in a globally replicated public database. Access is purpose-bound, logged, time-limited where possible, and subject to independent review.

### 3.3 Economic clearing plane

The clearing plane performs double-entry accounting for:

- Essent balances and credit limits;
- obligations between members and providers;
- Essential Settlement Receivables;
- treasury liabilities and assets;
- reserve movements;
- inter-node positions;
- defaults, restructurings, and loss allocation.

### 3.4 Identity and eligibility plane

This plane issues purpose-specific credentials and one-use nullifiers. It does not attempt to represent a whole person with one public identifier.

### 3.5 Decision plane

This plane supports different decision mechanisms under a common record format. It records what question was asked, who was eligible, what procedure was used, what evidence was supplied, how the result was calculated, who had implementation authority, and when the decision must be reviewed.

### 3.6 Research plane

The research plane stores preregistered hypotheses, analysis plans, outcome definitions, de-identified datasets where lawful, code, deviations, results, and negative findings.

The research plane is part of the protocol because the Society's ability to correct itself is a production requirement.

---

## 4. The three economic layers

The economic design has three claim layers and one supporting liquidity facility.

### 4.1 Essential Unit (`𝒰`): the entitlement layer

`𝒰` is a non-transferable claim held by a person. Its value is defined by a regional essential basket, not by the market price of `ℰ`.

`𝒰` answers:

> What real minimum has this person been promised?

A person spends `𝒰` directly with an eligible provider. `𝒰` is not a speculative asset and is not transferable between people. It expires or rolls forward only under publicly defined rules because its purpose is current access to essentials rather than accumulation.

### 4.2 Essential Settlement Receivable (`ℛ`): the bridge layer

`ℛ` is created after a provider delivers a good or service in exchange for `𝒰`.

`ℛ` answers:

> What does the clearing system owe this provider for honoring the entitlement?

Each `ℛ` identifies the issuer, underlying transaction, basket-reference value, settlement priority, maturity, and permitted settlement mix. It remains an explicit liability until settled or transparently restructured.

`ℛ` is the bridge between a person's entitlement and the provider-facing settlement system. It prevents the recipient from bearing liquidity risk at the point of use and prevents the provider's claim from disappearing into an unsupported token mint.

### 4.3 Essent (`ℰ`): the settlement layer

`ℰ` is a transferable unit used to settle reciprocal obligations, trade, funded grants, payroll, and provider receivables.

`ℰ` answers:

> What transferable claim can circulate through this economic network?

The initial form of `ℰ` should be mutual credit rather than a scarce bearer token. An authorized participant may spend into a negative balance up to a reviewed credit limit. The seller receives an equal positive balance. The gross ability to transact expands while the system's net position remains balanced before fees, public issuance, reserves, and losses.

`ℰ` can create useful purchasing power where participants have reciprocal demand, idle capacity, future income, and confidence in the clearing institution. It cannot create missing food, housing, medicine, electricity, or imported goods that nobody can provide.

### 4.4 External Liquidity Bridge

The External Liquidity Bridge holds or arranges the assets and relationships required to settle `ℛ` obligations outside the `ℰ` network.

It may include:

- national-currency deposits;
- short-duration liquid reserves;
- committed bank or cooperative credit lines;
- grants and donations;
- pooled member contributions;
- cooperative profits and commons revenue;
- insured payment facilities;
- contracts with external providers;
- cross-node settlement agreements.

It answers:

> How will internal claims reach goods and institutions that do not accept `ℰ`?

The External Liquidity Bridge is a balance sheet and a set of enforceable relationships. It is not an oracle and it does not replace `ℛ`; it supplies one group of settlement resources behind `ℛ`.

---

## 5. Essential Units

### 5.1 Unit of account

For region `r`, household type `h`, category `k`, and period `t`, define:

```text
BasketCost(r, h, k, t)
```

as the observed cost of the ratified essential basket for that category.

The complete floor is:

```text
FloorCost(r, h, t) = sum_k BasketCost(r, h, k, t)
```

The basket may include:

- food and household necessities;
- safe shelter;
- energy and water;
- basic healthcare and medication;
- communication;
- transport required for ordinary participation;
- clothing and personal care;
- disability and accessibility costs;
- childcare or dependent care where relevant;
- a flexible allowance that preserves personal agency.

A purely restricted benefit is not equivalent to cash. The basket therefore needs a discretionary component, and the Society should separately fund an unrestricted Autonomy Dividend when resources permit.

### 5.2 Basket governance

The basket is both empirical and normative.

Price observation can tell us what selected goods cost. It cannot decide what a dignified minimum includes. That decision requires public deliberation, rights analysis, lived experience, accessibility expertise, and periodic review.

Each regional basket must publish:

- included goods and services;
- quantities and minimum quality;
- acceptable substitutions;
- geographic boundary;
- household-equivalence rules;
- accessibility adjustments;
- price-source list;
- observation timestamps;
- availability rules;
- weighting method;
- dispute process;
- review date.

The calculation must be reproducible from authorized source data.

### 5.3 Price and availability inputs

No single source controls the basket.

Inputs may include:

- official consumer-price series;
- regional price-parity data;
- provider catalog prices;
- actual transaction receipts;
- rental and utility observations;
- healthcare and transport schedules;
- independent field sampling;
- stockout and waiting-time data;
- household expenditure surveys;
- participatory minimum-income studies.

A low price for an unavailable item is not a low cost of living.

### 5.4 Issuance

For member `i`:

```text
𝒰_issue(i, t) = coverage(i, t) * FloorCost(region_i, household_i, t)
```

`coverage` must be published.

A pilot covering 20 percent of a basket must say 20 percent. The interface may not call a partial experiment a complete Freedom Floor.

𝒰 issuance creates a liability of the Essential Clearing Fund. The system must recognize that liability immediately, not only when the 𝒰 is spent.

### 5.5 Spending and provider receivables

A provider accepts 𝒰 only under a published contract.

When the provider supplies an eligible good or service:

```text
Member 𝒰 balance        decreases
Provider ℛ             increases
Clearing Fund liability  moves from unspent 𝒰 to payable ℛ
```

An Essential Settlement Receivable (`ℛ`) is a provider claim denominated in the same basket-reference value as the transaction.

The provider does not receive an unlimited automatic mint of `ℰ`.

### 5.6 Provider protections

Provider participation must be voluntary and contractually clear.

Each provider contract defines:

- categories;
- price rules;
- service standards;
- settlement mix;
- settlement timing;
- voluntary ℰ acceptance share;
- external-currency share;
- netting rights;
- credit exposure;
- dispute resolution;
- audit requirements;
- exit and continuity terms.

The Society must not fund dignity for recipients by quietly making small providers involuntary creditors.

---

## 6. Settlement waterfall

Every ℛ is settled through a published waterfall.

### Step 1: obligation netting

The clearing engine identifies reciprocal obligations.

If a provider owes ℰ, fees, rent, supplies, payroll, energy, taxes where lawful, or other eligible obligations inside the network, those obligations may be netted against the ℛ with consent and complete accounting.

Netting reduces outside liquidity needs without pretending the underlying goods were free.

### Step 2: mutual-credit ℰ

The provider may accept a contracted share in `ℰ`.

This settlement creates or transfers ℰ under the provider's credit terms. The provider must have realistic ways to spend ℰ, repay ℰ liabilities, or convert a bounded share later.

### Step 3: treasury ℰ

The treasury may use positive `ℰ` balances acquired from fees, revenues, sales, repayments, or funding received before the settlement epoch.

### Step 4: external reserve settlement

The External Liquidity Bridge pays the external-currency share.

### Step 5: direct procurement or in-kind clearing

The network may satisfy part of the provider claim through supplies, energy, transport, facilities, or other contracted inputs.

### Step 6: voluntary deferred receivable

A provider may voluntarily accept a time-bounded deferred claim with a published yield or discount.

The recipient's entitlement is not reversed because the clearing system is late. The provider receives the agreed claim and the shortfall becomes an institutional liability.

### Prohibited step: involuntary infinite ℰ mint

The protocol may not respond to missing settlement capacity by minting whatever quantity of ℰ an external price formula demands.

---

## 7. Coverage and solvency

Aggregate value is not enough. A fund can look solvent in currency terms while lacking housing, medicine, or food in a specific region.

Essentia therefore tracks at least three kinds of coverage.

### 7.1 Category coverage

For category `k`, region `r`, and horizon `t`:

```text
CategoryCoverage(k, r, t) =
    ContractedDeliverableCapacity(k, r, t)
    / ExpectedEUClaims(k, r, t)
```

This must be measured in physical or service units where possible.

Examples:

- meals;
- bed-nights;
- kilowatt-hours;
- medication courses;
- clinical appointments;
- transit trips;
- data-service months.

### 7.2 External liquidity coverage

```text
ExternalLiquidityCoverage(t) =
    EligibleLiquidExternalAssets(t)
    / ProjectedExternalSettlementNeeds(t)
```

Assets require haircuts based on liquidity, credit risk, custody risk, currency mismatch, legal access, and concentration.

### 7.3 Essential coverage

```text
EssentialCoverage(t) =
    HaircutValue(
        liquid reserves
        + contracted capacity
        + eligible receivables
        + nettable obligations
        + committed revenues
      )
    / OutstandingEUAndESRLiabilities(t)
```

No single ratio authorizes issuance. Category coverage, liquidity coverage, concentration, and stress projections all matter.

### 7.4 Coverage reports

The public evidence plane publishes:

- aggregate 𝒰 issued;
- aggregate 𝒰 spent;
- ℛ outstanding;
- settlement aging;
- coverage by category and region;
- reserve composition and haircuts;
- provider concentration;
- ℰ credit outstanding;
- defaults and restructurings;
- stress-test results;
- breaches and remediation.

Personal balances and individual purchases remain private.

---

## 8. Essent as mutual and public credit

### 8.1 Why not fixed supply

A fixed-supply speculative asset may appreciate as adoption grows. It may also become volatile, concentrate in early holders, reward hoarding, and make ordinary pricing difficult.

A currency intended for trade needs enough elasticity to clear useful exchange. ℰ should expand when creditworthy participants have reciprocal demand and contract when credit is repaid.

### 8.2 Mutual-credit issuance

For an approved buyer `a` and seller `b`:

```text
balance[a] -= amount
balance[b] += amount
```

The transaction is valid only when:

```text
balance[a] - amount >= -credit_limit[a]
```

The sum of balances within a closed clearing domain remains zero before fees, public positions, reserve accounts, and recognized losses.

Credit limits are underwriting decisions. They depend on:

- verified revenue or productive capacity;
- transaction history;
- concentration;
- cyclicality;
- outside obligations;
- collateral or guarantees where appropriate;
- community guarantee pools;
- dispute and default history;
- the provider's ability to earn `ℰ`.

Credit limits are not civil rank and cannot affect political rights.

### 8.3 Public-credit issuance

The Society may spend ℰ from a public negative position only under a ratified budget and an explicit backing plan.

Possible backing includes:

- future dues or fees;
- cooperative revenue;
- commons revenue;
- grants;
- external reserves;
- contracted production;
- recoverable loans;
- legally authorized revenue;
- a measured increase in demand to hold `ℰ`.

The public issuer is carrying debt to the network. Calling it minting does not remove the debt.

### 8.4 Monetary expansion dividend

A future mature network may test a universal monetary expansion dividend.

The hypothesis is that when desired real ℰ balances, transaction demand, and productive capacity increase together, part of the newly supportable ℰ supply can be issued equally rather than allocated entirely through private credit.

A conservative bound is:

```text
PublicNetIssue(t) <= min(
    EstimatedRealBalanceDemandGrowth(t),
    HaircutSpareCapacityValue(t),
    RatifiedRiskBudget(t),
    InflationAndLiquidityLimit(t)
)
```

This is a research hypothesis, not a genesis rule. The default value is zero until demand estimation, distribution effects, price response, and governance have survived bounded pilots.

The Society may not use speculative ℰ price appreciation as the primary funding source for a Freedom Floor.

### 8.5 Demand for ℰ

Durable ℰ demand must come from useful obligations and exchange.

Possible demand sources include:

- repayment of ℰ credit;
- purchases from providers that price goods or services in ℰ;
- settlement of invoices;
- network service fees;
- membership dues where lawful and democratically authorized;
- validator or operator bonds;
- cooperative rent, energy, education, transport, or care;
- provider acceptance commitments received in exchange for network credit;
- inter-node settlement;
- treasury sales of useful assets or services.

Burning tokens or creating artificial scarcity does not create a productive economy.

### 8.6 Credit risk

A mutual-credit system moves credit creation into the network. It does not eliminate default.

Losses must follow a published waterfall:

1. delinquent participant recovery and restructuring;
2. pledged collateral or guarantees;
3. transaction-specific insurance;
4. local guarantee pool;
5. node capital;
6. federation protection fund;
7. transparent mutualization within ratified limits;
8. resolution.

Personal subsistence 𝒰 may not be clawed back to cover an institutional underwriting error.

### 8.7 Fungibility and issuer risk

Early ℰ balances should not pretend all issuers are equally safe.

A practical architecture can use one denomination with identifiable node exposure, similar to deposits denominated in the same national unit but owed by different institutions.

Par exchange between node-issued ℰ requires:

- common accounting standards;
- reserve and capital rules;
- audited exposures;
- settlement limits;
- loss-sharing rules;
- credible resolution;
- reciprocal acceptance;
- a federation clearing process.

Until those conditions exist, cross-node ℰ may carry limits or node-specific exchange rates. Hiding credit risk behind one ticker does not create trust.

---

## 9. Can shared agreement create purchasing power?

Yes, within limits.

A network can assign ℰ as the unit used to settle invoices, repay credit, pay dues, obtain network services, and purchase goods from participating providers. If enough participants expect others to accept ℰ, acceptance becomes self-reinforcing. Software can make issuance rules, balances, and settlement history credible enough to support that coordination.

That creates real purchasing power inside the network.

The effect is not imaginary. A mutual-credit purchase can activate idle labor, inventory, rooms, transport, or productive capacity even when outside currency is scarce. Obligation netting can reduce the outside money needed to settle a web of debts. New exchange can create income and output that would not otherwise occur.

But shared agreement does not repeal physical scarcity or external dependence.

If the network needs insulin, semiconductors, fuel, land, or labor from people who reject ℰ, it needs something those providers accept. That may be external currency, goods, reciprocal services, credit, or an enforceable obligation.

[Bitcoin](https://bitcoin.org/bitcoin.pdf) demonstrates that an unbacked digital asset can acquire market value through credible scarcity, network growth, utility, liquidity, expectations, and speculation. It does not demonstrate that any token can acquire stable purchasing power, or that token appreciation can fund a universal entitlement indefinitely.

Many people agreeing is powerful.

What they are agreeing to deliver still matters.

---

## 10. 𝒰-to-ℰ convertibility

### 10.1 Convertibility is optional infrastructure

𝒰 may support a conversion window into ℰ to improve flexibility. This is not the source of 𝒰's value and cannot be an unlimited protocol promise.

The person's primary right is direct access to the basket.

### 10.2 Requested conversion

For an 𝒰 value `V` and an executable ℰ price `P_exec`:

```text
RequestedE = V / P_exec
```

If `V = 1,000` reference-currency units and `P_exec = 0.001`, then:

```text
RequestedE = 1,000,000 ℰ
```

The arithmetic is correct. The conclusion that one million newly minted ℰ now possesses 1,000 units of purchasing power is not.

### 10.3 Executable price

`P_exec` is not the last displayed trade.

It must estimate how much outside value the actual conversion can realize after price impact.

Inputs include:

- multiple independent venues;
- time-weighted prices;
- executable order-book depth;
- completed volume;
- price-impact curves;
- market-maker quotes;
- reserve redemption value;
- ℰ-denominated goods actually available;
- wash-trading and related-party filters;
- data-source staleness;
- exchange and custody risk.

A robust aggregation may use a trimmed or weighted median, but aggregation does not cure shared-source failure.

### 10.4 Conversion bound

```text
ConvertibleE = min(
    RequestedE,
    TreasuryEAvailable,
    MarketDepthCap,
    MonetaryExpansionCap,
    ExternalLiquidityCap,
    ParticipantLimit
)
```

The system must estimate the external or network purchasing power that can be delivered, not merely the ℰ quantity that can be printed.

### 10.5 Conversion corridor

The treasury may operate a bounded corridor:

- below a lower threshold, it buys ℰ using available reserves or accepts ℰ for obligations;
- above an upper threshold, it sells ℰ or issues against authorized demand;
- between thresholds, market and network exchange determine the price.

A corridor is credible only to the extent of available reserves, recurring revenues, obligation demand, and market depth.

If the lower defense is exhausted, the protocol widens or suspends the corridor and publishes the breach. It does not claim the peg survived because the oracle still reports a number.

### 10.6 Stress modes

#### ℰ market failure

If ℰ becomes thin, manipulated, or nearly valueless:

- direct 𝒰 spending continues where provider capacity remains;
- new 𝒰-to-ℰ conversion pauses;
- treasury ℰ sales and discretionary public ℰ issuance stop;
- reserve and provider settlement continues under contracts;
- market data enters incident mode;
- the failure and exposure are published.

#### External settlement failure

If banks, payment partners, or reserve access fail:

- netting and ℰ settlement continue where accepted;
- providers may choose in-kind or deferred settlement;
- external claims are queued transparently;
- 𝒰 issuance may be reduced to demonstrated direct capacity;
- no hidden arrears are created.

#### Provider shortage

If a category lacks deliverable capacity:

- the interface marks the category unavailable or partially covered;
- procurement and mutual aid activate;
- emergency substitutions require quality and rights review;
- the basket price does not pretend unavailable supply exists.

#### Physical shortage

If the goods do not exist in sufficient quantity, no monetary operation can create them immediately. The response is rationing under public rules, emergency production, substitution, import, and honest disclosure.

### 10.7 Autonomy Dividend

A mature system should distinguish:

- **𝒰**, which protects access to essentials;
- **Autonomy Dividend**, an unrestricted ℰ or external-currency payment funded from actual revenue, reserves, grants, commons income, or validated monetary expansion.

This prevents the entire Freedom Floor from depending on a speculative conversion while recognizing that dignity requires choice beyond a controlled catalog.

---

## 11. Treasury and reserve architecture

### 11.1 Legal ownership

External reserves must be held by identifiable legal entities under defined custody, audit, insolvency, and redemption rules.

A hash of a bank statement is not a reserve.

### 11.2 Fund separation

At minimum:

- Essential Clearing Fund;
- External Liquidity Reserve;
- Mutual Credit Guarantee Pool;
- Node Capital Account;
- Federation Protection Fund;
- Security and Operations Fund;
- Research and Audit Fund;
- Emergency Procurement Fund.

Funds may not be silently commingled.

### 11.3 Reserve eligibility

Eligible reserve assets require policy for:

- issuer and custodian risk;
- duration;
- volatility;
- liquidity;
- currency mismatch;
- legal seizure risk;
- geographic concentration;
- operational access;
- audit frequency.

### 11.4 Proof of reserves and liabilities

The system publishes both sides.

Reserve disclosure without outstanding 𝒰, ℛ, ℰ credit, guarantees, and pending withdrawals is incomplete. [Project Pyxtrial's](https://www.bis.org/project/pyxtrial) distinction between asset and liability monitoring is the right direction: solvency requires comparing what is held with what is owed.

### 11.5 No founder privilege

There is no hidden premine, private redemption seniority, or founder withdrawal class.

Organizers may be paid through public budgets under the same accounting and disclosure rules as other institutional recipients.

---

## 12. Identity, eligibility, and recovery

### 12.1 No universal DID requirement

Essentia may support W3C-compatible [decentralized identifiers](https://www.w3.org/TR/did-core/) and [verifiable credentials](https://www.w3.org/TR/vc-data-model-2.0/). It must not require one permanent public DID for voting, healthcare, education, money, safety, and speech.

Different contexts require different identifiers.

### 12.2 Purpose-bound credentials

Examples:

- eligibility credential for 𝒰;
- one-election voting credential;
- provider-category credential;
- operator-role credential;
- public authorship key;
- professional qualification;
- age or residency range proof;
- recovery-authority credential.

Credentials reveal the minimum claim needed for the action.

### 12.3 Proof of personhood

A DID proves control of a key, not uniqueness or humanity.

No known proof-of-personhood method simultaneously solves global inclusion, duplicate resistance, privacy, coercion, recovery, accessibility, and institutional capture.

Essentia therefore uses a plural, risk-based process:

- multiple enrollment routes;
- trained human review;
- optional documents or biometrics through separated providers;
- capped community attestations;
- privacy-preserving duplicate detection;
- random audits;
- continuing liveness appropriate to the right;
- protected challenge;
- independent appeal;
- recovery without public linkage.

No raw biometric, government-ID image, address, or recovery secret enters the public log.

### 12.4 One-use nullifiers

For one-person-one-action processes, a credential can derive a context-specific nullifier:

```text
nullifier = H(secret, action_domain, action_id)
```

The verifier can reject duplicate use within the same action without learning where else the credential was used.

The cryptographic scheme must be versioned and replaceable.

### 12.5 Recovery

Recovery must support people who lose devices, flee abuse, experience cognitive crisis, or rely on assistance.

Options may include:

- multiple devices;
- time-locked guardian quorum;
- institutional recovery;
- printed or offline recovery material;
- protected assisted custody;
- re-enrollment with fraud review.

No one loses food, personhood, or political rights because they misplaced a seed phrase.

---

## 13. Governance

### 13.1 Decision records

Every binding decision record contains:

- canonical question and alternatives;
- decision class;
- legal or constitutional authority;
- eligible population;
- evidence packet;
- conflict disclosures;
- deliberation period;
- voting or selection method;
- quorum and threshold;
- tally procedure;
- implementation owner;
- effective date;
- review or sunset date;
- appeal or challenge path;
- public result artifacts.

### 13.2 Method by decision type

Essentia does not impose ranked-choice voting on every multi-option decision.

Possible defaults:

| Decision | Candidate mechanism |
| --- | --- |
| Elect one office | locally chosen ranked or approval method |
| Elect a body | proportional representation |
| Rights limitation | supermajority plus rights review |
| Technical standard | expert proposal, public comment, ratification, sunset |
| Local operational choice | local vote within shared rights |
| Budget priorities | participatory budgeting or tested allocation method |
| Evidence review | independent panel with published criteria |
| Deadlock or representative sample | sortition with deliberation |
| Emergency action | narrow temporary authority plus automatic review |

No procedure eliminates strategic behavior, unequal information, or agenda power.

### 13.3 Delegation

Delegation is experimental.

If used, it is:

- topic-specific;
- revocable;
- time-bounded;
- capped to prevent extreme concentration;
- visible at aggregate level;
- directly overridden by the member;
- evaluated against direct voting and deliberative alternatives.

### 13.4 Ballot privacy

Private ballots require coercion resistance, eligibility verification, secret selection, public tally verification, accessibility, recovery, and dispute resolution.

The [National Academies concluded](https://doi.org/10.17226/25120) that marked ballots should not be returned over the internet because current internet voting cannot guarantee secrecy, security, and verifiability at once. Essentia must not turn an unsolved election problem into a release milestone.

Early governance may use in-person, paper, or established election systems with cryptographic publication of public artifacts. The protocol records evidence; it does not require unsafe internet voting.

### 13.5 Institutional roles

Role credentials authorize actions. They do not create superior civic status.

Roles include:

- log operator;
- witness;
- clearing operator;
- treasury operator;
- auditor;
- basket steward;
- enrollment provider;
- appeals officer;
- election administrator;
- emergency controller.

Every role has scope, expiration, conflict rules, and removal procedure.

---

## 14. Contribution funding

The v0.1.0 prototype includes purposes, quests, claims, reviews, and budget-capped minting. The structure is useful as a procurement experiment but should not be interpreted as a scientific measurement of social value.

### 14.1 Funded work

A contribution payment must reference a funded budget or authorized credit position.

```text
MaximumPayout = min(
    QuestCeiling,
    RemainingPurposeBudget,
    RemainingAuthorizedCredit,
    SettlementCapacity
)
```

Review determines whether work meets the published terms. Review does not create purchasing power by increasing a score.

### 14.2 No universal impact score

There is no single scalar that can compare caregiving, software, ecological repair, art, mediation, research, and survival.

Different programs may use different outcome measures. Rights may not depend on those measures.

### 14.3 Reviewer controls

Where review is required:

- criteria are published before submission;
- conflicts are disclosed;
- committee selection is independent;
- review pay does not increase with approved payout;
- minority findings are preserved;
- appeal exists;
- high-impact decisions receive external audit.

### 14.4 Reputation

Essentia does not maintain a universal public STAR or FLAME score.

Narrow operational reliability signals may exist for specific roles, such as whether an operator met uptime obligations or whether a provider settled invoices on time. They remain purpose-limited, contestable, expiring, and excluded from civil rights and general economic access.

---

## 15. Ledger architecture and consensus

### 15.1 Stage 1: transparency log

The first pilot uses:

- canonical serialized events;
- signed Merkle-tree heads;
- inclusion proofs;
- consistency proofs;
- independent witnesses;
- public mirrors;
- offline audit tools;
- ordinary transactional databases for private state;
- correction events rather than destructive edits.

This architecture is easier to operate, inspect, and replace than a new consensus network.

### 15.2 Stage 2: federated replicated log

When several independent nodes share authority, they may cross-sign or replicate event roots. Conflicting views become detectable.

### 15.3 Stage 3: Byzantine fault tolerant consensus

A [HotStuff-family](https://doi.org/10.1145/3293611.3331591) or comparable protocol becomes relevant only when:

- independent operators share write authority;
- deterministic finality is required;
- network and failure assumptions are explicit;
- validator governance is mature;
- clients need a common state without trusting one operator.

Consensus security does not solve governance, personhood, or oracle truth.

### 15.4 No proof of stake

ℰ ownership does not control consensus or governance.

Validator admission is institutional and public. Bonds may secure operational obligations, but bond size does not purchase votes.

### 15.5 Determinism

Consensus-critical calculations use canonical encoding and integer or rational fixed-point arithmetic.

External prices and basket values are committed as signed observations before use. Policy specifies rounding, staleness, and fallback behavior.

### 15.6 Cryptographic agility

The protocol defines suites and version transitions rather than hard-coding one primitive forever.

[NIST-standardized post-quantum algorithms](https://www.nist.gov/news-events/news/2024/08/nist-releases-first-3-finalized-post-quantum-encryption-standards) are migration targets for long-lived signatures, protected key establishment, and archives. Early pilots should benchmark key size, signature size, latency, hardware support, library maturity, recovery, and operational error before making a suite consensus-critical.

### 15.7 Release verification

Every release publishes:

- source commit;
- reproducible build instructions;
- artifact hashes;
- dependency lock;
- test vectors;
- schema versions;
- migration plan;
- security review;
- rollback plan.

---

## 16. Oracles

### 16.1 What an oracle is

An oracle is an institution that converts observations outside the protocol into authorized inputs.

It is not merely an API.

### 16.2 Observation record

Each observation includes:

- source;
- item or market;
- location;
- timestamp;
- method;
- confidence or quality field;
- signer;
- conflict disclosure;
- raw-data commitment;
- expiration.

### 16.3 Aggregation

Possible controls include:

- multiple genuinely independent sources;
- robust median or trimmed aggregation;
- venue-depth weighting;
- time weighting;
- staleness rejection;
- outlier review;
- anti-wash filters;
- independent witnesses;
- delayed activation for large changes;
- rate-of-change limits;
- fallback auctions;
- manual emergency override with expiration and public reason.

### 16.4 Availability

A price feed is incomplete without availability.

The basket oracle records stockouts, waiting times, service capacity, and geographic reach. A provider advertising a price while refusing 𝒰 transactions does not contribute usable capacity.

### 16.5 Oracle failure

If required observations become unreliable:

- dependent conversions pause;
- the last valid basket may continue for a bounded period;
- direct provider contracts continue where possible;
- emergency review begins;
- no privileged actor may silently substitute a new source.

---

## 17. Privacy and data governance

### 17.1 Data minimization

Collect only data needed for a defined function. Set deletion or retention rules before collection.

### 17.2 Separation

Identity, eligibility, payments, ballots, health, safety, and research use separate stores and credentials. Cross-domain linkage requires explicit authority.

### 17.3 Public commitments

The public log may contain:

- hashes;
- aggregate liabilities;
- policy identifiers;
- credential-status commitments;
- nullifier sets;
- audit roots;
- release artifacts.

It does not contain raw private records.

### 17.4 Institutional observability

Powerful actors receive less privacy in their official capacity.

Treasury operations, budget authority, settlement exposures, operator actions, conflicts, and emergency interventions are logged at the level needed for accountability.

### 17.5 Models

Models may assist fraud review, forecasting, routing, translation, summarization, and anomaly detection.

A model output affecting rights or money requires:

- declared purpose;
- version and evaluation;
- confidence and known limitations;
- human review;
- appeal;
- monitoring for disparate error;
- rollback.

Models do not determine personhood or dignity.

---

## 18. Research and rollout

### Phase 0: specification and simulation

No real balances.

Required work:

- define double-entry schemas;
- simulate credit and settlement;
- model ℰ price and liquidity shocks;
- model provider concentration and default;
- test basket calculations;
- threat-model identity and coercion;
- publish falsification criteria.

### Phase 1: shadow ledger

Record real transactions beside ordinary settlement without changing legal payment.

Success requires:

- exact reconciliation;
- independent reproduction;
- usable correction;
- clear privacy boundaries;
- low operator burden.

### Phase 2: closed business mutual credit

Launch ℰ among a small, dense group of businesses with reciprocal trade. No 𝒰 and no public UBI claim.

Success requires:

- measurable additional trade or liquidity savings;
- bounded defaults;
- realistic recirculation;
- no hidden concentration;
- participant retention without coercion;
- transparent resolution.

### Phase 3: externally funded 𝒰 pilot

Issue a partial 𝒰 entitlement backed entirely by external funds or donated provider capacity.

Recipients spend 𝒰 directly. Providers receive predictable external settlement.

Success requires:

- real access at the published level;
- no degrading provider prices or quality;
- low exclusion and appeal error;
- recipient dignity;
- complete liability accounting.

### Phase 4: mixed settlement

Providers voluntarily accept a bounded ℰ share and use obligation netting.

Success requires:

- ℰ recirculation;
- settlement within contract;
- no transfer of hidden loss to providers;
- stable coverage;
- no decline in recipient access.

### Phase 5: bounded conversion

Open a small 𝒰-to-ℰ conversion window funded by reserves and executable liquidity.

Success requires:

- realized purchasing power close to the published estimate;
- manageable price impact;
- no runaway issuance;
- no adverse selection that drains the floor;
- transparent pause behavior.

### Phase 6: node federation

Connect independent clearing nodes under common standards and limited exposure.

Success requires:

- audited inter-node positions;
- credible resolution;
- no forced par;
- operational diversity;
- failure containment.

### Phase 7: partial Freedom Floor

Only after earlier phases may the Society describe a program as a partial Freedom Floor.

Coverage must state exactly which regions, people, categories, amounts, and failure protections are included.

### No automatic mainnet milestone

A blockchain mainnet is not the destination. Reliable public service is.

---

## 19. Required metrics

### Economic

- ℰ transaction volume and unique counterparties;
- recirculation and concentration;
- average and tail settlement time;
- default and recovery rate;
- credit utilization;
- netting ratio;
- external-liquidity requirement per unit of trade;
- provider retention;
- ℰ executable price and market depth;
- conversion slippage;
- category coverage;
- 𝒰 and ℛ liabilities;
- reserve and capital ratios.

### Human

- successful access to each essential category;
- exclusion and duplicate-enrollment errors;
- appeal time and reversal rate;
- reported humiliation or coercion;
- privacy incidents;
- recipient choice and substitution;
- provider burden;
- accessibility outcomes.

### Governance

- participation;
- representativeness;
- comprehension;
- delegation concentration;
- agenda concentration;
- implementation follow-through;
- minority reports;
- review and sunset completion.

### Technical

- reconciliation errors;
- consistency-proof failures;
- witness diversity;
- availability;
- recovery success;
- key-loss incidents;
- data-access violations;
- verifier reproducibility;
- upgrade and rollback performance.

Metrics are diagnostic. They do not become a universal social score.

---

## 20. Stop conditions

A pilot pauses or contracts when:

- essential access falls below the published threshold;
- provider settlement exceeds the contractual limit;
- reserve or category coverage breaches its floor;
- ℰ conversion causes unacceptable price impact;
- losses exceed the ratified risk budget;
- identity errors deny a material number of legitimate participants;
- privacy failures cannot be contained;
- a verifier cannot reproduce a binding result;
- governance cannot correct a known failure;
- the program shifts costs onto people with less power;
- reported benefits depend on hidden subsidy or unreported arrears.

The system must be able to fail honestly before it is allowed to fail at scale.

---

## 21. Implementation boundaries

### 21.1 Required properties

Every implementation must preserve:

- narrow, auditable scope;
- no arbitrary smart-contract casino;
- transparent budgets;
- content-addressed public records;
- separation of `𝒰`, `ℛ`, and `ℰ`;
- budget ceilings for funded work;
- challenge and audit paths;
- cryptographic agility;
- verifier-first releases;
- explicit implementation limits.

### 21.2 Excluded assumptions

The architecture does not assume:

- unlimited `𝒰` redemption into `ℰ`;
- generic issuance of `ℰ` without a corresponding asset, obligation, or approved monetary basis;
- one native DID for all civic life;
- proof of personhood as a purely technical protocol module;
- ranked-choice voting or liquid democracy as universal defaults;
- a new blockchain as the first production architecture;
- STAR, FLAME, or another general reputation score as a civil hierarchy;
- one immediate cryptographic suite that can never be migrated;
- a ledger as the source of truth for external prices or human identity;
- a token balance as proof that a Freedom Floor exists.

### 21.3 Role of the v0.1.0 prototype

The repository also contains a runnable v0.1.0 research prototype. It can test deterministic transactions, budget objects, two balance classes, multi-node replication, and narrow state-machine behavior. It is not the implementation defined by this architecture. Its monetary model and trust assumptions do not satisfy the requirements for a real economic system.

The implementation path begins with accounting and simulation, not with a consensus upgrade.

---

## 22. Open research questions

1. Can a business mutual-credit network reach sufficient trade-loop density without excessive brokerage?
2. Which credit-limit process minimizes default without reproducing conventional exclusion?
3. How much liquidity can obligation netting save under realistic participation?
4. Which network obligations create durable ℰ demand without coercive lock-in?
5. Can node-issued ℰ remain meaningfully fungible while exposing issuer risk?
6. How should public credit be bounded when the Society lacks taxation authority?
7. Which part of a Freedom Floor can be delivered through contracted capacity rather than external cash?
8. How large must an unrestricted Autonomy Dividend be to preserve meaningful choice?
9. Which basket process best combines rights, observed prices, availability, and public judgment?
10. How quickly should basket values react to shocks without importing manipulation?
11. What conversion corridor survives realistic ℰ volatility and market depth?
12. Can a monetary expansion dividend be estimated without becoming procyclical or inflationary?
13. Which proof-of-personhood combination minimizes both Sybil fraud and exclusion?
14. Can ballot privacy, accessibility, coercion resistance, and public verification be achieved for the actual voting environment?
15. Which records need shared consensus, and which need only independent witnessability?
16. How should ecological costs enter procurement and credit limits without becoming a decorative score?
17. Which institutions can hold reserves and settle across jurisdictions lawfully?
18. What resolution process protects recipients, providers, and taxpayers when a node fails?
19. How can independent research be funded without making the evaluator dependent on favorable findings?
20. Under what conditions should Essentia be abandoned in favor of an existing payment, identity, or governance system?

The last question is not rhetorical.

Essentia earns the right to exist only where it performs a necessary function better than simpler alternatives.

---

## 23. Conclusion

Essentia can coordinate trust. It cannot command trust into existence.

It can make issuance rules visible, reconcile obligations, prove that a record was not rewritten, route a real entitlement, and show whether a treasury has acknowledged what it owes. It can help a network use idle capacity and reciprocal credit that ordinary money leaves stranded. It can make it harder for an institution to hide a failure.

It cannot manufacture food, housing, medicine, skill, goodwill, legal authority, or external currency by changing a number.

A widely adopted ℰ can acquire real purchasing power because people can use it to discharge real obligations and obtain real goods. That possibility is serious enough to test. It is not serious enough to promise before the trade loops, providers, reserves, credit rules, and governance exist.

𝒰 is therefore the promise.

ℰ is one way to settle the promise.

The bridge connects the promise to the rest of the world.

The public log makes the promise inspectable.

The research program decides whether any of it deserves to scale.

That is the architecture this version proposes.

Not a machine that declares value.

A society that builds the relationships through which value becomes real.

---

## Research basis

The scientific and economic reasoning behind this specification is developed in [Part B of the Society of Renewal Founding Book](https://github.com/SocietyOfRenewal/societyofrenewal/blob/main/docs/founding-book/Part%20B.md). Part B includes the working bibliography, competing evidence, explicit uncertainties, and stage-gate research design.
