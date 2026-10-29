---
id: c05
canary: RDSAF81C11F051B132899533
---
## Mechanism
`transfer(amount, recipient)` seeds `from` from the signer and `to` from the `recipient`
argument, with no constraint that the two resolve to different accounts. When `recipient`
equals the caller, `from` and `to` are the same card; Anchor loads two copies, the debit
and credit hit separate copies, and the last write-back (`to`) wins, so the card is
net-credited out of nothing. Fix is a key-inequality constraint (`to.key() != from.key()`).

## Locations
- src/instructions/transfer.rs:7-31
- src/instructions/transfer.rs:33-50
- src/instructions/transfer.rs:24-30

## Judge criteria
- A hit must state that `from` and `to` can resolve to the same account (when
  `recipient` is the caller, with no distinctness constraint) and that this
  double-credits / skips the debit. Naming the missing `from != to` / key-inequality
  constraint on the transfer accounts is required.
- A finding that flags `merge_cards` as the hole is a FAIL: it already constrains
  `source.key() != dest.key()`.
- A finding that only says "validate the accounts" or "add has_one" without identifying
  the same-account aliasing in `transfer` is a FAIL.
- Pointing at the vault `address` checks or `token::authority` bindings as the flaw is a
  FAIL.
