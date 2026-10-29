---
id: c10
crate: fx_corridor
pattern: role-inversion
class: design
group: g1
vulnerable: a
role: heldout
author: case-author
canary: RDSAF81C11F051B132899533
---
# Case c10

`fx_corridor` is an FX desk that converts between two stable tokens at a rate signed by an external publisher. `convert` trades only on the rate stored in the desk. `Desk.usable_rate` requires `now - rate_published_at <= max_rate_age`. The stored rate is written by `post_rate`, which checks the publisher's Ed25519 signature through instruction introspection. The quotes are public: per `ARCHITECTURE.md`, the publisher serves the latest signed quote from an HTTPS endpoint.

The planted flaw is a design one. `PostRate` accepts only `desk.operator` as submitter, and `ARCHITECTURE.md` says a team relay posts every 30 seconds with that key. The party that needs a fresh rate is the converting user. That user could fetch the signed quote and carry it in its own transaction, and the program already has everything needed to verify it. Instead every conversion depends on the team's relay staying up. The operator gate adds no safety, because the publisher signature is verified either way.

## Attack scenario
This is a design case. The concrete failures:
1. The team's relay stops: an outage, an expired cloud account, a lost or rotated operator key, or a team that winds down. The publisher keeps serving signed quotes.
2. 120 seconds after the last post, `Desk.usable_rate` fails with `RateExpired` for every `convert`. Every user of the desk is blocked. The liquidity sits idle, and no one but the operator key can restore service, even though valid signed quotes are publicly available.
3. While the relay is merely slow, conversions run on a rate up to `max_rate_age` (120 s) old. The rate is chosen by the relay's schedule, not by the user's transaction. Anyone watching the publisher's endpoint can see the market move and convert against the stale desk rate before the next post. The team has to keep `max_rate_age` wide enough to absorb relay jitter, which widens this window.

## Why fixed closes it
`src/instructions/rate.rs`, two hunks:
- `PostRate.desk` loses `constraint = desk.operator == submitter.key()`. Any signer may submit a publisher-signed quote. The program still requires the Ed25519 verify instruction right before `post_rate`, with the instructions-sysvar address pinned, the Ed25519 program id checked, all three offset indices pointing into that instruction, the public key equal to `desk.publisher` and the message equal to `FXQUOTE1 || base mint || quote mint || rate || published_at`. It also still rejects future timestamps and zero rates.
- A quote that is not newer than the stored one now returns `Ok(())` without writing, instead of failing with `StaleQuote`. Two users who bundle the same or an older quote in one slot therefore do not make each other's conversions fail.

`src/errors.rs`: the `StaleQuote` variant is no longer used and is removed.

`ARCHITECTURE.md`: the relay bullet is removed. Partner wallets now put `[Ed25519 verify, post_rate, convert]` in the user's own transaction. With fresh quotes on every conversion, `max_rate_age` drops from 120 s to 15 s.

Rust changes: 4 lines removed and 4 added. ARCHITECTURE.md: 3 removed and 2 added. That is 13 diff lines in total: within the 30 allowed for design cases, because the deployment description has to change along with the code.

The scenario no longer works. If the team disappears, users keep converting by bringing the publisher's quote themselves, and the rate a conversion uses is at most 15 s old. The normal flow still works: initialize, provide liquidity, post a quote, convert, pause, withdraw liquidity. A relay may still post quotes, but nothing depends on it.

## Decoys
- `set_pause` is also an operator-driven control, the same family of dependency on a team key. It does not create a liveness dependency: each pause is capped at `MAX_PAUSE_SECS` and lapses by itself, so if the operator disappears, conversions resume without anyone acting. The admin can also lift it. The operator can chain pauses to keep the desk stopped, but that is an active use of a power listed under Trusted roles, not a dependency on the team staying online.
- `verify_signed_message` parses Ed25519 instruction data by hand. It is correct. It loads the instruction immediately before the current one through the address-checked instructions sysvar, and requires the Ed25519 program id, exactly one signature, and `u16::MAX` for the signature, public key and message instruction indices, so the data cannot be taken from another instruction. The key must equal `desk.publisher` and the message must be byte-equal to the expected quote message. The message binds both mints, so a quote for another pair is rejected.
- `PostRate.instructions` is an `UncheckedAccount`, pinned by `address = INSTRUCTIONS_SYSVAR_ID`, and the `load_*_checked` helpers check the ID again.
- `amount_out` uses u128 and rounds down. The fee is taken from the gross output. `convert` enforces `min_out`, a non-zero output and vault liquidity. `quote -> base` divides by `rate`, which `usable_rate` guarantees is positive.
- `MoveLiquidity.vault` is a single account constrained to one of the two desk vaults. `admin_token` must match the vault's mint.

## Trusted roles
- Admin: creates the desk, sets `operator`, `publisher`, `fee_bps` (at most 200) and `max_rate_age` (5 to 600 s), and provides and withdraws liquidity from both vaults.
- Publisher (off-chain Ed25519 key): its signed quotes are taken as the true rate.
- Operator: in the defective version it is the only key that may call `post_rate`, and it may call `set_pause` for up to `MAX_PAUSE_SECS` at a time, repeatedly if it chooses. In the fixed version it may only call `set_pause`. In both versions it can submit only genuine publisher quotes.
- Anyone (fixed version): may submit a publisher-signed quote newer than the stored one.
