---
type: llm
focus: { source: file, path: solana-audit-report.md }
---
You are grading a security audit report of a Solana program.

The program contains this flaw: `withdraw` computes the notes to burn for a requested asset amount with floor division
(`assets * total_notes / total_assets`). Once assets-per-note exceeds 1 (after
`accrue_yield`), a small withdrawal truncates the burn to fewer notes than it is worth
— and to zero when `assets * total_notes < total_assets` — so a user withdraws assets
without paying the corresponding notes. The charge must round up (ceiling).

Acceptable places for the finding (paths are relative to the program folder; a report may prefix them with "program/"):
- src/instructions/withdraw.rs lines 40-44
- src/instructions/withdraw.rs lines 34-74

PASS if the report contains a finding that identifies this flaw (the same root cause, not just a nearby symptom or a generic best-practice remark) and ties it to one of the acceptable places, either by a line inside a listed range or by naming the function, instruction or account struct at that place.
The severity of the finding and the section it appears in do not matter.
FAIL otherwise.

Additional guidance: - A hit must identify that the notes-to-burn computation in `withdraw` rounds in the
  user's favour (floor instead of ceiling), letting a withdrawal cost too few notes (or
  zero). Naming the exact `as u128 ... / total_assets` site, or the rounding direction,
  is required.
- A finding that only flags `deposit` or `redeem` rounding is a FAIL: those round down
  correctly (against the user).
- A finding that says "use checked math" or "possible overflow" without identifying the
  rounding direction in `withdraw` is a FAIL; the math already uses checked `u128`.
- Pointing at `accrue_yield` as the flaw is a FAIL; it is normal operation and a trusted
  role.

<!-- RDSAF81C11F051B132899533 -->
