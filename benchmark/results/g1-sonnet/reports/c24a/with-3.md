# Solana audit report: points_staking (Anchor 0.31.1)

## Findings

### F1
Severity: High
Location: program/src/lib.rs:193
Mechanism: `close_epoch` can only be called by `pool.authority` (a single operator wallet run by one keeper), and `withdraw` (lib.rs:81) needs `pool.epoch > request.epoch`, which only `close_epoch` advances (lib.rs:121); if the key is lost or the keeper stops (or the operator withholds), every pending unstake and so all staked funds are frozen forever, with no authority rotation or permissionless fallback.
Verification: confirmed by finding-verifier (could be argued Medium because it needs failure of a privileged key).

### F2
Severity: Medium
Location: program/src/lib.rs:106
Mechanism: The points engine uses `total_staked` and balances at close time as a point-in-time snapshot with no time weighting. Anyone can watch the public epoch end, `stake` just before the keeper's `close_epoch`, earn full-epoch points, then `request_unstake` right after, so points can be farmed with short-lived or borrowed capital. A late keeper widens the window.

### F3
Severity: Low
Location: program/src/lib.rs:170
Mechanism: The unbonding PDA is seeded only by (pool, owner), so a user can have just one pending request. They cannot unstake more until the first is withdrawn, which itself depends on F1's epoch close. Partial exits are blocked for up to an epoch (7 days on mainnet).

### F4
Severity: Low
Location: program/src/lib.rs:129
Mechanism: Pools are created permissionlessly and are seeded by (authority, mint), and nothing marks which pool is official. An attacker can create a look-alike pool with a malicious mint or authority and lure users who trust a UI. The stake mint's freeze authority is also not checked (lib.rs:133), so a freezable mint lets its issuer freeze the vault and block withdrawals.

### F5
Severity: Info
Location: program/src/lib.rs:140
Mechanism: `Staker` accounts can never be closed and `epoch_length`/`authority` can never be changed, so rent is locked and the pool cannot be reconfigured or handed over (this also feeds F1).

## Design findings

- Liveness / centralization: one operator key and one self-hosted keeper control epoch progress and so all withdrawals (F1). Recommended fix: let anyone call `close_epoch` once `epoch_length` has passed (it is only a time check and an event), and add authority rotation.
- Off-chain trust: points exist only in off-chain event data, so the points engine is a trusted dependency, and a missed `EpochClosed` event cannot be recovered on chain.

## Coverage

| Item | Verdict |
|---|---|
| root-of-trust | clear (pool PDA seeds :129; `has_one`, `address = pool.vault` :154-157) |
| identity-rejection | n/a (no allow lists) |
| untrusted-executor | found (F1) |
| role-inversion | n/a (no pushed data) |
| lazy-loading | n/a (no accrual on chain) |
| pay-as-you-act | clear (totals updated on stake/unstake :54-67) |
| divide-and-conquer | clear (per-user Staker/Unbonding PDAs) |
| complexity-asymmetry | clear (no loops) |
| commit-execution-split | n/a |
| action-as-data | n/a |
| virtual-execution | n/a (unbonding requests settle once, then the PDA closes :180) |
| one-way-state | clear (epoch only increments :121; request PDA closed on withdraw) |
| boundary-of-consent | n/a (1:1 amounts, no rates) |
| internal-oracle | found (F2, snapshot manipulable) |
| defensive-computation | n/a |
| fail-fast | clear (:81, :113) |
| execution-layer-shift | n/a |
| dependency-independency | found (F1) |
| checklist-arithmetic | clear (checked ops :54-67, :112, :121) |
| checklist-duplicate-accounts | clear (no mutable same-type pairs) |
| checklist-unsafe-and-panics | clear |
| checklist-reload-after-cpi | clear (no reads after CPI) |
| checklist-token-2022 | clear (`Program<Token>` only, classic SPL) |
| checklist-closing | clear (`close = owner` :180) |
| checklist-seed-collisions | clear (distinct prefixes) |
| checklist-init-frontrun | found (F4, pool creation is permissionless) |
| checklist-realloc | n/a |
| checklist-authority-transfer | found (F1/F5, no rotation) |
| checklist-signers-bumps-cpi | clear (bump stored, `Program<Token>` :161) |
