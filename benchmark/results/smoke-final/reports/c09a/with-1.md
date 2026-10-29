# Solana audit: season_locker (Anchor 0.31.1)

Scope: `program/src/**`. Instructions: create_season, open_position, add_stake, extend_lock, merge_positions, withdraw, finalize_season, claim_reward. No Critical or High issues found. Account validation chains end at season/position/vault PDAs with verified seeds and `has_one`. Weight accounting (`total_weight` equals the sum of position weights) holds on every path, and pro-rata payouts round down, so the reward vault stays solvent.

## Findings

Severity: Medium
Location: program/src/instructions/season.rs:72
Mechanism: The reward budget is fixed at creation and there is no path to recover any of it. If `total_weight` is 0 at finalize (nobody enrolled, or everyone withdrew early), `pro_rata` returns 0 and the whole `reward_amount` stays in the reward vault forever. The same happens to the rounding dust and to the share of any holder who never claims.

Severity: Low
Location: program/src/instructions/exit.rs:35
Mechanism: Between `ends_at` and the first `finalize_season` call, a holder whose lock has expired can withdraw and shrink `total_weight`. Because the snapshot at season.rs:72 is taken by whoever calls first, the reward split depends on transaction ordering. Holders who need liquidity are pushed to forfeit their reward, and a caller can time finalization to favour themselves.

Severity: Low
Location: program/src/instructions/position.rs:112
Mechanism: `extend_lock` (and `merge_positions` at :145) re-price weight at `now` until `ends_at`, although `add_stake` is closed after `starts_at`. A holder can raise their weight in the last second before the season ends, when others can no longer react. Weight is also never decayed (`open_position` :38), and a lock may expire before the season ends. Reward share is therefore decoupled from how long tokens were actually committed to the season.

Severity: Low
Location: program/src/instructions/season.rs:14
Mechanism: `stake_mint` is chosen freely by any season creator and is never checked for a freeze authority or against a mint allow-list. A creator can use a mint they can freeze, which locks every depositor's stake in the stake vault. Seasons are also permissionless, so users must verify out of band that a season is legitimate.

Severity: Low
Location: program/src/instructions/position.rs:15
Mechanism: Every user instruction writes the single `Season` account (`total_weight`). This makes it a hot account that serializes all transactions for a season, which hurts scalability and lets an attacker with many tiny positions contend for write locks during enrollment and around `ends_at`. `amount > 0` is the only minimum, so there is no cost to doing this.

## Design findings

- Liveness: finalization is permissionless, which is good. Nothing forces it, though, and claims and withdrawals after `ends_at` depend on it (exit.rs:61). Users must either wait for someone to call it or call it themselves.
- Centralization: none. The season authority has no powers after creation (see the unrecoverable budget above).

## Coverage

All 18 patterns and 11 checklist items were considered. The ones that produced findings: untrusted-executor / one-way-state (ordering around finalize), lazy-loading and divide-and-conquer (hot Season account), boundary-of-consent (late weight re-pricing), dependency-independency (mint freeze authority). The rest are clear or not applicable (no oracles, no CPI beyond SPL Token, no realloc, no authority transfer, no Token-2022 since `Token` program is pinned, checked arithmetic throughout, duplicate positions rejected at position.rs:128, PDA seeds include season and owner).
