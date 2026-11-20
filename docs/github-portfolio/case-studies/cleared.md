# Case study: CLEARED verified benchmark

**Label: prototype / benchmark, synthetic data.** Not a product, not a live service, and not evidence of customers or revenue. Tracker: [#11](https://github.com/elliyeen/elliyeen/issues/11). Repository: [cleared-public](https://github.com/elliyeen/cleared-public).

## Problem

An invoice can be real while the payment destination is wrong. The checker returns `approve`, `hold` or `escalate` with rule IDs and reasons for duplicate invoices, changed remit-to details, unknown vendors, and missing dual approval over a threshold. It never moves money. A *critical violation* is defined as approving something that should have been held or escalated.

## Method

1. Freeze a baseline (`checker_v01`) and check that repeated runs give identical output.
2. Keep labels in the evaluator, not in the checker.
3. Make one change at a time (`v0.2`, then `v0.2.1`).
4. Test on a dev set, a holdout, and a second holdout written before the candidate was tried.
5. Reject a candidate automatically if it adds any unauthorized approval against baseline, or lacks evidence for a non-approve.

## Results (synthetic; critical violations in brackets, out of 13 cases per set)

| Set | v0.1 baseline | v0.2 | v0.2.1 |
| :--- | :--- | :--- | :--- |
| dev | 9 passed [4] | 12 [1] | 12 [1] |
| holdout | 8 [5] | 11 [2] | 12 [1] |
| holdout2 (clean gate for v0.2.1) | 8 [5] | 10 [3] | 11 [1] |

## Failure cases, reported and not hidden

- **A bug the holdout caught.** v0.2 stripped separators before leading zeros and merged digit runs. Fixed in v0.2.1.
- **Contamination.** Because the first holdout had been seen, it is flagged contaminated for v0.2.1, and the fresh `holdout2` is the real gate.
- **A regression.** v0.2.1 wrongly holds one genuinely different invoice (`G07`) whose number collides after normalization. The comparison output lists it.
- **Still open.** The missing-purchase-order case is approved by every version.

## Reproduce

```bash
pip install pytest
python eval/compare.py        # regenerates evidence/comparison.json
python -m pytest -q tests/
```

**Verified 2026-11-20** on a clean clone at commit `ecdd8fd`: `pytest` reported 5 passed; `compare.py` regenerated the evidence and the critical-violation counts above match it. Only latency timings and the commit field differed from the committed file. A CI workflow (`benchmark`) succeeded on 2026-11-02.

## Limits

Same author wrote the checker and the fixtures. 13 hand-built cases per set, so the rates are illustrative and not statistical. The checker is deterministic code with no LLM, so this shows determinism and evaluation method, not model behavior. Nothing here shows performance on real invoices.

## Not published

A larger set of adversarial fraud scenarios and a risk register exist in private planning documents. They are not part of this public evidence and are not cited as results.
