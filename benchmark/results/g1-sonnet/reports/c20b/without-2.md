# Audit: outcome_pools (program/src/lib.rs)

Severity: Critical
Location: program/src/lib.rs:173-179
Mechanism: `Resolve` never checks that `reporter` (or `report.reporter`) equals `market.reporter`. `has_one = reporter` only compares the report to whatever reporter account the caller passes. Anyone can register their own reporter (`register_reporter` is permissionless), post a report with the target `event_id` and the result they want, and call `resolve` with it. They then claim the whole vault.

Severity: Medium
Location: program/src/lib.rs:38-52
Mechanism: `create_market` accepts any creator-supplied `mint` and does not check for a freeze authority. The creator can pick a mint they can freeze, then freeze the vault or individual accounts, which locks stakers' funds or blocks claims. The doc comment says staking closes before the event starts, but `closes_at` is not tied to the event, so a creator can also set a close time after the outcome is known.

Severity: Medium
Location: program/src/lib.rs:79-91
Mechanism: `resolve` does not check `report.posted_at` against `market.closes_at`, and a report is final with no dispute or correction path. A reporter can post early, or post a wrong result, and anyone can immediately lock it in as the winner. Settlement depends entirely on one creator-chosen reporter.

Severity: Medium
Location: program/src/lib.rs:79-117
Mechanism: Liveness and centralization. If the reporter never posts, or nobody calls `resolve`, funds are locked until `closes_at + 14 days`. There is no way to cancel a market, and the 14-day window is fixed. A malicious or offline reporter can freeze all stakes for that period. A reporter who is also the creator can resolve in their own favor.

Severity: Low
Location: program/src/lib.rs:162
Mechanism: The ticket PDA is seeded only by (market, owner) and is created with `init`. Each user can stake once per market, and the stake can't be topped up or placed on both sides. A second `place_stake` fails, so users can't adjust positions and there is no cancel or withdraw before close.

Severity: Low
Location: program/src/lib.rs:94-117
Mechanism: `claim` never closes the vault or the market, so their rent stays locked permanently. Integer-division dust from the pro-rata payout (line 251) also stays in the vault with no way to sweep it.

Severity: Info
Location: program/src/lib.rs:56
Mechanism: `place_stake` has no check on a minimum amount or a maximum stake window, and `closes_at` has no upper bound. A creator can set a far-future close time, which pushes the refund path (`closes_at + RESOLVE_WINDOW`) arbitrarily far out and locks any stakes made in the meantime.
