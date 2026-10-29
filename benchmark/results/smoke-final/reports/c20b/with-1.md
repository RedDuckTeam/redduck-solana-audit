# Solana audit report: outcome_pools

## Scope
`program/src/lib.rs` (Anchor 0.31.1, single file). Instructions: register_reporter, post_report, create_market, place_stake, resolve, claim. No toolchain was run, so no exploit tests.

## Summary
1 Critical, 0 High, 2 Medium, 3 Low. The key issue is F-1: anyone can resolve any market with their own reporter.

## Findings

### F-1: resolve does not bind the report to the market's reporter

Severity: Critical
Location: src/lib.rs:177
Mechanism: `Resolve.reporter` is not checked against `market.reporter`, and `report` only has `has_one = reporter` against that caller-supplied account. An attacker registers their own reporter, posts a report for the market's event_id after close, and resolves the market to the side they staked on, then claims the whole pool.

Pattern: root-of-trust
Scenario:
1. Attacker stakes on Yes before `closes_at`.
2. After close, attacker calls `register_reporter` and `post_report(event_id, Yes)`.
3. Attacker calls `resolve` with their own reporter and report. All checks pass (event_id matches, `posted_at >= closes_at`).
4. Attacker calls `claim` and receives the losing side's stakes.
Test: unconfirmed (no toolchain)
Verification: confirmed by verifier
Fix: Add `constraint = reporter.key() == market.reporter` (or `address = market.reporter`) on `Resolve.reporter`, and derive `report` from seeds `[b"report", market.reporter, event_id]`.

## Design findings

### F-2: A single report per event can leave a market unresolvable

Severity: Medium
Location: src/lib.rs:85
Mechanism: `post_report` creates a PDA per (reporter, event) that cannot be replaced, and `resolve` rejects reports with `posted_at < closes_at`. A report posted early, or posted once for several markets with different close times, can never resolve some markets, which then sit until the 14-day refund. A wrong report can never be corrected.

Pattern: fail-fast
Scenario:
1. Reporter posts the event result before a market's `closes_at`.
2. `resolve` fails with `EarlyReport`, and the report cannot be re-posted.
3. Funds stay locked for `RESOLVE_WINDOW`, then everyone is refunded.
Test: not attempted
Fix: Allow the reporter to post a report with a fresh timestamp or per-market report, or drop the `posted_at` check in favour of an event-end time.

### F-3: The market creator picks the reporter, mint and close time unchecked

Severity: Medium
Location: src/lib.rs:38
Mechanism: `create_market` accepts any reporter, any mint and any `closes_at`. A creator can name a reporter they control, or set `closes_at` after the event so the outcome is known, and then collect the pool. The mint can also have a freeze authority that freezes the vault. The reporter and close time are visible to stakers, so this is a trusted-role design issue.

Pattern: dependency-independency
Scenario:
1. Creator registers a reporter and makes a market naming it.
2. Users stake. The creator's reporter posts the result they want.
Test: not attempted
Fix: Use an allow-listed or governance-chosen reporter set, require a freeze-authority-free mint, and bound `closes_at`.

### F-4: Only one stake per user per market

Severity: Low
Location: src/lib.rs:163
Mechanism: The ticket PDA is seeded by (market, owner) and created with `init`, so a second `place_stake` fails. Users cannot top up or hedge, and a front-runner cannot block anyone except by staking from the same wallet.

Pattern: divide-and-conquer
Test: not attempted
Fix: Use `init_if_needed` with a side check, or add a ticket index to the seeds.

### F-5: Token-2022 and unusual mints are unsupported

Severity: Low
Location: src/lib.rs:150
Mechanism: Only the legacy Token program is accepted. Fee-on-transfer is not possible there, but the creator-chosen mint may be freezable (see F-3).

Pattern: checklist-token-2022
Test: not attempted
Fix: Restrict mints to an allow list.

### F-6: Payout rounding dust stays in the vault

Severity: Low
Location: src/lib.rs:252
Mechanism: Pro-rata payouts use floor division, so a few base units stay in the vault forever. Rounding favours the protocol and cannot be exploited.

Pattern: defensive-computation
Test: not attempted
Fix: Let the creator sweep dust after all tickets are claimed, if desired.

## Coverage

| Item | Verdict | Evidence |
|---|---|---|
| root-of-trust | found | F-1 |
| identity-rejection | n/a | no allow or deny lists |
| untrusted-executor | clear | `resolve` and `claim` are permissionless, src/lib.rs:79 |
| role-inversion | found | F-1, F-2 |
| lazy-loading | n/a | no accruals or loops |
| pay-as-you-act | n/a | pools updated in `place_stake`, src/lib.rs:67 |
| divide-and-conquer | found | F-4 |
| complexity-asymmetry | clear | no loops or growing data |
| commit-execution-split | found | F-3 (close time unbounded) |
| action-as-data | n/a | no stored actions |
| virtual-execution | n/a | no deferred obligations |
| one-way-state | clear | `winner` set once, src/lib.rs:84; ticket closed on claim, src/lib.rs:187 |
| boundary-of-consent | n/a | stake amounts are exact |
| internal-oracle | n/a | no derived prices |
| defensive-computation | found | F-6 |
| fail-fast | found | F-2 |
| execution-layer-shift | n/a | no rollup |
| dependency-independency | found | F-3 |
| checklist-arithmetic | clear | checked_add / u128 math, src/lib.rs:67, 252 |
| checklist-duplicate-accounts | clear | no two mutable accounts of the same type |
| checklist-unsafe-and-panics | clear | no unsafe, no unwraps |
| checklist-reload-after-cpi | clear | no reads after CPI |
| checklist-token-2022 | found | F-5 |
| checklist-closing | clear | `close = owner` with has_one, src/lib.rs:187 |
| checklist-seed-collisions | clear | distinct seed prefixes |
| checklist-init-frontrun | clear | PDAs keyed to the creator, so there is no frontrun |
| checklist-realloc | n/a | no realloc |
| checklist-authority-transfer | n/a | no authority transfer |
| checklist-signers-bumps-cpi | clear | market PDA signs the vault transfer with stored bump, src/lib.rs:108 |
