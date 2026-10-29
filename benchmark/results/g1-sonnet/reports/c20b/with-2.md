# Solana audit report: outcome_pools

## Scope
program/src/lib.rs (Anchor). Instructions: register_reporter, post_report, create_market, place_stake, resolve, claim. No toolchain run; no tests executed.

## Summary
1 Critical, 0 High, 2 Medium, 3 Low. The key issue is that `resolve` does not bind the report to the market's chosen reporter.

## Findings

### F-1: Resolve does not check the reporter against market.reporter

Severity: Critical
Location: program/src/lib.rs:176
Mechanism: `market.reporter` is never compared with the `reporter` account in `Resolve`, and `report` only has_one the caller-supplied reporter, so anyone can register their own reporter, post a report for the market's event_id and resolve the market to a result they chose, then claim the other side's stakes.

Pattern: root-of-trust
Scenario:
1. Attacker calls register_reporter and post_report(event_id of victim market, side they staked on).
2. Attacker stakes on that side before closes_at.
3. After closes_at, attacker calls resolve with their own reporter and report. Both constraints pass and the winner is set.
4. Attacker calls claim and receives a pro-rata share of both pools. Any caller can also force a false result on every participant.
Test: not attempted
Verification: confirmed by verifier (note: attacker takes the whole pool only if sole backer of the chosen side; otherwise a pro-rata share)
Fix: Add `address = market.reporter` (or `has_one = reporter`) on `Resolve.reporter`.

### F-2: One ticket per (market, owner) blocks adding stake or hedging

Severity: Low
Location: program/src/lib.rs:162
Mechanism: The ticket PDA is `init` with seeds [ticket, market, owner], so a second place_stake by the same wallet fails; users cannot top up or stake the other side, and the same transaction fails for an attacker-irrelevant but legitimate repeat use.

Pattern: divide-and-conquer
Fix: Use a ticket per (market, owner, side) or `init_if_needed` with accumulation.

### F-3: Resolution ignores when the report was posted

Severity: Low
Location: program/src/lib.rs:86
Mechanism: `report.posted_at` is never checked, so a reporter can post a result before the event or before staking closes, and `resolve` accepts it as the outcome as soon as closes_at passes.

Pattern: fail-fast
Fix: Require `report.posted_at >= market.closes_at` (or an event end time).

### F-4: Unbounded closes_at and arbitrary mint at market creation

Severity: Low
Location: program/src/lib.rs:39
Mechanism: The creator can set closes_at arbitrarily far in the future (funds locked until then, with refunds only after closes_at + 14 days) and choose any mint, including worthless or freezable tokens, so stakers rely on the creator's honesty.

Pattern: boundary-of-consent
Fix: Bound closes_at, and allow-list mints or show them to users.

## Design findings

### D-1: Single unaccountable reporter, chosen by the market creator

Severity: Medium
Location: program/src/lib.rs:43
Mechanism: The market creator picks the reporter, and the reporter alone decides the winner with no dispute, bond or reputation. A colluding creator and reporter can open a market, attract stakes, and post a result that favours themselves.

Pattern: dependency-independency
Fix: Add reporter bonds or a dispute period, multiple reporters or a quorum, and let stakers see or approve the reporter.

### D-2: Winner cannot change but a wrong or never-posted result has no remedy short of full refund

Severity: Medium
Location: program/src/lib.rs:99
Mechanism: If no report is resolved within 14 days, every stake is refunded, so a reporter can withhold a result (or a griefer can delay) to force a refund when a side is losing; once resolved, a wrong result is final (winner at line 87 cannot be overturned).

Pattern: one-way-state
Fix: Add a dispute window before finality and penalise withheld reports.

## Coverage

| Item | Verdict | Evidence |
|---|---|---|
| root-of-trust | found | F-1 |
| identity-rejection | n/a | no allow lists or per-wallet rules |
| untrusted-executor | clear | resolve and claim callable by anyone (lib.rs:79, 94); reporter dependency in D-1 |
| role-inversion | found | D-1 |
| lazy-loading | n/a | no loops or accrual |
| pay-as-you-act | clear | pools updated on stake (lib.rs:67) |
| divide-and-conquer | found | F-2 |
| complexity-asymmetry | n/a | no loops or hints |
| commit-execution-split | n/a | no randomness; outcome fixed by report |
| action-as-data | n/a | no stored actions |
| virtual-execution | n/a | claims are pull-based per ticket |
| one-way-state | found | D-2 |
| boundary-of-consent | found | F-4 |
| internal-oracle | n/a | no internal price |
| defensive-computation | clear | u128 math, floor division in lib.rs:250-251 |
| fail-fast | found | F-3 |
| execution-layer-shift | n/a | no other layer |
| dependency-independency | found | D-1 |
| checklist-arithmetic | clear | checked_add / u128 checked_mul at lib.rs:67, 251 |
| checklist-duplicate-accounts | clear | no two mutable accounts of the same type in one instruction |
| checklist-unsafe-and-panics | clear | no unsafe, no unwrap |
| checklist-reload-after-cpi | n/a | no reads of token balances after CPI |
| checklist-token-2022 | n/a | uses legacy anchor_spl::token only |
| checklist-closing | clear | ticket closed with `close = owner` at lib.rs:186, PDA cannot be reinitialised for payout abuse after close (claim re-checks nothing after closing) |
| checklist-seed-collisions | clear | distinct prefixes reporter/report/market/vault/ticket (lib.rs:122-162) |
| checklist-init-frontrun | clear | market PDA seeded by creator (lib.rs:144) |
| checklist-realloc | n/a | no realloc |
| checklist-authority-transfer | n/a | no authority transfer |
| checklist-signers-bumps-cpi | clear | vault signed by market PDA with stored bump (lib.rs:107); token program typed (lib.rs:192) |
