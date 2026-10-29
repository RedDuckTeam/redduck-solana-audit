---
id: c05
crate: gift_card
pattern: checklist-duplicate-accounts
class: exploit
group: g3
vulnerable: b
role: heldout
author: case-author
canary: RDSAF81C11F051B132899533
---
# Case c05

## Attack scenario
`gift_card` holds prepaid balances. Each card's spendable credit is an internal number
(`Card.balance`); the matching tokens sit in one shared hub vault. `transfer(amount,
recipient)` moves credit from the caller's card (`from`, seeded by the signer) to the
`recipient`'s card (`to`, seeded by the `recipient` argument): it debits `from` and
credits `to`. Both accounts are fully seeded, so the struct looks correct in isolation;
the two PDAs collide only when `recipient` equals the caller.

1. The attacker opens a card and `load`s 100 tokens, so `balance = 100`.
2. The attacker calls `transfer(100, recipient = <their own pubkey>)`. `from` is seeded
   from the signer and `to` from `recipient`; since they are equal, both resolve to the
   **same** card PDA. Every constraint (seeds, `has_one`) passes.
3. Anchor deserializes `from` and `to` into two independent in-memory copies of the
   same account, each starting at `balance = 100`. The handler sets the `from` copy to
   `0` and the `to` copy to `200`. On exit Anchor serializes both back to the one
   account; `to` is written last, so the account persists at `balance = 200`.
4. Each call doubles the card's credit. After a few calls the attacker calls `redeem`
   and withdraws real tokens from the shared vault, draining other users' deposits.

The root cause is that `transfer` lets `from` and `to` resolve to the same account (when
`recipient` is the caller) with no constraint that they are distinct, so a debit plus
credit becomes a net credit. Reading the accounts struct alone does not reveal it,
because both sides are legitimately seeded; the collision only exists for the specific
`recipient == owner` argument.

## Why fixed closes it
In `src/instructions/transfer.rs` the `to` account gains
`constraint = to.key() != from.key() @ CardError::SameCard`. Passing `recipient` equal
to the caller now fails before the handler runs, so the debit and credit always land on
different accounts and balance is conserved. A genuine transfer to another card is
unaffected.

## Decoys
- `merge_cards` also takes two `Card` accounts of the same type (`source`, `dest`) and
  moves a balance between them (consolidating the caller's card into another card and
  closing the caller's). It is reachable — `dest` is any other card in the hub — and it
  already carries `constraint = source.key() != dest.key()`, so the duplicate-account
  case is correctly blocked there. An auditor sees the project knows the idiom, which
  makes the omission in `transfer` easy to overlook.
- `load` and `redeem` take a vault token account constrained by `address = hub.vault`,
  and the owner's token account is pinned by `token::authority`; these address checks
  look like a place a wrong account could slip in but are correctly bound.

## Trusted roles
- **Hub authority** (`Hub.authority`): the account that called `initialize`. It sets up
  the hub and its escrow vault. It holds no runtime power over user cards or balances.
