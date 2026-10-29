# Solana Audit Report: outcome_pools (`program/src/lib.rs`)

## 1. Resolve accepts a report from any reporter, not the one the market chose
Severity: Critical
Location: program/src/lib.rs:176
Mechanism: The `Resolve` accounts only check that `report.reporter == reporter` and that the event IDs match. Nothing checks `reporter.key() == market.reporter`. Anyone can call `register_reporter`, `post_report` for the market's `event_id` with whichever side they want, then call `resolve` (permissionless) right after `closes_at`. They fix the winner and can drain the vault through a winning ticket.

## 2. Report timing is never checked; a report posted before staking closes is accepted
Severity: Medium
Location: program/src/lib.rs:86
Mechanism: `resolve` ignores `report.posted_at`, and `post_report` can be called at any time. A report can therefore exist before `closes_at`, which is before the event has even started. The reporter (or anyone who sees the account on-chain) knows the "result" while staking is still open and can stake on the winning side. A premature or guessed report also becomes final the moment staking closes.

## 3. Single reporter has unchecked, final control of every outcome (centralization)
Severity: Medium
Location: program/src/lib.rs:79
Mechanism: The outcome comes from a single reporter's word, with no dispute period, quorum, bond or slashing. The creator picks the reporter and can be that reporter. A dishonest or compromised reporter, or a creator-reporter who also stakes, can post a false result and take the losing pool. Stakers can't challenge it.

## 4. Reports are immutable and the first resolve is final; mistakes can't be fixed
Severity: Low
Location: program/src/lib.rs:84
Mechanism: The report PDA is `init`-only and can never be updated, and `resolve` permanently sets `winner` the first time it runs. An honest reporter's typo, or a report for a reused or mis-mapped `event_id`, settles the market wrongly and can't be reversed.

## 5. Liveness depends on one reporter; funds are locked 14 days if it is absent
Severity: Low
Location: program/src/lib.rs:99
Mechanism: If the chosen reporter never posts, no one can resolve the market. Every staker's funds stay locked until `closes_at + RESOLVE_WINDOW` (14 days). There is no fallback reporter and no early cancel or refund path.

## 6. `closes_at` is not tied to the event, which allows late betting
Severity: Low
Location: program/src/lib.rs:39
Mechanism: `closes_at` only has to be in the future. The program has no link between `event_id` and the event's real start time. A careless or malicious creator can set `closes_at` after the event has finished, and then informed users can stake on the known result and take the other side's pool.

## 7. Mint freeze authority can lock the vault
Severity: Low
Location: program/src/lib.rs:149
Mechanism: Any mint is accepted, including one with a freeze authority. That authority can freeze the market vault (or winners' accounts), which blocks every claim and refund indefinitely.

## 8. One ticket per user per market; a user can't add to or change their stake
Severity: Low
Location: program/src/lib.rs:162
Mechanism: The ticket PDA is seeded only by `(market, owner)` and uses `init`, so a second `place_stake` from the same wallet fails. Users can't top up or hedge without switching wallets. This is a usability problem and pushes users toward Sybil wallets.

## 9. Rounding dust and rent are locked forever
Severity: Info
Location: program/src/lib.rs:251
Mechanism: Payouts round down, so leftover dust stays in the vault. The program has no instruction to sweep it or to close the vault and market accounts, so the creator's rent and the dust are locked permanently.

## 10. Hot market account serializes all stakes (scalability)
Severity: Info
Location: program/src/lib.rs:158
Mechanism: Every `place_stake` writes `market.pools`, so the market account is write-locked by every stake. All stakes on a popular market are processed one at a time, which limits throughput near `closes_at`, exactly when demand peaks.

## 11. The reporter's `reports` counter is a write-locked hotspot
Severity: Info
Location: program/src/lib.rs:132
Mechanism: `post_report` marks the reporter account `mut` just to increment `reports`. All reports from one reporter are serialized, and the counter is not used anywhere else.
