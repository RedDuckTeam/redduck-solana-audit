---
id: p06
crate: reward_market
pattern: untrusted-executor
class: design
group: g1
vulnerable: a
fixed: none
role: public
author: case-author
canary: RDSAF81C11F051B132899533
source_vulnerable: benchmark/public/programs/p06
---
# Case p06

## Attack scenario
This is a design flaw. Two parts of the program run only while a team-operated key keeps sending transactions.

1. The admin credits rewards with `accrue`, and they sit in `User.pending`. Only `pay_reward` can move them out, and `PayReward` requires `admin: Signer` with `has_one = admin`. A user has no way to claim their own pending reward, and neither does anyone else on the user's behalf. If the admin's service stops or the admin key is lost, every accrued reward stays unpaid in the vault for good.
2. Only `set_price` writes `config.price`, and it requires the `bot` signer. `buy` reverts with `StalePrice` once `now - price_time > max_price_age`. If the bot stops, the market closes for everyone one `max_price_age` later.

The config already stores an `oracle` key and the crate has an Ed25519 parser, but neither is used. Nothing the operators do needs their privileged keys: a claim could be sent by anyone (paid to the user, optionally with a capped fee), and a price could be pulled from an oracle-signed message. The program still depends on both servers being alive.

## Source
Template task folder `programs/03-run-it-without-the-operator` of https://github.com/RedDuckTeam/redduck-academy-solana-pro-template at commit `782fb2d28eb1ad47ebd6028b1df870706b1133c3`, the version given to students. Renamed the crate, `[lib] name` and `#[program]` module `operator_free` to `reward_market`, and removed the Cargo `description`. Replaced five comments that explained the liveness dependency, restated the access rule, or described the exercise, keeping neutral wording: src/instructions/accrue.rs:5, src/instructions/pay_reward.rs:6, src/instructions/set_price.rs:5, src/state/config.rs:7 (now "The team's bot key."), and src/state/config.rs:9 (dropped "Unused by the given program"). The logic is unchanged and `declare_id!` is kept.

## Trusted roles
- **admin**: runs `initialize` (sets bot, oracle, mint, `max_price_age`) and credits rewards with `accrue`. The admin is the source of rewards and may decide their amounts. In the given program the admin also sends `pay_reward`.
- **bot**: the team's key that writes `config.price` through `set_price`. It is trusted to report a correct price.
- **oracle**: a key that signs prices off-chain. It is stored in config and never read.
- Users and buyers are untrusted.
