# Solana audit report: outcome_pools

## Scope
Anchor 0.31.1 program in `program/` (`src/lib.rs`, single file). Instructions: register_reporter, post_report, create_market, place_stake, resolve, claim. No tests or toolchain run; findings are from source review plus a verifier agent for the High-candidate findings.

## Summary
Critical 0, High 0, Medium 1, Low 5, Info 1. Most important: reports are not tied to market timing (F-1), and the reporter is chosen by the market creator with no dispute path (F-2). Account validation, PDA seeds, payout math and the refund path were checked and look sound.

## Findings

### F-1: Report timing is never checked against staking or closing
Severity: Medium
Location: program/src/lib.rs:86
Mechanism: `post_report` has no time constraint and `place_stake` never checks whether a report exists, so a report posted before `closes_at` (or a `closes_at` set after the event ends) lets anyone stake on the known winner and take the losers' funds; `resolve` also accepts a report posted before the event could have finished.
Pattern: commit-execution-split
Scenario:
1. Reporter posts report `Yes` for the event while the market is still open.
2. Attacker stakes on `Yes` in `place_stake` (only `now < closes_at` is checked, line 56).
3. After `closes_at`, anyone calls `resolve`; attacker claims a pro-rata share of the No pool at no risk.
Test: not attempted
Verification: downgraded by verifier from High: needs a misbehaving reporter or a creator-misconfigured `closes_at`; honest use is safe.
Fix: In `resolve` require `report.posted_at >= market.closes_at`; optionally reject `place_stake` when a report for the event exists.

### F-3: One ticket per owner per market, no top-up
Severity: Low
Location: program/src/lib.rs:162
Mechanism: Ticket PDA is seeded only by market and owner and created with `init`, so a second `place_stake` by the same wallet fails; users cannot add to a stake or back both sides, and are pushed to use many wallets.
Pattern: divide-and-conquer
Fix: Use `init_if_needed` with a side check and add to `amount`, or include a nonce in the seeds.

### F-4: Creator-chosen mint can freeze or lock the vault
Severity: Low
Location: program/src/lib.rs:149
Mechanism: `create_market` accepts any SPL mint; a mint with a freeze authority (or a malicious mint) lets its authority freeze the vault and lock all stakes, and there is no mint allow-list.
Pattern: dependency-independency
Fix: Require `freeze_authority` to be none, or restrict to an allow-list of mints.

### F-5: `closes_at` is unbounded
Severity: Low
Location: program/src/lib.rs:39
Mechanism: Only `closes_at > now` is required, so a creator can set a date years away; stakes cannot be resolved or refunded until `closes_at + 14 days` (line 99), locking funds.
Pattern: fail-fast
Fix: Bound `closes_at` to a maximum horizon.

### F-7: Rounding dust is stranded in the vault
Severity: Info
Location: program/src/lib.rs:252
Mechanism: Payouts round down per ticket, so a few base units stay in each vault forever; no sweep exists. No user-fund risk.
Pattern: defensive-computation
Fix: Optional: let the last claimant take the remainder.

## Design findings

### F-2: Reporter is self-registered and chosen by the market creator, with no dispute
Severity: Low
Location: program/src/lib.rs:148
Mechanism: `register_reporter` is permissionless and `create_market` accepts any reporter, so a creator can register as their own reporter, stake on one side, and post the result they want; stakers have no recourse except checking `market.reporter` first.
Pattern: root-of-trust
Scenario:
1. Attacker registers a reporter and creates a market naming it.
2. Victims stake; attacker stakes on the side they will report.
3. Attacker posts the chosen report, calls `resolve`, and claims.
Test: not attempted
Verification: downgraded by verifier from High: documented trust model (lib.rs:13), reporter is fixed and visible on-chain at creation, no third party can hijack a market.
Fix: Reporter allow-list or stake/bond with a dispute window before resolution.

### F-6: Reports are one-shot and final
Severity: Low
Location: program/src/lib.rs:134
Mechanism: The report PDA is `init`-only per reporter and event, and `resolve` makes the first report final (line 84), so a mistaken or compromised-key report cannot be corrected and the reporter key cannot be rotated.
Pattern: one-way-state
Fix: Add a challenge period before resolution and a reporter authority-transfer instruction.

## Coverage

| Item | Verdict | Evidence |
|---|---|---|
| root-of-trust | found | F-2 (reporter chain ends at self-registered account); market/report PDAs and `has_one` checks at lib.rs:176-178 are sound |
| identity-rejection | n/a | no allow/deny lists or per-wallet rules besides ticket PDA (F-3) |
| untrusted-executor | clear | resolve is permissionless (lib.rs:79); unresolved markets refund after window (lib.rs:99) |
| role-inversion | found | F-1 (reporter pushes data with no timing relation) |
| lazy-loading | n/a | no accrual or iteration |
| pay-as-you-act | clear | pools updated on each stake (lib.rs:66); no batch step |
| divide-and-conquer | found | F-3 |
| complexity-asymmetry | n/a | no loops or user-sized data |
| commit-execution-split | found | F-1 |
| action-as-data | n/a | no stored executable actions |
| virtual-execution | n/a | claims are paid immediately from the vault |
| one-way-state | found | F-6; ticket closed on claim (lib.rs:187) prevents double claim |
| boundary-of-consent | clear | stake amount is exact and user-specified; payout depends only on final pools, no swap rate |
| internal-oracle | n/a | no price derived from own state |
| defensive-computation | found | F-7; payout floors in protocol's favour (lib.rs:252) |
| fail-fast | found | F-5 |
| execution-layer-shift | n/a | no rollup or delegation |
| dependency-independency | found | F-4 |
| checklist-arithmetic | clear | checked_add/checked_mul and u128 math at lib.rs:67, 251-253 |
| checklist-duplicate-accounts | clear | vault pinned to `market.vault` (lib.rs:160,185); tickets are per-owner PDAs |
| checklist-unsafe-and-panics | clear | no unsafe, no unwrap/index on user input (`side.index()` is 0/1) |
| checklist-reload-after-cpi | clear | no account data read after the token CPI that matters |
| checklist-token-2022 | clear | uses `Program<Token>` (lib.rs:168), legacy token only |
| checklist-closing | clear | ticket closed with `close = owner` (lib.rs:187), market never closed |
| checklist-seed-collisions | clear | distinct prefixes and fixed-size keys (lib.rs:122,134,144,146,162) |
| checklist-init-frontrun | clear | PDAs seeded by creator/authority/owner, so no one else can pre-create them |
| checklist-realloc | n/a | no realloc |
| checklist-authority-transfer | found | F-6 (no transfer for reporter authority) |
| checklist-signers-bumps-cpi | clear | stored bump used for market signer seeds (lib.rs:106-114); token program typed |
