# Solana Audit Report: outcome_pools (program/src/lib.rs)

Severity: High
Location: program/src/lib.rs:38
Mechanism: `create_market` only requires `closes_at` to be in the future, with no upper bound. A creator can set `closes_at` near `i64::MAX`. Stakes are then locked forever, because `resolve` needs `now >= closes_at` and the refund path in `claim` (line 99) needs `now >= closes_at + RESOLVE_WINDOW`, which saturates to `i64::MAX`.

Severity: High
Location: program/src/lib.rs:38
Mechanism: The doc says staking closes before the event starts, but nothing enforces it, and `post_report` (line 24) never checks that the event has finished or that `posted_at` is after `closes_at`. A creator, or a reporter they work with, can set `closes_at` after the result is posted. The creator can then stake on the known winner and drain honest stakers' funds, and anyone can do the same when a report is posted before staking closes.

Severity: High
Location: program/src/lib.rs:43
Mechanism: The creator picks any `Reporter` for the market, and the reporter is the only source of truth. There is no dispute process, no staking or bonding, and no check that the reporter is independent. A creator can register their own reporter, open a market, stake, and post the result that pays them. Stakers must trust the creator and the reporter completely, and a wrong report cannot be corrected, because each reporter can post an event only once (line 134).

Severity: Medium
Location: program/src/lib.rs:79
Mechanism: Liveness depends on a single reporter. If the reporter withholds the report, resolution is impossible until the 14-day window ends, and all funds stay locked until then. A reporter who sees the stake distribution can also choose to withhold the report and force refunds, or post it only if it benefits them. `resolve` reads `report.result` without checking `posted_at`.

Severity: Medium
Location: program/src/lib.rs:44
Mechanism: `create_market` accepts any mint without checks. A creator can use a malicious mint, or one with a freeze authority, and freeze the vault or the stakers' token accounts. This blocks `claim` and traps funds. The program never checks that the mint is trusted.

Severity: Low
Location: program/src/lib.rs:162
Mechanism: The ticket PDA is derived from (market, owner), so each wallet can stake once, on one side. It cannot top up, hedge or change a stake. Large stakers must split across wallets, so the ticket design does not scale and encourages Sybil wallets.

Severity: Low
Location: program/src/lib.rs:252
Mechanism: Payouts round down, so dust stays in the vault forever. Nothing closes the vault or the market account, so their rent is never recovered. This is a minor scalability and resource leak.

Severity: Info
Location: program/src/lib.rs:213
Mechanism: `Reporter.reports` is a counter that nothing uses. Reporters have no reputation, bonding or slashing, even though the design relies on them for correctness.
