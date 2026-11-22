# Case study: PayBlue

**Label: concept and early prototype. Not a live payment product.** Tracker: [#26](https://github.com/elliyeen/elliyeen/issues/26). Public page: [payblue.app](https://payblue.app). Source is a private repository, so the code cannot be inspected here.

**What is not claimed:** no approval, certification or endorsement by any government body, including Texas DIR; no live payments; no connection to any benefits system, retailer or processor; no fraud or dollar figures.

## Customer and market problem

Public-benefit cards (SNAP, WIC, TANF in Texas) can be skimmed or cloned. The idea is to move the secret from a plastic card to a protected identity on the phone. This is a problem statement and a design, not a solved problem.

## Research evidence

None published here. Any numbers about fraud losses would need a named public source, and none is cited.

## My role

Sole designer and builder of the front end and the API skeleton.

## Constraints

Handles sensitive benefit and identity data, so it must not claim security it has not built. Any real use needs an agreement with the state and its processors, which does not exist.

## Architecture (intended)

```
Phone app (React) -> API (Rust / Axum) -> database
   biometric or PIN sign-in, device binding, signed transactions, audit trail
```

## Implementation (what exists, verified by reading the code 2026-11-22)

- **Built:** React front end on Cloudflare (marketing, security, accessibility and policy pages), Rust API skeleton with configuration, error handling and an audit module with unit tests, CI definitions (format, lint, tests), and a static security check script for the front end.
- **Not built:** account register and login, token refresh, wallet, stores and transactions routes (they return `501 Not Implemented`), biometric token hashing and transaction signing (the functions are `unimplemented!`), and authentication middleware.

## Security

A design intent only. Encryption, tokenisation, biometric sign-in and device binding appear in planning text but are **not implemented** in the code. No security test of the product exists.

## Deployment status

The public site loads (HTTP 200). It is a presentation of the concept. The owner reports awaiting a decision from Texas DIR; the type of approval is unconfirmed and is not claimed.

## Reproducible validation

Not available publicly. Reproduction needs the private repository. I read the source and did not run its test suite for this page.

## Measurable results

None.

## Limitations

Everything above. The honest summary: a convincing front end and a well-organised back-end skeleton, with the security-critical parts still to be built.

## Next step

Implement authentication and cryptography with tests first, publish a threat model, and keep the maturity label until each piece is verified.
