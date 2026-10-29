# Solana Audit Report: outcome_pools (`program/src/lib.rs`)

Scope: `program/src/lib.rs`, `program/Cargo.toml` (Anchor 0.31.1).

## 1. Resolve accepts a report from any reporter, not the market's chosen one

Severity: Critical
Location: program/src/lib.rs:176
Mechanism: The `Resolve` accounts check that `report.reporter == reporter` and that `report.event_id == market.event_id`, but they never check that `reporter.key() == market.reporter`. An attacker can call `register_reporter` for themselves, call `post_report` with the market's `event_id` and the side they staked on, then call `resolve` as soon as `closes_at` passes. Because the first resolve is final, they decide the outcome of any market and collect the losers' pool.

## 2. Staking stays open after the result is already public on-chain

Severity: High
Location: program/src/lib.rs:56
Mechanism: `place_stake` only checks `closes_at`. Neither `post_report` nor `resolve` requires the report to be posted at or after `closes_at` (`report.posted_at` is never checked). If the reporter posts before `closes_at` (for example because the creator set `closes_at` after the event ended, by mistake or on purpose), anyone can read the report and stake on the known winner with no risk, taking the losers' funds.

## 3. A single reporter has full, unaccountable control over each outcome

Severity: High
Location: program/src/lib.rs:24
Mechanism: The outcome comes from one reporter key, with no dispute period, bond, quorum or way to correct a report (the report PDA can be posted once and never changed). The creator can name themselves (or a sock puppet) as the reporter, stake, and then post a false result. They also hold a free option: report only when their side wins, and otherwise stay silent so that everyone just gets a refund after `RESOLVE_WINDOW`.

## 4. Resolution is first-call-final and can't be fixed

Severity: Medium
Location: program/src/lib.rs:84
Mechanism: `resolve` is permissionless and sets `winner` permanently the first time it runs. There is no challenge window or admin override. A wrong or early report (including the issues in findings 1 and 2) can't be reversed once someone calls `resolve`, and anyone may call it the moment `closes_at` passes.

## 5. Funds depend on the reporter key staying available, and the key can't be rotated

Severity: Low
Location: program/src/lib.rs:14
Mechanism: `Reporter.authority` is fixed at registration, and the program has no instruction to rotate or delegate it. If the key is lost or compromised, every market that uses it either can't resolve (funds wait 14 days for a refund) or is controlled by whoever holds the compromised key.

## 6. Arbitrary mint lets the mint's freeze authority lock the vault

Severity: Low
Location: program/src/lib.rs:149
Mechanism: `create_market` accepts any SPL mint. If the mint has a freeze authority (which a malicious creator could control), that authority can freeze the vault after people stake, blocking every claim and refund.

## 7. Each wallet gets one ticket per market and can't add to it

Severity: Low
Location: program/src/lib.rs:162
Mechanism: The ticket PDA is seeded only by `(market, owner)` and created with `init`, so a wallet can stake once, on one side, and can't top up. Users have to spread their stakes across several wallets, and integrators that expect to add to a stake will get failed transactions.

## 8. Rounding dust and rent are locked forever

Severity: Low
Location: program/src/lib.rs:251
Mechanism: Pro-rata payouts round down, so leftover tokens stay in the vault, and no instruction sweeps them or closes the vault or market accounts. The dust and the rent for the market and vault accounts are locked permanently.

## 9. Every stake write-locks the market account, which limits throughput

Severity: Info
Location: program/src/lib.rs:159
Mechanism: Every `place_stake` writes the same `Market` account (to update `pools`). As a result, all stakes on a popular market run one at a time and compete for the same write lock, especially near `closes_at`, so transactions fail or get dropped exactly when demand peaks.

## 10. Pari-mutuel odds can be sniped at the last moment

Severity: Info
Location: program/src/lib.rs:56
Mechanism: Payouts depend on the final pool sizes, and staking is allowed until the exact `closes_at` second. A large staker who places a stake at the last moment can shift the odds after others have committed, and nobody can respond.
