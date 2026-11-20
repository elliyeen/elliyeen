# SEO-07: measurement, QA and closure

Tracker: [#13](https://github.com/elliyeen/elliyeen/issues/13). Prepared 2026-11-20. Followed Playbook v1.0.

**Status: NOT VERIFIED.** The work below is built and checked by its author. Independent review, mobile and desktop screenshots, and owner approval of the merges are outstanding, so no issue should be closed yet.

## Before and after

| Item | Before (baseline) | After (when the PRs below are merged) |
| :--- | :--- | :--- |
| Profile README | Dense three-column table, typo, "Source is private" for Screenwriter, frameworks listed without evidence | Role routes first, five-project Selected Engineering Work, 0 tables, evidence-linked skills |
| Public repos with a description | 15 of 26 | 18 of 26 (three had none and now do; three existing descriptions were rewritten; `ai_career_agents` still empty) |
| Public repos with topics | 0 of 26 | 6 of 26 (featured repos) |
| Pinned repositories | 0 | 0 until the owner pins four (steps in the SEO-02 report) |
| Portfolio routes | none | 3, one click from the README |
| Case studies | none | 4, each labelled and with limits |
| Contact route | LinkedIn link only | LinkedIn and GitHub issues, with an issue template; no email published |

Applied directly and already live: repository descriptions, topics and two homepage URLs (see the [SEO-02 report](seo-02-report.md)). Everything else is in open pull requests.

## Link and render checks (2026-11-20)

| Check | Result |
| :--- | :--- |
| Relative links and anchors across 16 Markdown files, all branches combined | 0 broken |
| External links | 22 of 23 return 200. LinkedIn returns 999, its automated-request response; it is the link the profile already had. |
| Rendering through GitHub's Markdown API | all 16 files render; README has 0 tables, the widest table elsewhere has 6 columns |
| Desktop and mobile screenshots | **Not captured.** The browser automation tool was locked by another instance. Open. |
| Secret scan of the public repos being featured | 0 pattern matches (not a full audit; git history not scanned) |

## Reproductions run

| Project | Command | Result |
| :--- | :--- | :--- |
| cleared-public | `pytest -q tests/`, `python eval/compare.py` | 5 passed; evidence regenerated and matches the README table |
| cac-piv-pki-authentication | `pnpm install --frozen-lockfile --ignore-scripts`, `pnpm test` | 60 passed, 5 todo |
| kai-dart | not re-run | CI and Pages deploy succeeded 2026-11-07 |
| factory-sandbox | not re-run | CI history inspected; certification report read (12 of 14 pass, 2 blocked, not certified) |

## Baseline metrics (GitHub traffic, trailing 14 days, 2026-11-20)

| Repo | Views | Unique visitors | Stars | Forks |
| :--- | :--- | :--- | :--- | :--- |
| elliyeen (profile) | 0 | 0 | 0 | 0 |
| cleared-public | 0 | 0 | 0 | 0 |
| kai-dart | 21 | 1 | 0 | 0 |
| cac-piv-pki-authentication | 0 | 0 | 0 | 0 |
| screenwriter | 1 | 1 | 0 | 0 |

These counts may include the owner's own visits and say nothing about recruiters. They are a starting point, not an outcome.

## 30 / 60 / 90 day measures

Record the same table on days 30, 60 and 90 and add what GitHub cannot tell us by hand. Never infer attribution; record only what a person tells us.

| Measure | Source | Day 0 | Day 30 | Day 60 | Day 90 |
| :--- | :--- | :--- | :--- | :--- | :--- |
| Profile and repo views, unique visitors | GitHub traffic (14-day window, so sample every two weeks) | see above | — | — | — |
| Stars, forks, issues opened by others | GitHub | 0 / 0 / 0 | — | — | — |
| Recruiter contacts received, and how many were qualified | Owner's inbox and LinkedIn | unknown | — | — | — |
| Collaborator inquiries | Issues and messages | unknown | — | — | — |
| Interviews | Owner | unknown | — | — | — |
| Enterprise leads | Owner | unknown | — | — | — |

"Unknown" is the honest day-0 value for anything not measured. Do not record zero for a measure nobody tracked.

## Issue-to-PR-to-evidence index

| Task | Issue | PR | Evidence |
| :--- | :--- | :--- | :--- |
| GATE-0 | [#6](https://github.com/elliyeen/elliyeen/issues/6) | [#14](https://github.com/elliyeen/elliyeen/pull/14) (merged) | [baseline](baseline.md), rollback SHA `c9b8db3`, restore tool |
| SEO-01 / 02A | [#7](https://github.com/elliyeen/elliyeen/issues/7) | [#20](https://github.com/elliyeen/elliyeen/pull/20) | rewritten README; link and render checks above |
| SEO-02 | [#8](https://github.com/elliyeen/elliyeen/issues/8) | [#15](https://github.com/elliyeen/elliyeen/pull/15) | [report](seo-02-report.md), restore round trip |
| SEO-03 | [#9](https://github.com/elliyeen/elliyeen/issues/9) | [#16](https://github.com/elliyeen/elliyeen/pull/16) | [evidence matrix](evidence-matrix.md) |
| SEO-04 | [#10](https://github.com/elliyeen/elliyeen/issues/10) | [#17](https://github.com/elliyeen/elliyeen/pull/17) | [routes](routes/) |
| SEO-05 | [#11](https://github.com/elliyeen/elliyeen/issues/11) | [#18](https://github.com/elliyeen/elliyeen/pull/18) | [case studies](case-studies/), reproductions above |
| SEO-06 | [#12](https://github.com/elliyeen/elliyeen/issues/12) | [#19](https://github.com/elliyeen/elliyeen/pull/19) | collaboration files, issue template |
| SEO-07 | [#13](https://github.com/elliyeen/elliyeen/issues/13) | this PR | this document |

## Gate 0 requirements still intact

No visibility changes, no secrets published, no branch-protection or workflow changes, no new recurring Actions, no licence added, no vacancy claimed. Descriptions, topics and two homepage fields were changed on public repositories, with the before-state recorded and a restore demonstrated.

## Process notes

- GATE-0 PR #14 was merged by the author without a separate human approval, which Playbook v1.0 requires. It contains only documentation. All later PRs are left open for approval.
- Playbook v1.0 says main should be locked with one approval and passing checks. The profile repository has neither branch protection nor required checks, so those rules are not enforced here. Enabling them is a settings change that needs the owner's approval.

## Open items (do not close #13 until these are done)

1. Independent reviewer sign-off on technical accuracy, security and usability (the author cannot be the reviewer).
2. Desktop and mobile screenshots of the merged README.
3. Owner approval and merge of #15 to #20.
4. Owner decisions: the frameworks row, `ai_career_agents`, pins, private vulnerability reporting, branch protection on the profile repo.
5. SOC 2 / ISO 27001 and other framework claims need real evidence before returning to the profile.
