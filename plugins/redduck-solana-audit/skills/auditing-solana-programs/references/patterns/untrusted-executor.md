# Untrusted executor

## Rule
Any work the protocol needs done must meet three conditions:
- it is callable by anyone;
- it pays the caller enough to make doing it worthwhile;
- it is safe whoever calls it, with value going to its owner, not to the caller.

A protocol that depends on its team's bot lives only as long as that server does.

## Why Solana forces it
Nothing on-chain runs by itself. Settlements, reward payouts, liquidations, round closing and price updates all need somebody to send and pay for a transaction. The tempting design restricts these instructions to an operator key. That turns the operator's uptime and honesty into a dependency for every user's funds.

## Code signals
- Instructions that users depend on, gated by `has_one = operator`, `has_one = admin`, `constraint = signer.key() == config.keeper`, or a hardcoded keeper pubkey. Typical names are `settle`, `distribute`, `close_round`, `process_withdrawals`, `liquidate`, `crank`, `update_price` and `finalize`.
- User withdrawals or claims that require a prior operator step, such as "request" followed by the operator's "process". Deposits that only an operator can move into the state where users can exit.
- Notes in deployment documents such as "a service run by the team calls X every N minutes".
- The inverse risk: permissionless instructions that pay out to `ctx.accounts.caller` or another account the caller picks, instead of to the stored owner.

## Auditor questions
1. List the actions that must happen for users to get their money out, or for the protocol to progress. Who can trigger each one?
2. If the operator key is lost or the bot stops forever, what is stuck? Can users exit by themselves?
3. If the action were permissionless, would the caller be able to choose the recipient, choose a profitable moment, or repeat it?
4. Is there an incentive for outsiders, such as a fee, a tip, a liquidation bonus or a Dutch-auction reward, that pays even when the protocol is small?
5. Are timing conditions enforced on-chain (round end, expiry, health factor), so that opening the call to anyone does not let them act early?

## How to confirm
- **Design finding:** cite the constraint that restricts the needed action to the operator. Then describe the consequence when the operator is absent, for example "deposits stay locked: `withdraw` requires `round.settled`, and only `settle_round` (operator-only, line N) sets it".
- **Inverse finding:** show a call by an unrelated wallet that sends value to itself.

## Not a bug if
- The gated action is optional: parameter tuning, or adding markets that do not block existing users.
- A permissionless fallback exists, such as a timeout after which anyone can settle, or a direct user exit.
- The gate is on a privileged function that users never depend on, and that is documented as such.

## Real-world cases
- **Done right.** A device-rewards network lets anyone call claim for any device. The oracle-signed total and the stored paid amount decide the payout, and the tokens go to the device owner whoever calls.
- **Done right.** Lending and perpetuals venues pay liquidators part of the collateral. Nobody runs a dedicated liquidation service, because independent bots watch every position.
- **Done right.** An index fund rebalances by Dutch auction. The price improves for the taker each slot until a bot accepts, so the reward is set by competition.
- **Done right.** A task scheduler stores each job with a prepaid reward and a start condition. Anyone can run the job and take the reward.

## Severity
High when users' funds stay locked while the operator is absent. Medium when the protocol only pauses: no new rounds, stale prices. A permissionless action that lets the caller redirect value is Critical.
