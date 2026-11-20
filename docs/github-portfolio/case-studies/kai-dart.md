# Case study: KAI × DART

**Label: demonstration on synthetic data.** No stations are connected to live DART data. Not affiliated with or endorsed by DART. Pilot scope and pricing are Elliyeen proposals. Tracker: [#11](https://github.com/elliyeen/elliyeen/issues/11). Live: [elliyeen.com/dart](https://www.elliyeen.com/dart/). Source: [kai-dart](https://github.com/elliyeen/kai-dart).

## Problem

Station and shelter quality is hard to see and harder to prove. A contractor's own completion claim is not independent evidence.

## Design

- **Flow:** inspection, then work order, then assignment to a named owner, then proof submitted by the crew, then approval or rejection by an independent inspector. The result appears in the executive view only after that sign-off, and the person who did the work cannot be the one who verifies it.
- **Fit:** it sits on top of systems of record such as Maximo and adds a station-level view and independent verification.
- **Roles:** executive, operations, station manager, inspector, contractor and crew, security and compliance, admin. In the demo, role switching is a simulation, not access control; the site states that a production rollout would enforce access on the server.
- **Rules, not a model:** the repository README describes the inspection loop as plain rules and not a machine-learning model, with state kept in the browser.

## Evidence

| Item | State |
| :--- | :--- |
| Live site | HTTP 200 on 2026-11-20 |
| CI and Pages deploy | Succeeded 2026-11-07 (latest runs) |
| Test suite | 540 end-to-end tests across several browsers and viewports, per the repository README. **Not re-run** for this page. |
| Stack | Next.js 16, React 19, TypeScript, Tailwind CSS v4, Playwright, axe, Vitest |

## Limits

Static export with synthetic data. The pilot success measures named on the site (time to resolution, verified-resolution rate, repeat issues) are proposals and have no measured values. No claim of integration, adoption or savings is made.
