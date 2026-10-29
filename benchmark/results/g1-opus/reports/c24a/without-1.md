# Security Audit: points_staking (program/src/lib.rs)

Scope: `program/src/lib.rs`, with deployment assumptions from `program/ARCHITECTURE.md`.

## Findings

### 1. Withdrawals depend on a privileged, centralized keeper (liveness / custody risk)
Severity: High
Location: program/src/lib.rs:193
Mechanism: `close_epoch` can only be signed by `pool.authority` (`has_one = authority`), and `withdraw` needs `pool.epoch > request.epoch`. So every user's unstaked tokens stay locked until the team's single keeper advances the epoch. If the keeper server goes down, the fee wallet runs dry, or the operator key is lost, all withdrawals freeze. The operator can also refuse to close epochs and hold user funds hostage or censor exits. Nothing lets anyone else close an epoch that has already expired.

### 2. No way to rotate or recover the pool authority
Severity: Medium
Location: program/src/lib.rs:129
Mechanism: The pool PDA is derived from the operator wallet and `authority` is fixed for good, with no instruction to transfer or rotate it. If that hot keeper key is lost or compromised, nobody can recover the pool: funds stay frozen (finding 1) or an attacker controls epoch progression. Because the operator wallet is part of the PDA seeds, a replacement pool has a different address, and every user would have to migrate by hand.

### 3. Points snapshot is point-in-time, not time-weighted (just-in-time staking)
Severity: Medium
Location: program/src/lib.rs:115
Mechanism: `EpochClosed` only emits `total_staked`. The off-chain engine then credits each epoch from current `Staker.staked` balances, which hold no history and no stake timestamp. Someone who stakes seconds before `close_epoch` (or after it, before the engine reads balances) earns the same points as someone staked for the whole epoch. That person can then leave by calling `request_unstake` right before the next close, so a whole epoch's points cost only about one epoch of lockup, timed to the keeper.

### 4. Epoch backlog lets late stakers collect points for epochs they missed
Severity: Medium
Location: program/src/lib.rs:122
Mechanism: `close_epoch` sets `epoch_started_at = ends_at` instead of `now`. After keeper downtime spanning k epochs, k closes can be called back-to-back right away, each emitting the current balances. Anyone who staked during the outage gets points for every missed epoch. Pending unbonding requests also all unlock with the first catch-up close.

### 5. Unbonding delay is anywhere from about 0 to one full epoch
Severity: Low
Location: program/src/lib.rs:73
Mechanism: A request is tagged with the current epoch and can be withdrawn as soon as that epoch is closed. A request sent just before `close_epoch` is withdrawable almost immediately, so the unbonding period is not a real lockup. It mainly rewards users who time their actions to the keeper's schedule (see finding 3).

### 6. Only one pending unstake request per user
Severity: Low
Location: program/src/lib.rs:170
Mechanism: The `Unbonding` PDA is seeded only by `(pool, owner)` and created with `init`. A user with a pending request cannot unstake more until they withdraw, which can mean waiting up to a full epoch (7 days on mainnet), or forever if epochs stall (finding 1). Requests also cannot be cancelled or topped up.

### 7. Staker accounts can never be closed
Severity: Info
Location: program/src/lib.rs:143
Mechanism: No instruction closes a `Staker` account, so its rent is locked for good even after the user has fully exited. Accounts also pile up indefinitely, and the off-chain points engine has to scan all of them every epoch.
