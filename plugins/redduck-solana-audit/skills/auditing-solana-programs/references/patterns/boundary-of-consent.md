# Boundary of consent

## Rule
Users sign bounds, not exact expectations: a minimum output, a maximum input, a limit price, a maximum oracle age, a deadline. The program runs the action anywhere inside the bounds and fails outside them. Two conditions make this work:
- Every value the instruction computes from state that can change before it lands has a bound the user sets.
- The bound is checked on what actually moves, after fees and transfer fees, on every route through the code.

## Why Solana forces it
Between signing and execution, other transactions change pool reserves, oracle prices, rates and fees. The leader orders the block, and bundles let a third party place transactions right before and right after the user's. A user who demands an exact result fails on any change. A user with no bound accepts whatever the attacker leaves. Stored orders, durable-nonce transactions and keeper-run requests can execute long after signing, so the age of the price and of the action matters too.

## Code signals
- **Bound arguments, usually good:** `min_amount_out`, `minimum_amount_out`, `other_amount_threshold`, `max_amount_in`, `slippage_bps` together with a quoted amount, `limit_price`, `sqrt_price_limit`, `acceptable_price`, `min_shares`, `max_cost`, `deadline`, `expiry`, `max_age` or `max_staleness`. In native code, the same fields in the instruction data struct.
- **Missing bounds:**
  - `swap(amount_in)` with no output bound;
  - `deposit(amount)` that mints shares at a computed rate with no `min_shares`;
  - `withdraw(shares)` or `redeem` that pays a computed amount with no `min_out`;
  - a bonding-curve `buy` with no `max_cost`, or a `sell` with no minimum proceeds;
  - market orders filled at the oracle price with no acceptable price;
  - `liquidate` where the seized collateral is computed and has no minimum.
- **Misplaced checks:**
  - the bound compared with a pre-fee figure, or with an amount computed before a Token-2022 transfer fee is taken;
  - a check on each hop of a route instead of on the final output;
  - an exact-out path that skips the `max_in` check;
  - a tolerance computed from the current on-chain price, which moves together with the manipulation;
  - a CPI wrapper or keeper path that passes `0` or `u64::MAX` as the bound.

## Auditor questions
1. List every value the instruction computes from mutable state that changes what the user gives or gets: output, input, shares, fill price, fee, collateral seized. Can the user bound each one?
2. Is the bound checked against the amount actually moved, ideally the change in the user's token account, after protocol fees and Token-2022 transfer fees?
3. Is the bound a fixed number the user chose off-chain, or is it derived on-chain from state the attacker can move?
4. Does every route reach the check: exact-in and exact-out, both swap directions, multi-hop, partial fills, wrapper programs?
5. For actions that run later, such as limit orders, keeper-executed requests or durable-nonce transactions: is there a deadline or a maximum age, and a limit price checked at execution?
6. Can an admin change fees or parameters between the quote and execution, beyond what the user's bounds cover?

## How to confirm
Write a sandwich test (see `exploit-tests.md`). In order: the attacker swaps to move the price; the victim's transaction runs with the bound a normal client would sign, or with none because the instruction takes none; the attacker swaps back. Show that the victim received less than they agreed to, or that the instruction accepted an amount below the signed minimum, and that the attacker ended with a profit. For deadlines, execute a stored order after its deadline or with a stale price and show it succeeds.

## Not a bug if
- The instruction takes the bound and checks it on the final amount on every path.
- Nothing depends on mutable state: a fixed-price sale, or a 1:1 wrap.
- An outer program enforces the bound, for example an aggregator that checks the user's net balance change, and users never call the inner instruction directly.
- The action is a stored limit order, and the limit is checked on each fill.
- The only party without a bound is a profit-seeking bot, such as a liquidator or an arbitrageur, that can add its own balance check to its transaction. Report it as Low at most.

## Real-world cases
- **Done right.** A swap aggregator quotes a route off-chain, but the user signs a minimum output derived from the quote and a slippage tolerance chosen before signing. The route runs as long as the output is at or above that minimum, and the whole transaction fails below it.
- **Done right.** Concentrated-liquidity pools take both an amount threshold and a price limit, so a swap stops at the price the user accepted instead of walking the whole curve.
- **Done right.** Perpetuals venues let a trader attach a limit or acceptable price to a market order, and reject oracle prices older than a set age.

## Severity
High when a missing or misplaced bound lets a sandwich attacker take value from swaps, deposits, withdrawals or fills. Sandwiching is routine on Solana, so this is a realistic condition. Critical when the attacker can also trigger the trade, for example by executing a user's stored order that has no limit price at a moment they choose. Medium when another check caps the loss per transaction, such as a maximum price deviation, or when only a deadline is missing on an action that keeps a limit price. Low when the unbounded party is a bot that can protect itself. An admin who can change fees under a pending trade is a trusted role: report it as a design finding, at Medium at most.
