# Commit-execution split

## Rule
If knowing the outcome before acting is profitable, the commitment and the outcome go in different transactions:
- The user commits first. The stake is locked and the choice is fixed while the outcome does not exist yet.
- The outcome comes later, from a source the user cannot predict or pick.
- The user does not control resolution. Holding it back, cancelling or opening a second commitment never improves their result.

## Why Solana forces it
Whatever a transaction computes from on-chain state, its sender can see beforehand. Simulation is free. A failed transaction costs only its fee, or nothing when it travels in a bundle that does not land. A user can append an instruction that checks their balance after the bet and aborts on a loss, or call the game from their own program and revert. The clock, the slot and slot hashes are known to the sender, and the leader can influence them. So an outcome produced in the same transaction as the commitment is an outcome the user can choose. Ordering works the same way: whoever sees a price first, or lands first, takes value from a first-come design.

## Code signals
- **Same-transaction randomness:** `Clock::get()` (`slot`, `unix_timestamp`), the `SlotHashes` sysvar read by address, recent blockhashes, or `hash` / `keccak::hashv` over the user's key and the clock, used in the instruction that decides a payout.
- **One-step games:** `play`, `spin`, `flip`, `roll`, `open_box` or `mint_random` that take the stake and pay the prize together.
- **Two-step flows to check closely:** `request_randomness` or `commit`, followed by `settle`, `reveal`, `consume_randomness` or a VRF callback. Switchboard `RandomnessAccountData` with `seed_slot` and `get_value`, or an ORAO randomness account.
- **Weak two-step flows:**
  - the randomness account is not stored at commit and compared at settle;
  - the request may already be revealed when the user commits;
  - the user picks which of several results to use;
  - `cancel`, `refund` or `close` works while a request is pending;
  - nothing stops a new commitment while one is open.
- **Commit and resolve split only by a status flag:** two instructions in different transactions with nothing that forces them into different slots. A Jito bundle lands up to five transactions in order, in one slot, or none of them.
- **VRF callbacks:** a `receive_randomness`-style handler that takes the random bytes as an argument. Check which account must sign it, and whether that is the VRF program's own signing identity rather than its program id: an executable program account never signs.
- **Heavy callbacks:** a callback that walks a list, closes accounts or pays out, so a callback that reverts loses the randomness. Look for `re_request`, `retry` or `refund_after_timeout` paths and who may call them.
- **Short windows:** auctions, reveals or grace periods that end within a few slots.
- **User-secret commit-reveal:** a `commitment: [u8; 32]` checked against `hash(secret || choice)` at reveal. Check what happens when the user never reveals.
- **First-come mechanics:** limited mints, "first N callers", market orders filled at the current oracle price in the same instruction, auctions settled by the first bid.
- **A partial fix:** a check that the program is called at top level, or that no instruction follows it (`get_stack_height()`, the instructions sysvar). It closes some ways to revert a loss, but a predictable outcome stays predictable.

## Auditor questions
1. Which values would a user like to know before acting: a random result, the fill price, their place in a queue? In which transaction is each one fixed, relative to the commitment?
2. Where does the outcome come from? Could the user, the leader or the sender of the resolving transaction predict it or choose it?
3. Is the randomness request created or recorded at commit time, still unrevealed then, and compared with the one used at settle?
4. Who sends the resolution? If the user can hold it back, does that leave the stake locked and counted as lost, or let anyone settle after a timeout?
5. While a commitment is open, can the user cancel, get a refund, close the account or open a second commitment?
6. For trading: can an order fill at a price already known when it is sent, such as a stale oracle price? Or is the fill price set after the order, by a later price, a batch or an auction?
7. Is resolution forced into a later slot than the commitment, or into a value published after it? If not, can an attacker bundle commit, resolve and a balance check, and land only the wins?
8. Who can call the randomness callback? Does it check that the oracle's signing identity signed, that the round is still waiting for randomness, and then move the round forward?
9. If the callback never lands or reverts, how do funds get out? Can an entrant make the callback fail on purpose, for example by closing an account it writes or growing a list it walks, and then re-request after seeing the value? Is a re-request allowed only after a timeout and only while no value was delivered?
10. Does anyone gain by landing or keeping out one transaction near a deadline? A leader orders, and can drop, transactions for its four-slot window, and congestion drops transactions even when they pay priority fees.

## How to confirm
- **Same-transaction outcome:** build a transaction with the bet followed by an instruction that fails unless the user's balance went up. In a test (see `exploit-tests.md`), send it across many slots and show that only wins land. For clock or slot-hash randomness, compute the outcome off-chain from the slot the test sets and show the prediction matches.
- **Abandoned losses:** commit, read the revealed value off-chain, then call `cancel` or open a new round instead of settling. Show the stake comes back or the old round stops mattering.
- **Swapped randomness:** commit with request A, then settle with an already revealed request B. Show the program accepts B.
- **Bundle-atomic resolution:** send commit, resolve and a check that fails unless the attacker profited as consecutive transactions in the same slot, without warping, and treat the set as all-or-nothing.
- **Forged callback:** call the callback directly from the attacker's wallet with chosen bytes. Show the program accepts them.
- Without a toolchain, list what the attacker reads, when, and which instruction they skip or send.

## Not a bug if
- The outcome comes from a VRF or oracle in a later transaction. The request is bound to the commitment when it is made and checked at settle. Skipping settlement only hurts the user, or anyone can settle.
- The user's secret is combined with a value unknown at commit time, such as a later slot hash or an oracle value, and a missing reveal forfeits the stake after a deadline.
- Cancel exists but refunds only before the randomness has been requested.
- Nothing is gained by previewing: a deterministic swap protected by a minimum output (see boundary-of-consent), or a draw whose entries all close before the seed exists.
- Orders are collected over an interval and filled at one clearing price, so arriving first gives no edge.
- Settle requires `slot > commit_slot`, the outcome is fixed by a value bound at commit and published after it (a VRF result for the recorded request, or the hash of one specific later slot), and the user cannot choose when or whether settlement happens.
- The callback only stores the value and advances the status, and anyone can re-request after a timeout while no value has been delivered.
- Deadlines span many leader windows, or late bids extend the auction.

## Real-world cases
- **Done right.** A coin-flip game uses a VRF oracle. The player's first transaction locks the stake and the chosen side, and opens a randomness request that no one can answer yet. The oracle supplies the number in a second transaction, and the program settles. A player with an open round cannot start another, so a losing flip cannot be walked away from.
- **Done right.** A perpetuals venue takes each market order as a request account. Keepers execute it later at a fresh oracle price, so a trader cannot fill against a price they already know is stale.
- **Done right.** A synthetic-asset perpetuals venue batches orders over an interval and fills the whole batch at one price, so being first is worth nothing.
- **Done right.** A perpetuals exchange runs a short auction among market makers for each taker order before its own liquidity fills it. The taker's price is set by competition after the order is placed.

## Severity
Critical when any user can predict or choose the outcome and win from a shared pool on demand, or can cancel every loss. A callback anyone can call with chosen bytes is Critical. High when exploiting it needs a leader's help or precise timing but works repeatedly. Medium when the gain is bounded, such as a better place in a first-come allocation. If the outcome comes from a trusted oracle that could bias it, report the oracle's power as a design finding, at Medium at most.
