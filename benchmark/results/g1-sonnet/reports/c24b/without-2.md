# Solana audit: points_staking

Severity: High
Location: program/src/lib.rs:121
Mechanism: `close_epoch` is permissionless and `stake`/`request_unstake` have no warm-up or cool-down. An attacker can stake a large amount, call `close_epoch` in the same transaction (once the epoch has ended), and then unstake. The `EpochClosed` event reports the inflated `total_staked`, and the off-chain points engine credits the attacker for an epoch they never held stake through. The attacker also picks which snapshot is emitted. There is no per-staker, per-epoch checkpoint on chain, so the engine cannot tell time-weighted stake from a flash stake.

Severity: Medium
Location: program/src/lib.rs:136
Mechanism: `epoch_started_at` is set to `ends_at` instead of `now`. After keeper downtime, several epochs can be closed back to back in consecutive transactions. Each gets an event with identical `total_staked` and a near-zero real duration. The points engine can credit full epochs for no elapsed time, and unstake requests can be made to mature immediately, because `withdraw` only checks `pool.epoch > request.epoch`.

Severity: Medium
Location: program/src/lib.rs:93
Mechanism: Withdrawal liveness depends on someone calling `close_epoch`. With a 7-day epoch, a request made right after a close waits about 7 days, and it is never withdrawable if nobody closes the epoch. The only incentive is the tip pot, which is 0 when there are no requests, and the keeper is a single team-run service. Users' funds in `Unbonding` therefore depend on an off-chain actor, with no emergency exit.

Severity: Low
Location: program/src/lib.rs:138
Mechanism: The tip pot goes to whichever signer lands `close_epoch` first. Searchers or bots can front-run the keeper and take the tips. A user can also run many sybil stakers and recover their own tips by closing the epoch themselves, so the tip is not a reliable payment for the keeper. This weakens the liveness assumption above.

Severity: Low
Location: program/src/lib.rs:191
Mechanism: The `Unbonding` PDA is seeded by (pool, owner), which allows only one pending request per user. A user cannot queue a second unstake until the first epoch closes and they withdraw. A user who wants to exit more stake must wait a full epoch, and `request.epoch` cannot be refreshed or topped up. This is a usability and liveness limit rather than a loss of funds.

Severity: Low
Location: program/src/lib.rs:222
Mechanism: Scalability and centralization. Every stake, unstake and close writes to the single `Pool` account, so all user transactions serialize on one write lock. `Pool` also holds the tip lamports. Nothing on chain lets `authority` pause the pool, change `epoch_length`, or recover from a bad state, and the authority has no role after creation. Anyone can create a pool for the same mint under their own authority, so the front end must whitelist the legitimate pool.

Severity: Info
Location: program/src/lib.rs:34
Mechanism: `Staker` accounts can never be closed (there is no `close_account` instruction), so rent stays locked and accounts accumulate. `stake` also has no minimum amount, so dust stakers can bloat the set that the off-chain engine must scan.
