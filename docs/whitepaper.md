# Essentia Network White Paper

## Version 0.7.0 - Revised Technical Draft

Status: proposed replacement for v0.6.0  
Date: 2026-03-29  
Scope: civic ledger for the Society of Renewal, not a general-purpose speculative chain

---

## 0. Executive Summary

Essentia is the civic ledger and monetary core of the Society of Renewal. It exists to do five things well:

1. give each member a self-sovereign civic identity without putting personal data on-chain,
2. provide private ballots with public verification,
3. budget and verify contribution to the commons,
4. issue and settle two monetary instruments with different purposes, and
5. make power auditable: proposals, budgets, minting, and outcomes must all be inspectable.

This revision fixes the two core failures in v0.6.0:

- The cryptography stack was internally inconsistent. The draft claimed a post-quantum ledger while naming Ed25519 and BBS+/BLS12-381 in the normative path. v0.7.0 moves to a hybrid/PQ-ready architecture: ledger-critical signatures and encryption use NIST PQC primitives from genesis, while privacy-preserving credential and ballot proof suites are versioned and explicitly swappable.
- The minting function was gameable. In v0.6.0, the amount minted depended directly on complexity, impact, and the summed reputation of attestors. That lets colluding reviewers amplify issuance. v0.7.0 removes reviewer reputation from payout. Attestors certify validity only. Money is created only from pre-approved epoch budgets, after review, challenge, and audit.

Essentia is intentionally narrow. It is not a casino chain, a yield farm, or an open smart contract playground. Mainnet v1 exposes audited native modules only: identity, governance, treasury, contribution claims, Essent balances, Essential Unit balances, vendor redemption, and archival proofs.

---

## 1. Constitutional Alignment

Essentia is an implementation vehicle for the Society of Renewal, not an independent ideology.

The protocol MUST preserve these constitutional properties:

- dignity is a hard constraint on all protocol and policy design,
- one member, one voice is the baseline for governance,
- delegation is optional, topic-scoped, and revocable,
- ballots for binding governance are private and publicly verifiable,
- UBI, called the Freedom Floor, is delivered through a dedicated essentials instrument,
- privacy is an agency right, and
- institutional power must be transparent in proportion to its power.

Where a technical mechanism conflicts with those constraints, the mechanism loses.

---

## 2. Design Goals and Non-Goals

### 2.1 Goals

1. Security first
   - no unauthorized mint,
   - no double-spend,
   - no hidden policy changes,
   - no private re-interpretation of governance outcomes.

2. Civic legitimacy
   - one person, one member identity,
   - no rights gated by wealth, stake, or reputation,
   - transparent public budgets,
   - verifiable elections.

3. Low attack surface
   - no proof-of-work,
   - no arbitrary user-deployed contracts in v1,
   - small, auditable state machine.

4. Monetary separation of concerns
   - Essent (E) for general economic coordination,
   - Essential Unit (U) for baseline access to essentials.

5. Public legibility
   - every binding decision, mint, budget, and redemption path has a verifier path.

### 2.2 Non-Goals

- Not a general-purpose L1 for speculative DeFi.
- Not a privacy coin for unbounded anonymous transfer.
- Not an algorithmic stablecoin.
- Not a system where reputation or stake increases voting rights.
- Not a protocol that claims full post-quantum privacy before such a stack is mature enough for production.

---

## 3. Core Objects

| Module | Object | Purpose |
| --- | --- | --- |
| Identity | DID | stable civic identifier controlled by the member |
| Identity | Personhood Credential | uniqueness / membership eligibility without public PII |
| Governance | Proposal | canonical text hash, parameters, snapshot height, election mode |
| Governance | Delegation | topic-scoped, revocable voting delegation |
| Governance | Ballot | encrypted vote payload or public signal payload |
| Treasury | Budget | epoch budget for purposes, CGP, grants, audits, reserves |
| Contribution | Purpose | public mission with metrics, budget, and scope |
| Contribution | Quest | discrete funded work item or outcome contract |
| Contribution | Claim | request for payout based on work delivered |
| Contribution | Review | attestor assessment of a claim |
| Value | Essent (E) | transferable fungible currency |
| Value | Essential Unit (U) | non-transferable, expiring essentials instrument |
| Value | Vendor Credential | permission to accept U and redeem for E |

Only hashes, state commitments, ciphertexts, proofs, and minimal metadata live on-chain. Evidence bundles, large texts, and learning materials live off-chain under content addressing with on-chain commitments.

---

## 4. Identity, Membership, and Recovery

### 4.1 DID Method

Essentia defines a native DID method:

    did:essentia:<identifier>

A DID document contains verification methods and service endpoints. The chain stores the authoritative state commitment for each DID document version.

### 4.2 Separation of Authentication and Authorization

v0.6.0 mixed passkeys and ledger authority too loosely. v0.7.0 separates them.

1. Session authentication
   - A member logs into clients and relayers using WebAuthn passkeys.
   - This is UX and anti-phishing infrastructure, not the final monetary root of trust.

2. Ledger authorization
   - Every DID has a controller key for chain actions.
   - Mainnet validity requires an ML-DSA-65 signature on governance, treasury, and monetary transactions.
   - During migration, clients MAY attach a secondary classical signature for ecosystem compatibility, but consensus validity depends on the PQ ledger signature.

3. Recovery
   - Recovery is a separate authority path using a time-locked guardian quorum, recommended 3-of-5 or 4-of-7.
   - Recovery cannot move funds instantly. It can only rotate keys after a public delay and alert period.
   - Emergency freeze of a DID is limited to key compromise scenarios and must expire automatically unless ratified through review.

### 4.3 Proof of Personhood and Membership

A DID alone is not enough for voting or UBI. Binding civic actions require a Personhood Credential.

Personhood eligibility is established through a privacy-preserving, reviewable process combining several signals:

- device-bound keys,
- community vouching,
- optional third-party identity evidence when the member chooses to use it,
- anomaly detection and challenge review,
- appeals with human oversight.

The chain stores only:

- a blinded eligibility/nullifier commitment,
- status flags,
- issuance and revocation commitments,
- audit trail references.

No raw biometric, government-ID image, or home address is ever stored on-chain.

### 4.4 Protected and Temporary Custody

If a member is in crisis and cannot safely manage keys, a temporary custodial arrangement may exist under three hard rules:

- it is time-bounded,
- it is transparent to oversight,
- it includes a mandatory path back to full member control.

Permanent dependency is invalid.

---

## 5. Ledger Model

Essentia uses an account-based state machine with audited native modules only.

### 5.1 Why Account-Based

The chain tracks identity state, role credentials, proposal state, claim state, vendor credentials, and expiring U balances. These fit an account/object model better than a pure UTXO model.

### 5.2 Native Modules in v1

- DID Registry
- Credential Status and Revocation
- Governance
- Delegation
- Treasury and Budgeting
- Contribution Claims and Review
- Essent Balances
- Essential Unit Balances
- Vendor Redemption
- Archival Proof Anchors

General-purpose smart contracts are disabled in v1. Any future expansion requires separate constitutional approval and a new audit process.

### 5.3 Deterministic Execution

Protocol state transitions MUST be deterministic. No floating-point arithmetic is allowed on-chain. All monetary calculations use fixed-point integer math.

---

## 6. Consensus and Network Security

### 6.1 Consensus

Essentia uses a HotStuff-family Byzantine Fault Tolerant protocol with deterministic finality.

- Safety threshold: n >= 3f + 1 validators, tolerating up to f Byzantine validators.
- Finality: a block is final once it carries a valid quorum certificate.
- Mempool: a DAG-based dissemination layer MAY be added for throughput, but consensus-critical ordering remains the BFT finality protocol.

This fixes the v0.6.0 confusion between Narwhal/Bullshark-style plumbing and HotStuff finality. In v0.7.0, the consensus rule is singular: HotStuff-family finality is normative.

### 6.2 Validator Set

- Validators are public DIDs.
- Initial launch is federated and transparently disclosed.
- Validator admission, rotation, and removal are governed by public process.
- Validators must publish infrastructure attestations, conflict disclosures, and operational contacts.

### 6.3 Bonds and Slashing

Validators post operational bonds in E.

Slashable faults include:

- double-signing,
- invalid quorum certificate participation,
- provable censorship beyond policy thresholds,
- fraudulent state transition,
- undisclosed key compromise,
- repeated downtime above the epoch threshold.

Slashing proceeds go to the Security Fund and, where applicable, to challengers who provided valid evidence.

### 6.4 Transaction Ordering and MEV

To reduce extractive behavior:

- governance and UBI transactions have priority classes,
- within a class, ordering is deterministic by fee band then nonce then hash,
- proposer discretion is deliberately minimized,
- user-deployed arbitrary contracts are absent in v1, which removes the largest source of extractable ordering games.

---

## 7. Cryptography Profile

### 7.1 Normative Principle

Ledger-critical security and private-ballot integrity MUST not depend on undocumented or ad hoc crypto.

### 7.2 Required Primitives

1. Hashing
   - BLAKE3 or SHA3-256 with domain separation.
   - Every protocol object has its own domain tag.

2. Signatures
   - ML-DSA-65 is required for ledger-valid transaction signatures.
   - Validator consensus messages also use ML-DSA-65.
   - A secondary classical signature MAY be attached during migration, but it is not consensus-critical.

3. Encryption for protected evidence access
   - ML-KEM-768 is used to establish shared secrets.
   - Symmetric encryption uses an AEAD construction.

4. Credentials, presentations, and ballot-proof suites
   - Credential documents follow a versioned VC-compatible format.
   - Presentation, selective disclosure, and ballot-proof suites are versioned at the application layer, not hard-coded into consensus.
   - If a cryptosuite is deprecated, credentials and election tooling can be upgraded without rewriting the chain.

### 7.3 Explicitly Removed Claim

v0.7.0 does not claim that every privacy primitive in the system is fully post-quantum from genesis. It does claim:

- ledger signatures are PQ from genesis,
- protected evidence exchange is PQ from genesis,
- credential and ballot-proof systems are versioned and replaceable,
- the protocol does not hard-code a classical-only privacy suite into the irreversible monetary core.

That is technically honest and operationally safer.

---

## 8. Governance

### 8.1 Governance Rights

Each eligible member DID has one baseline vote. Rights do not increase with wealth, stake, office, or reputation.

### 8.2 Proposal Model

Every proposal includes:

- proposalId = hash(canonical_text),
- election type,
- snapshot height,
- open/close times,
- review period,
- quorum rule,
- threshold rule,
- sunset/review date,
- implementation hooks, if any.

The exact text decided MUST be content-addressed and hash-anchored.

### 8.3 Ballot Privacy

Binding elections MUST use encrypted ballots.

Commit-reveal is not valid for secret constitutional or operational elections because the reveal phase makes ballot contents public. Commit-reveal MAY still be used for public signaling, public ratifications, or low-stakes non-secret coordination.

### 8.4 Binding Election Modes

1. Ranked-choice elections
   - Ballots encode an ordered ranking.
   - Ballots are encrypted, shuffled through a verifiable mixnet, then threshold-decrypted after close.
   - The verifier replays the published ranking set under the deterministic RCV rules.

2. Binary and budget votes
   - MAY use threshold-encrypted tallying directly, if the tally scheme supports the ballot type.
   - QV remains limited to budgeting and prioritization.

### 8.5 Vote Updates

A voter may update a ballot before close by submitting a higher sequence number.

For an election E and voter nullifier N, the counted ballot is:

    highest valid seq for (E, N) submitted before close

Earlier ballots are ignored. Double-use of the same sequence number is invalid.

### 8.6 Delegation

- Delegation is topic-scoped.
- Delegation is revocable.
- Direct vote overrides delegation for that proposal.
- Chains are allowed up to a policy-bound depth.
- Cycles are resolved deterministically by dropping the newest edge in the cycle.

Delegations are snapshot-resolved at close. Post-close delegation changes do not alter a closed election.

### 8.7 Verifier Path

For each election the chain publishes or references:

- snapshot eligibility set,
- encrypted ballots,
- ballot validity proofs,
- update-resolution map,
- delegation trace,
- decryption shares and proofs,
- final ballot set for tally,
- canonical tally transcript,
- result digest.

A public verifier CLI MUST reproduce the result from public artifacts.

### 8.8 Civic Cost Model

Voting is fee-less to the member.

Chain costs are covered by a Civic Gas Pool (CGP) with public budgets, public relayer reimbursement, and per-election anti-spam limits.

---

## 9. Contribution System and Anti-Exploit Minting

### 9.1 Problem Statement

The v0.6.0 mint equation was exploitable because attestor reputation directly increased payout. Any design where reviewers can increase issuance by signing harder creates a bribery and cartel surface.

The fix is structural:

- reviewers do not create money,
- budgets create the upper bound,
- quests define the reward ceiling before work is accepted,
- review only determines whether the claim qualifies and what fraction of the ceiling is earned.

### 9.2 Contribution Pipeline

1. Purpose creation
   - Governance approves a Purpose with mission, metrics, epoch budget, and audit policy.

2. Quest creation
   - A Quest is opened under a Purpose.
   - The Quest has:
     - scope,
     - rubric,
     - maximum payout R_q,
     - evidence requirements,
     - risk band,
     - challenge window,
     - conflict rules.

3. Claim submission
   - The claimant submits:
     - questId,
     - evidence root,
     - metadata,
     - claimant bond,
     - conflict disclosures,
     - optional encrypted evidence for restricted reviewers.

4. Committee selection
   - A review committee is sampled after submission using public randomness.
   - Committee members must satisfy independence constraints:
     - no self-review,
     - no same-household or same-organization review,
     - no undeclared financial conflict,
     - geographic and trust-cluster diversity when available.

5. Review
   - Each reviewer scores the claim against the published rubric.
   - Reviewers do not choose an arbitrary payout.

6. Provisional outcome
   - The claim enters pending state with a provisional score and provisional payout.

7. Challenge window
   - Any member with standing may challenge by posting a challenge bond and evidence.

8. Finalization
   - If no valid challenge succeeds, the claim finalizes.
   - Payout is minted from the Purpose budget and epoch cap.

9. Audit tail
   - A portion of large payouts remains locked for a fixed audit horizon.
   - Fraud discovered during the audit tail triggers clawback from locked balances and bond seizure.

### 9.3 Payout Formula

For a claim c on quest q under purpose p in epoch e:

    Score(c) = sum over rubric dimensions d of:
               weight_d * median_j(score_j,d(c))

where each score_j,d(c) is in [0, 1] and the rubric weights sum to 1.

The payout is:

    P(c) = min( R_q * Score(c), B_rem(p,e), C_rem(e) )

where:

- R_q = quest reward ceiling approved before work,
- B_rem(p,e) = remaining purpose budget for epoch e,
- C_rem(e) = remaining global contribution mint capacity for epoch e.

Reviewer reputation is not in this formula.

### 9.4 Why This Is Harder to Exploit

- Splitting work into many micro-claims does not increase total payout beyond quest ceilings and purpose budgets.
- Cartel reviewers cannot inflate mint above approved budgets.
- Reviewers cannot mint outside an approved quest.
- Budget exhaustion stops over-issuance automatically.
- Challenge and audit create delayed finality for suspicious claims.
- Large claims are more expensive to fake because they require larger bonds, longer challenge windows, and larger locked tails.

### 9.5 Default Risk Controls

Starting defaults, subject to governance revision:

- review committee size: 5, or 7 for high-risk claims,
- claimant bond: 1 to 5 percent of R_q by risk band,
- minimum challenge window: 7 days,
- high-risk challenge window: 21 days,
- locked audit tail: 20 percent of payout for 90 days,
- mandatory second audit for top-decile payouts,
- maximum quest ceiling set by Purpose policy.

### 9.6 Reviewer Compensation

Reviewers are paid a fixed review stipend plus a small complexity increment. Reviewer pay is independent of claim size. This removes the incentive to approve bigger mints for bigger reviewer rewards.

### 9.7 Duplicate and Related-Party Controls

A claim is auto-flagged if any of the following hold:

- evidence root already used,
- same work unit key already finalized,
- substantial similarity to another claim above threshold,
- claimant and reviewer share an organization or disclosed financial tie,
- claimant is funding the reviewer through another active claim chain.

Flagged claims require expanded review or are rejected.

### 9.8 Failed and Fraudulent Claims

If a claim is rejected for ordinary insufficiency, the claimant bond may be partially refunded according to policy.

If a claim is fraudulent:

- claimant bond is slashed,
- reviewer bonds may be slashed for negligent or collusive approval,
- locked payout is clawed back first,
- unrecouped loss becomes a public fraud record and blocks future payouts until resolved,
- the challenger receives a reward if the challenge was valid.

---

## 10. Reputation and Integrity Signals

The protocol keeps STAR and FLAME, but only in limited roles.

### 10.1 STAR

STAR is a bounded reviewer reliability score. It affects:

- eligibility for higher-risk review committees,
- audit sampling probability,
- surfacing and routing of work.

It does not affect:

- voting rights,
- UBI rights,
- claim payout size.

### 10.2 FLAME

FLAME is a decaying misconduct score for reviewers and vendors.

A simple update rule is:

    FLAME_t+1 = min(1, FLAME_t * exp(-lambda * delta_t) + penalty_t)

High FLAME reduces review eligibility and can trigger mandatory oversight. It does not erase personhood or voting rights.

### 10.3 Why Reputation Is Constrained

Reputation systems become shadow caste systems if they touch rights or money creation directly. Essentia therefore uses reputation as a narrow operational signal, not as a civil hierarchy.

---

## 11. Monetary System

### 11.1 Instruments

1. Essent (E)
   - transferable fungible currency,
   - used for general exchange, payroll, grants, bonds, and settlement.

2. Essential Unit (U)
   - non-transferable,
   - expiring,
   - restricted to essentials spending,
   - redeemable by approved vendors for E at public rates.

### 11.2 Essent Issuance Sources

E can be created only by these modules:

- contribution mint from finalized claims,
- public treasury grants approved by governance,
- time-bounded bootstrap issuance approved in the launch constitution,
- emergency issuance approved by a higher constitutional threshold and automatic sunset.

No other mint path is valid.

### 11.3 Bootstrap Issuance

A pure percentage-of-supply cap fails at genesis because supply starts near zero.

Therefore mainnet uses two issuance eras:

1. Bootstrap era
   - a published absolute issuance ceiling per epoch,
   - a hard cumulative ceiling,
   - no founder liquid allocation,
   - every unit assigned to public purposes, public grants, security, or approved work.

2. Mature era
   - annual mint bound by policy and circulating supply.

Default mature-era bound:

    AnnualMint(E) <= min( rho * CirculatingSupply_prev, PolicyCap )

with rho set by governance inside a constitutionally bounded range.

### 11.4 Essential Unit (U) Policy

U is the Freedom Floor rail.

Properties:

- issued only to eligible members in active rollout waves,
- expires after a policy-set horizon,
- cannot be transferred peer-to-peer,
- may only be spent to vendors with the appropriate credential categories,
- redeemed by vendors for E through the clearing module,
- indexed to regional essentials baskets under public rules.

### 11.5 Vendor Redemption

Each region r and epoch e publishes a clearing rate:

    redeem_rate[r,e] = E per U

The rate is derived from the regional essentials basket process and bounded by a redemption band. The oracle path uses multiple sources, medianization, circuit breakers, and public override only through auditable governance action.

### 11.6 Sinks and Discipline

E sinks include:

- transaction fees outside subsidized civic actions,
- validator bonds locked for operations,
- slashing,
- optional policy burns if separately approved.

This is not a promise of zero volatility. It is a rules-based monetary constitution.

---

## 12. Treasury

### 12.1 Treasury Structure

The treasury is on-chain and partitioned into explicit funds:

- Civic Gas Pool,
- Security Fund,
- Purpose Budgets,
- UBI Clearing Reserve,
- Grants and Research,
- Emergency Buffer.

Funds cannot be silently commingled.

### 12.2 Spending Rules

Every spend must reference:

- budget authority,
- proposal or policy source,
- recipient class,
- reporting obligation,
- expiry or review date.

Unspent budget rolls forward only if policy allows it.

### 12.3 No Hidden Founder Class

There is no founder premine, hidden vest, or private mint authority. If bootstrap organizers require resources, those resources come from disclosed public budget lines with public terms.

---

## 13. Privacy, Data Minimization, and Public Legibility

### 13.1 Personal Privacy

The protocol minimizes personal data by design.

Protected data includes:

- private communications,
- biometric and health data,
- precise location history,
- identity linkages,
- recovery secrets,
- restricted evidence in restorative or crisis contexts.

### 13.2 Institutional Transparency

The stronger the public power, the stronger the transparency requirement.

The following must be public or publicly reproducible:

- governance rules,
- eligibility criteria,
- code hashes and release artifacts,
- treasury budgets and flows,
- claim rubrics and payouts,
- vendor redemption flows,
- validator set and slashing events,
- model and oracle inputs where they affect rights or money.

### 13.3 Content Addressing and Persistence

Critical texts and evidence bundles are content-addressed off-chain and anchored on-chain. Redundant storage providers are used so that no single actor can erase history.

---

## 14. Security Model

### 14.1 Threats

1. Sybil identity attacks
2. reviewer collusion
3. duplicate claims and payout farming
4. vendor fraud on U redemption
5. validator censorship or double-sign
6. key compromise
7. ballot coercion
8. oracle manipulation
9. roadmap drift into private power

### 14.2 Mitigations

- Personhood credentials with challenge and appeal
- reviewer independence rules and random committee selection
- bonds, slashing, challenge rewards, and audit tails
- explicit quest ceilings and epoch budgets
- vendor credentials, redemption audits, and category controls
- BFT finality with public validator identity and bonds
- social recovery and time-locked key rotation
- encrypted ballots with verifiable decryption
- multi-source medianized redemption inputs with circuit breakers
- sunset clauses and refactoring cycles for policy review

### 14.3 What the Chain Cannot Solve Alone

Essentia can reduce fraud and concentrate power less. It cannot make identity, prices, or human judgment perfect. The hardest parts remain social:

- personhood verification,
- conflict-of-interest disclosure,
- evidence quality,
- local legal compliance,
- vendor onboarding.

The protocol therefore favors bounded authority, public review, and reversible governance over magical guarantees.

---

## 15. Launch Criteria and Phased Rollout

### 15.1 Verifier-First Rule

No mainnet launch until these are public:

- reproducible node builds,
- verifier CLI for elections,
- verifier CLI for claim payouts,
- public test vectors,
- adversarial test reports,
- at least one public mock election,
- at least one public mock claim cycle,
- external audit of the minting and governance modules.

### 15.2 Phases

Phase 0 - simulation
- run elections and claim cycles off-chain,
- finalize data model, proofs, and verifier outputs.

Phase 1 - civic testnet
- DID registry,
- personhood nullifiers,
- proposals,
- encrypted ballots,
- treasury,
- claim reviews with fake money,
- vendor credential flows.

Phase 2 - guarded mainnet
- capped bootstrap issuance,
- real validator bonds,
- real claims under tight purpose budgets,
- small U pilots in limited regions.

Phase 3 - federated expansion
- broader validator rotation,
- more vendor regions,
- higher throughput dissemination layer if needed,
- formalized inter-community federation.

### 15.3 Kill Switches and Rollback

Emergency pause powers are strictly bounded:

- pause can stop new mint finalization and vendor redemption,
- pause cannot rewrite history,
- pause expires automatically unless reauthorized,
- any pause event triggers mandatory public review.

---

## 16. Parameters Requiring Public Ratification

The whitepaper defines the mechanisms. The following must still be ratified in policy:

- bootstrap issuance schedule,
- mature-era rho range,
- review committee size bands,
- challenge and audit horizons,
- default claimant and reviewer bond rates,
- vendor category taxonomy,
- regional basket methodology,
- validator set expansion schedule,
- emergency issuance threshold,
- emergency pause threshold.

---

## 17. Summary of Material Changes from v0.6.0

1. Removed direct attestor-weighted minting.
2. Replaced it with budgeted proof-of-contribution.
3. Removed commit-reveal as a valid private election mode.
4. Clarified that encrypted ballots are mandatory for binding secret votes.
5. Split passkey login from ledger-control signatures.
6. Moved ledger-critical signatures and KEMs to NIST PQC primitives.
7. Reduced the protocol scope to audited native modules.
8. Simplified reputation so it cannot become a civil caste or a mint amplifier.
9. Added bootstrap-era monetary logic so genesis is coherent.
10. Added audit tails, clawbacks, reviewer independence rules, and duplicate-claim controls.
