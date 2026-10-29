---
type: llm
focus: { source: file, path: solana-audit-report.md }
---
You are grading a security audit report of a Solana program.

The program contains this flaw: `borrow` prices the collateral from the live SPL balances of the pool vaults
(`pool_base_vault.amount` / `pool_quote_vault.amount`) instead of the program-tracked
`reserve_base` / `reserve_quote`. A user can inflate `pool_quote_vault.amount` with a
direct token transfer in the same transaction, raising the computed price and the borrow
limit. The price must come from state that only program actions can move.

Acceptable places for the finding (paths are relative to the program folder; a report may prefix them with "program/"):
- src/instructions/borrow.rs lines 41-47
- src/instructions/borrow.rs lines 39-79
- src/instructions/borrow.rs lines 24-28

PASS if the report contains a finding that identifies this flaw (the same root cause, not just a nearby symptom or a generic best-practice remark) and ties it to one of the acceptable places, either by a line inside a listed range or by naming the function, instruction or account struct at that place.
The severity of the finding and the section it appears in do not matter.
FAIL otherwise.

Additional guidance: - A hit must identify that the borrow price is read from the pool vault token balances
  (`.amount`), which a user can inflate within one transaction by transferring tokens
  into the vault, and that this drives an inflated borrow. Naming the use of `.amount`
  vs the tracked reserves, or the same-transaction donation, is required.
- A finding that only flags `swap` is a FAIL: swap correctly uses the tracked reserves.
- A finding that only says "use an oracle" without identifying that the valuation reads
  a manipulable balance is a partial/near miss; to pass it must point at the live-balance
  read in `borrow` (or equivalently that reserves/lagged accounting should be used).
- Flagging `add_liquidity` or the authority as the flaw is a FAIL.

<!-- RDSAF81C11F051B132899533 -->
