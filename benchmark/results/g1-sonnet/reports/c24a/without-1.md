# Solana audit: points_staking

Severity: High
Location: program/src/lib.rs:193
Mechanism: `close_epoch` can only be signed by `pool.authority`, a single key that ARCHITECTURE.md says is the operator wallet also used by the online keeper. `withdraw` (line 81) requires `pool.epoch > request.epoch`, so if that key is lost, compromised-and-rotated, or the keeper stops, the epoch never advances and every pending unstake is locked forever. There is no authority rotation and no permissionless fallback.

Severity: Medium
Location: program/src/lib.rs:39
Mechanism: Points are computed from the staked balance at close time, with no time-weighting or snapshot. Because the 7-day epoch end is public and the keeper closes it immediately, an attacker can stake a large amount just before `close_epoch` and `request_unstake` right after. They earn a full epoch of points for almost no time at risk, and they can flash-stake with borrowed funds.

Severity: Medium
Location: program/src/lib.rs:122
Mechanism: `epoch_started_at = ends_at` lets the epoch be closed repeatedly in consecutive transactions after any keeper downtime. Each catch-up `EpochClosed` event records the current `total_staked`, not the historical value. Combined with the off-chain engine reading live staker balances, this lets users move stake between catch-up closes, or misattributes points for the missed epochs.

Severity: Medium
Location: program/src/lib.rs:67
Mechanism: `request_unstake` immediately lowers `staker.staked` and `pool.total_staked`. A user who unstakes mid-epoch loses the whole epoch's points, while the tokens stay locked until the epoch closes. The off-chain engine's inputs (the event total and live balances) can no longer be reconciled, because there is no per-epoch on-chain record of stake. Points accounting depends entirely on off-chain trust.

Severity: Low
Location: program/src/lib.rs:170
Mechanism: The unbonding PDA is seeded only by pool and owner, so a user has at most one pending request. A second `request_unstake` fails (`init`) until `withdraw` closes the first. Partial unstakes are blocked, and a user cannot exit in stages. Users also cannot cancel a request, and the withdrawn funds sit idle.

Severity: Low
Location: program/src/lib.rs:131
Mechanism: The program does not check the stake mint's `freeze_authority` or `mint_authority`. If the mint has a freeze authority, it can freeze the vault and block all withdrawals. Only the classic SPL Token program is supported, so Token-2022 mints (transfer fees or hooks) are not handled.

Severity: Low
Location: program/src/lib.rs:129
Mechanism: The pool PDA is seeded with `authority`, so authority is permanently baked into the address and cannot be migrated to a multisig or new key. The single hot operator key is therefore both a centralization point and an operational risk. A keeper that closes every pool in a separate transaction with one key also scales linearly with the number of pools.

Severity: Info
Location: program/src/lib.rs:13
Mechanism: `create_pool` is permissionless, so anyone can create a pool for any mint, including a look-alike of the team's. The front end and points engine must filter pools by the authority they trust.
