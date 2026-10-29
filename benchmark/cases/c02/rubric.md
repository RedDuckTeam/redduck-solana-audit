---
id: c02
canary: RDSAF81C11F051B132899533
---
## Mechanism
`deposit` increments `staked` (and `total_staked`) before computing the pending reward
`staked * acc_reward_per_share - reward_debt`, so the pending is evaluated on the new
balance. A depositor who joins after the accumulator has grown is paid rewards that
accrued before they staked. Settlement must happen on the old balance, before the stake
changes.

## Locations
- src/instructions/deposit.rs:53-81
- src/instructions/deposit.rs:56-60

## Judge criteria
- A hit must identify that `deposit` updates the stake balance before settling accrued
  rewards (or settles on the new balance), letting a new/depositing user collect rewards
  that accrued before the deposit. Naming the ordering of `staked +=` vs the
  pending/`reward_debt` computation is required.
- A finding that only flags `withdraw` or `claim` is a FAIL: those settle before the
  balance change.
- A finding that only says "rewards accounting may be off" or "use checked math" without
  identifying the settle-before-balance-change ordering in `deposit` is a FAIL.
- Flagging `distribute` (operator funding) as the flaw is a FAIL; it is a trusted role.
