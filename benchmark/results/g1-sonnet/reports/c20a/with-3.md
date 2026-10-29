# Solana audit report: outcome_pools

## Scope
`program/src/lib.rs` (Anchor 0.31.1, single program). Instructions: register_reporter, post_report, create_market, place_stake, resolve, claim. No tests or toolchain run; findings are from source review.

## Summary
0 Critical, 0 High, 2 Medium, 4 Low, 1 Info, plus 2 design findings (1 Medium, 1 Low) listed below. The most important issue is that a market's outcome is decided by a single, self-chosen reporter with no dispute path. Both candidate High findings were downgraded by the verifier.

## Findings

### F-1: Market reporter is any self-registered account

Severity: Medium
Location: program/src/lib.rs:148
Mechanism: `create_market` accepts any `Reporter`, and `register_reporter` is permissionless, so a creator can register themself as reporter, stake on one side, post the result they want and drain the opposing pool of stakers who did not vet the reporter.

Pattern: root-of-trust
Scenario:
1. Attacker registers a reporter and creates a market naming it.
2. Victims stake on both sides; attacker stakes on one side.
3. After `closes_at` attacker posts a report for their side, calls `resolve`, then claims pro rata from both pools.
Test: not attempted
Verification: downgraded by verifier from High: reporter choice is documented intent (lib.rs:13), reporter is visible on-chain before staking, loss is limited to stakers of that market.
Fix: Whitelist or stake-bond reporters, or add a dispute window before `resolve`.

### F-2: Report timing is not bound to the staking window

Severity: Low
Location: program/src/lib.rs:82
Mechanism: `post_report` has no time check and `resolve` ignores `report.posted_at`, so a report can exist while staking is open and the reporter (or anyone who sees it) can stake on the known winner before `closes_at`.

Pattern: commit-execution-split
Scenario:
1. Reporter posts the result early.
2. Reporter stakes on the winning side before `closes_at`.
3. After `closes_at` the market resolves and the reporter claims from honest stakers' pool.
Test: not attempted
Verification: downgraded by verifier from High: only the market's reporter can create the report, so it requires a malicious or colluding trusted key.
Fix: Require `report.posted_at >= market.closes_at` in `resolve`.

### F-3: Creator picks an arbitrary mint, including one with a freeze authority

Severity: Medium
Location: program/src/lib.rs:149
Mechanism: The mint is unconstrained, so a creator can use a mint they control with a freeze authority (or a malicious mint); they can freeze the vault and lock all stakes, or the stake token can be worthless.

Pattern: dependency-independency
Scenario:
1. Creator creates a market on their own mint with freeze authority.
2. Users stake.
3. Creator freezes the vault; `claim` transfers fail and funds are stuck.
Test: not attempted
Fix: Allowlist mints, or require `mint.freeze_authority.is_none()`.

### F-4: One ticket per owner per market

Severity: Low
Location: program/src/lib.rs:162
Mechanism: The ticket PDA seeds are `[ticket, market, owner]` with `init`, so a second `place_stake` by the same wallet fails; users cannot top up or stake the other side, and a griefer cannot block others but can force users to split across wallets.

Pattern: divide-and-conquer
Scenario: User stakes once, then tries to add to the stake; the transaction fails because the ticket exists.
Test: not attempted
Fix: Use `init_if_needed` with a same-side check, or add a ticket index to the seeds.

### F-5: `closes_at` has no upper bound

Severity: Low
Location: program/src/lib.rs:39
Mechanism: A creator can set `closes_at` arbitrarily far away; the market cannot resolve or refund until `closes_at + 14 days`, so stakes are locked for that long with no early exit.

Pattern: fail-fast
Scenario: Creator sets `closes_at` decades ahead; stakers cannot withdraw.
Test: not attempted
Fix: Cap `closes_at - now`, and ideally allow withdrawal before close.

### F-6: Payout rounding leaves dust in the vault

Severity: Low
Location: program/src/lib.rs:252
Mechanism: The pro rata share is floored, so a remainder stays in the vault forever; the amount is small per market and there is no sweep.

Pattern: defensive-computation
Scenario: Pools of 3 and 4 with two equal winners leave 1 token unclaimable.
Test: not attempted
Fix: Accept and document, or let the creator sweep the remainder after all tickets close.

### F-7: Reporter name is unauthenticated

Severity: Info
Location: program/src/lib.rs:17
Mechanism: `name` is free text and not unique, so a malicious reporter can copy a well-known reporter's name to look legitimate in a UI.

Pattern: root-of-trust
Test: not attempted
Fix: Show the reporter authority key in UIs, not the name.

## Design findings

### D-1: Single immutable reporter result, no dispute or correction

Severity: Medium
Location: program/src/lib.rs:24
Mechanism: A report is written once per reporter and event (PDA `init`) and `resolve` copies it into `market.winner` permanently, so a mistaken or malicious report cannot be corrected or disputed, and every stake depends on one key.

Pattern: dependency-independency
Test: not attempted
Fix: Add a dispute period, multiple reporters or an on-chain oracle.

### D-2: Reporter downtime freezes funds for 14 days

Severity: Low
Location: program/src/lib.rs:7
Mechanism: If the reporter never posts, stakers can only recover funds after `closes_at + RESOLVE_WINDOW`; funds are recoverable, but locked during that time. Refunds do work after the window, so it is a delay and not a permanent loss.

Pattern: untrusted-executor
Test: not attempted
Fix: Shorten the window or allow creator and staker-agreed early cancellation.

## Coverage

| Item | Verdict | Evidence |
|---|---|---|
| root-of-trust | found | F-1, F-7; resolve chain ends at market.reporter address check (lib.rs:176) |
| identity-rejection | n/a | no allow lists or per-wallet rules beyond the ticket seed (F-4) |
| untrusted-executor | found | D-2; `resolve` is permissionless (lib.rs:79) |
| role-inversion | clear | the reporter brings the signed result and the program only verifies, lib.rs:132 |
| lazy-loading | n/a | no iteration or accrual |
| pay-as-you-act | clear | pools updated on stake, lib.rs:66 |
| divide-and-conquer | found | F-4; per-user tickets and per-market state, otherwise partitioned |
| complexity-asymmetry | n/a | no loops or client hints |
| commit-execution-split | found | F-2 |
| action-as-data | n/a | no stored executable actions |
| virtual-execution | n/a | no deferred obligations beyond tickets |
| one-way-state | clear | winner set once, lib.rs:84; ticket closed on claim, lib.rs:187 |
| boundary-of-consent | n/a | stake amounts are exact and payout is determined by pools at resolution; no slippage-style computation at entry |
| internal-oracle | n/a | no derived prices |
| defensive-computation | found | F-6 |
| fail-fast | found | F-5 |
| execution-layer-shift | n/a | no rollup or delegation |
| dependency-independency | found | F-3, D-1 |
| checklist-arithmetic | clear | checked_add lib.rs:67, u128 math lib.rs:251-253 |
| checklist-duplicate-accounts | clear | no two mutable accounts of the same type are usable for a double effect |
| checklist-unsafe-and-panics | clear | no unsafe, no unwrap |
| checklist-reload-after-cpi | clear | no account data read after CPI |
| checklist-token-2022 | n/a | uses `Program<Token>` only (lib.rs:152) |
| checklist-closing | clear | `close = owner` with has_one, lib.rs:187 |
| checklist-seed-collisions | clear | distinct prefixes reporter/report/market/vault/ticket |
| checklist-init-frontrun | clear | PDAs seeded by the signer (lib.rs:122, 144) |
| checklist-realloc | n/a | no realloc |
| checklist-authority-transfer | n/a | no authority transfer |
| checklist-signers-bumps-cpi | clear | stored bump used for signer seeds, lib.rs:106-107; owner signs stakes |
