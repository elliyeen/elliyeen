# Engineering secure Enterprise AI systems at scale.

I build secure, measurable AI and software systems, from discovery and architecture through implementation, testing and handoff, with a focus on securing agent-based systems. Dallas, TX.

[Website](https://www.elliyeen.com) · [LinkedIn](https://www.linkedin.com/in/abbasabdullah) · [Work with me](WORK_WITH_US.md)

This is my personal engineering profile: the proof, with limits stated. Product pages for Elliyeen LLC live on [elliyeen.com](https://www.elliyeen.com).

## Where to start

- **Forward deployed / enterprise delivery:** [Route A](docs/github-portfolio/routes/forward-deployed-engineer.md)
- **AI architecture, agentic systems, ML engineering:** [Route B](docs/github-portfolio/routes/ai-architect.md)
- **AI security and trust and safety:** [Route C](docs/github-portfolio/routes/ai-security.md)

Every capability I claim is mapped to something you can open in the [skills-to-evidence matrix](docs/github-portfolio/evidence-matrix.md), including the gaps.

## Selected engineering work

**[Screenwriter](https://write.elliyeen.com)** · live product, private beta · [public source snapshot (v0.6.3)](https://github.com/elliyeen/screenwriter)
One private place for screenplays, books, podcast scripts, research and journals, with exact page formats and print-ready export. Rule-based layout, no AI model. Cloudflare Workers, D1, Durable Objects, R2. Published [threat model and known gaps](https://github.com/elliyeen/screenwriter/blob/main/docs/SECURITY.md). No third-party security audit. The active repository is private and ahead of the public snapshot.

**[CLEARED benchmark](https://github.com/elliyeen/cleared-public)** · benchmark on synthetic data · [case study](docs/github-portfolio/case-studies/cleared.md)
A payment-authorization checker measured against a baseline, holdouts and automatic safety gates, with a regression reported and not hidden. Reproduces with a few commands. Deterministic code, no model, no customers or revenue.

**[KAI × DART](https://www.elliyeen.com/dart/)** · demonstration on synthetic data · [source](https://github.com/elliyeen/kai-dart) · [case study](docs/github-portfolio/case-studies/kai-dart.md)
Station-quality intelligence where a fix only counts when a different, independent inspector accepts the evidence. Sits beside systems such as Maximo. Not connected to live DART data, and not affiliated with or endorsed by DART.

**[CAC/PIV authentication](https://github.com/elliyeen/cac-piv-pki-authentication)** · sanitized reference · [case study](docs/github-portfolio/case-studies/cac-piv.md)
Smart-card sign-in without trusting spoofable identity headers: certificate chain and CRL validation, header provenance, revocable sessions. 60 tests pass on a fresh clone. Synthetic certificates only; OCSP not implemented; not a production system.

**[Engineering factory](https://github.com/elliyeen/factory-sandbox)** · tested in a synthetic sandbox · [case study](docs/github-portfolio/case-studies/engineering-factory.md)
Gates that stop AI coding agents from approving or merging their own work. 12 of 14 certification tests pass; two are blocked, so it is **not certified**.

**[Ahead / Aliya](https://aheadjobs.com)** · live product, source private
Matching companies and talent on evidence of what needs to get done, with deterministic filtering before AI, human approval gates and candidate consent before any introduction. The source is private, so this is not inspectable here.

## How I work

Define the failure and the acceptance criteria first. Build the smallest thing that removes it. Verify with tests and security review that someone else can re-run. Say plainly what it does not do.

## Skills, with evidence

- **Languages:** Rust, TypeScript, Python, Bash.
- **Platforms:** Cloudflare Workers, D1, Durable Objects, R2, Pages; GitHub Actions.
- **Agents and protocols:** Model Context Protocol, Claude Code, human-in-the-loop approval gates.
- **Security:** PKI and X.509, identity boundaries, threat modeling, architecture decision records, dependency auditing in CI, data-loss-prevention policy and data inventory (private work).
- **Compliance:** SOC 2 and ISO 27001 policies are drafts for my own products. There is no audit or certification, and no public evidence for other frameworks. See the [gap assessment](docs/github-portfolio/evidence-matrix.md#gap-assessment).

## Other public work

[Savannah PCS](https://elliyeen.github.io/savannah-pcs/) · [Associated Training Services](https://elliyeen.github.io/associatedtrainingservices/)

## Contact

[LinkedIn](https://www.linkedin.com/in/abbasabdullah)
