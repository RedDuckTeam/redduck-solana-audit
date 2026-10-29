# Solana audit report: outcome_pools

## Scope
`program/src/lib.rs` (Anchor 0.31.1, single file). Instructions reviewed: register_reporter, post_report, create_market, place_stake, resolve, claim. No toolchain was used, so no exploit tests were run.

## Summary
1 Critical, 0 High, 3 Medium, 3 Low, 0 Info. The finding that matters most is F-1: `resolve` never checks that the report comes from the market's chosen reporter, so anyone can decide the outcome of any market.

## Findings

### F-1: resolve does not bind the report to market.reporter

Severity: Critical
Location: program/src/lib.rs:176
Mechanism: `Resolve.reporter` has no constraint tying it to `market.reporter` (set at line 43 but never read again), and `has_one = reporter` on `report` only matches whichever reporter the caller passes. Anyone can register their own reporter, post a report for the market's public `event_id` with the result they want, and resolve the market with it, then claim the losing side's stakes.

Pattern: root-of-trust
Scenario:
1. Attacker stakes on Yes in market M (creator-chosen reporter R, event E).
2. Attacker calls `register_reporter` for reporter A and `post_report(E, Yes)`.
3. After `closes_at`, attacker calls `resolve` with M, A and the fake report. Every constraint passes and `winner = Yes`.
4. The honest reporter is locked out by `AlreadyResolved` (line 84), and the attacker claims a pro-rata share of both pools.
Test: unconfirmed (no toolchain)
Verification: confirmed by verifier
Fix: Add `address = market.reporter` on `reporter` in `Resolve` (and `has_one = reporter` on `report` stays), or check `report.reporter == market.reporter` in the handler.

### F-2: A report can be posted before the event and resolve accepts it immediately

Severity: Medium
Location: program/src/lib.rs:86
Mechanism: `post_report` accepts any result at any time, and `resolve` never checks `report.posted_at` (stored at line 29, never read). A report posted before the event ends can be used to resolve at `closes_at`, and since `winner` is set once and each reporter posts an event only once, a wrong or premature result is final.

Pattern: one-way-state
Scenario:
1. Reporter posts a result for event E before the event has happened.
2. Anyone calls `resolve` the moment `closes_at` passes, and `winner` becomes permanent.
Test: not attempted
Fix: Require `report.posted_at >= market.closes_at` plus an event-end time, and allow a dispute or correction path.

### F-3: closes_at has no upper bound, so a creator can lock stakes for an arbitrary time

Severity: Medium
Location: program/src/lib.rs:39
Mechanism: `create_market` only requires `closes_at` to be in the future. A creator can set it near `i64::MAX`, and stakers can neither resolve nor take the refund (line 99-100) until then.

Pattern: fail-fast
Scenario:
1. Creator opens a market with `closes_at = i64::MAX - RESOLVE_WINDOW`.
2. Users stake. Nothing can be resolved or refunded for effectively forever.
Test: not attempted
Fix: Cap `closes_at - now` to a maximum duration, and let stakers withdraw before a sane deadline.

### F-4: Market mint is unrestricted, and a freezable mint can freeze the vault

Severity: Low
Location: program/src/lib.rs:149
Mechanism: Any SPL mint is accepted. A mint with a freeze authority can freeze the vault token account, blocking every `claim` payout transfer.

Pattern: checklist-token-2022
Scenario:
1. Creator makes a market with a mint they control that has a freeze authority.
2. After stakes arrive, the creator freezes the vault and claims fail.
Test: not attempted
Fix: Use a mint allow-list, or reject mints that have a freeze authority.

### F-5: One ticket per user per market, so stakes cannot be added or changed

Severity: Low
Location: program/src/lib.rs:162
Mechanism: The ticket is created with `init` at a PDA of (market, owner), so a second `place_stake` fails. Users cannot top up, and they are stuck with one side and one amount.

Pattern: divide-and-conquer
Test: not attempted
Fix: Use `init_if_needed` with a check that the side matches, and add to the amount.

### F-6: Reporter names are unvalidated and not unique

Severity: Low
Location: program/src/lib.rs:17
Mechanism: `name` is arbitrary, so an attacker can register a reporter that carries a reputable reporter's name and mislead users who pick a reporter by its label.

Pattern: root-of-trust
Test: not attempted
Fix: Identify reporters only by authority key, or add a registry that is curated.

## Design findings

### D-1: Each market depends on one creator-chosen reporter, with no dispute, rotation or fallback

Severity: Medium
Location: program/src/lib.rs:43
Mechanism: The market creator picks a single reporter, who can post one result per event. A colluding or offline reporter can decide the winner, or freeze stakes for the full 14-day `RESOLVE_WINDOW` before the refund path opens. A creator who is also the reporter controls the outcome of their own market.

Pattern: dependency-independency
Test: not attempted
Fix: Add dispute and rotation, or use multiple reporters with a quorum. Show the reporter clearly to stakers.

## Coverage

| Item | Verdict | Evidence |
|---|---|---|
| root-of-trust | found | F-1, F-6 |
| identity-rejection | n/a | no allow lists or per-wallet rules |
| untrusted-executor | clear | `resolve` and `claim` are permissionless (lines 79, 94), but see F-1 for the report source |
| role-inversion | found | D-1 (reporter pushes the result) |
| lazy-loading | n/a | no iteration or accruals |
| pay-as-you-act | clear | pools updated on each stake, line 67 |
| divide-and-conquer | found | F-5 |
| complexity-asymmetry | n/a | no loops or client hints |
| commit-execution-split | n/a | no randomness or previewable outcome (the early-report issue is F-2) |
| action-as-data | n/a | no stored actions |
| virtual-execution | n/a | no deferred obligations |
| one-way-state | found | F-2 |
| boundary-of-consent | n/a | stake amount is exact, no computed rate at entry |
| internal-oracle | n/a | no derived prices |
| defensive-computation | clear | payout uses u128 and floor division, lines 250-252 |
| fail-fast | found | F-3 |
| execution-layer-shift | n/a | no rollup or delegation |
| dependency-independency | found | D-1 |
| checklist-arithmetic | clear | checked_add and u128 math, lines 67, 250-252 |
| checklist-duplicate-accounts | clear | no two mutable accounts of the same type are interchangeable |
| checklist-unsafe-and-panics | clear | no unsafe, no unwrap |
| checklist-reload-after-cpi | clear | state is not read after the transfer CPIs |
| checklist-token-2022 | found | F-4 (classic Token only, line 152) |
| checklist-closing | clear | ticket closed with `close = owner`, line 186 |
| checklist-seed-collisions | clear | distinct prefixes per account type, lines 122, 134, 144, 146, 162 |
| checklist-init-frontrun | clear | market seeds include the creator, line 144 |
| checklist-realloc | n/a | no realloc |
| checklist-authority-transfer | n/a | no authority transfer |
| checklist-signers-bumps-cpi | clear | vault authority is the market PDA with stored bump, lines 105-114 |
