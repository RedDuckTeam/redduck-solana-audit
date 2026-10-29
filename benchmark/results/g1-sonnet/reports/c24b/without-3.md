# Solana audit: points_staking

Token custody looks sound. The vault is pinned to `pool.vault`, the `has_one` checks are in place, signers are required, and the arithmetic is checked. The findings below are about points integrity and design.

Severity: High
Location: program/src/lib.rs:121
Mechanism: `close_epoch` emits only an instantaneous `total_staked`, and `stake` (line 43) records no per-epoch or time-weighted balance. The off-chain engine reads current staker balances, so someone can stake a large amount just before `close_epoch` (even in the same transaction) and be credited a full epoch of points. They can then request an unstake and withdraw after the next close, which dilutes honest stakers' points.

Severity: Medium
Location: program/src/lib.rs:137
Mechanism: `epoch_started_at = ends_at` lets a backlog of overdue epochs be closed back-to-back in one transaction. This happens if the keeper is down for more than one epoch. Each close emits an identical `EpochClosed` snapshot, so someone can stake just before the batch of closes and be credited for several epochs they were never staked for.

Severity: Medium
Location: program/src/lib.rs:96
Mechanism: `withdraw` only requires that the epoch of the request has closed. A request made just before a close can be withdrawn immediately afterwards, so the effective unbonding period is anywhere from zero to 7 days. Withdrawals also depend on someone calling `close_epoch`, and the only incentive is a tip that is zero when nobody is unstaking and small otherwise (0.0001 SOL per request). Liveness therefore rests on the team's keeper.

Severity: Medium
Location: program/src/lib.rs:191
Mechanism: The `Unbonding` PDA is seeded only by pool and owner, so each user can have one pending request. The user cannot unstake again until they have withdrawn, and they cannot split exits across epochs. The request is closed only when the owner calls `withdraw`.

Severity: Low
Location: program/src/lib.rs:140
Mechanism: The close tip goes to whichever wallet calls first, so bots can front-run the team's keeper and take it. The tip is also paid per request regardless of amount, so it adds nothing as an incentive when there is no unstake activity.

Severity: Low
Location: program/src/lib.rs:150
Mechanism: Anyone can create a pool for any mint with their own authority, and the `authority` field has no on-chain function beyond the PDA seed. Nothing on chain marks the team's pool as canonical, so fake pools for the same mint can phish users. The pool also has no admin functions, so there is no way to fix `epoch_length` or pause the program.

Severity: Low
Location: program/src/lib.rs:152
Mechanism: The vault is a plain SPL Token account. If the stake mint has a freeze authority, it can freeze the vault or a user's token account and block `withdraw`. Token-2022 mints (transfer fees, hooks) are unsupported by the `Token` program constraint.

Severity: Info
Location: program/src/lib.rs:16
Mechanism: `create_pool` has no check that `stake_mint` is trusted, and the pool is not upgrade-governed. Program upgrade authority and key management for the operator wallet are not described in the repo.
