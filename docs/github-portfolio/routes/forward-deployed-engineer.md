# Route A: Forward Deployed / Enterprise Delivery

For hiring managers looking for someone who takes an ambiguous customer problem to a working, verifiable system. Tracker: [#10](https://github.com/elliyeen/elliyeen/issues/10). Evidence levels are defined in the [evidence matrix](../evidence-matrix.md).

## The shape of the work

Define the failure first, build the smallest thing that removes it, prove it with evidence someone else can re-run, and say plainly what it does not do.

## Example 1: KAI × DART, enterprise workflow with independent verification

- **Problem:** Station quality is hard to see and harder to prove. A contractor saying a fix is done is not proof.
- **Architecture:** Inspection, then work order, then assignment to a named owner, then proof submitted by the crew, then approval or rejection by a *different* inspector. Role-based views (executive, operations, station manager, inspector, contractor and crew, security and compliance, admin). Sits beside existing work-order systems such as Maximo.
- **My contribution:** Designed and built the demonstration. Source in [kai-dart](https://github.com/elliyeen/kai-dart).
- **Security and reliability:** Separation of duties is part of the workflow. A Playwright and axe end-to-end suite (540 tests across several browsers and viewports, per the repository README) is part of CI; the last CI and Pages deploy runs succeeded on 2026-11-07. I did not re-run the suite for this page.
- **Try it:** [elliyeen.com/dart](https://www.elliyeen.com/dart/) has a two-minute guided walkthrough.
- **Outcome and limits:** Static demonstration on synthetic data. No stations are connected to live DART data, and it is not affiliated with or endorsed by DART. No measured outcome exists. Pilot scope and pricing are proposals.

## Example 2: Screenwriter, a live product

- **Problem:** Writers want one private place for screenplays, books, podcast scripts, research and journals, with exact page formats and print-ready export.
- **Architecture:** Plain JavaScript front end on Cloudflare Workers, D1, Durable Objects and R2. Rule-based layout, no AI model.
- **My contribution:** Built and operate it.
- **Security and reliability:** A [threat model and list of known gaps](https://github.com/elliyeen/screenwriter/blob/main/docs/SECURITY.md) are published. Soft deletes and nightly backups are described in the repository docs.
- **Try it:** [write.elliyeen.com](https://write.elliyeen.com) (private beta). Source: [public snapshot v0.6.3](https://github.com/elliyeen/screenwriter); the active repository is private and ahead of it.
- **Outcome and limits:** No third-party security audit. No user or revenue figures are published here.

## Example 3: CLEARED benchmark, defining and measuring a risk

See [CLEARED case study](../case-studies/cleared.md): a payment-authorization checker measured against baseline, holdouts and safety gates, with a regression reported and not hidden.

## What this route does not show

Customer deployments, production traffic, quantified ROI, or cross-team leadership. See the [gap assessment](../evidence-matrix.md#gap-assessment).

## Seven capabilities, with evidence

Levels are defined in the [evidence matrix](../evidence-matrix.md). "Gap" means no evidence was found.

| # | Capability | Evidence | Level | Gap or limit |
| :--- | :--- | :--- | :--- | :--- |
| 1 | Customer discovery | Writer-complaint research and ranked user stories behind Screenwriter | Private | Not inspectable by readers |
| 2 | End-to-end engineering | [Screenwriter](../case-studies/screenwriter.md) (27 unit checks reproduced); static sites [Associated Training Services](https://elliyeen.github.io/associatedtrainingservices/) and [Savannah PCS](https://elliyeen.github.io/savannah-pcs/) | Reproduced, Public | Customer scope and outcomes for the two sites are not documented |
| 3 | AI and architecture | [KAI × DART](../case-studies/kai-dart.md), [CLEARED](../case-studies/cleared.md) | Public, Reproduced | Synthetic data. No production LLM agents are evidenced publicly |
| 4 | Security and reliability | [CLEARED](../case-studies/cleared.md), [CAC/PIV](../case-studies/cac-piv.md), [Screenwriter threat model](https://github.com/elliyeen/screenwriter/blob/main/docs/SECURITY.md) | Reproduced, Public | [PayBlue](../case-studies/payblue.md) security is design only. Security Gatekeeper not found. No third-party audit |
| 5 | Business impact | None | None found | No baseline, customer or revenue figure exists |
| 6 | Communication | Case studies with identical sections and plain language | Public | No reader feedback recorded |
| 7 | Verifiable delivery | Issues and PRs on this repository, reproducible tests, rollback tag | Public | No independent reviewer sign-off yet. No documented user feedback |

Case-study section order used for new pages: customer problem, research evidence, my role, constraints, architecture, implementation, security, deployment status, reproducible validation, measurable results, limitations, next step.
