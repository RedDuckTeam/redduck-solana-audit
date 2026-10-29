---
type: llm
focus: { source: file, path: solana-audit-report.md }
---
You are grading a security audit report of a Solana program.

The program contains this flaw: The only way to get a card balance out of the vault is `complete_withdrawal`, which requires a co-signature from `PROCESSOR`, a key hardcoded in an immutable program; there is no way to replace it and no exit without it, so if the external processor's signing service stops, all user balances are locked permanently.

Acceptable places for the finding (paths are relative to the program folder; a report may prefix them with "program/"):
- src/state.rs lines 7-8
- src/instructions/card.rs lines 109-154
- src/instructions/processor.rs lines 13-15
- src/instructions/admin.rs lines 49-98

PASS if the report contains a finding that identifies this flaw (the same root cause, not just a nearby symptom or a generic best-practice remark) and ties it to one of the acceptable places, either by a line inside a listed range or by naming the function, instruction or account struct at that place.
The severity of the finding and the section it appears in do not matter.
FAIL otherwise.

Additional guidance: - PASS: the finding says withdrawals depend on the hardcoded processor key (or its co-signature) with no rotation path and no direct or emergency exit, so users' funds are locked if the processor key is lost or its service stops (or the processor can block withdrawals indefinitely). It must name `complete_withdrawal`, `CompleteWithdrawal` or the `PROCESSOR` constant.
- PASS: "the processor is a single point of failure for withdrawals; add a timelocked way to change it and a delayed self-service exit" when it is tied to user funds being stuck.
- FAIL: only "the processor is trusted and can drain cards through `settle_spend`" (a listed trusted role) without the locked-funds failure.
- FAIL: only "the admin cannot be rotated" or "the settlement account can be changed" (it is timelocked).
- FAIL: generic centralization remarks that do not identify the withdrawal path.

<!-- RDSAF81C11F051B132899533 -->
