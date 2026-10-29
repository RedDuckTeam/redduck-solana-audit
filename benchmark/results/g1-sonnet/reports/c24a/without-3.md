# Solana audit: points_staking

Severity: High
Location: program/src/lib.rs:193
Mechanism: `close_epoch` can only be signed by `pool.authority`, and `withdraw` (line 81) requires `pool.epoch > request.epoch`. If the operator wallet is lost, compromised, or the keeper stops running, the epoch never advances and every pending unstake is locked in the vault permanently. The authority is stored once at creation with no rotation, and there is no permissionless fallback.

Severity: Medium
Location: program/src/lib.rs:106
Mechanism: Closing an epoch is a single-keeper, single-key job. The architecture has one hot operator wallet that signs for every pool (ARCHITECTURE.md:6), which is a single point of failure for liveness and key theft. It also scales poorly, because the keeper must send one transaction per pool per epoch and must be funded. `epoch_started_at` is set to `ends_at`, so after downtime the epochs have to be closed one at a time. Each closure emits a snapshot at the time of closing, not at the epoch's real end, so the points data for the missed epochs is inaccurate.

Severity: Medium
Location: program/src/lib.rs:39
Mechanism: Points are computed off-chain from balances at epoch close, and the program stores no time-weighted or per-epoch stake snapshot. The close time is predictable (`epoch_started_at + epoch_length`). An attacker can stake a large amount just before `close_epoch`, earn points for the whole epoch, then call `request_unstake` (line 62) right after. The unstake takes effect immediately for accounting and withdrawal needs only that close. This lets them dilute honest stakers' points with a flash stake.

Severity: Low
Location: program/src/lib.rs:170
Mechanism: The `Unbonding` PDA is seeded only by pool and owner, so a second `request_unstake` fails while a request exists, and a request cannot be topped up. A user with a pending request who needs to unstake more must wait for the epoch to close, then `withdraw`. If `close_epoch` stalls (see the first finding), this also blocks any further unstaking.

Severity: Low
Location: program/src/lib.rs:13
Mechanism: `create_pool` is permissionless for any mint and authority, and the pool PDA is derived from the authority and mint. Anyone can create a look-alike pool for the same mint with themselves as authority, so users and the points engine must trust the off-chain app to identify the official pool. The pool also cannot be bound to a canonical registry or config.

Severity: Info
Location: program/src/lib.rs:2
Mechanism: The program uses only the legacy SPL Token program (`Program<'info, Token>`). Token-2022 mints (transfer fees, etc.) are rejected, which is safe. If support is added later, `stake` credits the nominal `amount` and not the amount actually received, which would break vault accounting.
