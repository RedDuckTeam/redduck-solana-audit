# Solana audit report: points_staking

## Scope
`program/src/lib.rs` (Anchor, 6 instructions: create_pool, open_account, stake, request_unstake, withdraw, close_epoch) and `program/ARCHITECTURE.md`. No Solana toolchain was used, so nothing was run as an exploit test. No High or Critical findings, so the verifier agent was not called.

## Summary
0 Critical, 0 High, 2 Medium, 3 Low, 1 Info. The vault and token accounting hold up: checks are tight and arithmetic is checked. The main problems are the epoch design. The unbonding delay is not a real time delay, and the points snapshot can be timed by the caller.

## Findings

### F-1: Unbonding delay depends on the epoch counter, so it can be near zero

Severity: Medium
Location: src/lib.rs:96
Mechanism: `withdraw` only requires `pool.epoch > request.epoch`. A request made just before a due `close_epoch`, or while epochs are overdue after keeper downtime, becomes withdrawable at once, because `close_epoch` is permissionless and can be called repeatedly in one transaction while `now >= ends_at` (src/lib.rs:128, 137). The advertised unbonding period (up to one epoch) is therefore not enforced.

Pattern: one-way-state
Scenario:
1. The keeper is offline for two or more epochs, so `now >= epoch_started_at + 2*epoch_length`.
2. In one transaction the attacker calls `stake`, `close_epoch` (credits the stake in epoch N), `request_unstake`, `close_epoch` (N+1) and `withdraw`.
3. The attacker earns points for epoch N with no lock, and the stake is back in their wallet.
Test: not attempted
Fix: Record a timestamp or unlock epoch in the request (for example `unlock_epoch = epoch + 2`, or `unlock_at = now + epoch_length`). Limit `close_epoch` to one epoch advance per slot, or move `epoch_started_at` to `now`.

### F-2: Points snapshot is a spot value that the caller can time

Severity: Medium
Location: src/lib.rs:130
Mechanism: `EpochClosed` carries `total_staked` at the instant of closing. The engine also reads per-staker balances at some other time (ARCHITECTURE.md:5). Nothing is time-weighted or snapshotted per staker, so a staker can time stake and unstake around the close, which anyone may call, and get credit for a full epoch. Stake changes between the event and the engine's balance read also make the two inputs inconsistent.

Pattern: lazy-loading
Scenario:
1. The attacker stakes in the same transaction as the due `close_epoch`.
2. The event includes the new stake, and the balance read sees it.
3. The attacker earns a full epoch of points for a position held for a moment, and with F-1 also avoids the lock.
Test: not attempted
Fix: Keep a cumulative stake-seconds accumulator on the pool and a per-staker checkpoint (updated on every stake and unstake). Emit values the engine can verify, or store points on chain.

### F-3: Epoch closing is not tied to real time (backlog)

Severity: Low
Location: src/lib.rs:137
Mechanism: `epoch_started_at = ends_at` leaves a backlog after downtime. Each overdue epoch is then closable immediately and emits the same current `total_staked`. This distorts the per-epoch history the points engine relies on, and only the first closer gets the tip.

Pattern: one-way-state
Scenario: after a 3-epoch outage, three back-to-back `close_epoch` calls emit three events with identical totals, though the stake was not constant over that time.
Test: not attempted
Fix: Set `epoch_started_at = now`, or skip the missed epochs explicitly.

### F-4: Event-only output can be lost

Severity: Low
Location: src/lib.rs:130
Mechanism: `emit!` writes to program logs, which can be truncated and are not guaranteed to be kept by RPC nodes. The points engine depends on them as its source of truth, so a missed event means lost points for that epoch.

Pattern: fail-fast
Scenario: the log buffer is full, or the indexer misses the transaction, so the epoch never gets credited. Nothing on chain records the closed epoch's totals.
Test: not attempted
Fix: Use `emit_cpi!`, or store the totals per epoch in an account.

### F-5: Anyone can create look-alike pools; the mint is not restricted

Severity: Low
Location: src/lib.rs:150
Mechanism: The pool PDA is seeded by the creator and the mint, so anyone can create a pool for the team's token. Only the off-chain app and the engine decide which pool counts. A mint with a freeze authority can also freeze the vault and block withdrawals.

Pattern: root-of-trust
Scenario: an attacker creates a pool for the same mint with an authority they control and links users to it. Users who deposit there get no points.
Test: not attempted
Fix: Publish the canonical pool address, or require a fixed admin or config PDA. Document the mint's freeze-authority risk.

## Design findings

### D-1: One pending unstake per user, and withdrawal depends on a third party closing the epoch

Severity: Info
Location: src/lib.rs:191
Mechanism: The request PDA seeds are `[unbonding, pool, owner]`, so a second `request_unstake` fails until `withdraw` closes the first. Withdrawal also needs someone to call `close_epoch`. That works today because the tip pays the closer, and the team's keeper also does it. If tips are small compared with the fee, the incentive is thin, though a user can close the epoch themself.

Pattern: untrusted-executor
Test: not attempted
Fix: Add a request id to the seeds. Document that users can call `close_epoch` themselves.

## Coverage

| Item | Verdict | Evidence |
|---|---|---|
| root-of-trust | found | F-5; otherwise pool, vault and staker are checked by Account<T>, has_one and address = pool.vault (src/lib.rs:175-177, 201-203) |
| identity-rejection | n/a | no allow lists or per-wallet gates |
| untrusted-executor | clear | `close_epoch` is permissionless and tipped (src/lib.rs:121-143); see D-1 |
| role-inversion | n/a | no operator-pushed data |
| lazy-loading | found | F-2 |
| pay-as-you-act | found | F-2 |
| divide-and-conquer | clear | per-user Staker and Unbonding PDAs (src/lib.rs:164, 191); pool is the only shared account |
| complexity-asymmetry | n/a | no loops or searches |
| commit-execution-split | n/a | no randomness or previewable outcomes |
| action-as-data | n/a | no stored executable actions |
| virtual-execution | n/a | unbonding is an escrowed amount paid from the vault, not a virtual fill |
| one-way-state | found | F-1, F-3 |
| boundary-of-consent | n/a | no computed rates; amounts are exact |
| internal-oracle | n/a | no derived prices |
| defensive-computation | n/a | no valuation or rounding |
| fail-fast | found | F-4 (the other fail-fast checks are n/a: no oracles) |
| execution-layer-shift | n/a | no rollup or delegation |
| dependency-independency | clear | only SPL Token and System are used; the pool has no admin powers over funds (src/lib.rs:93-117) |
| checklist-arithmetic | clear | checked_add and checked_sub throughout (src/lib.rs:58-60, 79-82, 136) |
| checklist-duplicate-accounts | clear | no two mutable accounts of the same type are taken in one instruction |
| checklist-unsafe-and-panics | clear | no unsafe or unwrap |
| checklist-reload-after-cpi | clear | no reads of CPI-modified token balances after a CPI |
| checklist-token-2022 | clear | `Program<Token>` only (src/lib.rs:157, 182), so Token-2022 mints are rejected |
| checklist-closing | clear | `close = owner` with has_one on the request (src/lib.rs:201); the request PDA is re-initable only after close, which is intended |
| checklist-seed-collisions | clear | distinct prefixes pool, vault, staker and unbonding (src/lib.rs:150-191) |
| checklist-init-frontrun | clear | the pool PDA is seeded by its authority, so only that authority can create it. See F-5 for look-alike pools |
| checklist-realloc | n/a | no realloc |
| checklist-authority-transfer | n/a | the authority has no powers and cannot be transferred |
| checklist-signers-bumps-cpi | clear | the pool signs with stored seeds and bump (src/lib.rs:98-114); the owner signs stake and unstake |
