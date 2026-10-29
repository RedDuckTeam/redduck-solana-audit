---
id: c04
crate: launchpad
pattern: identity-rejection
class: exploit
group: g1
vulnerable: a
role: heldout
author: case-author
canary: RDSAF81C11F051B132899533
---
# Case c04

## Attack scenario
`launchpad` runs a fixed-price token sale with a per-wallet cap meant to stop any one
address from taking an outsized share of the allocation. There are two purchase paths:
`buy` (plain) and `buy_bundle` (pay for `amount`, receive a small bonus on top).

1. The sale sets `per_wallet_cap = 1_000`, `per_tx_limit = 500`, `hard_cap = 100_000`,
   `bonus_bps = 0` for simplicity, and the authority has placed 100_000 sale tokens in
   `sale_vault`. The issuer intends at least 100 distinct buyers to share the allocation.
2. As soon as the window opens, the attacker opens one buyer account and calls
   `buy_bundle(500)` repeatedly (several per transaction). Each call checks only the
   per-transaction limit (`amount <= per_tx_limit`), the hard cap and that the vault
   backs the sale; it never checks the buyer's cumulative `purchased` against
   `per_wallet_cap`.
3. After 200 calls the single wallet has `purchased = 100_000`, the entire hard cap.
   Every later `buy` or `buy_bundle` by an honest buyer fails with `HardCapExceeded`.
4. After the sale ends the attacker `claim`s the whole allocation.

Affected parties: the honest buyers, who are shut out of a sale they were promised a
share of, and the issuer, whose distribution policy (at most 1_000 tokens per holder)
is broken, concentrating the supply in one holder. A per-wallet cap can in principle be
split across many wallets, but that costs a Sybil setup per extra 1_000 tokens (a buyer
account each, and wallets the issuer may screen); here one ordinary wallet takes the
whole cap with no such cost, so the cap protects nothing.

The root cause is that the per-wallet invariant is enforced on `buy` but not on the
equivalent `buy_bundle` path, so the limit is tied to one instruction rather than to
the invariant it is supposed to protect.

## Why fixed closes it
`src/instructions/buy_bundle.rs` gains the same cumulative check `buy` already has:
`new_purchased = buyer.purchased + credited` must be `<= sale.per_wallet_cap`
(`WalletCapExceeded`). With the invariant enforced on every purchase path, splitting a
large position across many `buy_bundle` calls stops at the cap. Normal usage (buying up
to the cap through either path, then claiming) is unchanged.

## Decoys
- `buy_bundle` already has a `per_tx_limit` check (`amount <= per_tx_limit`). It looks
  like the purchase-limiting guard, but it only bounds a single transaction, not the
  wallet's running total, so repeated calls walk around it. This resembles the missing
  protection but is the wrong invariant.
- The `hard_cap` and vault-backing checks are correctly applied on **both** `buy` and
  `buy_bundle`, so those invariants are guarded on every path while the per-wallet one is
  not.
- `payment_vault` / `sale_vault` are pinned by `address = sale.*` and the buyer's token
  accounts by `token::authority`, so the account wiring is correctly constrained.

## Trusted roles
- **Sale authority** (`Sale.authority`): creates the sale and transfers the sale
  tokens into `sale_vault` before or during the window. After `end_ts` it may take the
  payment token with `withdraw_proceeds` and the unsold sale tokens with
  `withdraw_unsold`. It cannot change a buyer's recorded allocation and cannot take tokens
  that buyers are owed: every purchase requires `sale_vault` to back the new `sold`
  total, and `withdraw_unsold` is limited to `sale_vault.amount - (sold - claimed_total)`.
