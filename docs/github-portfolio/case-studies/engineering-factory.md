# Case study: Engineering factory (governing AI coding agents)

**Label: tested in a synthetic sandbox. NOT CERTIFIED.** Tracker: [#11](https://github.com/elliyeen/elliyeen/issues/11). Sandbox: [factory-sandbox](https://github.com/elliyeen/factory-sandbox) (public, no real code, data or secrets).

## Problem

An AI coding agent that can approve or merge its own work can also merge its own mistakes. The goal was to let agents build, test and fix, while keeping merge, release and policy decisions with independent checks and a human.

## Design

- **Queue and recovery:** durable task state with checkpoints, a supervisor, and restart from the checkpoint without repeating finished work.
- **Gates tied to CI evidence, not self-report:** a failing test blocks merge; a security finding blocks merge; a pull request whose base is not the default branch is denied.
- **Authority:** production is denied without owner approval and on self-approval. Changes to protected files need independent approval, and a weakened gate is detected by a hash lock.
- **Learning:** lessons are written to a hash-chained ledger; an improvement is adopted only with a measured gain and reverted if it regresses.

## Results from the certification run (2026-11-18)

Of 14 tests, 12 passed and 2 were blocked. The overall verdict is **NOT CERTIFIED**.

| Result | Tests |
| :--- | :--- |
| PASS (12) | Task execution, quality gate, security gate, stacked-PR protection, recovery after a killed worker (restarts = 1, six tasks each executed once), authorization (mock only, no real deploy), lessons persist, failure-to-issue, tested adoption, rollback, self-modification guard, weekly report |
| **BLOCKED (2)** | **T4 merge verification:** auto-merge was requested but no independent approver exists, because GitHub does not let the author approve their own PR. **T13 measured improvement:** needs real before/after history across tasks, which synthetic data cannot prove. |

Public evidence from the sandbox includes issue 1 and pull requests 2, 6, 7 and 8, and CI runs where a deliberately failing test and a synthetic security finding blocked merge before the fix passed. The full certification report is a local file and is not published here.

## Limits

- A synthetic sandbox. This is not production traffic, and it is not a production deployment.
- Two required tests are blocked, so no certification is claimed.
- The auto-merge path under independent approval (T4) was never completed, so it is not demonstrated.
- No productivity multiple, cost saving or defect-rate improvement is claimed. Those metrics have no baseline yet.
