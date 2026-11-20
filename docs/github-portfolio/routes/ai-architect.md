# Route B: AI Architecture, Agentic Systems and ML Engineering

For hiring managers looking at how I structure AI-enabled systems: where a model belongs, where it does not, and how decisions are checked. Tracker: [#10](https://github.com/elliyeen/elliyeen/issues/10). Evidence levels are defined in the [evidence matrix](../evidence-matrix.md).

## The design position

Put deterministic checks before and around any model, keep a human approval gate on consequential actions, and keep an audit trail. Several of my systems use no model at all, on purpose, because rules were enough.

## Example 1: CLEARED, evaluation discipline

- **Problem:** Decide whether an invoice payment should be approved, held or escalated, and prove a change made things better.
- **Architecture:** A deterministic checker, a separate evaluator that owns the labels, a comparison script with automatic safety gates, and fresh holdout sets written before each candidate is tested.
- **Evidence:** [cleared-public](https://github.com/elliyeen/cleared-public). `pytest` passed 5 tests and `python eval/compare.py` regenerated the evidence on 2026-11-20. See the [case study](../case-studies/cleared.md).
- **Limits:** Deterministic code, so this shows evaluation method and not LLM behavior. 13 hand-built cases per set.

## Example 2: Engineering factory, governing AI coding agents

- **Problem:** Let agents build and fix code without letting them approve, merge or weaken their own checks.
- **Architecture:** A supervisor with checkpointed task state, merge gates tied to CI, a block on stacked PRs, a hash lock on protected files, and a ledger of lessons. See the [case study](../case-studies/engineering-factory.md).
- **Limits:** Synthetic sandbox. **Not certified**: two of fourteen tests are blocked (independent approval and measured improvement).

## Example 3: Ahead, agentic matching (private source)

- **Problem:** Match companies and talent on evidence of what needs to get done, with candidate consent before any introduction.
- **Architecture:** Deterministic filtering before any AI step, human approval gates and an audit trail.
- **Evidence:** The product is live at [aheadjobs.com](https://aheadjobs.com). Source and design documents are private, so this example cannot be inspected here.

## What this route does not show

No public ML training, model evaluation of learned models, or ML infrastructure. No AWS Bedrock. No public production multi-agent system. See the [gap assessment](../evidence-matrix.md#gap-assessment). If a role needs those, that is a gap and not something to infer from this page.
