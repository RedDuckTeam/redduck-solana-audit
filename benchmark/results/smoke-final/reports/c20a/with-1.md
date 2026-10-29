# Solana audit report: outcome_pools

## Scope
`program/src/lib.rs` (Anchor 0.31.1, SPL Token only). Instructions: register_reporter, post_report, create_market, place_stake, resolve, claim. Reviewed by manual reading. No toolchain tests were run. The per-pattern reference files were not opened, so the coverage verdicts come from reading the code directly.

## Summary
0 Critical, 0 High, 2 Medium, 2 Low, 2 Info. Fund accounting is sound: payouts never exceed the pools, tickets close on claim, and the vault authority is the market PDA. The main risks are in the resolution design. The market creator picks the sole reporter, and a report can neither be corrected nor be tied to a specific market.

## Findings

### F-1: Reports are single-shot and shared across markets, so an early or mistimed report permanently blocks resolution
Severity: Medium
Location: program/src/lib.rs:85
Mechanism: A report PDA is `[reporter, event_id]` and can be posted once (init at :135). `resolve` requires `report.posted_at >= market.closes_at`. A report posted before a market's `closes_at` can never be replaced, so that market can never resolve. This happens when the reporter posts early, by mistake or on purpose, or when several markets with the same event_id but different close times share the reporter. Stakers get funds back only after the 14-day refund window.

Pattern: one-way-state
Scenario:
1. Creators A and B each open a market on event 7 with the same reporter, with close times T1 < T2.
2. The reporter posts the result for event 7 at a time between T1 and T2.
3. A's market resolves. B's market fails `EarlyReport` forever, and the reporter cannot re-post.
4. B's stakers wait until T2 + 14 days for refunds.
Test: not attempted
Fix: Bind the report to a market, or allow a reporter to supersede a report until it is consumed. Alternatively, have the market record its own `closes_at` in the report seeds.

## Design findings

### D-1: A single creator-chosen reporter decides every outcome, with no dispute or correction
Severity: Medium
Location: program/src/lib.rs:43
Mechanism: `register_reporter` is permissionless (:14), and the market creator picks any reporter at :43. The creator can register themselves, stake on a side, and post the result they prefer. `resolve` only checks `posted_at >= closes_at` (:85), not that the event has finished, and the result is final once resolved. Stakers must trust the reporter. Nothing on-chain bonds, disputes or rotates the reporter.

Pattern: root-of-trust
Scenario:
1. The attacker registers a reporter and creates a market with it.
2. Victims stake on both sides.
3. After `closes_at`, the attacker posts the result favouring their own side and calls `resolve`.
4. The attacker's winning ticket drains the losing pool.
Test: not attempted
Fix: Use a reporter allow-list or a bonded, disputable reporter set. Require the report time to be after an event end time stored in the market. Show the reporter identity clearly in clients.

### D-2: Liveness depends on a single reporter key, with no fallback other than a 14-day full refund
Severity: Low
Location: program/src/lib.rs:7
Mechanism: If the reporter key is lost, goes offline, or is late by more than `RESOLVE_WINDOW`, every stake is refunded. The funds are safe, but the market is useless and the capital is locked for 14 days after close.

Pattern: dependency-independency
Scenario: The reporter never posts, and stakers wait until `closes_at + 14d` to claim refunds.
Test: not attempted
Fix: Allow a fallback reporter, or a shorter or configurable window.

### D-3: `closes_at` has no upper bound and the mint is unrestricted
Severity: Low
Location: program/src/lib.rs:39
Mechanism: A creator can set a far-future `closes_at`, which locks stakes until that time plus 14 days. A creator can also choose a mint with a freeze authority (:150), and that authority can freeze the vault so no one can claim. Stakers must vet both.

Pattern: boundary-of-consent
Scenario: The creator makes a market with a mint they can freeze, then freezes the vault after stakes arrive.
Test: not attempted
Fix: Cap the duration, and reject mints that have a freeze authority or are not allow-listed.

### D-4: Reporter names are free-form and unauthenticated
Severity: Info
Location: program/src/lib.rs:14
Mechanism: Anyone can register a reporter with any 32-byte `name`, including one that impersonates a well-known reporter. This only matters if a UI displays the name as a trust signal.

Pattern: identity-rejection
Test: not attempted
Fix: Identify reporters by authority key, not by name.

### D-5: Payout rounding dust stays in the vault, and one ticket allows one stake per user per market
Severity: Info
Location: program/src/lib.rs:253
Mechanism: Floor division leaves dust in the vault, and the vault is never closed. The ticket PDA (:163) prevents a second stake from the same wallet on the same market.

Pattern: defensive-computation
Test: not attempted
Fix: Optional. Sweep the dust, and allow top-ups.

## Coverage
| Item | Verdict | Evidence |
|---|---|---|
| root-of-trust | found | D-1; vault, market and ticket are PDAs or address-checked (:161, :177, :186) |
| identity-rejection | found | D-4 |
| untrusted-executor | clear | `resolve` and `claim` need no privileged caller (:79) |
| role-inversion | found | D-1 |
| lazy-loading | n/a | no loops or accruals |
| pay-as-you-act | clear | pools are updated on every stake (:67) |
| divide-and-conquer | clear | per-user ticket PDAs (:163) |
| complexity-asymmetry | n/a | no unbounded work |
| commit-execution-split | n/a | no randomness. Staking closes before resolution (:56) |
| action-as-data | n/a | no stored actions |
| virtual-execution | n/a | none |
| one-way-state | found | F-1. Ticket closed on claim (:188), `AlreadyResolved` (:84) |
| boundary-of-consent | found | D-3. Payout is pro rata with no user bound, but the stake is final |
| internal-oracle | n/a | none |
| defensive-computation | clear | u128 checked math (:252-254). Info D-5 |
| fail-fast | found | F-1 |
| execution-layer-shift | n/a | none |
| dependency-independency | found | D-2 |
| checklist-arithmetic | clear | checked_add/mul (:67, :253) |
| checklist-duplicate-accounts | clear | no two mutable accounts of the same type are aliased |
| checklist-unsafe-and-panics | clear | no unsafe code or unwraps |
| checklist-reload-after-cpi | clear | no state is read after a CPI |
| checklist-token-2022 | clear | `Token` program only (:153, :169, :194) |
| checklist-closing | clear | `close = owner` (:188) |
| checklist-seed-collisions | clear | distinct prefixes (:123, :135, :145, :147, :163) |
| checklist-init-frontrun | clear | PDAs are keyed to the signer (:123, :145) |
| checklist-realloc | n/a | none |
| checklist-authority-transfer | n/a | no transfer instruction |
| checklist-signers-bumps-cpi | clear | market PDA signs with a stored bump (:108) |
