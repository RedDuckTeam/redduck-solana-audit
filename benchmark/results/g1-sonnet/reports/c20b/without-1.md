# Solana Audit Report: outcome_pools (program/src/lib.rs)

Severity: Critical
Location: program/src/lib.rs:176
Mechanism: `Resolve` never checks that `reporter` (or `report.reporter`) equals `market.reporter`, so `market.reporter` is never enforced. An attacker can register their own reporter, post a report for the market's `event_id` with the result they want, and call `resolve` to set the winner, then drain the vault through their winning tickets.

Severity: Medium
Location: program/src/lib.rs:86
Mechanism: `resolve` ignores `report.posted_at` and does not check that the event has finished. A reporter can post a result before or during the event, and the first caller locks it in. Each reporter can post an event only once, so a wrong report can't be corrected, and there is no dispute path.

Severity: Medium
Location: program/src/lib.rs:39
Mechanism: `closes_at` has no upper bound and the creator picks both it and the reporter. A creator can set a far-future close time, so stakes can't be refunded until `closes_at` plus 14 days. They can also choose a colluding reporter. Users must fully trust the creator.

Severity: Medium
Location: program/src/lib.rs:149
Mechanism: `create_market` accepts any mint without checking its freeze authority. A mint authority can freeze the vault token account, which blocks every claim and refund and locks all staked funds.

Severity: Medium
Location: program/src/lib.rs:23
Mechanism: (Architecture) Settlement relies on a single self-registered reporter per market. There is no multisig, oracle, stake or slashing. A reporter that withholds a report only forces a 14-day wait before refunds, and a malicious reporter can pick the winner. This is a centralization and liveness weakness.

Severity: Low
Location: program/src/lib.rs:162
Mechanism: The ticket PDA is seeded only by (market, owner), so each wallet can stake once per market. A second `place_stake` fails on `init`, so users can't top up or hedge both sides. Users who want several positions need extra wallets.

Severity: Info
Location: program/src/lib.rs:251
Mechanism: Payouts round down, so dust remains in the vault. The vault and market accounts are never closed, so their rent and any leftover tokens are unrecoverable.
