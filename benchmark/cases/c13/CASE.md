---
id: c13
crate: stake_credit
pattern: defensive-computation
class: exploit
group: g3
vulnerable: b
role: heldout
author: case-author
canary: RDSAF81C11F051B132899533
---
# Case c13

## Attack scenario
Market: staking-token collateral, stablecoin debt, `ltv_bps = 7500`, `liquidation_threshold_bps = 8500`, `liquidation_bonus_bps = 500`, `borrow_fee_bps = 50`. Lenders have supplied 2,000,000 debt tokens.

1. The collateral token trades thinly across venues. Its feed account (owned by `FEED_PROGRAM_ID`, key stored in `market.collateral_feed`) publishes `price = 100.00`, `conf = 30.00`: quotes range from about 70 to 130. The debt feed publishes `1.00 ± 0.001`. Both are fresh and pass every check in `Quote::load`.
2. The attacker buys 10,000 collateral tokens on the cheapest venue at about 71 (cost about 710,000).
3. The attacker calls `open_account`, then `borrow(deposit = 10_000 tokens, amount = 746_000)`. The handler adds the fee (debt becomes 749,730) and calls `check_ltv`. `within_limit` values the collateral with `Quote::value` at the midpoint `price` (1,000,000) and the debt at the midpoint, rounded down, so `749_730 * 10_000 <= 1_000_000 * 7_500` passes. `Quote::load` reads `conf` from the feed and drops it.
4. The attacker receives 746,000 debt tokens for collateral that cost about 710,000 and never repays. The position is worth less than its debt at the price the attacker actually paid.
5. When the price settles near the low end of the published range, the position is liquidated. Liquidators take all of the collateral for about 676,000 repaid (`710_000 / 1.05`), and `liquidate` writes off the remaining debt of about 73,000 from `market.total_debt`. The share price of every lender drops by that amount. The attacker repeats this while the feed's range stays wide, limited only by pool cash.

The same midpoint valuation also lets `repay(0, withdraw)` release collateral that only the midpoint price covers.

## Why fixed closes it
`src/pricing.rs` changes in 13 lines: `Quote` keeps the feed's `conf`, gains `low()` and `high()` (price moved to the bottom or top of the confidence range), and `within_limit` values collateral at `low()` rounded down and debt at `high()` rounded up. In the scenario the collateral now counts as 700,000 and the debt as about `1.001` per token, so the largest borrow is about 521,000, below the 710,000 the attacker paid, and the position stays covered even at the bottom of the range.

Normal flow still works: with a narrow range (for example `conf` at 0.2% of price) borrowing, repaying, withdrawing and liquidation behave as before, with limits a fraction of a percent tighter. `within_limit` is shared by `check_ltv` and `liquidate`, so liquidation eligibility uses the same valuation. The amount seized in `liquidate` still uses the midpoint, which splits the repaid value fairly between borrower and liquidator and does not affect lenders.

## Decoys
- `collateral_feed` and `debt_feed` are `UncheckedAccount`s, but `Quote::load` checks the key against the market, the owner against `FEED_PROGRAM_ID`, a positive price, the exponent range and the age.
- Share math rounds against the user (`shares_for_deposit` and `assets_for_shares` round down) and the borrow fee rounds up. This is the same flaw class done correctly.
- Pool value uses the tracked `cash + total_debt`, not the vault balance, and the creator's seed shares are never redeemable, so donations and first-depositor share inflation do not work.
- `liquidate` takes any `CreditAccount` without a seeds check: `has_one = market` and the program-owned account type are enough, and the borrower gains nothing from it.
- The seized amount in `liquidate` is valued at the midpoint and rounded down. This is intended (see above).
- The write-off in `liquidate` happens only when the position has no collateral left, so it cannot erase debt that collateral still covers.

## Trusted roles
- Market creator (`admin`): creates the market once, chooses the mints, the two feed accounts and the risk parameters within the bounds checked in `init_market`, and locks the seed liquidity. It has no instruction after creation.
- Feed program (`FEED_PROGRAM_ID`) and its publishers: trusted to publish an honest price, confidence and timestamp in the feed accounts.
