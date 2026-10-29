# Solana audit report: season_locker (Anchor 0.31.1)

Scope: all files under `program/src`. Instructions: create_season, open_position, add_stake, extend_lock, merge_positions, withdraw, finalize_season, claim_reward.
Account validation (PDA seeds, `has_one`, token mint and authority constraints, signers) was reviewed on every instruction and no issues were found there. The vault authority is the season PDA, and the chain of trust ends at PDAs with verified seeds.
Not done: the skill's per-pattern reference files and the full 29-row coverage table were not worked through, and no exploit test was run (no toolchain used). The High finding was reviewed by an independent verifier and confirmed by code reading.

## Findings

### 1. merge_positions inflates a position's weight without updating the season total
Severity: High
Location: program/src/instructions/position.rs:144
Mechanism: `merge_positions` sets `target.weight = max(lock_weight(total amount, later lock_end, now), combined)`, which applies the longer lock's boost to all merged tokens, but never calls `season.add_weight` (the season account is not even `mut`, line 125). An attacker opens a large 7-day position and a dust 4-year position before `starts_at`, then merges the dust into the large one during the season. The result is a weight of roughly 4x the stake that `total_weight` does not include, so `claim_reward` (exit.rs:66) overpays the attacker, drains the reward vault, and makes later claims fail. Unfinalized `withdraw` (exit.rs:35) can also revert on `remove_weight` underflow.
Verification: confirmed by the finding-verifier. No exploit test was run.

### 2. Reward tokens have no recovery path and can be stranded
Severity: Medium
Location: program/src/instructions/exit.rs:66
Mechanism: The full `reward_amount` is escrowed at creation (season.rs:52), but payouts are rounded down and `pro_rata` returns 0 if `final_total_weight` is 0. Rounding dust, rewards of positions that withdrew before finalization, and the entire reward when nobody enrolled stay in the reward vault forever, because nothing lets the authority or anyone else sweep them.

### 3. Withdrawing after the season ends but before finalize silently forfeits the reward
Severity: Low
Location: program/src/instructions/exit.rs:35
Mechanism: If the season is not finalized, `withdraw` removes the weight and closes the position. A user whose lock has expired can therefore lose the whole reward by withdrawing before someone calls `finalize_season`. Finalization is permissionless but has no incentive for anyone to call it.

### 4. Reward weight is not tied to the season window
Severity: Info
Location: program/src/instructions/position.rs:37
Mechanism: Weight is computed from the lock length at deposit time, and `lock_end` is not required to cover `starts_at..ends_at`. A position whose lock ends mid-season earns the same weight as one locked for the whole season, as long as it stays unwithdrawn until finalize. Enrolment can happen long before the season starts, which also gives early depositors weight for time that is not part of the season.

### 5. Season setup is open to anyone and limited to the plain SPL Token program
Severity: Info
Location: program/src/instructions/season.rs:14
Mechanism: Anyone can create a season with any mint, so the UI or integrators must identify legitimate seasons themselves. A stake mint with a freeze authority lets the mint's authority freeze the vaults and block withdrawals. Only `anchor_spl::token::Token` is accepted, so Token-2022 mints (transfer fees, hooks) are not supported.

## Design findings

- Centralization: none material. There is no admin or upgrade-controlled parameter in the program. The season authority only funds the season and has no control afterwards. Because the authority has no way back to the funds (finding 2), the reward budget is effectively locked once deposited.
- Liveness: reward payout depends on someone calling `finalize_season` (finding 3). The call is permissionless, but there is no incentive.
- Scalability: no loops over users, and per-position state is separate. Every open, add, extend and withdraw writes the single `Season` account, which serialises these instructions but stays inside normal limits.
