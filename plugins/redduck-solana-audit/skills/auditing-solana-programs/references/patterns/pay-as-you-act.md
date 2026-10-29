# Pay-as-you-act

## Rule
Keep the result, not the list of inputs. Each action updates the aggregate it changes, in the same instruction and at the actor's cost: a count, a sum, a maximum, a current leader, the liquidity at a tick. The closing step then reads a few fields instead of walking every input. The aggregate is correct only if every path that adds, removes or changes an input updates it, including withdrawals, cancels and closes.

## Why Solana forces it
A closing step that processes every vote, bet or order pays for all of them in one transaction. It also has to fit in one transaction's compute and account limits. Nobody wants to pay for everyone's work, and past some size nobody can. Spreading the work across the actions that create it keeps every transaction small and makes the final step cheap.

## Code signals
- **Final steps that iterate:** `finalize`, `tally`, `resolve`, `close_round`, `settle_all`, `compute_winner`, `distribute` with `for x in ctx.remaining_accounts` or over a `Vec` of entries in a global account (`votes: Vec<Vote>`, `bets: Vec<Bet>`).
- **Aggregate fields:** `total_yes` and `total_no`, `side_totals`, `pool_total`, `total_staked`, `total_shares`, `leader` and `leader_votes`, `liquidity_net` and `liquidity_gross` on ticks, `sum` and `count` for averages.
- **Paths that change an input:** `vote`, `change_vote`, `relinquish_vote`, `bet`, `cancel_bet`, `deposit`, `withdraw`, `add_liquidity`, `remove_liquidity`, `increase_position`, `decrease_position`, `close_account`, emergency exits, liquidations.
- **Aggregates that cannot shrink in one step:** a stored maximum or leader in a design where inputs can be withdrawn. Removing votes from the leader cannot find the new leader without a scan.
- **Pro-rata claims:** `user_amount * pool / winning_total`, with the division order and rounding direction visible.

## Auditor questions
1. Is there a step that must process many items at once (tally, resolve, pay everyone)? How does its cost grow, and what happens when it no longer fits?
2. For each aggregate, list every instruction that changes an input. Does each one update the aggregate by the right amount in the same instruction?
3. Can the aggregate be undone in one step? Sums and counts can. A maximum or leader cannot, so check that inputs only grow while it is tracked.
4. After the result is fixed (round resolved, vote closed), can inputs still change? Are such changes refused, or do they move an aggregate that payouts already rely on?
5. Do the per-user claims add up to no more than the pool, with rounding in the pool's favour, and can each claim be made only once?

## How to confirm
Run the sequence: add inputs, remove or cancel one through the path that skips the aggregate, then resolve and let everyone claim. Show that the aggregate disagrees with the sum of live inputs, and that someone is overpaid or the last claimant cannot withdraw. For an iterating final step, find the number of items at which it exceeds compute or the account limit, and the cost for one actor to reach that number. In a test (see `exploit-tests.md`), fill the structure in a loop and then call the final step.

## Not a bug if
- The final step reads aggregates only, and each payout is a separate claim by its owner.
- Inputs can only grow until the round closes, so a stored maximum or leader stays right.
- A loop is bounded by a small constant fixed by the design, such as the number of outcomes.
- The aggregate is rebuilt by a permissionless, paginated crank with a cursor, and nothing can finalize until the cursor has covered every account.

## Real-world cases
- **Done right.** A pool-betting market adds each bet to its side's total as it is placed. After the outcome is set, each winner claims their stake's share of the whole pool, computed from the winning side's total.
- **Done right.** A governance program updates a proposal's vote totals in every vote, and withdrawing a vote while voting is open subtracts it. Ending the vote compares the totals.
- **Done right.** Concentrated-liquidity DEXes record liquidity changes only at the two ticks that bound a position. A swap reads only the ticks it crosses, so no step ever sums all positions.

## Severity
- Critical when a stale aggregate lets a user take funds that belong to others, for example a cancel that refunds a stake but leaves it in the total used for payouts.
- High when it leaves the vault short, so later claimants cannot withdraw.
- A final step that stops fitting as inputs grow is a design finding. It is High when it locks deposits with no other exit, and Medium when it halts the protocol until someone intervenes.
- Rounding in pro-rata claims is Low, unless it can be repeated for profit.
