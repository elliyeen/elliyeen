# GitHub portfolio baseline (GATE-0)

Captured 2026-11-20 by Claude Code. Tracker: [#6](https://github.com/elliyeen/elliyeen/issues/6). Brief: the Notion handoff "GitHub SEO, Talent Discovery & Engineering Portfolio" (reference only; GitHub Issues are authoritative).

## Recovery point

| Item | Value |
| :--- | :--- |
| Profile repo rollback SHA (`main`, before any SEO work) | `c9b8db3e3ab701574a1536ce85b67f182cc84d37` |
| Public repo metadata before-state | [`baseline-metadata.json`](baseline-metadata.json) |
| Restore tool | [`restore_metadata.py`](restore_metadata.py) (`diff` shows drift, `restore` re-applies description and topics only) |
| Profile README rollback | `git revert` the merge commit, or `git checkout c9b8db3e3ab701574a1536ce85b67f182cc84d37 -- README.md` on a branch |

**Restore verification:** `restore_metadata.py diff` against live GitHub reported 0 repositories differing from baseline immediately after capture. A change-and-restore round trip is recorded in the SEO-02 evidence.

## Inventory (live, revalidated)

The handoff's "71 repositories (27 public, 44 private)" was revalidated as **70 repositories: 26 public, 44 non-public**. Forks are listed but are not original work and are not featured.

| Public repo | Description (before) | Topics (before) | Last push |
| :--- | :--- | :--- | :--- |
| elliyeen | Profile README | — | 2026-11-20 |
| factory-sandbox | Synthetic sandbox for engineering factory certification (no  | — | 2026-11-18 |
| screenwriter | — | — | 2026-11-16 |
| elliyeen-get-started | — | — | 2026-11-08 |
| review-8f96d821 | — | — | 2026-11-07 |
| kai-dart | — | — | 2026-11-07 |
| cleared-public | Reproducible benchmark of a payment-authorization checker: b | — | 2026-11-02 |
| cac-piv-pki-authentication | Elliyeen LLC - CAC/PIV PKI Authentication Reference Implemen | — | 2026-10-26 |
| enterprise-security-agent | — | — | 2026-10-11 |
| rea (fork) | Reverse engineer anything with agents, from app behavior dow | — | 2026-10-11 |
| elliyeen-naval-portfolio | Interactive naval engineering and cybersecurity portfolio | — | 2026-10-10 |
| savannah-pcs | — | — | 2026-07-26 |
| g3-girls-flag-football | — | — | 2026-07-26 |
| ai_career_agents | — | — | 2026-07-26 |
| humanlayer (fork) | The best way to get AI coding agents to solve hard problems  | — | 2026-07-25 |
| tradingview-mcp-jackson (fork) | AI-assisted TradingView chart analysis — connect Claude Code | — | 2026-07-25 |
| qaf | Word-level Quranic data access layer — SQLite, REST API, and | — | 2026-07-25 |
| associatedtrainingservices | Hands on heavy equipment training program | — | 2026-07-02 |
| protobuf (fork) | Protocol Buffers - Google's data interchange format | — | 2026-04-24 |
| open-multi-agent (fork) | Production-grade multi-agent orchestration framework. Model- | — | 2026-04-01 |
| collection-claude-code-source-code (fork) | 🔥 A collection of the newest Claude Code open source | — | 2026-04-01 |
| claude-code-source-code-leak (fork) | Claude Code Snapshot for Research. All original source code  | — | 2026-03-31 |
| claude-code (fork) | Claude Code is an agentic coding tool that lives in your ter | — | 2026-03-31 |
| cakedup | — | — | 2026-02-19 |
| test | — | — | 2026-02-01 |
| linkedin | — | — | 2026-01-27 |

Profile state before SEO work: bio is set; **no pinned repositories**; **no repository has any topic**; 11 of 26 public repositories have no description.

## Featured-repo controls

| Repo | Branch protection | Auto-merge | Workflows | Open issues |
| :--- | :--- | :--- | :--- | :--- |
| cleared-public | none | off | benchmark | 0 |
| kai-dart | none | off | CI, Deploy to GitHub Pages | 5 |
| cac-piv-pki-authentication | none | off | CI, Deploy Preview, Security, Dependabot Updates | 0 |
| enterprise-security-agent | none | off | Security skills validation | 0 |
| screenwriter | none | off | none | 0 |
| qaf | none | off | ci.yml | 0 |
| ai_career_agents | none | off | none | 0 |
| elliyeen (profile) | none | off | none | 8 (this work) |

No existing issues were found for this effort, so nothing was duplicated.

## Findings that need an owner decision

1. **No featured repo has branch protection or auto-merge.** Per the engineering standard, auto-merge is therefore not available and each merge here is a manual, recorded action. Enabling protection is a settings change that was not made without approval.
2. **A public `screenwriter` repository exists.** It is a public snapshot of the Screenwriter application source (release 0.6.3, last pushed 2026-11-16, about 2.4 MB). The previous profile README said Screenwriter's source is private; the active repository (`internal_screenwriter`) is private. Both statements cannot be left as they were. This work corrects the wording to describe the public snapshot honestly. **Visibility was not changed** (out of scope for this effort). The owner should decide whether the snapshot should stay public.
3. **`enterprise-security-agent` and `ai_career_agents` have thin READMEs.** `ai_career_agents` is a one-line stub and is not featured on that basis.
4. **Pinned repositories cannot be set through the API.** Manual steps are in the SEO-02 report.

## Secret scan (public repos being featured)

A pattern scan (AWS access keys, PEM private keys, GitHub tokens, `sk-` keys, Resend keys) over the working trees of cleared-public, enterprise-security-agent, screenwriter, ai_career_agents and factory-sandbox found **0 matches**. This is a pattern scan, not a full secret audit, and git history was not scanned.

## Change scope approved by this baseline

Allowed: descriptions, topics, README and docs content in the profile repo, new docs in featured repos via reviewable PRs. Not allowed and not done: visibility changes, secrets, weakening of security or release gates, new recurring Actions, adding licences, claiming open positions.
