# Solana audit report: outcome_pools

## Scope
`program/src/lib.rs` (Anchor, single program). Instructions: register_reporter, post_report, create_market, place_stake, resolve, claim. No toolchain exercised; findings are from source review.

## Summary
1 High, 3 Medium, 3 Low. Most important: the market creator picks any self-registered reporter, so the creator controls the outcome and can drain stakers (F-1).

## Findings

### F-1: Market reporter is creator-chosen and unvetted, so the creator controls the outcome

Severity: High
Location: src/lib.rs:148
Mechanism: `create_market` accepts any permissionlessly registered `Reporter` (line 14), and `post_report` (line 24) verifies nothing about the event, so a creator registers their own reporter, stakes on one side, posts that side as the result, resolves, and claims the victims' pool.

Pattern: root-of-trust
Scenario:
1. Attacker registers reporter R and creates a market using R.
2. Attacker stakes 1 unit on Yes; victims stake on No.
3. After `closes_at`, attacker posts a Yes report with R and calls `resolve`.
4. Attacker claims `amount * total / winning`, i.e. the whole vault if they are the only Yes staker.
Test: unconfirmed (no toolchain)
Verification: downgraded by verifier from Critical: only markets the attacker created are drainable, and the creator-chosen reporter is documented behaviour (line 13), but the chain of trust ends at an attacker-created account.
Fix: Root the reporter in a trusted source (admin-curated reporter registry or allowlist PDA, or a staked/bonded reporter with a dispute window), and show/validate the reporter at stake time.

### F-2: Reports can be posted at any time, enabling insider staking and early results

Severity: Medium
Location: src/lib.rs:24
Mechanism: `post_report` has no time check against the market or event, so a reporter can publish the result before `closes_at` and stake on the known winner (or a creator-reporter can do so), and `resolve` (line 79) never uses `posted_at`.

Pattern: commit-execution-split
Scenario: Reporter posts a result before staking closes, then stakes the winning side with certainty; honest stakers are adversely selected.
Test: not attempted
Fix: Reject reports before `closes_at` (or a market-defined event end) and make staking close before any report is acceptable.

### F-3: Resolution is irreversible with no dispute or correction path

Severity: Medium
Location: src/lib.rs:87
Mechanism: `resolve` sets `winner` once from a single reporter's report (one report per reporter per event, PDA init at line 134), with no dispute, challenge window or fallback, so one wrong or malicious report permanently decides all payouts.

Pattern: one-way-state
Scenario: Reporter posts an incorrect result; anyone calls `resolve` immediately; claims pay out the wrong side.
Test: not attempted
Fix: Add a challenge period or multi-reporter quorum before the winner is final.

### F-4: Creator can choose an arbitrary mint and an arbitrarily distant close time, locking stakes

Severity: Medium
Location: src/lib.rs:38
Mechanism: `closes_at` has no upper bound and the mint (line 149) is unrestricted, so a creator can pick a far-future close time, or a mint with a freeze authority, and stakes cannot be withdrawn (no cancel/withdraw instruction) until the refund window after `closes_at`, or ever if the mint freezes the vault.

Pattern: checklist-token-2022
Scenario: Creator sets `closes_at` near `i64::MAX` or uses a freezable mint; victims stake and cannot exit.
Test: not attempted
Fix: Cap the market duration, allowlist mints and reject mints with a freeze authority.

### F-5: One ticket per owner per market; stakes cannot be added or placed on both sides

Severity: Low
Location: src/lib.rs:162
Mechanism: The ticket PDA is `init` with seeds [ticket, market, owner], so a second `place_stake` by the same owner fails, forcing users to use separate wallets and making per-wallet limits trivial to evade.

Pattern: identity-rejection
Test: not attempted
Fix: Use `init_if_needed` with a side check and add to the amount, or key tickets by an index.

### F-6: No slippage or consent bounds on payout and stakes are unbounded by odds

Severity: Low
Location: src/lib.rs:54
Mechanism: `place_stake` takes no bound on the pool state, so a user's payout depends on pools that other stakers can change until close (for example, a late large stake on the same side dilutes the payout).

Pattern: boundary-of-consent
Test: not attempted
Fix: Accept a minimum expected payout or maximum pool ratio.

### F-7: Reporter name unvalidated, reporter has no identity or reputation

Severity: Low
Location: src/lib.rs:14
Mechanism: `name` is free-form and the PDA is keyed only by authority, so anyone can impersonate a trusted reporter's name in the UI to attract stakes into a market they control.

Pattern: root-of-trust
Test: not attempted
Fix: Display and rely on the reporter authority key, not the name; use a curated registry.

## Design findings

### D-1: Resolution depends on a single reporter's liveness

Severity: Medium
Location: src/lib.rs:79
Mechanism: If the chosen reporter never posts, stakers are locked until `closes_at + 14 days` and then refunded (line 99), so a reporter can grief or hold funds for two weeks (and permanently in combination with F-4); there is no way to switch reporter.

Pattern: dependency-independency
Test: not attempted
Fix: Allow reporter replacement or fallback resolution by a quorum.

## Coverage

| Item | Verdict | Evidence |
|---|---|---|
| root-of-trust | found | F-1, F-7 |
| identity-rejection | found | F-5 |
| untrusted-executor | clear | `resolve` is permissionless, src/lib.rs:79; refund path needs no operator, line 99 |
| role-inversion | n/a | reporter pushes data but it is the root-of-trust problem (F-1) |
| lazy-loading | n/a | no iteration or accrual |
| pay-as-you-act | clear | pools updated on stake, line 67 |
| divide-and-conquer | clear | per-user ticket PDAs, line 162 |
| complexity-asymmetry | n/a | no loops or client hints |
| commit-execution-split | found | F-2 |
| action-as-data | n/a | no stored actions |
| virtual-execution | n/a | no deferred obligations |
| one-way-state | found | F-3 |
| boundary-of-consent | found | F-6 |
| internal-oracle | n/a | no derived prices |
| defensive-computation | clear | checked u128 math, floor rounding favours the vault, lines 251-253 |
| fail-fast | clear | window and event checks, lines 82-84, 178 |
| execution-layer-shift | n/a | no rollup or delegation |
| dependency-independency | found | D-1 |
| checklist-arithmetic | clear | checked_add / checked_mul, lines 67, 252 |
| checklist-duplicate-accounts | clear | vault fixed by `address = market.vault`; ticket PDA-bound, lines 160, 185 |
| checklist-unsafe-and-panics | clear | no unsafe; indices from enum, line 203 |
| checklist-reload-after-cpi | clear | no state read after CPI that depends on token balances |
| checklist-token-2022 | found | F-4 (mint unrestricted); only the legacy token program is accepted |
| checklist-closing | clear | `close = owner, has_one` at line 187; tickets cannot be re-created after close since staking is over |
| checklist-seed-collisions | clear | distinct prefixes and key components, lines 122, 134, 144, 146, 162 |
| checklist-init-frontrun | clear | PDA seeds include the creator/owner signer |
| checklist-realloc | n/a | no realloc |
| checklist-authority-transfer | n/a | no authority transfer; reporter authority fixed |
| checklist-signers-bumps-cpi | clear | stored bump and market seeds signer, lines 105-114 |
