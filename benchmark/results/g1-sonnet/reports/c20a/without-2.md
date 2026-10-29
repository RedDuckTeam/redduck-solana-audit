# Solana audit report: outcome_pools (program/src/lib.rs)

Severity: High
Location: program/src/lib.rs:79
Mechanism: `resolve` never checks `report.posted_at` against `market.closes_at`, and `post_report` (line 24) does not check that the event has finished. A reporter can post a result before or during staking, and the report account is public. A market creator can also pick an `event_id` that already has a report, so the outcome is known before anyone stakes. Insiders then stake on the known winner and drain the losing pool. Fix: require `report.posted_at >= market.closes_at`, and bind each market to a report that did not exist when it was created.

Severity: High
Location: program/src/lib.rs:38
Mechanism: The market creator picks any reporter (line 43), and `register_reporter` is permissionless. A creator can register their own reporter, open a market, attract stakers, and then post whichever result favors them. Settlement trusts a single party chosen by the creator, with no dispute, bond, quorum or allowlist.

Severity: Medium
Location: program/src/lib.rs:94
Mechanism: Settlement has a single point of failure and a liveness problem. If the one reporter never posts within `RESOLVE_WINDOW` (14 days) after `closes_at`, every stake is locked for that time and then refunded. A reporter can also post late on purpose to force a refund after seeing the market's imbalance. Funds are never lost permanently, but the market is easy to grief.

Severity: Medium
Location: program/src/lib.rs:38
Mechanism: `closes_at` has no upper bound and no link to the event's real start time (the comment says it should close before the event starts). A creator can set `closes_at` far in the future, which locks stakes and, through `saturating_add` at lines 83 and 99, delays both resolution and refunds. They can also set it after the event has started, so stakers bet with the result partly known.

Severity: Low
Location: program/src/lib.rs:162
Mechanism: The ticket PDA is seeded only by (market, owner), so each wallet can stake once per market. A second `place_stake` fails on `init`, and a wallet cannot stake on both sides or top up. Users must use extra wallets, which hurts scalability and usability. There is no way to cancel or withdraw a stake before close.

Severity: Low
Location: program/src/lib.rs:146
Mechanism: Any mint is accepted, including mints with a freeze authority. The mint's freeze authority can freeze the vault or the claimants' token accounts, which locks all funds in the market. Only the legacy Token program is supported, so there is no Token-2022 fee risk.

Severity: Info
Location: program/src/lib.rs:14
Mechanism: The reporter `name` is unvalidated, so reporters can impersonate others (spoofing). The reporter authority cannot be rotated. A compromised reporter key lets an attacker settle every market that uses that reporter.

Severity: Info
Location: program/src/lib.rs:252
Mechanism: Payouts use floor division. This is safe because the vault stays solvent, but dust remains in the vault permanently. The vault is never closed, and its rent cannot be reclaimed.
