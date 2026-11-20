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
