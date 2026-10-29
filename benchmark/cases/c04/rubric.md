---
id: c04
canary: RDSAF81C11F051B132899533
---
## Mechanism
The per-wallet purchase cap is enforced in `buy` (checks `buyer.purchased + amount <=
per_wallet_cap`) but not in the second purchase path `buy_bundle`, which only checks a
per-transaction limit and the hard cap. A buyer uses `buy_bundle` repeatedly to acquire
far more than `per_wallet_cap`. The invariant must be guarded on every path.

## Locations
- src/instructions/buy_bundle.rs:36-80
- src/instructions/buy_bundle.rs:41

## Judge criteria
- A hit must state that `buy_bundle` does not enforce the cumulative per-wallet cap that
  `buy` enforces, so a buyer exceeds `per_wallet_cap` by using `buy_bundle` (repeatedly).
  Identifying the missing `purchased <= per_wallet_cap` check on the bundle path is
  required.
- A finding that the `per_tx_limit` check in `buy_bundle` is sufficient, or that only
  flags the bonus arithmetic, is a FAIL.
- A finding that only says "add validation to buy_bundle" without naming the per-wallet
  cap / the fact that the limit is bypassable across transactions is a FAIL.
- Flagging the hard-cap handling is a FAIL; it is enforced on both paths.
