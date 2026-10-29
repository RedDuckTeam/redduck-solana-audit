---
type: llm
focus: { source: file, path: solana-audit-report.md }
---
You are grading a security audit report of a Solana program.

The program contains this flaw: `add_rewards` credits `pending` by iterating a caller-supplied list of staker accounts with no check that the list is complete, duplicate-free, or matches the pool's membership. Passing the same staker account multiple times credits its share once per occurrence, so an attacker can inflate their own `pending` beyond the tokens actually funded and drain the vault on `claim`. The same eager per-account design also makes a correct distribution impossible once stakers exceed one transaction.

Acceptable places for the finding (paths are relative to the program folder; a report may prefix them with "program/"):
- src/instructions/add_rewards.rs lines 21-57
- src/instructions/add_rewards.rs lines 41-55
- src/instructions/add_rewards.rs lines 7-19
- src/instructions/claim.rs lines 22-42
- src/lib.rs lines 29-34

PASS if the report contains a finding that identifies this flaw (the same root cause, not just a nearby symptom or a generic best-practice remark) and ties it to one of the acceptable places, either by a line inside a listed range or by naming the function, instruction or account struct at that place.
The severity of the finding and the section it appears in do not matter.
FAIL otherwise.

Additional guidance: HIT if the finding identifies that `add_rewards` accounting is driven by the caller's remaining-accounts list without validating it, and names a real consequence:
- a duplicated staker account is credited more than once, letting a caller over-credit `pending` and withdraw more than was funded (loss of funds); or
- the distribution does not scale / cannot credit stakers the caller omits, so rewards are incomplete or the instruction becomes unusable with many stakers (liveness).

Either consequence is a HIT; the duplicate-credit drain is the strongest.

FAIL:
- A bare "reward rounding loses dust" note with no over-credit or scaling claim.
- Findings about `stake`/`unstake`/`claim` arithmetic in isolation.
- Generic "loops are expensive" with no mention of caller-controlled accounts or a concrete consequence.
- Only that a staker can stake just before `add_rewards` and unstake right after to capture a share (just-in-time staking), with no mention of caller-chosen or duplicated remaining accounts.
- Other issues in this program do not count as a HIT: just-in-time or flash staking around a large reward (no lockup), and rounding dust left in the vault.

<!-- RDSAF81C11F051B132899533 -->
