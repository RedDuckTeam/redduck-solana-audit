# Execution layer shift

## Rule
Move execution off Solana only for a constraint L1 cannot meet: more accounts than one transaction can reference, more instructions than a transaction allows, or real-time moves in a game. Speed alone is not a reason, because Solana is already fast. When the move is justified:
- move the smallest piece of state for the shortest time;
- keep funds and final settlement on L1;
- let users exit on L1 without help from the other layer's operator.

## Why Solana forces it
A rollup, an ephemeral rollup or an off-chain matcher is another system with its own operator. Often one machine orders the transactions, and finality is usually weaker than Solana's. Programs there cannot call Solana programs directly. Leaving is slower than entering, because L1 must wait until the other layer's state can be trusted. With delegation, the account's owner on L1 becomes the delegation program, so the original program cannot write it on L1 until it is committed and undelegated. Whatever state comes back was written by that layer's validator.

## Code signals
- **Ephemeral rollups and delegation:** `ephemeral_rollups_sdk`, `#[ephemeral]`, `#[delegate]`, `#[commit]`, `delegate_account`, `commit_accounts`, `commit_and_undelegate_accounts`, `undelegate`, `magic_context`, `magic_program`. Note which accounts are delegated, who may delegate them, and which validator they go to.
- **Rollups and bridges:** state roots or batches posted by an operator (`submit_root`, `update_state_root`, `post_batch`), withdrawals proven against those roots, a `challenge_period`, and exit paths such as `force_withdraw` or `emergency_withdraw`.
- **Off-chain matching:** fills or batches signed by an operator key, such as `settle_batch` with an Ed25519 check against a `matcher` or `sequencer` pubkey stored in config.
- **Funds that move:** transfers out of an L1 vault to a bridge or layer escrow; balances that exist only in the other layer's state.
- **Deployment notes** saying that a market, game or order book runs on another network.

## Auditor questions
1. What leaves L1: which accounts are delegated, mirrored or bridged? Do any of them hold user tokens or lamports, or balances that decide payouts?
2. If the other layer stops forever, what is stuck? Can the user, or anyone after a timeout, undelegate or withdraw on L1 from the last committed state?
3. What does L1 accept back, and who signs it? Is the returned state checked on L1 against bounds, such as a payout no larger than the stake and conserved totals, or applied as it is?
4. Who chooses the validator or sequencer? If the caller can pick or run it, the caller writes the returned state.
5. While an account is delegated, which L1 instructions fail? Can someone delegate another user's account, or keep it delegated, to block that user's exit?
6. What real limit forced the move? If there is none, the move adds an operator and buys nothing. Report that only when funds depend on it.

## How to confirm
Simulate the layer disappearing (see `exploit-tests.md`). Set the delegated account's owner to the delegation program with `set_account`, or never commit, then call every user exit on L1 and list which fail. To test trust in returned state, write the data the layer could commit, such as a win larger than the pot, and show that the L1 payout instruction pays it. For forced delegation, show a stranger delegating a victim's account and the victim's withdrawal failing.

## Not a bug if
- Only per-session state, such as one game round, is delegated. Funds stay in an L1 vault, and the L1 payout is bounded by what was staked for that session.
- A timeout lets anyone undelegate, or lets users withdraw on L1 from the last committed state.
- The operator is a documented, trusted role that users see before depositing. Its power is a design finding at Medium at most (see "Trusted roles" in `report-format.md`).
- Matching happens off-chain, but L1 checks each fill against user-signed bounds and moves funds only on L1.

## Real-world cases
- **Done right.** An on-chain casino keeps deposits in its Solana program. For each round it delegates only that round's state to an ephemeral rollup, where every move lands at once. After the round, the state is committed back and the payout happens on Solana, from funds that never left.
- **Done right.** A derivatives exchange needed a cross-margin check over more markets than one transaction can reference. It moved trading to its own layer and left settlement and data on Solana. The price was a slower exit: withdrawing to Solana takes longer than a trade.
- **Done right.** A network of wireless devices left its own chain for Solana. It reshaped what did not fit instead of moving it elsewhere: devices became compressed NFTs, and daily payouts became claims that each owner sends.

## Severity
Critical when L1 pays out from returned state that the caller controls, for example because the caller picks the validator. High when user funds can leave only through the other layer and nothing on L1 recovers them if it stops, or when a stranger can lock a victim's account in delegation. Medium when an L1 exit exists but needs the operator or a long delay, and for a trusted operator's power over returned state. Low or Info when the move has no constraint behind it and no funds depend on it.
