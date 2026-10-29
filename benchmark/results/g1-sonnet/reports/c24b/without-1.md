# Solana audit: points_staking

Severity: High
Location: program/src/lib.rs:121
Mechanism: `close_epoch` is permissionless and only advances `epoch_started_at` by one `epoch_length`. If the epoch is overdue, anyone can close several epochs back to back in one transaction, and each `EpochClosed` event snapshots the same instantaneous `total_staked`. An attacker can stake a large amount, close N overdue epochs in the same transaction, and be credited by the points engine for N full epochs of stake they never held. Closing also needs no stake-time requirement, so just-in-time staking before any close is rewarded the same as holding for the whole epoch.

Severity: Medium
Location: program/src/lib.rs:43
Mechanism: Points are computed off-chain from a point-in-time snapshot (event totals plus staker balances), and the program records no per-epoch stake history or time-weighting. A user can stake just before `close_epoch` (even in the same transaction) to get a full epoch of credit. Staking is also not tied to the epoch in progress, so the snapshot is trivially gameable.

Severity: Medium
Location: program/src/lib.rs:96
Mechanism: The unbonding delay is only "the request's epoch has been closed", not a time period. When an epoch is overdue, a user can call `request_unstake`, then `close_epoch` themselves, then `withdraw` in one transaction. That gives a zero-length unbonding period and defeats any lockup assumption the points program relies on.

Severity: Low
Location: program/src/lib.rs:138
Mechanism: The close tip is paid to whichever wallet calls `close_epoch` first, so the keeper has no guaranteed reward. Searchers can front-run the keeper to collect the tip. If tips are small compared with fees, nobody has an incentive to close, and the epoch, points and withdrawals stall (liveness depends on an external actor). The tip pot is also unrelated to the actual cost of closing.

Severity: Low
Location: program/src/lib.rs:191
Mechanism: The `Unbonding` PDA is seeded only by (pool, owner), so a user can have only one pending request. Partial exits must be serialized, and every new request must wait for a close plus a withdraw. Each request also costs the 0.0001 SOL tip on top of rent, so the exit path is slower and costlier than necessary for users who want to unstake in several steps.

Severity: Low
Location: program/src/lib.rs:150
Mechanism: Pool PDA seeds are (authority, stake_mint), and anyone can create a pool for any mint with themselves as authority. Attackers can create look-alike pools for the team's token and lure users who do not verify the pool address. The `authority` field is stored but never used for authorization, which is mostly harmless but misleading. There are no admin controls: epoch length is immutable, and there is no pause or upgrade path other than the program upgrade authority.

Severity: Info
Location: program/src/lib.rs:174
Mechanism: Every `stake`, `request_unstake` and `close_epoch` write-locks the single `Pool` account, so throughput is serialized per pool. This is a scalability limit and a contention point: spamming transactions against the pool can delay the keeper's `close_epoch`.

Severity: Info
Location: program/src/lib.rs:161
Mechanism: `Staker` accounts are never closed, so rent for finished stakers is locked forever. The vault accepts only the classic SPL Token program, so Token-2022 mints (including ones with transfer fees) are unsupported, and `owner_token` is not constrained to be owned by `owner`. The latter is harmless because `owner` must sign.
