---
type: llm
focus: { source: file, path: solana-audit-report.md }
---
You are grading a security audit report of a Solana program.

The program contains this flaw: `merge_positions` gives the target a recomputed weight. The merged amount at the later lock end can exceed the two old weights combined. It never applies that change to `Season.total_weight`, which the other weight-changing paths keep in sync. `finalize_season` snapshots the stale, too-low total. `claim_reward` then pays `reward_amount * weight / final_total_weight` against it, so the claims add up to more than the reward vault and the merger takes other holders' rewards.

Acceptable places for the finding (paths are relative to the program folder; a report may prefix them with "program/"):
- src/instructions/position.rs lines 121-125
- src/instructions/position.rs lines 133-152
- src/instructions/season.rs lines 65-77
- src/instructions/exit.rs lines 58-76
- src/state.rs lines 22-25

PASS if the report contains a finding that identifies this flaw (the same root cause, not just a nearby symptom or a generic best-practice remark) and ties it to one of the acceptable places, either by a line inside a listed range or by naming the function, instruction or account struct at that place.
The severity of the finding and the section it appears in do not matter.
FAIL otherwise.

Additional guidance: - PASS: the finding says that merging positions changes position weight without updating the season's total weight, or that the total can fall below the sum of position weights through merge. It must also say that rewards are then over-allocated or mis-distributed at claim or finalization.
- PASS also if the finding is phrased as "merge recomputes weight with the longer lock but `total_weight` is unchanged, so reward shares sum to more than 100%".
- FAIL: a finding that only says merge lets someone boost their weight or "gain extra rewards", without pointing at the missing `total_weight` update. Boosting weight with a longer lock is intended.
- FAIL: findings about `withdraw` after finalization not reducing the total, about `extend_lock` or `add_stake` accounting, or about rounding in `pro_rata`. These paths are correct.
- FAIL: generic "add more validation to merge" without naming the season total weight.

<!-- RDSAF81C11F051B132899533 -->
