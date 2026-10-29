# Solana audit report: points_staking

## Scope
`program/src/lib.rs` (Anchor, single program). Instructions: create_pool, open_account, stake, request_unstake, withdraw, close_epoch. `ARCHITECTURE.md` read for deployment facts (operator wallet, team keeper, off-chain points engine). No toolchain run; no exploit tests.

## Summary
0 Critical, 0 High, 2 Medium, 3 Low, 1 Info. Account validation, vault custody and arithmetic are sound. The main problems are the operator-only `close_epoch`, which gates all withdrawals, and snapshot-based points that can be gamed.

## Findings

### F-1: Points snapshot at epoch close can be gamed (no time-weighting)

Severity: Medium
Location: src/lib.rs:115
Mechanism: Points are credited from the stake balance at the moment of `close_epoch`, and `stake`/`request_unstake` change balances instantly. Because the close time is predictable (`epoch_started_at + epoch_length`), an attacker can stake a large amount just before the close and request unstake right after, earning a full epoch of points for seconds of exposure.

Pattern: lazy-loading
Scenario:
1. Attacker watches for the epoch end time and the keeper's close.
2. Stakes a large amount shortly before `close_epoch`.
3. After the close, calls `request_unstake`, and withdraws after the next close.
4. The points engine credits a full epoch for the snapshot balance.
Test: not attempted
Fix: Track time-weighted stake on chain (running accumulator plus per-staker marker), or have the points engine use per-staker deposit timestamps.

### F-2: Loss of liveness: only the operator can close epochs, and withdrawals depend on it

Severity: Medium
Location: src/lib.rs:193
Mechanism: `close_epoch` is the only instruction that advances `pool.epoch`, and it requires the pool authority to sign, while `withdraw` requires `pool.epoch > request.epoch` (src/lib.rs:81). If the operator key is lost or the keeper stops, pending unstake requests stay locked in the vault. The operator can also hold them back on purpose. The authority cannot be rotated.

Pattern: untrusted-executor
Scenario:
1. Users call `request_unstake` in epoch N; tokens leave `staker.staked`.
2. The keeper goes down or the key is lost.
3. Nobody else can call `close_epoch`, so `withdraw` fails with `EpochNotClosed`.
Test: not attempted
Verification: downgraded by verifier from High: no ordinary user can trigger it, and a restored keeper unlocks everything. It is permanent only if the key is irrecoverably lost.
Fix: Make `close_epoch` permissionless once `now >= ends_at` (the event content is deterministic), and/or add authority transfer.

### F-3: Backlogged epochs close with the same totals

Severity: Low
Location: src/lib.rs:122
Mechanism: `epoch_started_at = ends_at` lets a late keeper close several epochs back-to-back, each emitting the current `total_staked`, so the points engine credits every missed epoch with a balance that may not have existed during them.

Pattern: one-way-state
Test: not attempted
Fix: Close at most one epoch per call and record real close times, or snapshot stake on chain.

### F-4: Single pending unstake request per user

Severity: Low
Location: src/lib.rs:170
Mechanism: The `Unbonding` PDA is seeded only by pool and owner, so a user with a pending request cannot make another until they withdraw. Combined with F-2, they cannot unstake any more of their stake while the operator is down.

Pattern: divide-and-conquer
Test: not attempted
Fix: Add a request index to the seeds.

### F-5: Variable and unbounded unbonding delay

Severity: Low
Location: src/lib.rs:81
Mechanism: The delay depends on when in the epoch the request is made and on the keeper closing promptly, so it ranges from near zero to over `epoch_length` (7 days on mainnet). Users get no bound.

Pattern: fail-fast
Test: not attempted
Fix: Store an explicit `unlock_at` in the request.

### F-6: Staker accounts are never closed

Severity: Info
Location: src/lib.rs:30
Mechanism: `open_account` creates a `Staker` PDA with no close path, so the rent stays locked after a user fully exits.

Pattern: checklist-closing
Test: not attempted
Fix: Add `close_account` requiring `staked == 0`.

## Design findings

F-2 (single operator key gating all withdrawals; keeper run on one team server) is the main design finding. The authority also cannot be rotated: the pool PDA seeds include the authority (src/lib.rs:129) and no instruction changes it.

## Coverage

| Item | Verdict | Evidence |
|---|---|---|
| root-of-trust | clear | pool PDA seeds checked at src/lib.rs:129; vault bound by `address = pool.vault` at 156/182; staker/request by `has_one` |
| identity-rejection | n/a | no allow/deny lists or per-wallet rules |
| untrusted-executor | found | F-2 |
| role-inversion | n/a | operator pushes no data on chain |
| lazy-loading | found | F-1 |
| pay-as-you-act | clear | no iteration; aggregates updated in stake and request_unstake (54-67) |
| divide-and-conquer | found | F-4 (per-user partitions otherwise; pool write is a single counter) |
| complexity-asymmetry | n/a | no loops or user-sized data |
| commit-execution-split | n/a | no randomness or previewable outcomes |
| action-as-data | n/a | no stored executable actions |
| virtual-execution | n/a | unbonding paid in full from vault; amounts backed by deposits |
| one-way-state | found | F-3 |
| boundary-of-consent | n/a | 1:1 amounts, no computed rates |
| internal-oracle | n/a | no derived prices |
| defensive-computation | n/a | no valuation or rounding |
| fail-fast | found | F-5 |
| execution-layer-shift | n/a | no rollup or delegation |
| dependency-independency | found | F-2 (operator key/keeper is the dependency, no exit) |
| checklist-arithmetic | clear | checked_add/sub at src/lib.rs:54,56,65,67,112,121 |
| checklist-duplicate-accounts | clear | no two mutable same-type accounts that could alias |
| checklist-unsafe-and-panics | clear | no unsafe, unwrap or indexing |
| checklist-reload-after-cpi | clear | no state reads after CPI that depend on it |
| checklist-token-2022 | clear | `Program<Token>` and `Account<Mint>` accept only the SPL Token program |
| checklist-closing | found | F-6; request closed with `close = owner` at src/lib.rs:180 |
| checklist-seed-collisions | clear | distinct prefixes pool/vault/staker/unbonding |
| checklist-init-frontrun | clear | pool seeds include the authority signer (129) |
| checklist-realloc | n/a | no realloc |
| checklist-authority-transfer | found | F-2 (no transfer instruction) |
| checklist-signers-bumps-cpi | clear | pool signs with stored bump (83-89); owner signs transfers |
