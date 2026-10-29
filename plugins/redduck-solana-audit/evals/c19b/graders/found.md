---
type: llm
focus: { source: file, path: solana-audit-report.md }
---
You are grading a security audit report of a Solana program.

The program contains this flaw: `claim` overwrites `affiliate.claimed` with the leaf's `earned_total` even when that total is lower than the current value, which `saturating_sub` hides. Proofs against older epoch roots stay valid. So an affiliate can roll the counter back with an older leaf, then claim the newer leaf again, and repeat to drain the vault. The counter has to move forward only: reject totals that are not above `claimed`.

Acceptable places for the finding (paths are relative to the program folder; a report may prefix them with "program/"):
- src/lib.rs lines 86-89
- src/lib.rs lines 72-128
- src/lib.rs lines 214-234
- src/lib.rs lines 257-264

PASS if the report contains a finding that identifies this flaw (the same root cause, not just a nearby symptom or a generic best-practice remark) and ties it to one of the acceptable places, either by a line inside a listed range or by naming the function, instruction or account struct at that place.
The severity of the finding and the section it appears in do not matter.
FAIL otherwise.

Additional guidance: - PASS: the finding says that `claimed` can decrease, for example because it is set to a lower `earned_total` from an older epoch or because `saturating_sub` plus the assignment lets the paid counter go back. It also explains that this allows paying the same earnings again (alternating old and new epoch proofs, double or repeated claims, vault drain).
- PASS: the finding is framed as "old epoch proofs can be replayed to reset `claimed`, then the newest proof pays again". Recommending to accept only the latest epoch is fine if the reset or double-pay mechanism is stated.
- FAIL: the finding only says "old Merkle roots are never invalidated" or "add a nonce or claimed bitmap" and does not explain how the paid counter goes back or how funds are paid twice.
- FAIL: the finding only remarks that `saturating_sub` hides an underflow or is a style issue, without the rollback and re-payment consequence.
- FAIL: findings about the authority publishing a malicious root (a trust assumption) are not this flaw.

<!-- RDSAF81C11F051B132899533 -->
