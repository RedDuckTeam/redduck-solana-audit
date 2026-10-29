---
id: p03
crate: device_rewards
pattern: one-way-state
class: design
group: g2
vulnerable: a
fixed: none
role: public
author: case-author
canary: RDSAF81C11F051B132899533
source_vulnerable: benchmark/public/programs/p03
---
# Case p03

## Attack scenario
This is a design flaw. The oracle signs a per-day reward message `(device, amount, nonce)`, and `claim` pays `amount` only when `nonce == device.nonce + 1`, then advances `device.nonce` to `nonce`. Each device's progress is a strict contiguous counter that only moves forward by exactly one.

1. The oracle issues a sequence of signed messages, one per day, with nonces `1, 2, 3, ...`. A device must apply every one, in order, with no gaps.
2. If one message is never delivered or is lost (a dropped relay, an oracle outage or lost record), `device.nonce` is stuck at the value before the gap. Every later message has a nonce `>= gap + 1`, so none satisfies `nonce == device.nonce + 1`, and all later rewards for that device are refused until the oracle re-signs that exact nonce out of band.
3. Two messages in flight that arrive out of order force the later one to be rejected until the earlier one lands; the system depends on exactly-sequential delivery to make progress.

The consequence is a liveness/availability failure: a single missing signed message freezes all of a device's future claims, with no way for the owner or the oracle to recover without re-signing that exact nonce.

## Source
Template task folder `programs/07-claims-in-any-order` of https://github.com/RedDuckTeam/redduck-academy-solana-pro-template at commit `782fb2d28eb1ad47ebd6028b1df870706b1133c3`, the version given to students. Renamed the crate, `[lib] name` and `#[program]` module `total_claims` to `device_rewards`, and removed the Cargo `description`. Replaced two comments: the `claim` doc comment (src/instructions/claim.rs:7) and the `Device.nonce` comment (src/state/device.rs:7). The logic is unchanged and `declare_id!` is kept.

## Trusted roles
- **admin**: runs `initialize` and sets the oracle key and mint. No per-claim power.
- **oracle**: `config.oracle`. Trusted to sign each device's reward messages honestly. It is relied on here to also deliver a perfectly contiguous, never-missing sequence of nonces.
- Device owners and callers of `claim` are untrusted; anyone may relay a signed message to `claim`.
