# Solana audit report: points_staking

## Scope
`program/src/lib.rs` and `program/ARCHITECTURE.md` (Anchor, 6 instructions: create_pool, open_account, stake, request_unstake, withdraw, close_epoch). The off-chain points engine and keeper are out of scope, but the design notes describe them.

## Summary
0 Critical, 0 High, 2 Medium, 3 Low, 1 Info. Custody of staked tokens looks sound: the vault is a PDA, every account chain ends at a pool PDA, and the arithmetic is checked. The findings that matter are in the epoch accounting that feeds the points engine (F-1, F-2). Both let a user collect points or skip the unbonding delay without taking the time-at-risk the design intends. No High or Critical findings, so no verifier calls were made.

## Findings

### F-1: Epoch backlog can be closed back-to-back, so epochs have no real duration

Severity: Medium
Location: program/src/lib.rs:137
Mechanism: `epoch_started_at` is set to the old `ends_at`, not to `now`, so after any gap of k epochs anyone can call `close_epoch` k times in a row, and each call emits an `EpochClosed` with the current `total_staked`. An attacker can stake, close the backlog in one transaction, and be credited k epochs of points. They can also call `request_unstake` and close one epoch to run `withdraw` at once, which skips the unbonding delay.

Pattern: one-way-state
Scenario:
1. The keeper is down for 3 weeks, or the pool is idle. With a 7-day epoch, 3 epochs are overdue.
2. The attacker stakes a large amount, then calls `close_epoch` three times in the same transaction.
3. The points engine sees 3 closed epochs with the attacker's balance. The attacker then calls `request_unstake` and one more `close_epoch`, then `withdraw`.
Test: not attempted
Verification: Medium, not verified (not required)
Fix: Allow only one close per elapsed `epoch_length` from the actual close time (`epoch_started_at = now`), or record the skipped epochs and have the engine weight them by time.

### F-2: Epoch totals are a snapshot at close time, and the closer controls the moment

Severity: Medium
Location: program/src/lib.rs:130
Mechanism: `EpochClosed` reports `total_staked` as it is when `close_epoch` runs, and the engine credits per-staker balances against it. Stake is not time-weighted, and `close_epoch` is permissionless, so a user can `stake` just before the close in the same transaction and `request_unstake` right after. They get full-epoch points for a few seconds of stake. A lock of up to one epoch applies only to the later withdrawal.
Pattern: commit-execution-split
Scenario:
1. The attacker bundles `stake(X)`, `close_epoch`, `request_unstake(X)`. The closer tip comes back to the attacker.
2. The event shows total_staked including X. The engine credits X's share of points for the whole epoch.
Test: not attempted
Verification: Medium, not verified (not required)
Fix: Accumulate a time-weighted stake (stake × seconds) on each stake or unstake and emit that, or record per-staker snapshots on chain.

### F-3: Stake in unbonding is held by the vault with no on-chain link to the points record

Severity: Low
Location: program/src/lib.rs:79
Mechanism: `request_unstake` removes `amount` from `staker.staked` and `pool.total_staked` immediately, in the middle of an epoch. The engine reads staker balances off chain at a time unrelated to the event, so honest users lose points for the whole epoch, and balance reads and event totals can disagree.
Pattern: lazy-loading
Test: not attempted
Verification: Low, not verified (not required)
Fix: Keep unstaking stake counted until the epoch closes, or track per-epoch weighted stake on chain.

### F-4: Staker accounts are never closed

Severity: Info
Location: program/src/lib.rs:34
Mechanism: There is no instruction to close an empty `Staker`, so each user's rent stays locked and the accounts accumulate. This is not exploitable.
Pattern: checklist-closing
Test: not attempted
Verification: not required
Fix: Add `close_account`, which requires `staked == 0`.

## Design findings

### F-5: Points, the reward itself, are entirely off chain and depend on the team's engine and keeper

Severity: Low
Location: program/src/lib.rs:130
Mechanism: The only on-chain output of an epoch is an event. If the team's indexer misses a log, which can be truncated or dropped, points for that epoch cannot be rebuilt from state, since only the current balances remain. Anyone can also create a pool for any mint (`program/src/lib.rs:150`), so the engine must filter by a trusted list of pools.
Pattern: dependency-independency
Test: not attempted
Verification: not required
Fix: Store per-epoch totals (or a cumulative index) in an account, and publish the list of trusted pools.

### F-6: The mint's freeze authority or the token program can lock the vault

Severity: Low
Location: program/src/lib.rs:154
Mechanism: `create_pool` accepts any SPL mint and does not check `freeze_authority`. A mint whose authority freezes the vault blocks every `withdraw`, and the program has no exit. Token-2022 mints are not supported, because `Program<Token>` rejects them. That is safe, but intended use should be documented.
Pattern: dependency-independency
Test: not attempted
Verification: not required
Fix: Require `stake_mint.freeze_authority.is_none()` at pool creation, or allow only an allow-listed mint.

## Coverage

| Item | Verdict | Evidence |
|---|---|---|
| root-of-trust | clear | Q: chain ends at pool PDA seeds, program/src/lib.rs:150; vault is `address = pool.vault` (177, 203); staker and request use `has_one` (175, 201) |
| identity-rejection | n/a | no allow lists or per-wallet rules besides the PDA keyed by owner |
| untrusted-executor | clear | Q: `close_epoch` is permissionless with a tip, lib.rs:121-143; the keeper is not required |
| role-inversion | n/a | no operator-pushed data |
| lazy-loading | found | F-3 |
| pay-as-you-act | clear | totals updated on stake (60) and on request_unstake (81) |
| divide-and-conquer | clear | per-user staker and request PDAs; the pool is written by every user but the size is fixed |
| complexity-asymmetry | n/a | no loops or searches |
| commit-execution-split | found | F-2 |
| action-as-data | n/a | no stored executable actions; the unbonding request is only amount data |
| virtual-execution | clear | Q: the unbonding request is closed on withdraw (201), so it cannot be settled twice |
| one-way-state | found | F-1 |
| boundary-of-consent | n/a | no computed rates; amounts are exact |
| internal-oracle | n/a | no price derived from state |
| defensive-computation | n/a | no valuation or rounding |
| fail-fast | n/a | no external inputs |
| execution-layer-shift | n/a | no rollup or delegation |
| dependency-independency | found | F-5, F-6 |
| checklist-arithmetic | clear | checked_add / checked_sub at lib.rs:58, 60, 79, 81, 82, 126, 136 |
| checklist-duplicate-accounts | clear | no two mutable accounts of the same type are mixed in one instruction; the vault is tied to the pool by address |
| checklist-unsafe-and-panics | clear | no unsafe, unwrap or indexing |
| checklist-reload-after-cpi | clear | no account data is read after a CPI that changes it |
| checklist-token-2022 | clear | `Program<Token>` only (lib.rs:157, 182); see F-6 |
| checklist-closing | found | F-4; the request is closed correctly at 201 |
| checklist-seed-collisions | clear | distinct prefixes pool/vault/staker/unbonding with fixed-size key seeds |
| checklist-init-frontrun | clear | PDAs keyed by the creator's key; nobody else can occupy them |
| checklist-realloc | n/a | no realloc |
| checklist-authority-transfer | n/a | `authority` has no powers after creation |
| checklist-signers-bumps-cpi | clear | owner is a Signer on stake/withdraw; the stored bump is used for the pool signer at lib.rs:98 |
