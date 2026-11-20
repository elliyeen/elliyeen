<img src="banner.png" alt="Forward Deployed Engineering: Enterprise AI, agentic systems, secure software delivery" width="100%">

I turn business problems into secure, measurable AI and software systems, from discovery and architecture through implementation, testing and handoff., with a focus on securing multi-agent AI systems. Based in Dallas, TX.

[Website](https://www.elliyeen.com) · [LinkedIn](https://www.linkedin.com/in/abbasabdullah)

---

## Now featuring: KAI × DART

**[KAI](https://www.elliyeen.com/dart/)** is a proposed station-intelligence layer for Dallas transit: station and shelter quality, contractor accountability, and independently verified outcomes. It is designed to complement existing asset systems such as Maximo, not replace them.

- **[Try the live demo](https://www.elliyeen.com/dart/)**: a guided walkthrough from inspection to work order to independent verification, across 115 station-line records.
- **The KAI loop**: Observe, Decide, Act, Verify, Learn. Plain rules, not a machine-learning model.
- **[Source](https://github.com/elliyeen/kai-dart)**: Next.js 16, React 19, TypeScript, Tailwind v4. CI runs lint, typecheck, unit tests, and a 540-test Playwright and axe suite across five browsers.
- **[Recent activity](https://github.com/elliyeen?tab=overview&from=2026-11-01&to=2026-11-20)**: November 2026 contributions.

Status: a static demonstration with synthetic data. No live DART data, no DART endorsement or integration, and pilot terms are Elliyeen proposals, not DART-approved.

---

## Featured engineering

| | [CAC/PIV Authentication](https://github.com/elliyeen/cac-piv-pki-authentication) | [Ahead / Aliya](https://aheadjobs.com) | [Screenwriter](https://write.elliyeen.com) |
| :--- | :--- | :--- | :--- |
| **Problem** | Adding smart-card (CAC/PIV) sign-in to an existing web application without trusting unverified identity headers. | Matching companies and talent on evidence of what needs to get done, with candidate consent before any introduction. | Writers need one private place for screenplays, books, podcast scripts, research papers and journals, with exact page formats and print-ready export, and without sending their writing to an AI service. |
| **Approach** | Trust boundary at the load balancer, header provenance checks, certificate chain and CRL validation, session controls. Documented in ADRs and a threat model. | Agentic matching with deterministic filtering before AI, human approval gates, and an audit trail. Documented in ADRs. | Rule-based layout engines (no AI model). Per-account authorization on every route, soft deletes and nightly backups. A journal designer that treats a design as validated data and renders a vector PDF at exact page sizes. A written data-loss-prevention policy and data inventory. |
| **Status** | Public reference implementation. Synthetic identities and certificates only. Not a production deployment. | Live product. Source is private. | Live in private beta. Source is private. No third-party security audit yet. |
| **Evidence** | [Tests](https://github.com/elliyeen/cac-piv-pki-authentication/tree/master/reference-impl/src), [ADRs](https://github.com/elliyeen/cac-piv-pki-authentication/tree/master/adr), [5-minute walkthrough](https://github.com/elliyeen/cac-piv-pki-authentication/blob/master/docs/WALKTHROUGH.md). Run it yourself: `pnpm install && pnpm demo` in `reference-impl/` | [aheadjobs.com](https://aheadjobs.com) | [write.elliyeen.com](https://write.elliyeen.com), [Security](https://write.elliyeen.com/security) (verified controls and known gaps), [Pricing](https://write.elliyeen.com/pricing). Unit, API and cross-browser tests run in CI. |
| **Stack** | TypeScript, Next.js, PKI/X.509 | TypeScript, Cloudflare Workers | JavaScript (no framework), Cloudflare Workers, D1, Durable Objects, R2 |

Known limitations are listed in each project, not hidden. For example, revocation checking in the CAC/PIV reference covers CRLs only, and Screenwriter has not had a third-party security audit.

---

## How I work

**Observe** the problem → **Define** users, constraints and acceptance criteria → **Design** and weigh alternatives → **Build** → **Verify** with tests and security review → **Operate** with monitoring and ownership → **Improve** in measured steps.

## Technical disciplines

| Domain | Core stack |
| :--- | :--- |
| **Systems and backends** | Rust (`tokio`, `axum`), TypeScript, Node.js, Python, Bash |
| **Agentic and protocols** | Model Context Protocol (MCP), Claude Code, human-in-the-loop workflows |
| **Infrastructure** | Cloudflare Workers, Pages, D1, GitHub Actions |
| **Security** | Agentic AI security (OWASP Top 10 for LLMs, deterministic filtering before AI, human-approval gates, audit trails), IAM, PKI/X.509, threat modeling, ADRs, data-loss-prevention policy and data inventory, secure SDLC, dependency auditing in CI |

## Other public work

[qaf](https://github.com/elliyeen/qaf) (Rust, SQLite, MCP server) · [Savannah PCS](https://elliyeen.github.io/savannah-pcs/) · [Associated Training Services](https://elliyeen.github.io/associatedtrainingservices/)

## Connect

[LinkedIn](https://www.linkedin.com/in/abbasabdullah)
