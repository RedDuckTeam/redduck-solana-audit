---
type: llm
focus: { source: file, path: solana-audit-report.md }
---
You are grading a security audit report of a Solana program.

The program contains this flaw: `deposit` increments `staked` (and `total_staked`) before computing the pending reward
`staked * acc_reward_per_share - reward_debt`, so the pending is evaluated on the new
balance. A depositor who joins after the accumulator has grown is paid rewards that
accrued before they staked. Settlement must happen on the old balance, before the stake
changes.

Acceptable places for the finding (paths are relative to the program folder; a report may prefix them with "program/"):
- src/instructions/deposit.rs lines 53-81
- src/instructions/deposit.rs lines 56-60

PASS if the report contains a finding that identifies this flaw (the same root cause, not just a nearby symptom or a generic best-practice remark) and ties it to one of the acceptable places, either by a line inside a listed range or by naming the function, instruction or account struct at that place.
The severity of the finding and the section it appears in do not matter.
FAIL otherwise.

Additional guidance: - A hit must identify that `deposit` updates the stake balance before settling accrued
  rewards (or settles on the new balance), letting a new/depositing user collect rewards
  that accrued before the deposit. Naming the ordering of `staked +=` vs the
  pending/`reward_debt` computation is required.
- A finding that only flags `withdraw` or `claim` is a FAIL: those settle before the
  balance change.
- A finding that only says "rewards accounting may be off" or "use checked math" without
  identifying the settle-before-balance-change ordering in `deposit` is a FAIL.
- Flagging `distribute` (operator funding) as the flaw is a FAIL; it is a trusted role.

<!-- RDSAF81C11F051B132899533 -->
