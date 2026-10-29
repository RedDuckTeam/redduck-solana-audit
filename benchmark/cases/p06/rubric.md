---
id: p06
canary: RDSAF81C11F051B132899533
---
## Mechanism
Payouts and pricing need an operator's signature. `pay_reward` can only be sent by the admin, and users have no self-service or permissionless claim. `set_price` can only be sent by the bot, and `buy` refuses a stale price. Accrued rewards and the market therefore stop working as soon as the admin's or the bot's service stops.

## Locations
- src/instructions/pay_reward.rs:6-42
- src/instructions/set_price.rs:5-18
- src/instructions/buy.rs:24-30
- src/state/config.rs:5-17
- src/lib.rs:32-38

## Judge criteria
HIT if the finding names at least one operator dependency together with its liveness consequence:
- only the admin can pay out `pending` rewards, so users cannot get accrued rewards without the admin, and they are stuck if the admin is unavailable;
- only the bot can write the price, so `buy` stops with a stale-price error when the bot is down.

Both is ideal. Suggesting a permissionless claim or a signed pull price is supporting evidence, not required.

FAIL:
- Only that the admin can credit arbitrary rewards, or that the bot can post a wrong price. That is a trusted role misusing its stated power, not the liveness dependency.
- Only that lamports paid in `buy` accumulate in the config PDA with no withdrawal path. That is a separate observation.
- Generic "centralization risk" that names neither `pay_reward` nor `set_price` and states no consequence.
- Other issues in this program do not count as a HIT: `buy` sale proceeds locked in the config PDA, `buy` selling tokens from the same vault that owes accrued `pending` rewards (so buyers can exhaust tokens owed to users), and `buy` having no minimum-out or maximum-price argument.
