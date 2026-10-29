# Role inversion

## Rule
Give each job to the party that needs its result, and keep only the check on-chain:
- a trader brings a source-signed price in the transaction that uses it;
- a bridge stores a verified message, and the receiver collects it;
- a client proposes where an entry goes in a sorted list, and the program checks the neighbours.

After the flip, the party that needs the action must still decide when it happens. Everything the old operator used to guarantee must now be verified on-chain, because the data arrives from whoever sends the transaction.

## Why Solana forces it
Every on-chain write costs a fee, whether or not anyone reads the value. An oracle that pushes every update of every feed pays for all of them. A program call must list every account the callee needs, so a sender that does not know the receiver cannot call it. Compute per transaction is capped, so on-chain search stops fitting as data grows. Checking a result is cheaper than producing it, which makes the flip attractive. It also hands untrusted callers the data the program acts on.

## Code signals
- **Not inverted:** `set_price`, `update_price`, `push_rate`, `report_result`, `post_score` gated by `has_one = oracle`, `operator` or `updater`; consumers that read the stored value with no age check; a `last_updated` field only the operator can advance.
- **Pull oracles:** `PriceUpdateV2` with `get_price_no_older_than(&clock, max_age, &feed_id)`. Look for `get_price_unchecked`, a feed id taken from the caller instead of config, a large `max_age`, and `UncheckedAccount` in place of the typed account.
- **Signed payloads:** Ed25519 or Secp256k1 verification through introspection. The signer key must match a stored key, and the message must bind the feed, the program, the amounts and an expiry.
- **Bridge messages:** posted message or VAA accounts. Check the owner is the core bridge program, check `emitter_chain` and `emitter_address`, and look for a consumed marker such as `init` on a PDA keyed by sequence or hash.
- **Account discovery:** an instruction like `lz_receive_types` that returns the accounts a receive needs. The receive handler must still validate each account it gets.
- **Client hints:** `prev`, `next`, `after`, `hint`, `insert_index` arguments for sorted structures.
- **Control handed away:** a needed action (rebalance, settlement, exit) that happens only when a third party trades, routes, relays or accepts.

## Auditor questions
1. For each value an operator pushes, who needs it? Could they bring it, signed by the source, in the transaction that uses it?
2. Where callers bring the data, is every guarantee the operator used to give checked on-chain: signer key, feed or emitter, freshness, target program, and that it was not used before?
3. Can the caller pick among several valid values, such as two signed prices inside a wide age window, and choose the one that pays?
4. When a client supplies a position, path or account list, is it verified fully, so a wrong hint fails instead of corrupting state?
5. If the operator stops, which instructions read stale or missing data? Do they refuse or proceed?
6. After the flip, who decides when the needed action happens? Is there a relayer, aggregator or counterparty whose cooperation is now required, with no fallback?

## How to confirm
- **Not inverted:** this is a design finding. Cite the operator-only write and each consumer that reads it, then say what happens when the operator is offline or posts a wrong value.
- **Incomplete verification:** build a payload the program accepts but should not, such as a price for another feed, an old price, a message from another emitter or a replayed message. In a test (see `exploit-tests.md`), post it and show the instruction acting on it.
- **Lost control:** list the steps that need a third party and what happens to users' funds if it declines.

## Not a bug if
- The pushed data comes from a deliberately trusted role, consumers check its age, and users can see it before they act. Report the operator's power as a design finding.
- Pull verification binds the signer, the feed or emitter, freshness and replay, all against values stored in config.
- A wrong or stale hint only makes the transaction fail, so the worst case is a retry.
- A discovered account list is used only to build the transaction, and the handler checks every account itself.

## Real-world cases
- **Done right.** An oracle network moved from publishers writing every update on-chain to traders attaching a signed update to their own trade. The program checks the signature and the age, and the trader still chooses when to trade.
- **Done right.** A cross-chain bridge saves each verified message in an account it owns. The receiving program reads that account in its own transaction and checks the owner, source chain, sender and that it has not consumed the message.
- **Done right.** A messaging protocol asks the receiving program, through a read-only instruction, which accounts it needs. The executor simulates that call and builds the real transaction from the answer.
- **Done right.** A prediction-market order book takes the insert position from the client and checks only the two orders around it.
- **Failure.** An index fund rebalanced by quoting prices to aggregators and letting outside trades move its weights. Rebalancing then waited on traders and routers it did not control.

## Severity
- Not inverted: an operator that pushes data users rely on is a trusted role, Medium at most. It is High when users' funds stay locked while the operator is absent and nothing else unlocks them.
- Incomplete verification: Critical when any user can submit a forged, foreign or replayed value and take funds. High when the abuse needs conditions, such as picking a favourable price inside a long age window.
- Lost control: Medium when the protocol stalls until a third party acts, High when users cannot exit meanwhile.
