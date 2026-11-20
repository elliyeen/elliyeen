# Route C: AI Security and Trust and Safety

For hiring managers looking for security engineering applied to identity, agents and high-stakes decisions. Tracker: [#10](https://github.com/elliyeen/elliyeen/issues/10). Evidence levels are defined in the [evidence matrix](../evidence-matrix.md).

## The security position

Do not trust a claim because it arrived. Verify provenance, fail closed, separate who does the work from who approves it, and publish what is not covered.

## Example 1: CAC/PIV smart-card authentication reference

- **Problem:** Add smart-card sign-in to a web application without trusting spoofable identity headers.
- **Architecture:** Trust boundary at the load balancer, header provenance checks, certificate chain and CRL validation (fail closed), revocable sessions with idle and absolute timeouts and CSRF protection. Documented in ADRs and a threat model.
- **Evidence:** [cac-piv-pki-authentication](https://github.com/elliyeen/cac-piv-pki-authentication). `pnpm test` gave 60 passed and 5 todo on 2026-11-20, matching its published [EVIDENCE.md](https://github.com/elliyeen/cac-piv-pki-authentication/blob/master/EVIDENCE.md). A red-team test file is included. See the [case study](../case-studies/cac-piv.md).
- **Limits:** Synthetic certificates. OCSP not implemented. Account linking, role mapping and audit logging are not in the repository. Not a production system, and no authorization claim.

## Example 2: Gates for AI coding agents

- **Problem:** An agent that can merge its own work can also merge its own mistakes.
- **Architecture:** A failing test blocks merge; a synthetic security finding blocks merge; a PR whose base is not the default branch is denied; production needs owner approval and self-approval is refused; protected files are hash-locked. See the [case study](../case-studies/engineering-factory.md).
- **Limits:** Synthetic sandbox. **Not certified**: independent-approval merge (T4) and measured improvement (T13) are blocked.

## Example 3: CLEARED safety gates

A change is rejected automatically if it approves anything the baseline would have held, on any evaluation set. See the [case study](../case-studies/cleared.md). Deterministic checker, synthetic data.

## Example 4: Published threat model and known gaps

Screenwriter publishes its [threat model, controls and known gaps](https://github.com/elliyeen/screenwriter/blob/main/docs/SECURITY.md). It has had no third-party security audit.

## What this route does not show

No penetration-test reports, no audited compliance (SOC 2 and ISO 27001 policies are drafts), no public model-poisoning or adversarial-ML research, and no customer incident response. See the [gap assessment](../evidence-matrix.md#gap-assessment).
