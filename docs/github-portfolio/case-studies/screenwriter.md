# Case study: Screenwriter

**Label: live product in private beta, built and operated by me.** Tracker: [#26](https://github.com/elliyeen/elliyeen/issues/26). Live: [write.elliyeen.com](https://write.elliyeen.com). Public source: [screenwriter](https://github.com/elliyeen/screenwriter), snapshot v0.6.3. The active repository is private and ahead of the snapshot.

## Customer and market problem

Screenwriters pay for specialist software to get exact page formats and clean export, and they complain about cost, lock-in, lost work and tools that only run on one computer. I wanted one private place, in any browser, that gets the format right and does not lose work.

## Research evidence

I collected writer complaints about Final Draft and other tools and turned them into ranked user stories. That backlog lives in private planning documents, so a reader cannot inspect it here. **Level: private.**

## My role

Sole builder and operator: product decisions, front end, Cloudflare back end, tests, deployment.

## Constraints

No AI model (writers asked for predictable formatting). No build step or framework, so it stays easy to audit. Work must survive offline and across devices. Studio page format must be exact.

## Architecture

```
Browser (plain JavaScript, installable web app, works offline)
  -> Cloudflare Worker (sign-in by emailed 6-digit code, API)
       -> D1 (scripts, versions)   -> Durable Object (live co-writing merge)
       -> R2 (nightly backup to separate storage)
```

Details: [ARCHITECTURE.md](https://github.com/elliyeen/screenwriter/blob/main/docs/ARCHITECTURE.md), [DATA-AND-BACKUPS.md](https://github.com/elliyeen/screenwriter/blob/main/docs/DATA-AND-BACKUPS.md).

## Implementation

Rule-based studio layout (Courier 12, US Letter, 54 lines per page), import from PDF, Word, text, Final Draft and Fountain, export to PDF, Fountain and zip, version history, soft delete, sharing by email with live merge.

## Security

Published [threat model and known gaps](https://github.com/elliyeen/screenwriter/blob/main/docs/SECURITY.md). Codes work once, lock after 5 tries, rate limits, secure cookies, cross-site requests refused (documented as browser-test checks).

## Deployment status

**Live, private beta.** The deployed build was checked only for the public landing page (HTTP 200, title "Screenwriter: Your story starts here"). Signed-in behaviour was not tested for this page. Guest writing work in the private repository is not claimed as live.

## Reproducible validation

```bash
git clone https://github.com/elliyeen/screenwriter && cd screenwriter
npm ci && npm test
```

**Reproduced 2026-11-22** on the public snapshot at `2c4434e`: four unit suites passed (11, 1, 7 and 8 checks, 27 in total). The browser suite (`npm run test:e2e`, about 95 checks including accessibility and visual comparison) is documented but was **not** run for this page.

## Measurable results

None published. A target of first accepted word in under 20 seconds (p95) exists, but it is a target, not a measured result. No user, revenue or retention figures are claimed.

## Limitations

No third-party security audit. Beta. The public snapshot is behind the private repository. Real user feedback is not documented here.

## Next step

Publish the first-word benchmark with its method, then add documented user feedback.
