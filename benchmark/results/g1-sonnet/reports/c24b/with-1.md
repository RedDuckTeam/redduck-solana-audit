# Solana audit report: points_staking

## Scope
`program/src/lib.rs` (Anchor, single program, 5 instructions: create_pool, open_account, stake, request_unstake, withdraw, close_epoch) and `program/ARCHITECTURE.md`. Off-chain points engine and keeper are out of scope, but treated as part of the design. No Solana toolchain was used, so nothing was run as a test.

## Summary
0 Critical, 0 High, 1 Medium, 3 Low, 2 Info. Funds handling is sound: the vault is a PDA owned by the pool PDA, and every chain of checks ends at a PDA or a signer. The main issue is that the on-chain data behind the points system (the epoch snapshot and the variable unbonding delay) is easy to game.

## Findings

### F-1: Epoch snapshot is a single total taken at an unpredictable moment, with no per-staker snapshot

Severity: Medium
Location: src/lib.rs:130
Mechanism: `close_epoch` emits only `total_staked` at the moment someone calls it, and `Staker.staked` is not snapshotted per epoch (src/lib.rs:57-58). A user can stake right before the close (and anyone can time the close) to be counted for a whole epoch of points, and the off-chain engine that reads current staker balances cannot tell what each staker held when the epoch closed.

Pattern: lazy-loading
Scenario:
1. Epoch ends at T. The attacker stakes a large amount at T+ε, and calls `close_epoch` in the same transaction or right after it.
2. The event carries a `total_staked` that includes the attacker. The attacker's balance is read as large.
3. The attacker requests unstake and waits for the following close (up to one epoch), or keeps the position open only around closes, and collects points for an epoch they did not stake through.
Test: not attempted
Verification: not required (Medium)
Fix: Keep a per-epoch checkpoint (a cumulative stake-time accumulator, updated in `stake`, `request_unstake` and `close_epoch`), and have the engine read that instead of current balances. Alternatively, count a stake only from the next epoch.

### F-2: Unbonding delay varies from seconds to a full epoch

Severity: Low
Location: src/lib.rs:96
Mechanism: `withdraw` only requires `pool.epoch > request.epoch`, so a request made just before `close_epoch` can be withdrawn seconds later, while one made just after waits a full 7-day epoch. Because `request_unstake` also removes the stake from `total_staked` immediately (src/lib.rs:81), users who watch the clock can exit without a real lock, and the cooldown does not mean what the docs imply.

Pattern: one-way-state
Scenario:
1. A staker requests unstake in the last minutes of an epoch.
2. Anyone calls `close_epoch` once it has ended.
3. The staker withdraws immediately.
Test: not attempted
Verification: not required (Low)
Fix: Record `request.available_epoch = pool.epoch + 1` for requests made after a minimum point, or define a fixed unbonding time in seconds.

### F-3: Pools are permissionless and look the same as the team's pools

Severity: Low
Location: src/lib.rs:150
Mechanism: Anyone can call `create_pool` with their own `authority` for any mint, and `close_epoch` is public, so an attacker can create pools for the same mint that emit genuine-looking `EpochClosed` events. An off-chain engine or UI that does not filter on the exact pool address (or authority) may credit points or show balances for the wrong pool.

Pattern: root-of-trust
Scenario:
1. Attacker creates a pool for the team's stake mint with the attacker as authority.
2. Attacker stakes, closes epochs and emits events with chosen totals.
3. An engine that accepts events by program id or mint credits points from that pool.
Test: not attempted
Verification: not required (Low)
Fix: Make the engine and the UI use a hardcoded allow list of pool addresses. On-chain, an optional fix is a global config PDA that records the official operator, with `create_pool` requiring it.

### F-4: Only one unbonding request per user, with no cancel or top-up

Severity: Low
Location: src/lib.rs:191
Mechanism: The `Unbonding` PDA uses fixed seeds [unbonding, pool, owner] with `init`, so a second `request_unstake` fails until the first is withdrawn, and there is no `cancel_unstake`. A user with a pending request cannot unstake more or restake the pending amount, and if closes stall they cannot change anything.

Pattern: action-as-data
Scenario:
1. User requests unstake for 1 token.
2. User wants to unstake the rest and the call fails with an account-already-in-use error until the next close and a withdraw.
Test: not attempted
Verification: not required (Low)
Fix: Add a request id or nonce to the seeds, or allow topping up the existing request. Add `cancel_unstake`.

### F-5: Mint freeze authority and Token-2022 are not handled

Severity: Info
Location: src/lib.rs:154
Mechanism: `stake_mint` is accepted with any freeze authority, so the mint's freeze authority can freeze the vault and lock every staker. Only the classic Token program is accepted, so Token-2022 mints (transfer fees, hooks) are rejected rather than mishandled.

Pattern: checklist-token-2022
Scenario:
1. A pool is created for a mint with a freeze authority.
2. The freeze authority freezes the vault, and `withdraw` fails for everyone.
Test: not attempted
Verification: not required (Info)
Fix: Require `stake_mint.freeze_authority.is_none()` in `create_pool`, or state it in the docs.

### F-6: Pool and vault rent can never be recovered, and `authority` has no function

Severity: Info
Location: src/lib.rs:22
Mechanism: `pool.authority` is only used as a PDA seed and has no instruction behind it (no close, no pause). That is good for trust, but the authority's rent payment for the pool and vault is locked forever, and a wrong `epoch_length` cannot be fixed.

Pattern: checklist-authority-transfer
Scenario:
1. Pool is created with a bad `epoch_length`.
2. There is no way to change it, so a new pool and a migration of all stakes are needed.
Test: not attempted
Verification: not required (Info)
Fix: Document it as intended, or add an authority-gated `set_epoch_length` within the allowed bounds.

## Design findings

### D-1: Epoch closing depends on a tip that only unstakers fund

Severity: Low
Location: src/lib.rs:10
Mechanism: The only reward for `close_epoch` is `CLOSE_TIP_LAMPORTS` per unstake request (paid by the unstaker), and the tip is 100,000 lamports, a flat amount unrelated to fees. The call is permissionless, so liveness is not owned by the team, but when no request is pending the points events and withdrawals rely on the team's keeper. Withdrawals are stuck until some wallet pays the transaction fee, though the tip of the very request being waited on is the incentive.

Pattern: untrusted-executor
Scenario:
1. Keeper goes offline.
2. Pending unstakers pay for the close themselves (their own tip pays them back), so withdrawals still work.
3. Epochs without requests are closed late, and the points engine sees delayed events.
Test: not attempted
Verification: not required (Low)
Fix: Document that anyone can close, and consider a time-weighted tip. Note that after a long delay `epoch_started_at = ends_at` (src/lib.rs:137) makes the backlog closable one epoch per call, and only the first caller gets the tip.

## Coverage

| Item | Verdict | Evidence |
|---|---|---|
| root-of-trust | found | F-3. Q1: pool/vault/staker/request are PDAs or linked by has_one (src/lib.rs:175,189,201,203), but pools are permissionless |
| identity-rejection | n/a | no allow lists or caller-based guards |
| untrusted-executor | found | D-1. Q1: close_epoch is open to any signer (src/lib.rs:121) |
| role-inversion | n/a | no operator-pushed data |
| lazy-loading | found | F-1. Q2: no per-staker epoch checkpoint |
| pay-as-you-act | clear | total_staked updated in stake and request_unstake (src/lib.rs:60,81) |
| divide-and-conquer | clear | per-user Staker and Unbonding PDAs (src/lib.rs:164,191); the pool write is small |
| complexity-asymmetry | n/a | no loops or user-supplied hints |
| commit-execution-split | n/a | no randomness or previewed outcomes |
| action-as-data | found | F-4. Q3: no cancel, one request slot |
| virtual-execution | clear | unbonding is a stored obligation, paid exactly once because the request is closed (src/lib.rs:201) |
| one-way-state | found | F-2. Q1: epoch only moves forward (src/lib.rs:136) but the unbonding delay is variable |
| boundary-of-consent | n/a | no computed rates; amounts are exact |
| internal-oracle | n/a | no derived prices |
| defensive-computation | n/a | no valuation or rounding |
| fail-fast | n/a | no external inputs |
| execution-layer-shift | n/a | no rollup or delegation |
| dependency-independency | clear | only the SPL Token program; no admin dependency (Program<Token>, src/lib.rs:157) |
| checklist-arithmetic | clear | checked_add/sub throughout (src/lib.rs:58,60,79,81,82,126,136) |
| checklist-duplicate-accounts | clear | only one account of each kind is mutated; vault is pinned by address (src/lib.rs:177) |
| checklist-unsafe-and-panics | clear | no unsafe, no unwrap |
| checklist-reload-after-cpi | clear | state is not read after the token CPI |
| checklist-token-2022 | found | F-5 |
| checklist-closing | clear | `close = owner` with has_one on Unbonding (src/lib.rs:201) |
| checklist-seed-collisions | clear | distinct prefixes pool/vault/staker/unbonding (src/lib.rs:150-191) |
| checklist-init-frontrun | clear | pool seeds include the authority and mint, so it cannot be squatted by another signer (src/lib.rs:150) |
| checklist-realloc | n/a | no realloc |
| checklist-authority-transfer | found | F-6 |
| checklist-signers-bumps-cpi | clear | stored bump is used for the vault signer seeds (src/lib.rs:98-104); owner is a Signer on stake, request and withdraw |
