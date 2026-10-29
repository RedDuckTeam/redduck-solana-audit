---
id: c01
canary: RDSAF81C11F051B132899533
---
## Mechanism
`borrow` prices the collateral from the live SPL balances of the pool vaults
(`pool_base_vault.amount` / `pool_quote_vault.amount`) instead of the program-tracked
`reserve_base` / `reserve_quote`. A user can inflate `pool_quote_vault.amount` with a
direct token transfer in the same transaction, raising the computed price and the borrow
limit. The price must come from state that only program actions can move.

## Locations
- src/instructions/borrow.rs:41-47
- src/instructions/borrow.rs:39-79
- src/instructions/borrow.rs:24-28

## Judge criteria
- A hit must identify that the borrow price is read from the pool vault token balances
  (`.amount`), which a user can inflate within one transaction by transferring tokens
  into the vault, and that this drives an inflated borrow. Naming the use of `.amount`
  vs the tracked reserves, or the same-transaction donation, is required.
- A finding that only flags `swap` is a FAIL: swap correctly uses the tracked reserves.
- A finding that only says "use an oracle" without identifying that the valuation reads
  a manipulable balance is a partial/near miss; to pass it must point at the live-balance
  read in `borrow` (or equivalently that reserves/lagged accounting should be used).
- Flagging `add_liquidity` or the authority as the flaw is a FAIL.
