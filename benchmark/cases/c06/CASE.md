---
id: c06
crate: loot_crate
pattern: commit-execution-split
class: exploit
group: g2
vulnerable: a
role: dev
author: case-author
canary: RDSAF81C11F051B132899533
---
# Case c06

## Attack scenario
`loot_crate` is a commit/reveal gamble. `open_crate` takes the `cost`, records a
`Commitment` for the player, and fixes `reveal_slot = current_slot + 1`. `reveal`
(callable by anyone) resolves the crate from the hash of `reveal_slot` taken from the
`SlotHashes` sysvar, paying the `prize` on a win. `cancel` lets the player abandon an
open crate and refunds the `cost`, with no restriction on when it may be called.

1. The attacker calls `open_crate`, paying `cost`. The outcome is now bound to the hash
   of slot `reveal_slot`, which is not yet known.
2. The attacker waits until `reveal_slot` has been produced. Its hash is now in the
   `SlotHashes` sysvar, so the attacker reads it off-chain and computes the exact
   outcome the program would compute in `reveal`.
3. If the computed outcome is a **win**, the attacker calls `reveal` and collects the
   prize. If it is a **loss**, the attacker calls `cancel` instead, which refunds the
   full `cost` and closes the commitment. Reveals are normally sent by the player, so the
   attacker simply submits `cancel` in the slot after `reveal_slot`, before anyone else
   bothers to reveal the crate.
4. Because a determined-but-losing crate can always be refunded, the attacker opens and
   resolves crates repeatedly at zero expected cost, taking only the wins — draining the
   prize pool (the house's liquidity, funded by the authority and by honest players' costs).

The root cause is that a commitment whose randomness is already determined can still be
abandoned for a refund, so the player effectively chooses the outcome after learning it.

## Why fixed closes it
`src/instructions/cancel.rs` gains a guard:
`require!(clock.slot < commitment.reveal_slot, CannotCancel)`. Cancellation is allowed
only *before* the randomness slot, while the outcome is still unknown; once `reveal_slot`
is reached the player can only `reveal` and must accept the result. Cancelling in the
commit slot (before any randomness exists) still works. A crate whose slot has passed is
never stuck: `reveal` settles it, and if the committed slot's hash is no longer available
(a skipped slot, or one older than the sysvar window) `reveal` settles it as a loss and
closes the commitment without a refund, so the per-player commitment PDA is always freed
for a new crate. Because the only path that returns the stake after the slot is `reveal`
(win or loss), there is no refund to abuse. The per-player commitment PDA already prevents
opening a second crate while one is open, and `reveal` is permissionless and sourced from
the committed slot hash, so neither re-rolling nor same-transaction resolution is possible.

## Decoys
- The commit/reveal split itself is implemented correctly: `reveal_slot` is
  `open_slot + 1`, `reveal` requires `slot > reveal_slot` (so it can never resolve in the
  commit transaction), the outcome comes from the `SlotHashes` entry of that exact slot
  (not a user-chosen value), and `reveal` is permissionless. It looks like a textbook
  commit-execution split — the only hole is the refundable `cancel`.
- `slot_hashes` is an `UncheckedAccount` pinned with `address = slot_hashes::id()`, which
  looks like an unchecked-account risk but is correctly constrained.

## Trusted roles
- **House authority** (`House.authority`): calls `init_house`, `fund_prizes`, `set_odds`
  and `withdraw`. It cannot influence the outcome of an open crate: each crate snapshots
  its `odds_bps` and `prize` into the `Commitment` at `open_crate`, and `reveal` uses the
  snapshot, so a later `set_odds` has no effect on crates already open. It cannot leave a
  won prize unpaid: `open_crate` only accepts a crate if the vault (after the player's
  cost arrives) holds at least `reserved + prize`, `reserved` holds back the prize of
  every open crate, and `withdraw` may only take the vault balance above `reserved`.
  Because the prize is at least the cost, a pre-slot cancel refund is covered as well.

Design trade-offs of the slot-hash source (identical in both versions, not part of the
planted defect): a crate whose `reveal_slot` produced no block, or that is revealed more
than about 512 slots later (after the hash leaves `SlotHashes`), settles as a loss, so
the effective odds sit slightly below `odds_bps` and a winner must reveal promptly
(anyone may call `reveal`, including the player). The leader of `reveal_slot` has the
usual limited influence of a slot-hash source.
