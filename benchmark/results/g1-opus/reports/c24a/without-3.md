# Security Audit: points_staking (./program)

Scope: `program/src/lib.rs`, `program/ARCHITECTURE.md`, `program/Cargo.toml` (Anchor 0.31.1).

## Findings

### 1. Withdrawals depend on a single permissioned keeper (liveness / fund lock)
Severity: High
Location: program/src/lib.rs:193
Mechanism: `close_epoch` can only be called by the pool `authority` (`has_one = authority` + `Signer`), and `withdraw` requires `pool.epoch > request.epoch`. If the team's single keeper server goes down, its fee wallet runs dry, or the operator key is lost, no epoch ever closes and all unstaked tokens stay locked in the vault indefinitely. No permissionless fallback exists. Because the epoch timing is already enforced on-chain (`now >= ends_at`), `close_epoch` could safely be permissionless.

### 2. Operator can freeze or censor user withdrawals (centralization)
Severity: High
Location: program/src/lib.rs:106
Mechanism: The operator wallet alone decides whether and when epochs close. A malicious, coerced or compromised operator can stop calling `close_epoch` and hold every pending unstake hostage. The operator can also delay closing selectively to manipulate the points snapshots. Users have no on-chain way to exit without the operator.

### 3. Pool authority is immutable and baked into PDA seeds (no key rotation)
Severity: Medium
Location: program/src/lib.rs:129
Mechanism: `authority` is part of the pool and vault PDA seeds and no instruction exists to change it. A leaked or lost operator key therefore can't be rotated. Combined with findings 1 and 2, losing the key permanently bricks withdrawals for that pool, and leaking it hands an attacker permanent control over epoch closing.

### 4. Hot operator key on keeper server
Severity: Medium
Location: program/ARCHITECTURE.md:6
Mechanism: The same wallet that owns the pools (the `authority`) has to sit on an internet-facing keeper server so it can sign `close_epoch` automatically. Compromising that server gives an attacker the irrevocable pool authority (see finding 3). The privileged authority and the routine cranking role aren't separated.

### 5. Points snapshot is not taken on-chain and is not time-weighted (points gaming)
Severity: Medium
Location: program/src/lib.rs:115
Mechanism: `EpochClosed` only emits the pool total. The off-chain engine reads per-staker balances separately, so balances are not snapshotted at the moment the epoch closes and no stake-duration accounting exists. A user can stake moments before `close_epoch` and get credited for the full epoch. Balance changes between the close and the engine's read also shift points between users or make them inconsistent with `total_staked`.

### 6. Catch-up epoch closes credit late stakers for missed epochs
Severity: Medium
Location: program/src/lib.rs:122
Mechanism: `epoch_started_at = ends_at` means that after a keeper outage several epochs can be closed back-to-back in one burst, each emitting the *current* `total_staked`. Anyone who stakes during the outage, right before the catch-up closes, gets points for every missed epoch even though they weren't staked during them.

### 7. Variable unbonding period allows near-instant exit
Severity: Low
Location: program/src/lib.rs:73
Mechanism: The unbonding lock is "until the current epoch closes", so it can be anything from 7 days down to a few seconds. A request made just before `close_epoch` can be withdrawn almost immediately. The intended lockup can be bypassed by timing, which, combined with finding 5, makes just-in-time staking cheap.

### 8. Off-chain points engine relies on transaction logs and full-account scans (scalability / data availability)
Severity: Low
Location: program/src/lib.rs:115
Mechanism: `emit!` writes to program logs. RPC nodes can truncate logs and don't keep them forever, so missed `EpochClosed` events lead to wrong or missing points. Reading "all staker balances" each epoch means a `getProgramAccounts` scan that grows linearly with users and isn't atomic with the close. `emit_cpi!` and on-chain snapshots would be more robust.

### 9. One pending unstake request per user blocks further unstaking
Severity: Low
Location: program/src/lib.rs:170
Mechanism: The `Unbonding` PDA is seeded only by pool and owner and created with `init`. While a request is pending, any further `request_unstake` fails, so a user who wants to unstake more has to wait a full epoch cycle per request. This is worst when the keeper is down (finding 1).

### 10. Accounts can never be closed (rent lock)
Severity: Info
Location: program/src/lib.rs:141
Mechanism: No instruction closes `Staker` or `Pool` accounts, so their rent deposits stay locked forever even when `staked == 0`. State also grows without bound, one account per user per pool.
