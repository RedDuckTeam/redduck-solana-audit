---
id: c20
crate: outcome_pools
pattern: root-of-trust
class: exploit
group: g1
vulnerable: b
role: dev
author: case-author
canary: RDSAF81C11F051B132899533
---
# Case c20

`outcome_pools` runs two-sided (Yes/No) pari-mutuel pools on real-world events. Anyone can register as a reporter and post one immutable result per event. Anyone can create a market, and the market stores the `Reporter` account that is supposed to settle it (`market.reporter`). Stakers back a side until `closes_at`. After that anyone can call `resolve` to copy a report's result into `market.winner`, and winners split both pools pro rata in `claim`.

## Attack scenario
1. Market M (event `E`, reporter R = the honest reporter, `closes_at` = T) holds 10,000 USDC on Yes and nothing on No. The event takes place after T.
2. Shortly before T, the attacker calls `place_stake(No, 1)` on M. Their ticket is 1 USDC on No, so the No pool is 1.
3. The attacker calls `register_reporter(name)`. This creates their own `Reporter` R' = PDA(`"reporter"`, attacker) with `authority` = attacker.
4. At T, when staking has closed but the event has not started, the attacker calls `post_report(event_id = E, result = No)` signed as R'. This creates `Report` P' = PDA(`"report"`, R', E) with `reporter = R'`, `event_id = E` and `posted_at = T`.
5. In the same transaction the attacker calls `resolve` with `market = M`, `reporter = R'`, `report = P'`. The checks pass. `posted_at >= closes_at` holds. `Account<Reporter>` and `Account<Report>` only prove that the program wrote these accounts, and anyone can make it write them. `has_one = reporter` compares P' with R', and both come from the attacker. `report.event_id == market.event_id` is true. Nothing compares R' with `market.reporter`. So `market.winner = No`.
6. The attacker calls `claim` and is paid `1 * 10,001 / 1 = 10,001` USDC, which is the whole Yes pool. Every Yes ticket pays 0. When R later posts the real result, `resolve` fails with `AlreadyResolved`.

## Why fixed closes it
The fixed program adds one constraint to the `Resolve` accounts struct (fixed `src/lib.rs:177`):
`#[account(address = market.reporter @ PoolError::WrongReporter)]` on `reporter`. It also adds the `WrongReporter` error variant (fixed `src/lib.rs:296-297`).

The chain of checks now ends at the market. `report.reporter == reporter` (has_one) and `reporter == market.reporter` (address). `market` is a PDA that only `create_market` can write, and its `reporter` field is fixed at creation and visible to stakers before they stake. R' differs from `market.reporter`, so step 5 fails with `WrongReporter`, and no other account the attacker can create passes.

The normal flow still works: the market's own reporter posts the report, anyone calls `resolve` with that reporter and report, and `claim` pays as before. The diff is 3 added lines.

## Decoys
- `resolve` has no signer, so anyone can resolve. In the fixed program this is correct, because the outcome comes only from the market's reporter and the caller has no say.
- `register_reporter` and `create_market` are open to everyone by design. Each market names its reporter at creation, and stakers can see which one it is.
- `market: Account<'info, Market>` in `PlaceStake`, `Resolve` and `Claim` has no seeds constraint. Markets can only be created through `create_market` at a PDA, and `ticket` is bound with `has_one = market`.
- `claim` lets the ticket owner pick any token account of the market mint. Only the owner can sign for the ticket, and the payout is their own money.
- `payout` uses u128 with `checked_mul` and floor division, so the winners' payouts never add up to more than the two pools. If nobody backed the winning side, every stake is refunded.
- `RESOLVE_WINDOW`: `resolve` is blocked after the window, and refunds of unresolved markets only start after it. A market cannot both pay winners and refund.
- `resolve` rejects a report posted before `closes_at` (`EarlyReport`), in both versions. So a result that was known while staking was open, for example from a market created for an event that was already reported, can never settle that market. Such a market becomes refund-only after `RESOLVE_WINDOW`.

## Trusted roles
- **The market's reporter** (the authority of the `Reporter` account stored in `market.reporter`): posts one immutable result per `event_id` with `post_report`. Its result settles the markets that named it, and it is trusted to post the true result after the event. It cannot touch any vault, change a posted result, or settle a market before `closes_at`.
- **The market creator:** chooses `event_id`, `closes_at` and the reporter when calling `create_market`. After that it has no powers, and in particular cannot move stakes or change the reporter.
- Nobody else is trusted. Any other registered reporter is an untrusted party for markets that did not name it. Callers of `resolve` and `claim` have no influence beyond passing accounts.
