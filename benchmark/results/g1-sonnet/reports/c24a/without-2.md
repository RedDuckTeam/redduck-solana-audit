# Solana audit: program/src/lib.rs (points_staking)

Severity: High
Location: program/src/lib.rs:193
Mechanism: `close_epoch` can only be signed by `pool.authority`, and `withdraw` (line 81) needs the epoch to be closed. If the operator key or keeper is offline, loses its fee wallet, is lost or is compromised, no epoch closes and every pending unstake is locked forever. `authority` can't be rotated and there is no emergency path, so the keeper is a single point of failure for user funds.

Severity: Medium
Location: program/src/lib.rs:62
Mechanism: Unstaking lowers `staked` and `total_staked` at once, and `withdraw` unlocks as soon as the request's epoch closes (line 81). A request made just before `close_epoch` can be withdrawn right after, so the unbonding period can be near zero. The program also records only `total_staked` at close and no per-staker snapshot. A user can stake just before the predictable close and unstake just after, which earns off-chain points for a full epoch without real commitment.

Severity: Medium
Location: program/src/lib.rs:106
Mechanism: Closing an epoch is gated only by the clock and the authority, and it sets `epoch_started_at = ends_at` (line 122). After keeper downtime, several epochs can be closed back-to-back in consecutive transactions. Each emits the same current `total_staked` as a "closed" epoch, which distorts points. The authority can also close epochs early in this catch-up state, which releases withdrawals at times the users can't predict.

Severity: Low
Location: program/src/lib.rs:170
Mechanism: The `Unbonding` PDA is seeded only by pool and owner, so a user can have just one pending request. A second `request_unstake` fails until `withdraw` runs, and withdrawal depends on the authority-gated epoch close. Because of that, a stalled keeper (the first finding) also stops users from unstaking further.

Severity: Low
Location: program/src/lib.rs:13
Mechanism: The pool's `authority` is the operator wallet. It pays for pool creation and also signs automated `close_epoch` calls from an online keeper server, so a hot key is exposed. The pool has no authority rotation or multisig support. The design also centralizes trust in one key per pool, and one transaction per pool per epoch is needed from that key, which scales poorly with the number of pools.

Severity: Info
Location: program/src/lib.rs:133
Mechanism: `stake_mint` is not checked for a freeze authority. If the mint has one, the mint's freeze authority can freeze the vault token account, which blocks all withdrawals. Only a plain SPL Token mint is supported, and the pool does not account for transfer fees.
