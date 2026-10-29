# Solana audit: outcome_pools (program/src/lib.rs)

Severity: High
Location: program/src/lib.rs:38
Mechanism: Any creator can attach any self-registered reporter (registration at line 14 is permissionless) to a market. A malicious creator can register their own reporter, open a market, wait for stakes, then post whichever result they like and drain the pool. Settlement trust rests on one unvetted key, with no allowlist, no stake or bond, no dispute process and no multi-reporter quorum.

Severity: High
Location: program/src/lib.rs:24
Mechanism: `post_report` does not check the time. A reporter can publish a result before `closes_at`, or before the event has happened, and `resolve` (line 79) only checks `now >= closes_at`. Since staking closes before the event starts, a market can be resolved on a report posted before the event finished. A reporter who posts before staking closes lets others, or themselves, stake on the known winner. `Report.posted_at` is stored but never used.

Severity: Medium
Location: program/src/lib.rs:79
Mechanism: Resolution is final and the report is immutable (one report per reporter and event, PDA `init`). If the reporter makes a mistake or is compromised, anyone can lock in the wrong winner and the funds are paid out wrongly. There is no dispute window, correction or override.

Severity: Medium
Location: program/src/lib.rs:146
Mechanism: The creator chooses the mint with no validation. A mint with a freeze authority can freeze the vault, or any staker's token account, and lock all funds. A malicious or non-standard mint can also be used to build misleading markets. Require a mint allowlist, or require that no freeze authority is set.

Severity: Medium
Location: program/src/lib.rs:94
Mechanism: Liveness and centralization. Every payout depends on the single reporter key (`Reporter.authority`, with no rotation or recovery). If the reporter withholds or loses the key, all stakers are locked until `closes_at + 14 days` and then only get refunds. A reporter who dislikes the outcome can deliberately withhold the report to force refunds (griefing). The window is a fixed constant and cannot be changed.

Severity: Low
Location: program/src/lib.rs:162
Mechanism: The ticket PDA is seeded only by market and owner, so each wallet can stake once per market. A second `place_stake` fails because the account already exists. A user cannot top up or hedge the other side from the same wallet. Funds are not at risk, but anyone who wants to add to a position must use a new wallet.

Severity: Low
Location: program/src/lib.rs:38
Mechanism: `closes_at` has no upper bound. A creator can set a very distant time, such as `i64::MAX`, and stakers have no exit until `closes_at + RESOLVE_WINDOW`, so the funds are locked indefinitely. There is also no minimum lead time before the event.

Severity: Low
Location: program/src/lib.rs:252
Mechanism: Payouts use floor division, so dust from rounding stays in the vault permanently. The vault is never closed and the market is never closed, so the rent for the market and vault accounts is never recovered. Tickets are also not closed by the creator.

Severity: Info
Location: program/src/lib.rs:14
Mechanism: Reporter registration is permissionless and `name` is unvalidated. Reporters can be impersonated by name (look-alike names), which can mislead users into trusting a malicious reporter. The client UI must show the authority key.
