# Internal oracle

## Rule
A program may derive prices, rates and facts from on-chain state instead of asking an outside oracle. The source must be state that nobody can move and then use inside the same transaction:
- **Correct sources:** accumulated or lagged state that needs time to move, such as a TWAP read across a window, and state the program accounts for itself, such as a `total_deposits` field or a vote PDA that exists only after a vote.
- **Unsafe sources:** spot values that a swap, a flash loan, a deposit or a plain token transfer can change, such as pool reserves, `vault.amount`, `lamports()` or the current tick.

A price taken from an unsafe source is a severe flaw. Derived state must also be updated on every path that changes its inputs.

## Why Solana forces it
One transaction can hold many instructions. An attacker can borrow, swap, call the victim and swap back, all atomically. Anyone can send tokens or lamports to any account, so a vault's balance is not under the program's control. The program sees only the current state. History exists only when earlier transactions stored it, which is exactly what an accumulator does. Done right, the pattern removes an external feed, which is often the dependency that sets the most sensitive number in the protocol.

## Code signals
- **Spot values used as prices:**
  - `reserve_a / reserve_b`, `pool.token_a_amount`, `vault.amount` on an Anchor `TokenAccount`, or `spl_token::state::Account::unpack(..)?.amount` feeding a price, rate, collateral value or mint amount;
  - concentrated-liquidity fields such as `sqrt_price_x64` or `tick_current`, read once from a pool account;
  - share math `total_assets / total_shares` where `total_assets` is a token balance or `lamports()`, not a stored field;
  - LP token value computed from the pool's current balances.
- **Accumulators:** `observations[..]`, `observation_index`, `cumulative_price`, `price_cumulative`, `tick_cumulative`, `last_update_timestamp`. Check the window length, and whether the code reads two observations or only the newest one.
- **Existence as the answer:** `init` of a receipt or vote PDA keyed by subject and user; `data_is_empty()` or `lamports() == 0` used to decide "already done".
- **Program-accounted totals:** `total_deposited`, `total_borrowed`, `total_shares`. Look for any instruction that changes the inputs without updating them.

## Auditor questions
1. For each price, rate or value computed from on-chain state, which fields does it read? Is any of them a balance, a reserve or a current tick?
2. Can an attacker move that source and use the result in the same transaction, through a flash loan, a swap before and after, or a direct transfer to the vault?
3. For an accumulator: how long is the window, is a minimum elapsed time enforced, and is the pool account anchored to a trusted address (see root-of-trust)? How deep is the pool, and what would it cost to hold a false price for the whole window?
4. Is the derived state updated on every path that changes its inputs, including withdrawals, liquidations, closes and admin actions?
5. For existence checks: is the record a PDA with anchored seeds, created with `init`? Can someone pre-fund the address so it looks taken, or close and recreate it to reset the answer?

## How to confirm
For a spot source, build one transaction with three instructions (see `exploit-tests.md`). First move the source: a large swap into the pool, or a token transfer straight into the vault. Then call the instruction that uses the derived value, such as borrow, mint or redeem. Then reverse the move. Show the attacker ends with more than they started with. For a donation, transfer tokens to an empty vault before the first deposit and show the next depositor's shares round down to zero. For an accumulator, state how long the price must be held and what that costs at the pool's depth.

## Not a bug if
- The value is the difference of two cumulative readings divided by the time between them, over a window long enough that one transaction moves it very little.
- The program reads its own accounting fields, and a direct transfer to the vault does not change them.
- The spot price is the trade itself: an AMM swap priced from its own reserves, bounded by a user-signed minimum out (see boundary-of-consent). It values nobody else's funds.
- The spot value is used only for display, events or a check that cannot move funds.
- The vault resists a first-deposit donation through virtual shares, an offset, or a minimum first deposit that stays locked.

## Real-world cases
- **Done right.** A constant-product exchange keeps a ring of price observations. Each swap adds the price times the elapsed seconds to a running total. A reader takes two observations and divides the change in the total by the seconds between them. A swap that holds the price for a few seconds barely moves a ten-minute average.
- **Done right.** A concentrated-liquidity exchange accumulates the tick times elapsed time, and converts the average tick back into a price.
- **Done right.** A governance program stores each vote as a PDA derived from the proposal and the voter. No list of voters exists: if the address is taken, the vote happened. The owner record keeps only a count of open votes, which blocks withdrawals while voting runs.
- **Failure.** A protocol priced its own token from its bonding curve and paid out of its treasury at that price. An attacker took a flash loan, bought along the curve to push the price up, exchanged at the raised price and repaid the loan, all in one transaction.

## Severity
Critical when a spot value that a swap, donation or transfer can move in the same transaction decides a mint, a loan, a redemption, a liquidation or a share price. High when the source lags but its window is so short, or its pool so thin, that holding a false price for a few slots pays off; a leader produces several slots in a row. High also when derived state misses an update on some path and drifts from the truth in a way users can exploit. Medium when the drift or manipulation causes only bounded loss. A value read from program-accounted state that every path keeps current is not a finding (see item 14 in `false-positives.md`).
