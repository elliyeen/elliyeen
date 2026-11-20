# Skills-to-evidence matrix

Tracker: [#9](https://github.com/elliyeen/elliyeen/issues/9). Verified 2026-11-20. Followed Playbook v1.0.

Every capability below maps to something a reader can inspect. Where the evidence is private or missing, the row says so. Nothing here is a claim of customers, revenue, certification or government authorization.

**Evidence levels:** *Reproduced* (I ran it on 2026-11-20 and got the stated result), *Public* (inspectable in a public repository or live URL, not re-run), *Private* (exists but cannot be inspected from here), *None found*.

## Capability to evidence

| Capability | Evidence | Level | Limits |
| :--- | :--- | :--- | :--- |
| Identity and trust boundaries (PKI, revocation, header provenance, sessions) | [cac-piv-pki-authentication](https://github.com/elliyeen/cac-piv-pki-authentication): `pnpm test` gave 60 passed, 5 todo; [EVIDENCE.md](https://github.com/elliyeen/cac-piv-pki-authentication/blob/master/EVIDENCE.md), ADRs and threat model in the repo | Reproduced | Synthetic certificates only. OCSP not implemented. No real cards, CA, load balancer or customer environment. |
| Measuring a high-stakes decision system (baseline, holdouts, safety gates, reporting a regression) | [cleared-public](https://github.com/elliyeen/cleared-public): `pytest` 5 passed, `python eval/compare.py` regenerates the evidence; [improvement report](https://github.com/elliyeen/cleared-public/blob/main/evidence/improvement-report.md) | Reproduced | 13 hand-built cases per set, same author wrote checker and fixtures. Deterministic code, no model. Rates are illustrative, not statistical. |
| Customer-facing product engineering and secure delivery | [write.elliyeen.com](https://write.elliyeen.com) (HTTP 200, no redirect, 2026-11-20); [screenwriter](https://github.com/elliyeen/screenwriter) public snapshot v0.6.3 with [threat model and known gaps](https://github.com/elliyeen/screenwriter/blob/main/docs/SECURITY.md) | Public | Snapshot is behind the private active repository. Live in private beta. No third-party security audit. Signed-in features were not tested for this matrix. |
| Cloudflare Workers, D1, Durable Objects, R2 | Screenwriter architecture docs and `wrangler.jsonc` in the public snapshot | Public | Not independently load-tested. |
| Enterprise workflow design with independent verification and separation of duties | [kai-dart](https://github.com/elliyeen/kai-dart) and [elliyeen.com/dart](https://www.elliyeen.com/dart/); CI and Pages deploy succeeded 2026-11-07 | Public | Static demonstration on synthetic data. No live DART data, not affiliated with or endorsed by DART. Pilot terms are proposals. |
| Agent delivery gates (failing test blocks merge, security finding blocks merge, stacked PR denied, worker restart without duplicate work, self-modification guard) | [factory-sandbox](https://github.com/elliyeen/factory-sandbox) CI runs and the factory certification report (11 of 14 tests PASS) | Public (sandbox), report is local | **Not certified.** T4 (auto-merge under independent approval) is BLOCKED for lack of a second approver; T13 (measured improvement) is BLOCKED for lack of real before/after history. Sandbox is synthetic. |
| Security engineering toolkit for agent projects | [enterprise-security-agent](https://github.com/elliyeen/enterprise-security-agent): 12 skills, templates, validation workflow | Public | Proposed. The workflow only checks that skill files exist. Not an audit, scanner or certification. |
| Agentic matching with deterministic filtering before AI and human approval gates | [aheadjobs.com](https://aheadjobs.com) is live (HTTP 200) | Private | Source and ADRs are private, so this cannot be inspected here. Scale and usage figures are not published. |
| Rust systems and MCP servers | [qaf](https://github.com/elliyeen/qaf) (Rust workspace: SQLite layer, REST API, MCP server) | Public | **CI is currently failing** (last runs 2026-06-14 and 2026-07-25). Not featured as evidence until fixed. |
| Data-loss-prevention policy, data inventory, retention design | Screenwriter work tracked in a private repository | Private | Not inspectable here. |

## Gap assessment

These are the areas a reader would reasonably ask about. The honest answer for each is below, and none is filled with a claim.

| Area | What the evidence shows | Gap |
| :--- | :--- | :--- |
| Production LLM deployment (years) | None found in public repositories. Screenwriter uses no AI model by design. CLEARED's checker is deterministic. | Any production LLM history is in private work (for example Ahead) and is not publicly evidenced. The owner can state years and scale directly. |
| Multi-agent production experience | The factory sandbox shows gates and recovery on synthetic work, not production traffic. | No public production multi-agent system. |
| AWS (including Bedrock) | No AWS implementation found in the public repositories. Infrastructure evidence is Cloudflare. | AWS Bedrock should not be claimed. |
| ML engineering (training, evaluation of learned models, ML infrastructure) | None found. Evaluation discipline is shown on a deterministic checker. | No public model training or ML infrastructure work. |
| Cross-team technical leadership | None inspectable in repositories. | Needs written examples from the owner, outside this repository. |
| Quantified business ROI | None. CLEARED states it is not a product and has no customers or revenue. | No measured ROI is published. Do not imply one. |
| Compliance frameworks | Draft SOC 2 and ISO 27001 policies (v0.1) exist in a private workspace. No audit or certification. | HIPAA, PCI-DSS, FedRAMP, CMMC and NIST AI RMF have no evidence in any repository. |

## Role families this supports

The three routes ([Forward Deployed](routes/forward-deployed-engineer.md), [AI Architect](routes/ai-architect.md), [AI Security](routes/ai-security.md)) cover principal-level roles in forward-deployed engineering, AI architecture and AI engineering, and AI security. Specific postings were not retrieved for this work, so no employer requirement is asserted. Verify any posting directly before treating a title as matching, including its stated location and remote status.

## How to update

Add a row only with a link a stranger can open. If a row's evidence stops being true (for example a CI run fails), change its level the same day.
