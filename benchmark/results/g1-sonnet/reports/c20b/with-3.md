# Solana audit report: outcome_pools

## Scope
`program/src/lib.rs` (Anchor, single program, 5 instructions: register_reporter, post_report, create_market, place_stake, resolve, claim). Reviewed by manual read; no toolchain was run, so no exploit tests. Pattern reference files were not opened individually; verdicts use the skill's one-line rules.

## Summary
1 Critical, 0 High, 3 Medium, 3 Low, 1 Info. The most important finding is F-1: `resolve` never binds the reporter to the market, so anyone can decide any market's winner and drain it.

## Findings

### F-1: resolve does not check the report comes from market.reporter

Severity: Critical
Location: src/lib.rs:176
Mechanism: `Resolve.reporter` is not compared with `market.reporter`; `has_one = reporter` on `report` only ties the report to whichever reporter the caller passes. An attacker registers their own reporter, posts a report for the market's event_id and resolves with it, choosing the winner.

Pattern: root-of-trust
Scenario:
1. Victims stake on market M (event E, reporter R).
2. Attacker stakes on side X, calls register_reporter (R') and post_report(E, X).
3. After closes_at the attacker calls resolve(M, R', report); constraints at lines 177 pass; winner = X.
4. Attacker claims a pro-rata share of both pools; the other side gets nothing.
Test: unconfirmed (no toolchain)
Verification: confirmed by verifier
Fix: Add `address = market.reporter` (or `has_one = reporter` on `market`) in `Resolve`.

### F-2: Late staking after the result is already public

Severity: Medium
Location: src/lib.rs:56
Mechanism: `place_stake` only checks `closes_at`, and `create_market` accepts any future `closes_at` (line 39) with no relation to the event time; `post_report` has no time bound (line 29) and `resolve` ignores `posted_at`. Anyone can stake on the known winner once a report exists, or a creator can set `closes_at` after the event, taking value from honest stakers.

Pattern: commit-execution-split
Scenario:
1. Report for event E is posted while the market is still open.
2. Attacker stakes on the reported side and claims a share of the losing side's stakes.
Test: not attempted
Fix: Reject staking once a report exists or require `report.posted_at >= closes_at` in resolve; bound `closes_at` against the event start.

### F-3: Arbitrary mint chosen by the creator can freeze or break the vault

Severity: Medium
Location: src/lib.rs:149
Mechanism: `create_market` accepts any mint without restricting freeze authority or an allow-list. A creator can use a mint they can freeze, then freeze the vault or users' token accounts so stakes and payouts are locked.

Pattern: dependency-independency
Scenario:
1. Creator opens a market with a mint that has a freeze authority.
2. Users stake; creator freezes the vault; claims and refunds fail.
Test: not attempted
Fix: Require `mint.freeze_authority.is_none()` or use an approved mint list.

### F-4: Creator can lock stakes for an arbitrarily long time

Severity: Medium
Location: src/lib.rs:39
Mechanism: `closes_at` has no upper bound, and refunds are only possible after `closes_at + RESOLVE_WINDOW` (line 99). A creator can set a far-future close time, so stakes are locked with no exit until then.

Pattern: dependency-independency
Scenario:
1. Creator sets `closes_at` to many years ahead.
2. Users who stake cannot claim or get a refund until that time.
Test: not attempted
Fix: Cap `closes_at - now` to a maximum horizon.

### F-5: One ticket per wallet per market

Severity: Low
Location: src/lib.rs:162
Mechanism: The ticket PDA is `init` with seeds (market, owner), so a second `place_stake` by the same wallet fails. Users cannot top up or back both sides, and must use extra wallets.

Pattern: divide-and-conquer
Test: not attempted
Fix: Use `init_if_needed` with a side check and accumulate the amount, or document the limit.

### F-6: Report timing and finality are unchecked

Severity: Low
Location: src/lib.rs:24
Mechanism: `post_report` can be called at any time, for any event_id, and is one-shot (PDA init), so a reporter that posts early or wrongly cannot correct it. Combined with first-caller-wins `resolve` (line 84), a wrong report becomes final.

Pattern: fail-fast
Test: not attempted
Fix: Require the report to be posted after the event time and add a dispute or correction path.

### F-7: Payout rounding dust is never recoverable

Severity: Info
Location: src/lib.rs:251
Mechanism: Division floors each winner's share, so a small remainder stays in the vault forever. The loss is negligible.

Pattern: checklist-arithmetic
Test: not attempted
Fix: Optional sweep after all claims, or ignore.

## Design findings

### D-1: Each market trusts a single, unaccountable reporter

Severity: Medium
Location: src/lib.rs:43
Mechanism: Reporters are self-registered (line 14), with no stake, reputation or dispute mechanism, and the creator picks one per market. Even after F-1 is fixed, a creator who is also the reporter can stake and then post whichever result benefits them; if the reporter never posts, funds are locked until the 14-day refund.

Pattern: dependency-independency
Test: not attempted
Fix: Add reporter bonding or multi-reporter quorum with disputes, and show the reporter prominently so stakers can judge it before staking.

## Coverage

| Item | Verdict | Evidence |
|---|---|---|
| root-of-trust | found | F-1 |
| identity-rejection | n/a | no allow/deny lists or per-wallet rules beyond F-5 |
| untrusted-executor | clear | resolve and claim are permissionless, src/lib.rs:79; refund path at line 99 |
| role-inversion | found | D-1 (reporter pushes the result) |
| lazy-loading | n/a | no loops or accrual |
| pay-as-you-act | clear | pools updated on every stake, src/lib.rs:65-67 |
| divide-and-conquer | found | F-5 |
| complexity-asymmetry | n/a | no loops, search or hints |
| commit-execution-split | found | F-2 |
| action-as-data | n/a | no stored actions |
| virtual-execution | n/a | no deferred obligations |
| one-way-state | clear | ticket closed on claim, src/lib.rs:186; winner set once, line 84 |
| boundary-of-consent | n/a | no computed rate the user must bound; payout is fixed by pools at close |
| internal-oracle | n/a | no prices from own state |
| defensive-computation | clear | u128 math, floor rounding, src/lib.rs:250-252 |
| fail-fast | found | F-6 |
| execution-layer-shift | n/a | no rollups or bridges |
| dependency-independency | found | F-3, F-4, D-1 |
| checklist-arithmetic | found | F-7; checked_add at src/lib.rs:67, u128 at 251 |
| checklist-duplicate-accounts | clear | vault fixed by `address = market.vault`, lines 160, 184; no two mutable same-type accounts |
| checklist-unsafe-and-panics | clear | no unsafe, no unwrap/index panics (side index is 0/1) |
| checklist-reload-after-cpi | clear | no account data read after CPI |
| checklist-token-2022 | clear | `Program<Token>` only, src/lib.rs:152, so Token-2022 is rejected |
| checklist-closing | clear | `close = owner` with has_one, src/lib.rs:186 |
| checklist-seed-collisions | clear | distinct prefixes b"market", b"vault", b"ticket", b"report", b"reporter" |
| checklist-init-frontrun | clear | PDAs include creator/owner/authority in seeds, so no one can front-run another's init |
| checklist-realloc | n/a | no realloc |
| checklist-authority-transfer | n/a | no authority transfer |
| checklist-signers-bumps-cpi | clear | market PDA signs with stored bump, src/lib.rs:105-114; token program typed |
