# FDE portfolio audit

Tracker: [#26](https://github.com/elliyeen/elliyeen/issues/26). Audited 2026-11-22 from public pages, public repositories and the owner's private repositories (read through `gh`). Nothing private is quoted here.

**Evidence levels:** *Reproduced* (I ran it), *Public* (inspectable, not re-run), *Private* (exists, not inspectable by readers), *None found*.

## Listed sites and repositories

| Item | Link status | What was verified | Not established | Level |
| :--- | :--- | :--- | :--- | :--- |
| Screenwriter | [write.elliyeen.com](https://write.elliyeen.com) HTTP 200, title "Screenwriter: Your story starts here" | Public snapshot v0.6.3 (`2c4434e`). `npm test` after `npm ci`: 27 unit checks passed (11, 1, 7 and 8 in four suites). Architecture, security, testing and backup docs exist. | Browser suite (about 95 checks per `docs/TESTING.md`) not run. Signed-in features not tested. Active private repo is ahead of the snapshot. No user numbers. | Reproduced (unit), Public |
| PayBlue | [payblue.app](https://payblue.app) HTTP 200, title "PayBlue — Secure Texas EBT App", a script-rendered page with almost no readable text | Source appears to be a private repository of mine (matching description). Rust/Axum API skeleton plus React front end on Cloudflare. Config, audit and error modules are implemented. | **`register`, `login`, `logout`, `refresh`, wallet, stores and transactions routes return 501 Not Implemented. Crypto functions are `unimplemented!`. Auth middleware is TODO.** No live payments, no benefit data, no state approval found. The repo README states a dollar "Measured Impact"; no measurement backs it. | Private |
| CLEARED | `cleared-public` public | Reproduced 2026-11-20: 5 tests passed, comparison regenerates ([case study](case-studies/cleared.md)) | Synthetic data, deterministic checker | Reproduced |
| KAI × DART | [Demo](https://elliyeen.github.io/kai-dart/platform) HTTP 200 | Public repo, CI per earlier audit | Synthetic data, not affiliated with DART | Public |
| Associated Training Services | [Site](https://elliyeen.github.io/associatedtrainingservices/) HTTP 200 | Public repo `associatedtrainingservices` (JavaScript, 10 commits, created 2026-07-02). Commit history shows accessibility, PWA, cross-browser and GitHub Pages base-path work. | Customer relationship, scope, who paid, outcomes. The site's own claims (training history, approvals) belong to that business and are not repeated here. | Public |
| Savannah PCS | [Site](https://elliyeen.github.io/savannah-pcs/) HTTP 200 | Public repo `savannah-pcs` (HTML, 30 commits, created 2026-06-15). Latest commits add accessibility and performance audit tooling and reports. | Customer relationship, scope, outcomes. Business claims on the site (licensing, ratings) belong to that business and are not repeated here. | Public |
| Security Gatekeeper | No matching repository found by name in the `elliyeen` account | None | Exact repository, policy-as-code, signed evidence, restore and rollback | None found |
| Rust trading engine | Private repositories exist | None published | Needs owner authorisation and a sanitised write-up | Private |
| CAC/PIV | `cac-piv-pki-authentication` public | Reproduced 2026-11-20: 60 passed, 5 todo | Synthetic certificates, OCSP not implemented | Reproduced |

## Decisions that follow from the audit

1. **PayBlue is labelled "concept and early prototype".** Public wording says what is built (front end, API skeleton, configuration, audit and error handling) and what is not (authentication, wallet, transactions, cryptography, any live payment). It does not repeat the private README's impact figure or its security claims, and it makes no statement about Texas DIR beyond "no approval, certification or endorsement is claimed".
2. **Associated Training Services and Savannah PCS are listed as static sites with verified engineering work only.** No customer, revenue or outcome claim.
3. **Security Gatekeeper and the Rust trading engine are not on the public profile** until a repository is named or the owner authorises a sanitised case study.
4. **The private PayBlue repository's README needs correcting.** Its "Measured Impact" and "Technical Architecture" sections describe design intent as if it were built. This is a finding for the owner, not changed here.

## Seven FDE capabilities

See [Route A](routes/forward-deployed-engineer.md#seven-capabilities-with-evidence). Business impact and real user feedback have no evidence and are listed as gaps.

## Not done in this change

- Contribution visualisation from real `contributionCalendar` data (separate issue).
- Independent reviewer sign-off (owner action).
- Mobile and desktop screenshots of the rendered profile (needs the merged page).
