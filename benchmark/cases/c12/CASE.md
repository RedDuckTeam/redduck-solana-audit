---
id: c12
crate: group_buy
pattern: divide-and-conquer
class: design
group: g2
vulnerable: a
role: dev
author: case-author
canary: RDSAF81C11F051B132899533
---
# Case c12

## Attack scenario
Design failure (capacity and griefing): every buyer of a deal is stored in one `Vec<Commitment>` inside the shared `Deal` account, capped at `MAX_BUYERS = 100` because the account is allocated once at `create_deal`.

1. A merchant creates a deal: `min_units = 600`, `max_units = 1_200`, `max_units_per_buyer = 4`, unit price 20 USDC, deadline in 7 days.
2. A competitor calls `join_deal` from 100 fresh wallets. `join_deal` needs no payment and no rent (the Vec space was paid by the merchant), so the cost is 100 transaction fees.
3. `deal.buyers.len()` is now `MAX_BUYERS`. Every real buyer's `join_deal` fails with `DealFull`, and `commit` needs an entry in the list (`NotJoined`), so nobody else can buy.
4. At the deadline `total_units` (0) is below `min_units`, the deal fails, the merchant cannot `claim_proceeds`, and real buyers who joined earlier have to `refund`. The competitor repeats this for every new deal of that merchant.

The deal fails even without an attacker: 100 buyers with at most 4 units each can buy at most 400 units, below the 600 minimum, so under the deployment in ARCHITECTURE.md (per-buyer limits of 1 to 5 and minimums of 150 to 1,000 units) most deals cannot succeed. `join_deal`, `commit` and `refund` also scan the whole list on every call.

## Why fixed closes it
20 changed lines (a design case: the per-buyer data moves to a new account type, and the three buyer instructions each get that account):
- `state.rs`: `Commitment` becomes its own PDA account (`[b"commitment", deal, buyer]`, with a `deal` field); `Deal` replaces the `buyers` Vec with a `buyer_count` summary; `MAX_BUYERS` is removed.
- `buyer.rs`: `join_deal` creates the buyer's `Commitment` (paid by the buyer) and increments `buyer_count`; a second `join_deal` fails because the PDA exists, so the "joining again has no effect" branch and the `MAX_BUYERS` check are gone. `commit` and `refund` take the buyer's `Commitment` with `has_one = deal, has_one = buyer @ NotJoined` instead of searching the list.
- `errors.rs` is identical in both versions; `DealFull` ("Deal is full") is still used for the `max_units` check in `commit`.

The number of buyers is no longer bounded by one account. Sybil joins cost rent each and do not block anyone. The only shared limit is `max_units`, and that can be reached only with paid units, which makes the deal succeed. The normal flow is unchanged: join, commit, claim on success, refund on failure; the shared `Deal` account stays a fixed size.

## Decoys
- `Merchant` keeps only a `deal_count` counter and every deal is its own PDA (`[b"deal", merchant, deal_count]`). This is the same pattern applied correctly, so a merchant's deals never share one growing account.
- `total_units` is a running total in `Deal`, not recomputed from the list; `claim_proceeds` pays `cost(total_units)`, which equals the escrow contents.
- `refund` checks `!entry.refunded && entry.units > 0` and marks the entry before paying, so a buyer cannot refund twice; `claim_proceeds` requires `status == Open`, so it runs once and never after a cancel.
- `buyer_token` in `refund` is checked only for the mint; the buyer signs and chooses where to receive the refund.
- `create_deal` checks that `unit_price * max_units` fits in a `u64`, so `cost` can never overflow for committed units.
- `join_deal` returns without changes when the buyer is already in the list, so a repeated join neither duplicates an entry nor resets the buyer's units.

## Trusted roles
- Merchant authority: registers its merchant account, creates deals with its own parameters and mint, cancels its own open deals (buyers can then refund), and claims the escrow of its own deals that succeeded. It cannot touch buyers' funds in a failed or cancelled deal.
