# Defensive computation

## Rule
Value every number at the end of its plausible range that protects the protocol:
- collateral and assets received at the low end, for example price minus confidence;
- debt and assets owed at the high end, for example price plus confidence;
- every division rounded against the user: shares minted down, assets paid out down, debt and fees up.

Actions the user chooses (open, borrow, withdraw) get stricter limits than actions that happen to the user (liquidation). Otherwise a small move liquidates a fresh position, or a user opens right at the edge.

## Why Solana forces it
Solana oracles publish a price together with an uncertainty band. Pyth gives `conf` and Switchboard gives a standard deviation. The middle value is only the best guess. Users choose when to act, so they act when the guess favours them. Programs compute in integers, so every division rounds one way or the other. Fees are low and one transaction can hold many instructions, so a rounding gain of one unit can be repeated thousands of times for almost nothing.

## Code signals
- **Pyth:** `PriceUpdateV2`, `get_price_no_older_than`, `price_message.price`, `.conf`, `.exponent`, `ema_price`, `ema_conf`; legacy `Price { price, conf, expo, publish_time }` from `load_price_feed_from_account_info`. The signal is `conf` never read, or one price used for both collateral and debt.
- **Switchboard:** `get_result()`, `latest_confirmed_round.std_deviation`, `PullFeedAccountData`, `std_dev`. Same signal for the deviation field.
- **Scaling:** `price as u64` with no sign check; `expo` or `exponent` applied in one direction only; mint `decimals` not reconciled with the oracle exponent; `u128` results cast `as u64`.
- **Rounding:** plain `/` or `checked_div` in share, collateral and fee math with no stated direction; `try_round_u64` or `.round()` where floor or ceil is needed; `mul_div_floor` and `mul_div_ceil` swapped.
- **Thresholds:** one ratio for both borrowing and liquidation (`max_ltv == liquidation_threshold`), or no buffer between them.
- **Two prices for one asset:** spot in one instruction and an average in another, or different rounding on deposit and on withdrawal.

## Auditor questions
1. Wherever the program values collateral, debt, shares or a mint amount, which price does it use? Is collateral at the low end and debt at the high end?
2. Is the confidence capped, and is the price refused when the band is too wide (see fail-fast)?
3. For each division, which way does it round, and who keeps the remainder? Are the directions consistent across deposit, withdraw, borrow, repay and fee paths?
4. Can a user gain by repeating a minimal action, such as a withdrawal of one unit that burns zero shares, or a fee that rounds to zero?
5. Is there a round trip across two prices or two rounding modes, such as deposit at one valuation and withdrawal at another, that ends in profit?
6. Is the opening limit stricter than the liquidation limit? Are exponent, sign and token decimals handled for every feed?

## How to confirm
Write a round-trip test (see `exploit-tests.md`): deposit then withdraw, or borrow then repay, and assert the user ends with more than they put in. For rounding, loop the smallest action in one or several transactions, then compare the vault's assets with its liabilities. For confidence, forge the oracle account with the same price and a much wider `conf`, and show that the borrow limit does not change. A position that should have lost borrowing power keeps all of it.

## Not a bug if
- The remainder favours the user by at most one base unit per action, and repeating the action costs more than it gains.
- Liquidation uses the plain oracle price on purpose. It protects the protocol and must react quickly.
- Collateral is valued at the lower of the spot price and a moving average, and debt at the higher, instead of using the band.
- The price is part of a trade the user bounds with a signed minimum out (see boundary-of-consent), and it values no one else's funds.

## Real-world cases
- **Done right.** A lending protocol values collateral at the bottom of the oracle's band and debt at the top. When the band widens, borrowing power falls from both sides at once. It caps the widened band and refuses the price beyond a per-asset limit.
- **Done right.** Lending and perpetuals venues keep two lines: a strict one for opening and borrowing, and a looser one for liquidation. The gap protects a fresh position from the next small move.
- **Done right.** A perpetuals exchange lends against a deposit at the lower of the live price and a five-minute average. A short spike, real or manipulated, cannot be borrowed against until the average catches up. Liquidations still use the plain price.
- **Done right.** A wireless network sells usage credits for its token. It prices the token at its moving average minus twice the confidence, so burning tokens never yields more credits than they were worth.
- **Failure.** A widely forked lending program rounded the amount paid on redemption to the nearest unit instead of down. Redeeming the smallest amount many times per transaction paid out a little more than it burned each time.

## Severity
Critical when any user can repeat a rounding or price-gap round trip for profit with no special conditions. High when collateral or debt is valued at the middle price, or one threshold serves both borrowing and liquidation, so that realistic volatility lets users borrow more than their collateral is worth and leave bad debt. Medium when the loss per action is bounded or needs unusual market conditions. Low when rounding favours the user by dust that cannot be repeated at a profit.
