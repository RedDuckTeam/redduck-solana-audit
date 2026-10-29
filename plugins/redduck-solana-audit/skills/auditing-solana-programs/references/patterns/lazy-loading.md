# Lazy loading

## Rule
Never push a periodic result to every account. Keep two numbers instead:
- a global running total in the shared account, such as reward per share, a funding index or a version counter;
- a marker in each account, holding the value of that total it last saw.

When an instruction touches the account, it settles the difference first and then moves the marker to the current total. The settlement must run before anything changes the amount the difference is multiplied by.

## Why Solana forces it
A program reads and writes only the accounts listed in the transaction, and each transaction has a bounded account list and compute budget. No scheduler exists. A design step that says "for every holder", "each period" or "update everything when the signers change" means somebody must send and pay for many transactions. Any loop over a growing list stops fitting at some point. Settling on touch makes the user who wants the result pay for their own update.

## Code signals
- **Global totals:** `reward_per_token_stored`, `acc_reward_per_share`, `cumulative_funding_rate`, `cumulative_interest`, `global_index`, `stale_index`, with `last_update_ts` or `last_update_slot`.
- **Per-account markers:** `reward_debt`, `user_reward_per_token_paid`, `last_cumulative_funding`, `last_index`, `pending_rewards`, `rewards_owed`.
- **Settlement helpers:** `update_pool`, `accrue`, `refresh`, `settle`, `update_rewards`. Each should run at the top of every handler that touches the account.
- **Balance changes to check:** `stake`, `unstake`, `deposit`, `withdraw`, `liquidate`, `close_position`, `split`, `merge`, emergency withdraw, receipt-token transfers and Token-2022 transfer hooks.
- **Marker set at creation:** `init` or `init_if_needed` on a user account that leaves the marker at zero while the global total is already high.
- **The opposite design:** `for acc in ctx.remaining_accounts`, `pool.users.iter_mut()`, `distribute_rewards`, `apply_funding_all`, `epoch_rollover` walking holders, a `Vec<UserInfo>` in a global account.
- **Accrual math:** `rate * elapsed / total_staked` with no guard for `total_staked == 0`, and no scaling factor for precision.

## Auditor questions
1. Does any instruction loop over holders, positions, proposals or periods? What bounds the loop, and who pays to run it?
2. List every instruction that changes the multiplier (stake, shares, position size, debt) or moves the receipt token. Does each settle the difference and move the marker before the change?
3. When a user account is created, or re-created after a close, is its marker set to the current global total rather than zero?
4. Is the global total brought up to date before it is read? While total supply is zero, are rewards lost, stuck or divided by zero?
5. Can the receipt token move outside the program, for example by a plain token transfer, without settlement running for both sides?
6. Must anything happen to accounts nobody touches, such as liquidation, expiry or a deadline? Who triggers it, and are they paid to?
7. A lock start is a marker too. When a stake is topped up, does the new amount inherit an older lock start and skip the lock? If a top-up resets the start instead, can someone other than the owner top up a victim's position to keep it locked?

## How to confirm
Write the sequence test. Advance the global total by letting time pass or adding rewards. Then change a balance through the path that skips settlement, and claim. Compare the payout with the correct share. For the creation case, deposit after rewards have built up, claim at once, and show rewards paid for time before the deposit. In a test (see `exploit-tests.md`), move time with a clock override or slot warp in LiteSVM or bankrun.

## Not a bug if
- Settlement lives in one helper or shared constraint called by every handler that touches the account, transfer hooks included.
- The receipt token cannot move outside the program, because it is non-transferable or held in program custody, or a transfer hook settles both sides.
- The marker is set to the current total when the account is created.
- Each top-up creates its own lock, or only the owner can top up a position.
- A loop is bounded by a small constant fixed by the design, such as a few reward mints, not by the number of users.
- Untouched accounts stay stale, but nothing reads them before they are touched, and actions that must reach them are permissionless and paid.

## Real-world cases
- **Done right.** A perpetuals venue keeps a cumulative funding value per unit of position in each market. A trade, close or liquidation first settles the gap times the position size, then stores the current value in the position.
- **Done right.** A multisig retires every older proposal by storing one index when the member set changes. Reading a proposal compares its index with that number.
- **Done right.** A restaking token updates the holder's reward balance inside its transfer hook, so moving the token cannot skip the accounting.
- **Done right.** A device-rewards network stores what each device has been paid and pays the gap to an oracle-signed lifetime total when anyone claims.

## Severity
- Critical when any user can claim rewards they did not earn from a shared pool, for example a marker left at zero on a new account or a balance raised before settlement.
- High when the bug loses or freezes users' accrued rewards or funding under realistic conditions.
- A loop over all holders that will stop fitting is a design finding. It is High when it blocks withdrawals and nothing else unlocks them, and Medium when it only halts distribution until someone intervenes.
- Rounding in the accrual is Low, unless a user can repeat it for profit (see defensive-computation).
