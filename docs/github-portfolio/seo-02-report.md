# SEO-02: public repository metadata

Tracker: [#8](https://github.com/elliyeen/elliyeen/issues/8). Applied 2026-11-20. Before-state: [`baseline-metadata.json`](baseline-metadata.json). Followed Playbook v1.0.

## What changed

Descriptions and topics were set from what each repository's own README and code state. No visibility, branch protection, secret or workflow was touched.

| Repo | Description now says | Topics added | Homepage |
| :--- | :--- | :--- | :--- |
| cleared-public | Reproducible benchmark; synthetic data; deterministic checker, not a product | 6 | none |
| kai-dart | Demonstration; synthetic data; not affiliated with or endorsed by DART | 8 | https://www.elliyeen.com/dart/ |
| cac-piv-pki-authentication | Clean-room reference; synthetic certificates; not a production system | 9 | none |
| enterprise-security-agent | Proposed toolkit; not an audit, scanner or certification | 5 | none |
| screenwriter | Public snapshot v0.6.3; rule-based; no AI model; points to the live product | 7 | https://write.elliyeen.com |
| qaf | Rust data layer, REST API and MCP server | 6 | none |
| ai_career_agents | **not changed (blocked, see below)** | 0 | none |

Check the live state with `python3 restore_metadata.py diff` (differences from the baseline) or `gh api repos/elliyeen/<repo>`.

## Restore round trip (recovery demonstrated)

1. Applied the new metadata to all six repositories.
2. Ran `restore_metadata.py restore cleared-public`: description and topics returned to baseline.
3. Re-ran `diff`: no drift reported for `cleared-public` against the baseline.
4. Re-applied the new metadata and confirmed 6 topics on `cleared-public`.

## Blocked: `ai_career_agents`

The repository has one commit ("Initial commit") and a one-line README (`# ai_career_agents`). There is nothing to describe accurately, and inventing a description or topics would breach the "no unsupported claims" rule. **Owner action:** say what the repository is for, or archive it. It is excluded from featured work until then, and this issue stays open for that reason.

## Pinned repositories (manual step)

GitHub does not expose pinning through its API. Recommended pins, strongest evidence first:

1. `screenwriter` (public snapshot of a live product)
2. `cleared-public` (reproducible, CI green)
3. `cac-piv-pki-authentication` (60 tests reproduced)
4. `kai-dart` (CI green)

To pin: open https://github.com/elliyeen, choose **Customize your pins**, select the four above, **Save pins**. Not recommended yet: `qaf` (its CI is failing), `enterprise-security-agent` (its README is two sentences and its workflow only checks that skill files exist), `ai_career_agents` (empty).

## Open items

- README gaps on `enterprise-security-agent` (setup, tests, limitations) were not rewritten in the target repository. That is a separate PR in that repository.
- Pins are unverified until the owner confirms them.
