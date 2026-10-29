# Fail-fast

## Rule
Before acting on an input, check that it is good enough for this action. If it is not, refuse and say why:
- **Price inputs:** check age, confidence, distance from an average or the last value, sign, and the feed's identity.
- **Per-action thresholds:** actions that add risk (borrow, open, mint, fill) need the strictest checks. Actions that reduce risk (repay, deposit, liquidate) must still work with a weaker input.
- **Late inputs:** when an input has not arrived yet (randomness, a result, a settlement), store "pending" and block every action that depends on it.

A refusal is a designed outcome, not a crash. Falling back to a cached or default value is acting, not refusing.

## Why Solana forces it
A transaction runs once and cannot wait for a better input. The program decides with whatever the accounts hold in that slot. Oracle accounts change only when someone posts an update, so a feed can go quiet without any error. With pull oracles the caller picks which signed update to post, and may pick an older one that suits them, as long as it passes the age check. Randomness and other answers arrive in a later transaction, so the gap between request and answer is state the program has to remember.

## Code signals
- **Pyth pull:** `price_message` read directly instead of through `get_price_no_older_than(&clock, max_age, &feed_id)`; a large `max_age`, or one constant for every action; `feed_id` never compared; partial verification accepted through `get_price_no_older_than_with_custom_verification_level`.
- **Legacy Pyth:** `get_price_unchecked()`, or `publish_time` never compared with `Clock::get()?.unix_timestamp`; price status not checked.
- **Switchboard:** `get_result()` without `check_staleness(..)` or `check_confidence_interval(..)`; on-demand `get_value(..)` with loose `max_stale_slots` or `min_samples`; `result.slot` never compared with the current slot.
- **Randomness:** `RandomnessAccountData`, `seed_slot`, `reveal_slot`, `get_value(&clock)`. Look for settle paths that can run before the reveal, and for rounds with no stored open status.
- **Fallbacks:** `unwrap_or(last_price)`, `unwrap_or_default()`, `Err(_) => cached` branches with no age limit on the cached value.
- **Range checks:** `price <= 0` not rejected; no deviation check against a TWAP or the last stored price; no plausible range for a pegged asset.
- **Uneven coverage:** checks in `borrow` but not in `withdraw` or `mint`; one shared helper with one threshold for every action.

## Auditor questions
1. List every instruction that reads an outside input. Before each use, which checks run: age, confidence, deviation, sign, feed identity, availability?
2. Do the thresholds fit each action? Is the strictest check on risk-increasing actions, and do repay, deposit, withdraw and liquidate still work when the feed is degraded?
3. When a check fails, does the instruction return an error, or log and skip one step, or fall back to a cached or default value? Is the fallback's age limited?
4. With a pull oracle, can the caller choose an older or a different update within the allowed age, to their advantage?
5. For inputs that arrive later, is "pending" stored? Can the user settle, cancel, withdraw or start another round while it is pending?

## How to confirm
Forge the input in a test (see `exploit-tests.md`). Use `set_account` to give the oracle account an old `publish_time`, a wide `conf`, a non-positive price, a price far from its average, or another feed's id, and advance the clock if needed. Call the risk-increasing instruction and show that it succeeds. For late inputs, commit, then call settle, cancel or a new commit before the reveal, and show it goes through. For checks that are too strict, make the feed stale and show that repay or liquidate fails, leaving positions stuck.

## Not a bug if
- Liquidation or repay accepts an older or wider price than borrow does. That asymmetry is the design.
- A failed check logs and skips one step without failing the transaction, for example an AMM leg that does not fill while the order stays open.
- A refresh instruction runs the checks and stores the results as flags valid only for the current slot, and every dependent instruction requires a refresh in that slot.
- The input is the program's own state, kept current on every touch (see internal-oracle).

## Real-world cases
- **Done right.** A perpetuals exchange grades the oracle before each action: too uncertain, too old, too far from its own short average, or not positive. Each action lists the grades it accepts. Its built-in market maker fills only on a fully valid price, while liquidation accepts almost every grade. When the market maker cannot fill, the program logs it and the order stays open for other takers.
- **Done right.** A lending market runs all price checks in a refresh step and stores the results as flags for that slot. A borrow needs every flag. A liquidation needs only a loaded, fresh and usable price. A plausible range per token stops new borrows when a stablecoin leaves its peg, even if the oracle is confident.
- **Done right.** A coin-flip game on a verifiable-randomness service stores the open round. The player cannot start another round while it is open, and settling fails until the value is revealed.

## Severity
Critical when any user can choose the input the program acts on: a wrong feed accepted, any old update accepted, or a negative price cast into a huge one. High when missing age or confidence checks let users borrow or fill against a bad price in realistic conditions, or when a missing pending state lets a user re-roll randomness. Medium when checks are so strict that repay, withdraw or liquidation stop while the feed is degraded, which is a temporary freeze. Low when one layer of defence, such as a deviation check, is missing while age and confidence are enforced.
