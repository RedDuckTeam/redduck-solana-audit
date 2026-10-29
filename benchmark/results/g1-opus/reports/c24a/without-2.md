# points_staking audit report

Scope: `program/src/lib.rs`, `program/ARCHITECTURE.md`, `program/Cargo.toml`.

## 1. Withdrawals depend on a single permissioned keeper (liveness / centralization)

Severity: High
Location: program/src/lib.rs:193
Mechanism: `close_epoch` needs `has_one = authority`, so only the operator wallet can move the epoch forward, and `withdraw` (lib.rs:81) needs `pool.epoch > request.epoch`. If the keeper server goes down, its fee wallet runs dry, or the operator key is lost or withheld, no epoch ever closes again and every pending unstake stays locked in the vault forever. The team can also freeze withdrawals at will just by not calling it. Nothing lets anyone else close an epoch once its time has passed, and the authority can't be rotated.

## 2. Points come from a single end-of-epoch snapshot, so a user can stake briefly and get points for the whole epoch

Severity: Medium
Location: program/src/lib.rs:115
Mechanism: `EpochClosed` only records `total_staked` at the moment of closing, and the off-chain engine reads staker balances with no time weighting and no per-staker snapshot. Closing happens at a known time (`epoch_started_at + epoch_length`), so a user can stake just before the keeper's close and get full-epoch points for capital that was staked for seconds. They can then `request_unstake` just before the next close and withdraw right after it, so their capital is barely locked. Balances that change between the close and the engine's read can also be credited wrongly.

## 3. After a keeper outage, several epochs close at once using today's balances

Severity: Medium
Location: program/src/lib.rs:122
Mechanism: `epoch_started_at = ends_at` lets the keeper close every missed epoch in quick succession, and each `EpochClosed` reports the current `total_staked`. Anyone who staked during or right after an outage (which is visible on-chain) gets points for all the missed epochs, even though they weren't staked during them.

## 4. Unbonding delay can be close to zero

Severity: Low
Location: program/src/lib.rs:73
Mechanism: The request stores the current `pool.epoch`, so a request made just before the epoch closes can be withdrawn right after the close. The real unbonding period ranges from 0 to `epoch_length` depending on timing, which defeats any purpose of the lockup and makes item 2 worse.

## 5. Only one pending unstake request per user

Severity: Low
Location: program/src/lib.rs:170
Mechanism: The `Unbonding` PDA uses fixed seeds `[unbonding, pool, owner]` with `init`, so a second `request_unstake` fails until the first one is withdrawn. Users can't unstake more in the meantime, and if item 1 happens their request account (and its rent) is stuck too.

## 6. One keeper process handles every pool (scalability / operational single point of failure)

Severity: Low
Location: program/ARCHITECTURE.md:6
Mechanism: One team-run server signs every `close_epoch` with one hot operator wallet for every pool. Cost and failure risk grow with the number of pools, and that hot key is both the creator/authority of every pool and the only one that can unlock withdrawals.

## 7. Anyone can create pools, and nothing marks the official ones

Severity: Info
Location: program/src/lib.rs:129
Mechanism: `create_pool` has no access control. Any wallet can create a pool for any mint (including copies of real mints) under its own authority, and that pool emits valid `EpochClosed` events. The points engine and the web app have to filter by the operator authority, or fake pools could be credited or shown to users.

## 8. Stake mint is not checked for freeze authority or extensions

Severity: Info
Location: program/src/lib.rs:133
Mechanism: Any SPL mint is accepted. If the mint has a freeze authority, whoever holds it can freeze the vault and block every withdrawal. Pools should only be created for mints whose freeze authority has been revoked or is trusted.

## 9. Staker accounts can never be closed

Severity: Info
Location: program/src/lib.rs:143
Mechanism: There's no instruction to close a `Staker` account, so users can't get back its rent after leaving a pool.
