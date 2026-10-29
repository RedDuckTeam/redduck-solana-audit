# Solana audit: season_locker (Anchor)

## Findings

### F1
Severity: High
Location: program/src/instructions/position.rs:144
Mechanism: `merge_positions` raises `target.weight` to `lock_weight(total amount, later lock_end, now)`, but `Season.total_weight` is never updated (season is not `mut`, line 125). A user merges a large 7-day position into a tiny 4-year position and gets 4x weight on all tokens with no matching total. `claim_reward` (exit.rs:66) then overpays them, and the sum of claims can exceed `reward_amount`. Late claimers fail, and since `withdraw` requires `reward_claimed` after finalization (exit.rs:33), their stake is frozen. Before finalization, `remove_weight` (exit.rs:35) can underflow and block withdrawal.
Verification: confirmed by finding-verifier (source-level; no exploit test run, no toolchain).

### F2
Severity: Medium
Location: program/src/instructions/exit.rs:33
Mechanism: After finalization, stake withdrawal depends on a successful reward claim. If `claim_reward` fails for any reason (an underfunded reward vault, as in F1, or a token account problem), the user's principal stays locked in the stake vault. Withdrawal should not depend on the reward payout.

### F3
Severity: Medium
Location: program/src/instructions/position.rs:106
Mechanism: `extend_lock` (and `merge_positions`, line 136) are open until `ends_at`, while new stake closes at `starts_at`. Weight depends only on the remaining lock at the time of the call, not on time spent in the season. A user can watch the other weights and extend at the last moment to take the exact share they want. This is a sniping and fairness issue, and it makes the reward split manipulable.

### F4
Severity: Medium
Location: program/src/instructions/exit.rs:35
Mechanism: Reward forfeiture is silent. A position withdrawn after its lock expires but before `finalize_season` (line 32) loses its reward, and the position account is closed. Locks as short as 7 days (state.rs:10) can end before `ends_at`, so the user can lose rewards by timing alone. Forfeited rewards also stay in the vault with no recovery path (see F5).

### F5
Severity: Low
Location: program/src/instructions/season.rs:72
Mechanism: No instruction returns undistributed rewards to the funder. Rounding dust, forfeited rewards (F4), and the whole reward if `final_total_weight` is 0 (`pro_rata` returns 0, math.rs:27) stay in `reward_vault` permanently. `Season.authority` has no powers after creation.

### F6
Severity: Low
Location: program/src/math.rs:11
Mechanism: Weight is computed once at the time of the call and never decays. `lock_weight` rounds down and uses `now` at each action, so `add_stake`, `extend_lock` and `merge_positions` give different weight for the same remaining lock. This makes the weight inconsistent between users.

### F7
Severity: Info
Location: program/src/instructions/season.rs:14
Mechanism: `stake_mint` is chosen freely by the season creator, and only the legacy SPL Token program is supported. A season with a malicious or freezable mint can trap depositors' funds. This is a trust assumption on the season creator, and users should check the mint.

## Design findings

- Liveness: finalization is permissionless (season.rs:66), which is good. But exits are coupled to reward claims (F2).
- Scalability: each season is a single hot account written by every opener (`total_weight`). It is fine at this scale, but concurrent writes serialize.
- Centralization: none. There is no admin, upgrade authority beyond deployment, or oracle.

## Coverage

Patterns: root-of-trust clear (PDA seeds + has_one on all position accounts); identity-rejection n/a; untrusted-executor clear (finalize permissionless, season.rs:66); role-inversion n/a; lazy-loading clear; pay-as-you-act found (F1, total not updated on merge); divide-and-conquer clear (per-position PDAs); complexity-asymmetry n/a; commit-execution-split n/a; action-as-data n/a; virtual-execution n/a; one-way-state found (F3, F4); boundary-of-consent n/a (no slippage; amounts fixed); internal-oracle n/a; defensive-computation found (F6); fail-fast n/a; execution-layer-shift n/a; dependency-independency found (F2, F7).

Checklist: arithmetic clear (checked ops; `capped * MAX_BOOST_BPS` cannot overflow u64); duplicate-accounts clear (merge checks source != target, position.rs:127); unsafe-and-panics clear (no unsafe; `weight - position.weight` at position.rs:113 is safe because of `.max`); reload-after-cpi n/a; token-2022 n/a (legacy Token only, F7); closing clear (Anchor `close`); seed-collisions clear (distinct seed prefixes); init-frontrun clear (seeds include the authority/owner); realloc n/a; authority-transfer n/a; signers-bumps-cpi clear (stored bumps, season PDA signs, exit.rs/mod.rs:40).
