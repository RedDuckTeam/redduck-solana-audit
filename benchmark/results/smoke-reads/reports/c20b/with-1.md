# Solana audit report: outcome_pools

## Scope
Single Anchor program `program/src/lib.rs` (register_reporter, post_report, create_market, place_stake, resolve, claim). No toolchain run; findings are from source review.

Surfaces: trusted data pushed by a reporter (resolution), time-gated first-come mechanics, claims/closing, pro-rata payout math, SPL token CPI.
Pattern files read: none opened (the pattern reference files were not read; verdicts below come from the skill's one-line rules and direct code reading).

## Summary
1 Critical, 1 Medium, 3 Low. The key issue is F-1: `resolve` never checks that the report comes from the market's chosen reporter, so anyone can pick the winner of any market.

## Findings

### F-1: Resolve does not bind the reporter to the market

Severity: Critical
Location: program/src/lib.rs:177
Mechanism: `Resolve.reporter` is any `Reporter` account and is never compared with `market.reporter`, so an attacker can register their own reporter, post a report for the market's event_id after close, and resolve the market with a winner of their choice, then claim the pools.

Pattern: root-of-trust
Scenario:
1. Attacker stakes on Yes in market M before `closes_at`.
2. After close, attacker calls `register_reporter` and `post_report(E, Yes)` with their own reporter.
3. Attacker calls `resolve` with M, their reporter and their report; `has_one = reporter` and the event_id check pass, `market.reporter` is never read.
4. Attacker calls `claim` and takes both pools.
Test: unconfirmed (no toolchain)
Verification: confirmed by verifier
Fix: Add `has_one = reporter` on `market` in `Resolve` (and `address = market.reporter` on the reporter).

### F-2: Market creator can choose a reporter it controls

Severity: Medium
Location: program/src/lib.rs:149
Mechanism: `create_market` accepts any registered reporter, with no allow-list or reputation, so a creator can name their own reporter, stake the larger-opposed side, and report the winner after staking closes, taking other users' stakes. Users can see `market.reporter` before staking, so this is a trusted-role risk.

Pattern: root-of-trust
Fix: Restrict reporters to a vetted registry, or use a dispute or multi-reporter mechanism.

### F-3: One stake per user per market, no top-up

Severity: Low
Location: program/src/lib.rs:163
Mechanism: The ticket PDA is seeded by (market, owner) and created with `init`, so a second `place_stake` fails. Users cannot add to a stake or hedge, and a griefer cannot block others, but the behaviour is likely unintended.

Pattern: divide-and-conquer
Fix: Use `init_if_needed` with a side check, or add an index to the seeds.

### F-4: Reports not tied to event completion, and one report per event is permanent

Severity: Low
Location: program/src/lib.rs:85
Mechanism: The only timing check is `posted_at >= closes_at`, which is set before the event starts, so a reporter can publish a result before the event ends. The report PDA is unique per (reporter, event_id), so a wrong report can never be corrected and event_ids are creator-chosen, so a report can resolve unrelated markets sharing that id.

Pattern: fail-fast
Fix: Add an event end time to the market and require the report after it. Scope reports to a market.

### F-5: No upper bound on `closes_at`

Severity: Low
Location: program/src/lib.rs:39
Mechanism: A creator can set `closes_at` arbitrarily far in the future, and stakes then cannot be refunded until `closes_at` plus 14 days, which locks funds for as long as the creator chooses.

Pattern: boundary-of-consent
Fix: Cap `closes_at` relative to now.

## Design findings

### F-6: Resolution depends on a single reporter, with no fallback except refund

Severity: Medium
Location: program/src/lib.rs:79
Mechanism: Only the one reporter named at creation can supply a result, with no dispute, replacement or alternative source. If it is offline, stakes are locked 14 days and then refunded, and if it is malicious the outcome is theirs (see F-2).

Pattern: dependency-independency
Fix: Allow multiple reporters, quorum or dispute, and a reporter replacement path.

## Coverage

| Item | Verdict | Evidence |
|---|---|---|
| root-of-trust | found | F-1 |
| identity-rejection | n/a | no allow/deny lists |
| untrusted-executor | clear | `resolve` and `claim` callable by anyone/owner after the windows, lib.rs:79 |
| role-inversion | found | F-2, F-6 |
| lazy-loading | n/a | no accrual or loops |
| pay-as-you-act | clear | pools updated per stake, lib.rs:67 |
| divide-and-conquer | found | F-3 |
| complexity-asymmetry | n/a | no loops or searches |
| commit-execution-split | clear | staking closes before resolve, lib.rs:56, 82 |
| action-as-data | n/a | no stored actions |
| virtual-execution | n/a | payouts claimed by owner |
| one-way-state | clear | winner set once, lib.rs:84; ticket closed on claim, lib.rs:187 |
| boundary-of-consent | found | F-5 |
| internal-oracle | n/a | no derived prices |
| defensive-computation | clear | payout rounds down, u128 math, lib.rs:252 |
| fail-fast | found | F-4 |
| execution-layer-shift | n/a | no other layer |
| dependency-independency | found | F-6 |
| checklist-arithmetic | clear | checked_add/mul, lib.rs:67, 252 |
| checklist-duplicate-accounts | clear | no two mutable same-type accounts that can alias harmfully |
| checklist-unsafe-and-panics | clear | no unsafe; no unwraps |
| checklist-reload-after-cpi | clear | no state read after CPI |
| checklist-token-2022 | n/a | classic Token program only, lib.rs:2 |
| checklist-closing | clear | `close = owner` on ticket, lib.rs:187 |
| checklist-seed-collisions | clear | distinct prefixes, lib.rs:123-163 |
| checklist-init-frontrun | clear | PDAs seeded by creator/owner key |
| checklist-realloc | n/a | no realloc |
| checklist-authority-transfer | n/a | no authority transfer |
| checklist-signers-bumps-cpi | clear | owner Signer, market PDA signs with stored bump, lib.rs:108 |
