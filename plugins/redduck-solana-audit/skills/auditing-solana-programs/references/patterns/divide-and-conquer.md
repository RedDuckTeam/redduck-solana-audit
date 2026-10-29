# Divide and conquer

## Rule
Split state into partitions, such as a PDA per user, per position, per tick range or per market. Each transaction touches only the partitions its action needs, plus a small central account that holds the totals. No single account should grow with the number of users. Opening, changing or closing a partition must keep the central totals equal to what the partitions hold.

## Why Solana forces it
Transactions are capped in size, account count and compute. Accounts have a maximum size, and an instruction can grow one only by a limited step, paying rent for the space. Anchor's `Account<T>` deserializes and reserializes the whole account on every use, so cost grows with size even when one field changes. Transactions that write the same account cannot run in parallel, and a block limits how much compute can write to one account. A single hot or growing account therefore caps throughput and eventually stops working.

## Code signals
- **Growing collections in one account:** `Vec<UserInfo>`, `Vec<Pubkey>` of members, `BTreeMap`, `users` or `orders` inside `Config`, `Pool` or `State`; `realloc = ` with a size from `len() + 1`; large `zero_copy` arrays with a `count` field.
- **Linear edits:** `push`, `retain`, `remove(i)`, `iter().position(...)` on those collections.
- **Hot accounts:** a global account marked `#[account(mut)]` in every user instruction. Check what each instruction writes there.
- **Partitions:** `seeds = [b"user", pool.key().as_ref(), owner.key().as_ref()]`, `[b"position", ...]`, `[b"tick_array", pool, start_index]`, bitmaps that track which partitions exist, per-market accounts.
- **Partition lifecycle:** `close = owner`, manual lamport draining, `init_if_needed` on reopen. Central counters such as `total_users`, `total_staked`, `open_positions` and `liquidity` must move on each of these.
- **Compressed state:** merkle tree accounts, `concurrent_merkle_tree`, `canopy_depth`, `max_buffer_size`, proof nodes in `remaining_accounts`.

## Auditor questions
1. Which accounts does every user instruction write? If one global account, is what it holds a small, fixed-size summary?
2. Does any account grow with users, orders or positions? What caps it, who pays for growth, and what happens when it is full? Can an attacker fill it cheaply?
3. For each partition type, who can create it, and do its seeds bind it to both its owner and its parent pool or market? Can a caller pass someone else's partition, or one from another parent?
4. When a partition opens, changes or closes, do the central totals change by the same amount in the same instruction? Can a partition close while it still holds value or obligations?
5. Does any instruction need every partition at once? If so, how does it get them, and what bounds the number?

## How to confirm
- **Growth:** compute the size or count at which the account or its loop breaks, and what an attacker pays to get there. In a test (see `exploit-tests.md`), fill it to the cap and show the next honest call, ideally a withdrawal, fails.
- **Totals:** open, change, close and reopen partitions in sequence. Show the central total drifting from the sum of partitions, then show the effect: a wrong payout, or the last withdrawer blocked.
- **Hot account:** cite the writable global account in each user instruction. This is usually a throughput issue, not a security one.

## Not a bug if
- The global account every instruction writes is a small, fixed-size summary, and contention is an accepted throughput trade-off.
- The collection has a small cap fixed by the design, filling it is gated or costly, and a full collection never blocks exits.
- Partitions are PDAs seeded by owner and parent, closing requires a zero balance, and the totals are adjusted on close.
- The only on-chain state is a merkle root, and every change verifies a proof against it (see complexity-asymmetry).

## Real-world cases
- **Done right.** A concentrated-liquidity DEX stores ticks in fixed-size arrays that are created only when needed. A swap passes only the arrays it crosses, and a position updates only the two ticks at its edges.
- **Done right.** A compressed NFT program keeps a million items under one merkle root. To change an item, the client supplies the leaf and its path. The account caches the upper levels of the tree, so the path the client sends stays short.
- **Done right.** A compression layer applies the same idea to general accounts and merges the proofs for several accounts into one proof of fixed size.

## Severity
- Critical when central totals drift from the partitions in a way that lets a user withdraw funds that belong to others.
- High when the drift or a full account locks users' funds with no other exit.
- Medium when an attacker can cheaply fill a shared account and block new users or new positions, while existing users can still exit.
- A hot account that only limits throughput is Low or Info, as a design note.
