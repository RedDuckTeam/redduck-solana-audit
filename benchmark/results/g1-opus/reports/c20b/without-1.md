# Solana Audit Report — outcome_pools (`program/src/lib.rs`)

## 1. Resolve accepts any reporter, not the market's chosen reporter
Severity: Critical
Location: program/src/lib.rs:176
Mechanism: The `Resolve` accounts never check `reporter.key() == market.reporter`. `report` only has `has_one = reporter` and a matching `event_id`. So anyone can call `register_reporter` for themselves, `post_report` with the market's `event_id` and whichever result they want, and then call `resolve` (it is permissionless) as soon as staking closes. The attacker picks the winning side and takes the whole losing pool through their own winning ticket.

## 2. Report timing is not tied to the market, so a report can exist while staking is still open
Severity: Medium
Location: program/src/lib.rs:56
Mechanism: `create_market` lets the creator set any future `closes_at`, which has no link to the event's real start time. Neither `place_stake` nor `resolve` checks `report.posted_at` against `closes_at`. If `closes_at` is set after the event (by mistake or on purpose), or the reporter posts early, the result is public on-chain while staking is open. Anyone who knows the result can then stake on the winning side and take the losers' funds.

## 3. Single trusted reporter, chosen by the creator, with no dispute or correction path
Severity: Medium
Location: program/src/lib.rs:43
Mechanism: One reporter key alone decides each market, and a `Report` can never be changed (`init`, one per reporter and event). The creator can name themselves or a colluding party as reporter, stake, and then report the side that benefits them. A wrong report, honest or not, cannot be challenged or fixed.

## 4. Reporter can withhold the report to force refunds (free option / liveness)
Severity: Medium
Location: program/src/lib.rs:83
Mechanism: If no report is resolved within `RESOLVE_WINDOW`, every stake is refunded. A reporter who also staked, or who is bribed, can watch the real outcome and stay silent when their side lost. That turns the losing bet into a refund at the winners' expense, and any market depends on one party staying live.

## 5. Arbitrary mint lets the mint's freeze authority lock the vault
Severity: Low
Location: program/src/lib.rs:149
Mechanism: `create_market` accepts any SPL mint and does not check its `freeze_authority`. Whoever holds the freeze authority (possibly the creator) can freeze the vault after people stake, so every claim and refund fails and user funds stay locked or can be held for ransom.

## 6. Rounding dust and rent are stranded permanently
Severity: Low
Location: program/src/lib.rs:251
Mechanism: Payouts round down, so dust stays in the vault after every winning ticket is claimed. No instruction exists to sweep the vault or close the `Market`, vault or `Report` accounts, so these tokens and the rent-exempt lamports are locked forever.

## 7. Every stake write-locks the shared Market account
Severity: Low
Location: program/src/lib.rs:159
Mechanism: `place_stake` takes `market` as mutable to update `pools`. All stakes on one market therefore run one at a time through a single write-locked account. Close to `closes_at`, users compete for that lock, transactions get dropped, and spammers can crowd out other stakers.

## 8. One ticket per owner per market
Severity: Info
Location: program/src/lib.rs:162
Mechanism: The ticket PDA is seeded with `[market, owner]` and created with `init`. A second stake from the same wallet fails, so users cannot add to a position or hedge without using extra wallets. Integrators that expect top-ups will see their transactions fail.

## 9. Reporter names are not unique and can be impersonated
Severity: Info
Location: program/src/lib.rs:17
Mechanism: `name` is a free-form 32-byte field with no uniqueness check. An attacker can register a reporter with the same name as a trusted one. Then they create markets that point to it, so users who identify reporters by name are tricked into trusting an attacker-controlled oracle.
