# Solana audit report: points_staking

## Scope
Single Anchor 0.31.1 program in `program/` (`src/lib.rs`, `ARCHITECTURE.md`). Instructions: create_pool, open_account, stake, request_unstake, withdraw, close_epoch. The off-chain keeper and points engine are out of scope, except for what ARCHITECTURE.md says about them. No Solana toolchain was used, so nothing was tested.

## Summary
0 Critical, 0 High, 2 Medium, 3 Low, 0 Info.
The one that matters most is F-1. `close_epoch` is the only way to advance the epoch, and only one team key can call it. Every withdrawal depends on that key. Account validation (PDAs, `has_one`, vault address, mint) holds up.

## Findings

### F-2: Points snapshot can be gamed by staking just before close and unstaking right after

Severity: Medium
Location: src/lib.rs:62
Mechanism: Points come from a single snapshot at `close_epoch` (lib.rs:115-120) with no time weighting. The keeper closes at a predictable time (`epoch_started_at + epoch_length`, ARCHITECTURE.md:6), so a user can stake just before it and call `request_unstake` right after, getting full-epoch points for a few seconds of stake.

Pattern: lazy-loading / pay-as-you-act
Scenario:
1. Attacker stakes a large amount shortly before the epoch ends.
2. The keeper closes the epoch, so `total_staked` includes the attacker's stake.
3. Attacker calls `request_unstake` in the new epoch and withdraws after the next close. Their balance counted at the snapshot but was not held through the epoch, which dilutes honest stakers' points.
Test: not attempted
Verification: not required (Medium)
Fix: Accrue time-weighted stake on chain (a per-staker last-update marker plus a global accumulator), or add a minimum stake duration.

### F-3: Event total and staker balances can disagree for the points engine

Severity: Low
Location: src/lib.rs:118
Mechanism: `EpochClosed` carries only `total_staked`. The engine also reads per-staker balances (ARCHITECTURE.md:5) at some other moment, and `staked` changes with every stake and unstake. The sum of those balances can differ from the emitted total, so the engine can misallocate points.

Pattern: one-way-state
Scenario:
1. A user calls `stake` or `request_unstake` between the close and the engine's read.
2. The engine sees a balance that was not part of the snapshot.
Test: not attempted
Verification: not required (Low)
Fix: Snapshot per-staker balances on chain per epoch, or have the engine read account state at the close slot.

### F-4: Staker accounts can never be closed, so rent stays locked

Severity: Low
Location: src/lib.rs:154
Mechanism: There is no instruction to close a `Staker` account (or the Pool), so the rent paid in `open_account` is never returned to the user.

Pattern: checklist-closing
Scenario:
1. A user opens an account, stakes and fully withdraws.
2. The empty `Staker` account stays and its rent cannot be reclaimed.
Test: not attempted
Verification: not required (Low)
Fix: Add `close_account` that requires `staked == 0` and uses `close = owner`.

## Design findings

### F-1: Only the pool authority can close epochs, and withdrawals depend on epoch closing

Severity: Medium
Location: src/lib.rs:193
Mechanism: `close_epoch` requires `has_one = authority` and the authority's signature. It is the only instruction that advances `pool.epoch`, and `withdraw` requires `pool.epoch > request.epoch` (lib.rs:81). If the single team wallet or its keeper goes offline, is lost or turns hostile, no unstake request can be paid. The program has no authority-rotation instruction.

Pattern: untrusted-executor
Scenario:
1. A user stakes and calls `request_unstake` in epoch N.
2. The operator key is lost or the keeper stops (ARCHITECTURE.md:6).
3. `close_epoch` can no longer be called, so `withdraw` always fails with `EpochNotClosed`. The user's tokens stay in the vault.
Test: unconfirmed (no toolchain)
Verification: downgraded by verifier from High: an ordinary user cannot trigger it, a temporary outage heals because `epoch_started_at = ends_at` (lib.rs:122) allows catch-up calls, and a hostile authority can withhold epoch closes but cannot steal funds. A permanent freeze remains only if the key is lost for good.
Fix: Make `close_epoch` permissionless, since the time check at lib.rs:113 already gates it. If the authority is kept, add authority rotation. Optionally let the keeper be paid per close.

### F-5: Keeper catch-up can emit back-to-back epoch closes with identical totals

Severity: Low
Location: src/lib.rs:122
Mechanism: After downtime, several `close_epoch` calls can run in one block, because each sets `epoch_started_at = ends_at` and only needs `now >= ends_at`. They emit several `EpochClosed` events with the same `total_staked` and nearly the same `closed_at`. The authority can use this to credit points for many epochs from one instant.

Pattern: fail-fast
Scenario:
1. The keeper is down for 3 weeks.
2. The authority calls `close_epoch` three times in a row.
3. The points engine credits three full epochs to whoever is staked at that moment.
Test: not attempted
Verification: not required (Low)
Fix: Have the engine account for elapsed time using `closed_at` and the epoch boundaries, or skip the missed epochs on chain.

## Coverage

| Item | Verdict | Evidence |
|---|---|---|
| root-of-trust | clear | Chains end at pool PDA seeds (lib.rs:129), vault `address = pool.vault` (lib.rs:156, 182), and `has_one` on staker and request (lib.rs:154, 168, 180) |
| identity-rejection | n/a | No allow lists or per-wallet rules |
| untrusted-executor | found | F-1 |
| role-inversion | n/a | Operator pushes no data to the chain |
| lazy-loading | found | F-2 |
| pay-as-you-act | found | F-2 |
| divide-and-conquer | clear | Staker and Unbonding are per-user PDAs (lib.rs:143, 170). `Pool` is written on every stake but holds only a counter |
| complexity-asymmetry | clear | No loops over user data |
| commit-execution-split | n/a | No randomness or outcome preview |
| action-as-data | clear | Unbonding request is closed on withdraw (lib.rs:180), so it cannot run twice |
| virtual-execution | n/a | Withdraw pays the recorded amount directly to its owner |
| one-way-state | found | F-3 |
| boundary-of-consent | n/a | 1:1 token amounts, no computed rates |
| internal-oracle | n/a | No prices derived from state |
| defensive-computation | n/a | No valuation or rounding |
| fail-fast | found | F-5 |
| execution-layer-shift | n/a | No rollup or delegation |
| dependency-independency | found | F-1 (single authority key, no rotation) |
| checklist-arithmetic | clear | `checked_add` / `checked_sub` at lib.rs:54, 56, 65, 67, 112, 121 |
| checklist-duplicate-accounts | clear | No two mutable accounts of the same type are accepted in one instruction |
| checklist-unsafe-and-panics | clear | No `unsafe`, `unwrap` or indexing |
| checklist-reload-after-cpi | clear | No account data is read after the token CPIs |
| checklist-token-2022 | clear | `Program<Token>` pins the classic token program (lib.rs:136, 161, 188) |
| checklist-closing | found | F-4 |
| checklist-seed-collisions | clear | Distinct seed prefixes `pool`, `vault`, `staker`, `unbonding` |
| checklist-init-frontrun | clear | Pool PDA is seeded by authority, so an attacker cannot front-run another authority's pool |
| checklist-realloc | n/a | No realloc |
| checklist-authority-transfer | found | F-1 (no rotation) |
| checklist-signers-bumps-cpi | clear | Stored `pool.bump` is used for signer seeds (lib.rs:83-89). `owner` is a Signer in stake, unstake and withdraw |
