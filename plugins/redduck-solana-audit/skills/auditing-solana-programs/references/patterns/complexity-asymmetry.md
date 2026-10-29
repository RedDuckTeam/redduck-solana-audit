# Complexity asymmetry

## Rule
Search off-chain and verify on-chain in bounded work:
- The client finds the position, the neighbour or the proof. The program only checks the answer, in constant or logarithmic work.
- No on-chain loop, search or account size may grow with data that users can add.
- Every hint the client supplies is verified in full, and every structure the program walks has a hard size limit.

## Why Solana forces it
Each transaction has a fixed compute budget, a cap on the accounts it can lock and a size limit. Work that is cheap off-chain can be unaffordable on-chain. If an instruction's cost grows with data anyone can add, an attacker adds cheap entries until the instruction no longer fits. From then on it fails for everybody, and the deployed code cannot recover. The client, by contrast, reads all state through RPC for free. It can do the expensive search and hand the program a result that is cheap to check.

## Code signals
- **Loops over stored data:** `for` or `.iter()` over a `Vec` field of an account (`orders`, `bids`, `stakers`, `entries`, `whitelist`, `positions`). Look for `.iter().find()`, `.position()`, `.sort()`, `.retain()`, `.insert(i, _)` or `.remove(i)` on account data, and a linked list walked from its head (`while let Some(next) = node.next`).
- **Growth:**
  - an Anchor `realloc = ...` constraint that grows the account on every insert;
  - `AccountInfo::realloc` or Pinocchio `resize` called per entry;
  - a `Vec` with no `#[max_len]`, or a `push` with no `require!(len < MAX)`;
  - a loop bound by a stored counter (`for i in 0..state.count`).
- **Client hints:** arguments named `prev`, `next`, `insert_after`, `hint`, `index`, `slot`, `leaf_index`, `proof: Vec<[u8; 32]>`, or proof nodes passed in `remaining_accounts`.
- **Weak hint checks:**
  - only one neighbour is compared;
  - no check that the neighbour cell is in use, or that it belongs to the same book or market;
  - a proof whose length is not tied to the tree depth;
  - a leaf hashed the same way as an inner node;
  - a leaf that does not commit to its own index or a unique id.
- **Weighted draws:** an `Entry { start, end }` ticket range per deposit, a winner found as `r % total_tickets` through a client-supplied entry. Look for `r <= end`, a top-up that does `end += amount` on an existing entry, and `init_if_needed` on `[raffle, user]`.
- **Merkle claims:** claim-record seeds such as `[b"claim", claimer]` in a program that supports several distributors or a `set_root`; a proof path and a signature path that `init` different guard accounts; a leaf hashed with `amount.to_le_bytes()` or sorted pairs that the repository's tree generator may encode differently.
- **Bounded designs, usually fine:** zero-copy slabs (`#[account(zero_copy)]`, `AccountLoader`, `bytemuck::Pod`) with fixed arrays such as `[Node; 1024]`, critbit or red-black trees inside a fixed-size account, concurrent merkle trees from the account-compression program.

## Auditor questions
1. Which loops, searches and account sizes depend on how much data is stored? Who can add to that data, and what does one entry cost them?
2. What is the worst case? Take the largest size the program allows, or the largest an attacker can afford, and estimate or measure the compute units. Does it fit with margin?
3. When the structure is full, what still works? Can users still cancel, withdraw and be liquidated? Must a new entry beat the worst one, or can dust entries lock everyone out?
4. When the client supplies a position or neighbour, does the program check everything the search would have guaranteed: order on both sides, both entries in use, same structure, index in range?
5. For merkle proofs: is the root read from an anchored account, is the proof length equal to the tree depth, are leaves hashed differently from inner nodes, and is the leaf bound to its index or a unique id?
6. Does removal free the slot in bounded work, or does it leave dead entries that later loops still walk?
7. For weighted draws: are the ticket ranges `[start, end)` disjoint and contiguous over `[0, total)`, so that exactly one entry matches any `r`? Does a top-up append a new range at the current total instead of extending an existing one? Are `total` and the entry set frozen before randomness is requested?
8. For merkle claims: do the claim-record seeds include the distributor, or the round when the root can change? Does every claim path, by proof or by signature, `init` the same guard account? If the tree generator is in the repository, does it produce exactly the leaf the program hashes?

## How to confirm
- **Growth denial of service:** fill the structure through the public instruction with the cheapest valid entries, then call the instruction users depend on. In a test (see `exploit-tests.md`), loop the inserts in LiteSVM until the victim call fails with an exceeded compute budget or a failed realloc. Report how many entries it took and what they cost the attacker.
- **Weak hint:** pass a wrong hint, such as a neighbour out of order, an empty cell, a cell from another market, or a proof for a different leaf or index. Show the program accepts it and the structure is now wrong: an order jumps the queue, or a leaf is changed or claimed twice.
- Without a toolchain, give the per-entry cost, the loop line and a compute estimate per iteration.

## Not a bug if
- The collection has a hard cap checked on insert, the worst case is measured to fit, and a full collection does not block exits.
- The loop runs over `remaining_accounts` that the caller chose for their own benefit, such as claiming several of their own positions at once, and nothing requires every entry to be present.
- The hint is checked against both neighbours, liveness and membership. A wrong hint only fails the caller's own transaction, and the client retries.
- Proofs go through the account-compression program or an equivalent verifier, with depth and canopy fixed when the tree was created.
- Each user's data lives in their own account, so a user can only grow what they themselves use (see divide-and-conquer).
- A weighted draw checks `start <= r < end` on an entry bound to this raffle, and top-ups create new entries.
- A merkle program has one immutable distributor, so claim records keyed only by the claimer cannot collide. A leaf encoding that differs from the generator fails closed: it is a liveness finding, not theft.

## Real-world cases
- **Done right.** A betting exchange stores its order book in a single account with a fixed number of slots. Each order records the slot numbers of the orders priced just above and below it. The client works out the right place, and the program checks the price ordering against both recorded neighbours and that both slots are occupied. The cost does not depend on the book's depth, and the book cannot outgrow its account.
- **Done right.** A compressed NFT standard keeps one merkle root on-chain for a large collection, while an indexer holds the full tree. To transfer one NFT, the client sends the leaf and the sibling hashes on its path, and the program recomputes the root. Storage is constant and the check is logarithmic.
- **Done right.** Some central limit order books keep each side of the book in a fixed-size, zero-copy tree. When a side is full, a new order must be priced better than the worst resting order, which is then removed, so spam cannot lock the book.

## Severity
- Critical when a weak proof or hint check lets a user change or claim someone else's entry, such as a forged merkle claim, or claim twice through two paths with separate guards.
- High when overlapping or gapped ticket ranges let two entries match the winning number, or none.
- For a leaf encoding that differs from the generator: Low when the root can be replaced or the funds clawed back, High otherwise, since every claim fails.
- High when cheap entries push an exit path (withdraw, cancel, settle, liquidate) past the compute or size limit, so users cannot get funds out with the deployed code.
- Medium when growth blocks only new activity, when an admin or permissionless clean-up restores service, or when a weak hint lets orders jump the queue with bounded gain.
- Low when a cap exists but leaves little margin under the compute budget.
