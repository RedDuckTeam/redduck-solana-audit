---
id: c10
canary: RDSAF81C11F051B132899533
---
## Mechanism
Rates reach the desk only when the team's operator key calls `post_rate`, and `convert` requires that stored rate to be fresh. The publisher-signed quote is public and fully verified on chain, so the user could supply it in its own transaction. Instead every conversion depends on a team relay: when the relay stops, the desk is blocked, and while it lags users trade on stale rates.

## Locations
- src/instructions/rate.rs:9-39
- src/state.rs:56-64
- src/instructions/convert.rs:24-35
- ARCHITECTURE.md:5-7

## Judge criteria
- PASS: the finding says that the desk depends on the operator or relay to post rates, so that if the relay or operator stops, conversions halt with an expired rate, or the rate goes stale. It must also say that the fix is to let users or anyone submit the publisher-signed quote themselves, for example in the same transaction as `convert`, with the operator restriction on `post_rate` removed.
- PASS also: "centralization or liveness: only the operator can update the rate, although the quote is already publisher-signed; make `post_rate` permissionless or verify the quote inside `convert`."
- FAIL: a finding that only says the operator is a trusted or centralized key, with no liveness consequence and no user-supplied quote remedy.
- FAIL: findings that the Ed25519 parsing is unsafe, or that the rate can be manipulated by the operator (it can submit only publisher-signed quotes).
- FAIL: a finding that the operator can keep the desk paused by chaining `set_pause`. Pausing is within the operator's listed trust, and a pause never needs the operator to act in order to lapse.
- FAIL: a generic "use a decentralized oracle" with no reference to the operator-gated `post_rate`.
