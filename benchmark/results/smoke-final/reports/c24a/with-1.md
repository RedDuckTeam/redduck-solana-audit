# Solana audit report: points_staking

## Scope
Single Anchor program `program/src/lib.rs` plus `program/ARCHITECTURE.md`. Instructions reviewed: create_pool, open_account, stake, request_unstake, withdraw, close_epoch. Off-chain points engine and keeper are out of scope, but their described behaviour was treated as part of the design.

## Summary
0 Critical, 0 High, 2 Medium, 2 Low, 1 Info. Account validation is sound: PDA seeds, `has_one`, `address = pool.vault` and checked arithmetic are all in place. The main issue is that unstake withdrawals depend on a single team-run keeper calling `close_epoch`.

## Findings

### F-1: Withdrawals depend on an authority-only close_epoch

Severity: Medium
Location: src/lib.rs:193
Mechanism: `close_epoch` is the only way to advance `pool.epoch` and only `pool.authority` can call it, while `withdraw` requires `pool.epoch > request.epoch` (lib.rs:81). If the keeper stops or withholds, or the operator key is lost, every pending unstake (already removed from `staked` and `total_staked`) stays locked, with no rotation or other exit.

Pattern: untrusted-executor
Scenario:
1. User calls `request_unstake` in epoch N.
2. The keeper goes offline or the operator wallet is lost.
3. `withdraw` fails with `EpochNotClosed` indefinitely.
Test: not attempted
Verification: downgraded by verifier from High: needs the team's own key to fail or withhold, the authority cannot steal funds, and the freeze ends when the keeper resumes; it is permanent only if the key is lost.
Fix: Make `close_epoch` permissionless once `now >= ends_at` (drop the authority check). The instruction only acts as a timer and event emitter.

### F-2: Unbonding does not impose a real delay or lock

Severity: Low
Location: src/lib.rs:81
Mechanism: A request can be withdrawn as soon as the current epoch closes, which can be seconds after the request, and stake has no minimum lock. Because the off-chain engine credits points from balances at close, a staker can deposit just before the predictable `close_epoch` call and earn a full epoch of points.

Pattern: boundary-of-consent
Scenario:
1. The keeper closes epochs on a known schedule.
2. The attacker stakes shortly before `epoch_started_at + epoch_length`.
3. The engine credits a full epoch, and the attacker unstakes in the next epoch.
Test: not attempted
Verification: self-checked; no High/Critical verifier needed
Fix: Have the engine use time-weighted balances, or enforce a minimum stake duration on chain.

## Design findings

### F-3: Single operator key with no rotation or recovery

Severity: Medium
Location: src/lib.rs:19
Mechanism: `pool.authority` is set once at creation and is both the PDA seed and the only `close_epoch` signer, so it cannot be rotated, multisig-replaced or recovered. One compromised or lost wallet halts epoch closing for the pool, and a compromised key can stall the points engine's input.

Pattern: dependency-independency
Scenario:
1. The operator wallet is lost or compromised.
2. No instruction can change `authority`.
3. The pool is stuck, and new pools need a new authority and mint pair.
Test: not attempted
Verification: self-checked; Medium, no verifier needed
Fix: Add a two-step authority transfer, and make `close_epoch` permissionless (see F-1).

### F-4: Only one pending unstake request per user

Severity: Low
Location: src/lib.rs:170
Mechanism: The `Unbonding` PDA is seeded only by pool and owner, so a second `request_unstake` fails until the first is withdrawn. Combined with F-1, a user cannot unstake further while epoch closing is stalled.

Pattern: divide-and-conquer
Scenario:
1. User requests an unstake and cannot withdraw it yet.
2. A second `request_unstake` fails because the PDA already exists.
Test: not attempted
Verification: self-checked
Fix: Seed requests with an index or the epoch, or allow topping up an existing request.

### F-5: Points depend on events only

Severity: Info
Location: src/lib.rs:115
Mechanism: Per-epoch totals exist only as an emitted event, not in account state. If RPC or indexer logs are lost, closed-epoch totals cannot be reconstructed on chain.

Pattern: dependency-independency
Scenario:
1. The indexer misses an `EpochClosed` event.
2. The totals for that epoch cannot be recovered from account state.
Test: not attempted
Verification: self-checked
Fix: Store a per-epoch snapshot account or a cumulative counter.

## Coverage

| Item | Verdict | Evidence |
|---|---|---|
| root-of-trust | clear | pool, staker and request PDAs are seeded (lib.rs:129,143,170); vault is checked with `address = pool.vault` (lib.rs:156,182); `has_one` ties the accounts together |
| identity-rejection | n/a | no allow or deny lists |
| untrusted-executor | found | F-1 |
| role-inversion | n/a | no operator-pushed data |
| lazy-loading | n/a | no accrual in the program; rewards are off chain |
| pay-as-you-act | clear | `total_staked` is updated on both stake and unstake (lib.rs:56,67) |
| divide-and-conquer | found | F-4 (per-user partitions otherwise fine) |
| complexity-asymmetry | n/a | no loops or user-sized data |
| commit-execution-split | n/a | no randomness or previewable outcome |
| action-as-data | clear | unbonding request is closed on withdraw (lib.rs:180), so it cannot run twice |
| virtual-execution | n/a | obligations are paid from the vault at withdraw |
| one-way-state | clear | epoch only increases (lib.rs:121); request is closed on withdraw (lib.rs:180) |
| boundary-of-consent | found | F-2 |
| internal-oracle | n/a | no derived prices |
| defensive-computation | n/a | no valuation or rounding |
| fail-fast | n/a | no external data |
| execution-layer-shift | n/a | no rollup or delegation |
| dependency-independency | found | F-3, F-5 |
| checklist-arithmetic | clear | checked_add/checked_sub throughout (lib.rs:54-67,110-121) |
| checklist-duplicate-accounts | clear | no instruction takes two mutable accounts of the same type that could alias |
| checklist-unsafe-and-panics | clear | no unsafe, unwrap or indexing |
| checklist-reload-after-cpi | clear | no account data is read after the token CPIs |
| checklist-token-2022 | clear | `Program<Token>` (lib.rs:136,161) accepts only the classic token program |
| checklist-closing | clear | `close = owner` on request (lib.rs:180) |
| checklist-seed-collisions | clear | distinct prefixes (pool, vault, staker, unbonding) with fixed-size key seeds |
| checklist-init-frontrun | clear | pool seeds include the authority (lib.rs:129), so no one else can take the address |
| checklist-realloc | n/a | no realloc |
| checklist-authority-transfer | found | F-3 |
| checklist-signers-bumps-cpi | clear | owner is a Signer for stake, unstake and withdraw; stored bump used for vault signer seeds (lib.rs:83-89) |
