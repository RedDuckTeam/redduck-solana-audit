# Solana Audit Report: outcome_pools

Severity: Critical
Location: program/src/lib.rs:175
Mechanism: `Resolve.reporter` is never checked against `market.reporter`, and `report` only has `has_one = reporter` against that caller-supplied account. An attacker can register their own reporter, post a false report for the same `event_id`, and call `resolve` to set any winner. They then drain the vault.

Severity: Medium
Location: program/src/lib.rs:79
Mechanism: Resolution trusts a single reporter's report with no dispute, correction or multi-party mechanism. `posted_at` is never checked against `closes_at` or the event time, and a report is final once posted. A malicious or mistaken reporter (possibly the market creator's own choice) can settle early or wrongly and redirect all funds.

Severity: Medium
Location: program/src/lib.rs:146
Mechanism: The creator picks any mint, and `mint` is not checked for a freeze authority, a mint authority or non-standard behavior. A creator-controlled mint can freeze the vault or the users' accounts, locking stakes. Users have no on-chain guarantee that the mint is safe.

Severity: Low
Location: program/src/lib.rs:162
Mechanism: The ticket PDA is seeded only by market and owner, and `init` is used. Each user can stake only once per market, on one side, and cannot top up or hedge. A second call fails, and a griefer can't block others, but the design limits usability.

Severity: Low
Location: program/src/lib.rs:38
Mechanism: `closes_at` has no upper bound, and nothing requires it to be before the event starts. A creator can set it far in the future to hold stakes for a long time, or set it so that staking stays open after the event outcome is known, which lets informed users stake on the winner.

Severity: Info
Location: program/src/lib.rs:251
Mechanism: Pro-rata payout uses floor division, so rounding dust stays in the vault forever. No instruction closes the vault or the market, so rent and dust are never reclaimed.

Severity: Info
Location: program/src/lib.rs:188
Mechanism: `owner_token` is only checked for mint, not for owner. This is safe because the ticket owner signs, but a mistaken destination account sends payouts to a third party.
