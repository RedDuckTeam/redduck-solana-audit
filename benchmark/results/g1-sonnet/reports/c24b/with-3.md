# Solana audit report: points_staking

## Scope
`program/src/lib.rs` (Anchor, single program) and `program/ARCHITECTURE.md`. Instructions: create_pool, open_account, stake, request_unstake, withdraw, close_epoch. Pattern reference files were not opened; verdicts use the one-line rules. No toolchain run, so no exploit tests. No High or Critical findings, so the verifier agent was not called.

## Summary
0 Critical, 0 High, 3 Medium, 3 Low, 1 Info. Funds custody is sound: vault and pool are PDAs, and every transfer is checked against the pool and owner. The weak point is the points design. Rewards depend on end-of-epoch snapshots and an off-chain engine, and epoch closing has no catch-up rule.

## Findings

### F-1: Backlogged epochs can be closed back-to-back, so late stakers get credit for all of them

Severity: Medium
Location: program/src/lib.rs:137
Mechanism: `close_epoch` sets `epoch_started_at = ends_at` instead of `now`. After a keeper outage, anyone can close several epochs in one transaction, each emitting the same `total_staked`. An attacker can stake a large amount just before that batch, collect points for every backlogged epoch, then request unstake and withdraw after one more close.

Pattern: one-way-state
Scenario:
1. The keeper is down for k epochs and nobody else closes them.
2. The attacker calls `stake` with a large amount, then `close_epoch` k times in the same transaction.
3. The engine credits the attacker's balance for k epochs it was not staked for, which dilutes honest stakers.
Test: not attempted
Verification: self-checked, medium severity, no verifier required
Fix: Close at most one epoch per call and set `epoch_started_at = now`, or record a per-epoch snapshot of each staker's balance.

### F-2: Epoch snapshots keep no per-staker balance, so points cannot be computed fairly

Severity: Medium
Location: program/src/lib.rs:130
Mechanism: `EpochClosed` records only `total_staked`, and `Staker` keeps only its current `staked`. The off-chain engine (ARCHITECTURE.md:5) has to read balances at an arbitrary later time. A user can stake just before `close_epoch` and get full-epoch credit, or unstake after the close and be credited for an epoch using a balance they no longer hold.

Pattern: lazy-loading
Scenario:
1. The attacker stakes seconds before an epoch ends. They can also be the caller of `close_epoch`.
2. The snapshot includes their stake for the whole epoch, and they earn the same points as someone staked for 7 days.
Test: not attempted
Verification: self-checked, medium severity, no verifier required
Fix: Keep a time-weighted accumulator (`stake * seconds`) per pool and per staker, settled on every balance change, and emit it in the event.

### F-3: Unbonding tokens are withdrawable after the next close, which may be seconds away

Severity: Low
Location: program/src/lib.rs:96
Mechanism: A request made just before a close becomes withdrawable right after it, so the unbonding delay ranges from about 0 to one epoch. Tokens leave `total_staked` immediately at line 81. The delay therefore is not a reliable lock, and users can step out of the pool just before each snapshot.

Pattern: one-way-state
Scenario: The user calls `request_unstake` one block before the close, then `withdraw` right after it, and the stake was never counted at the snapshot.
Test: not attempted
Verification: self-checked
Fix: Require at least one full epoch to have elapsed since the request, for example `pool.epoch >= request.epoch + 2`.

### F-4: Only one pending unstake per user, and further unstakes are blocked until withdraw

Severity: Low
Location: program/src/lib.rs:191
Mechanism: The request PDA is seeded only by `[pool, owner]` and created with `init`. A second `request_unstake` fails until the first is withdrawn, so partial unstakes are blocked for up to an epoch (7 days on mainnet). The Unbonding account's lamports are also locked for that time.

Pattern: checklist-seed-collisions
Scenario: The user requests a small unstake and then cannot request a larger one until the next close.
Test: not attempted
Verification: self-checked
Fix: Add a per-user nonce to the seeds, or let the existing request be topped up.

### F-5: Permissionless pool creation allows look-alike pools, and mints with a freeze authority are accepted

Severity: Low
Location: program/src/lib.rs:150
Mechanism: Anyone can create a pool for any mint, and the seeds contain only the creator's key and the mint. An attacker can create a pool for the same mint, with a malicious authority and a short epoch, and lure users to it through a fake front end. A mint with a freeze authority can also freeze the vault token account, which locks all stakers (line 154 does not check `freeze_authority`).

Pattern: root-of-trust
Scenario: Users are tricked into staking in a pool that is not the team's. No program-level loss occurs unless the mint's freeze authority acts.
Test: not attempted
Verification: self-checked
Fix: Gate pool creation to a hardcoded admin or check that `stake_mint.freeze_authority` is none. Document the canonical pool address.

### F-6: No on-chain pool-level handling for a stale or absent keeper

Severity: Info
Location: program/src/lib.rs:140
Mechanism: The tip (0.0001 SOL per unstake request) is the only incentive to call `close_epoch`. A pool with no unstake requests pays nothing, and the epoch closes only when the team keeper runs. Withdrawals then stall behind the close at line 96.

Pattern: untrusted-executor
Scenario: Unstakers wait for a close in a quiet pool and nobody is paid to call it.
Test: not attempted
Verification: self-checked
Fix: Pay a small fixed reward from the pool or the team for each close, or let `withdraw` pass after a time limit without a close.

## Design findings

### D-1: Points and rewards depend entirely on an off-chain engine and a single team keeper

Severity: Medium
Location: program/ARCHITECTURE.md:5
Mechanism: Points are not stored on chain, and the engine trusts events and balances read off chain. Closing epochs relies on the team keeper (ARCHITECTURE.md:6), because the tip is the only other incentive. If the engine or keeper stops, epochs stop closing, users' unstake requests are not withdrawable (line 96), and points cannot be re-derived.

Pattern: dependency-independency
Scenario: The keeper stops. `withdraw` fails with `EpochNotClosed` until someone pays for `close_epoch` without a tip. Funds are not lost, but the exit is blocked until then.
Test: not attempted
Verification: self-checked
Fix: Allow `withdraw` once `now > request time + epoch_length` even if no close happened, and store points or an accumulator on chain.

### D-2: Pool authority is stored but never used and cannot be changed

Severity: Info
Location: program/src/lib.rs:22
Mechanism: `authority` only forms the PDA seeds. There is no admin function, no pause and no way to recover from a bad configuration, so `epoch_length` is fixed for the life of the pool.

Pattern: dependency-independency
Scenario: A pool made with a wrong epoch length cannot be fixed, and users must migrate.
Test: not attempted
Verification: self-checked
Fix: Document it as intentional, or add an authority-gated parameter update.

## Coverage

| Item | Verdict | Evidence |
|---|---|---|
| root-of-trust | found | F-5; pool, vault, staker PDAs at lib.rs:150-164, vault `address = pool.vault` at 177 |
| identity-rejection | n/a | no allow or deny lists |
| untrusted-executor | found | F-6, D-1; close_epoch is permissionless (lib.rs:217) |
| role-inversion | n/a | no operator-pushed data |
| lazy-loading | found | F-2 |
| pay-as-you-act | found | F-2 |
| divide-and-conquer | clear | per-user Staker and Unbonding PDAs (lib.rs:164, 191); the pool is written on every user action, but only for counters |
| complexity-asymmetry | clear | no loops |
| commit-execution-split | n/a | no randomness or previewable outcome |
| action-as-data | n/a | unstake requests hold only an amount and an owner |
| virtual-execution | n/a | the unstake request is settled by its owner only, at lib.rs:201 |
| one-way-state | found | F-1, F-3 |
| boundary-of-consent | n/a | no computed rates; amounts are 1:1 |
| internal-oracle | n/a | no derived prices |
| defensive-computation | n/a | no valuation or rounding |
| fail-fast | n/a | no external inputs |
| execution-layer-shift | n/a | no rollup or delegation |
| dependency-independency | found | D-1, D-2 |
| checklist-arithmetic | clear | checked_add and checked_sub at lib.rs:58, 60, 79, 81, 82, 126, 136 |
| checklist-duplicate-accounts | clear | no two mutable accounts of the same type are written; vault is fixed by address (177, 203) |
| checklist-unsafe-and-panics | clear | no unsafe code or unwrap |
| checklist-reload-after-cpi | clear | no state read after a CPI in withdraw or stake |
| checklist-token-2022 | clear | `Program<Token>` only (lib.rs:182) |
| checklist-closing | clear | request closed with `close = owner` at lib.rs:201, and the PDA cannot be revived within the transaction |
| checklist-seed-collisions | found | F-4; the prefixes pool, vault, staker and unbonding are distinct |
| checklist-init-frontrun | clear | PDAs are bound to the creator's or owner's key (150, 164, 191) |
| checklist-realloc | n/a | no realloc |
| checklist-authority-transfer | n/a | no authority transfer exists (D-2) |
| checklist-signers-bumps-cpi | clear | stored bump used for pool signer seeds (lib.rs:98-104); owner signs stake and withdraw (181, 208) |
