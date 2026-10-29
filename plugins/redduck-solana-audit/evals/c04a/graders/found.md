---
type: llm
focus: { source: file, path: solana-audit-report.md }
---
You are grading a security audit report of a Solana program.

The program contains this flaw: The per-wallet purchase cap is enforced in `buy` (checks `buyer.purchased + amount <=
per_wallet_cap`) but not in the second purchase path `buy_bundle`, which only checks a
per-transaction limit and the hard cap. A buyer uses `buy_bundle` repeatedly to acquire
far more than `per_wallet_cap`. The invariant must be guarded on every path.

Acceptable places for the finding (paths are relative to the program folder; a report may prefix them with "program/"):
- src/instructions/buy_bundle.rs lines 36-80
- src/instructions/buy_bundle.rs lines 41-41

PASS if the report contains a finding that identifies this flaw (the same root cause, not just a nearby symptom or a generic best-practice remark) and ties it to one of the acceptable places, either by a line inside a listed range or by naming the function, instruction or account struct at that place.
The severity of the finding and the section it appears in do not matter.
FAIL otherwise.

Additional guidance: - A hit must state that `buy_bundle` does not enforce the cumulative per-wallet cap that
  `buy` enforces, so a buyer exceeds `per_wallet_cap` by using `buy_bundle` (repeatedly).
  Identifying the missing `purchased <= per_wallet_cap` check on the bundle path is
  required.
- A finding that the `per_tx_limit` check in `buy_bundle` is sufficient, or that only
  flags the bonus arithmetic, is a FAIL.
- A finding that only says "add validation to buy_bundle" without naming the per-wallet
  cap / the fact that the limit is bypassable across transactions is a FAIL.
- Flagging the hard-cap handling is a FAIL; it is enforced on both paths.

<!-- RDSAF81C11F051B132899533 -->
