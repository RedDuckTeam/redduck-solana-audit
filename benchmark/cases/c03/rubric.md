---
id: c03
canary: RDSAF81C11F051B132899533
---
## Mechanism
`withdraw` computes the notes to burn for a requested asset amount with floor division
(`assets * total_notes / total_assets`). Once assets-per-note exceeds 1 (after
`accrue_yield`), a small withdrawal truncates the burn to fewer notes than it is worth
— and to zero when `assets * total_notes < total_assets` — so a user withdraws assets
without paying the corresponding notes. The charge must round up (ceiling).

## Locations
- src/instructions/withdraw.rs:40-44
- src/instructions/withdraw.rs:34-74

## Judge criteria
- A hit must identify that the notes-to-burn computation in `withdraw` rounds in the
  user's favour (floor instead of ceiling), letting a withdrawal cost too few notes (or
  zero). Naming the exact `as u128 ... / total_assets` site, or the rounding direction,
  is required.
- A finding that only flags `deposit` or `redeem` rounding is a FAIL: those round down
  correctly (against the user).
- A finding that says "use checked math" or "possible overflow" without identifying the
  rounding direction in `withdraw` is a FAIL; the math already uses checked `u128`.
- Pointing at `accrue_yield` as the flaw is a FAIL; it is normal operation and a trusted
  role.
