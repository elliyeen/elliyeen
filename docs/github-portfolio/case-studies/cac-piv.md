# Case study: CAC/PIV authentication reference

**Label: sanitized educational reference.** Synthetic certificates generated at run time with throwaway keys. Not a production system, no authorization process, and no claim about any government environment. Tracker: [#11](https://github.com/elliyeen/elliyeen/issues/11). Repository: [cac-piv-pki-authentication](https://github.com/elliyeen/cac-piv-pki-authentication).

## Problem

Add smart-card sign-in to a web application without trusting an identity header that anyone could forge.

## Design

- **Certificate chain validation:** signature, validity, CA constraints, path length, client-auth EKU.
- **Revocation:** signed-CRL parsing with a cache of bounded staleness, failing closed by default.
- **Header provenance:** trust identity headers only when they provably came from the load balancer (mTLS or source IP; IPv4 only). The signed-header method is not implemented and fails closed.
- **Sessions:** idle and absolute timeouts, CSRF protection and revocation, against an in-memory store.
- Architecture decision records and a threat model are in the repository.

## What is and is not built

| Area | State per the repository |
| :--- | :--- |
| Chain validation, CRL, header provenance, sessions | Implemented and tested (synthetic PKI) |
| OCSP | Planned, not implemented |
| Web app, database, account linking, role mapping, audit logging | Not in this repository (5 `it.todo` tests) |
| Load balancer and VDI deployment | Described in ADRs only |

## Reproduce

```bash
cd reference-impl
pnpm install --frozen-lockfile --ignore-scripts
pnpm test
```

**Verified 2026-11-20** on a fresh clone: 6 test files passed, **60 tests passed, 5 todo**, matching the repository's own [EVIDENCE.md](https://github.com/elliyeen/cac-piv-pki-authentication/blob/master/EVIDENCE.md). The Security workflow succeeded on 2026-11-16. Coverage and demo figures in EVIDENCE.md were not re-run.

## Limits

No real cards, certificate authorities, load balancer or customer environment. No throughput or latency claims. No ATO, STIG or compliance claim follows from this repository. Revocation covers CRLs only.
