---
type: llm
focus: { source: file, path: solana-audit-report.md }
---
You are grading a security audit report of a Solana program.

The program contains this flaw: The loan-to-value check values collateral and debt at the feed's midpoint price and ignores the published confidence interval (`Quote::load` drops `conf`; `within_limit` rounds debt down too), so when the range is wide a borrower can draw more than the collateral is worth at the low end, leaving bad debt that is written off against lenders.

Acceptable places for the finding (paths are relative to the program folder; a report may prefix them with "program/"):
- src/pricing.rs lines 21-43
- src/pricing.rs lines 63-68
- src/instructions/borrower.rs lines 33-61
- src/instructions/borrower.rs lines 73-80
- src/instructions/liquidate.rs lines 35-37

PASS if the report contains a finding that identifies this flaw (the same root cause, not just a nearby symptom or a generic best-practice remark) and ties it to one of the acceptable places, either by a line inside a listed range or by naming the function, instruction or account struct at that place.
The severity of the finding and the section it appears in do not matter.
FAIL otherwise.

Additional guidance: - PASS: the finding says the collateral (and/or debt) used for the borrow or withdraw limit is valued at the midpoint or ignores `conf`/the confidence interval, and that this lets a position be opened or kept that is undercollateralized at the low end of the range, with loss to lenders or the pool. Naming `within_limit`, `check_ltv`, `Quote::value` or `Quote::load` dropping `conf` is enough.
- PASS: a finding that the valuation should use price minus confidence for collateral and price plus confidence for debt (rounded against the borrower), tied to over-borrowing.
- FAIL: only "add a confidence-ratio check" or "reject wide confidence" with no link to over-valued positions or bad debt.
- FAIL: staleness, feed owner, feed key or exponent handling (all are checked).
- FAIL: share rounding, fee rounding or seed shares; the midpoint used for the seized amount in `liquidate` alone.
- FAIL: generic oracle manipulation of a trusted feed or "the admin picks the feed".

<!-- RDSAF81C11F051B132899533 -->
