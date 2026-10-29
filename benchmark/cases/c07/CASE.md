---
id: c07
crate: wager_arena
pattern: execution-layer-shift
class: design
group: g3
vulnerable: b
role: heldout
author: case-author
canary: RDSAF81C11F051B132899533
---
# Case c07

## Attack scenario
`wager_arena` custodies player deposits for a ranked 1v1 game whose matches run on an
off-chain ephemeral rollup operated by the team (see `ARCHITECTURE.md`). The concrete
failure is a liveness / exit problem, not a theft. The affected party is every player
with a stake in an open match.

1. A player `deposit`s 1_000 tokens into the base-layer `vault` and calls
   `enter_match(400)`. `enter_match` flags the player `in_match`, records a
   `match_deadline`, and **moves the 400 staked tokens out of the base-layer `vault` into
   the `executor` account**, which is controlled by the team's operator key.
2. `withdraw` refuses while `in_match` is set, so even the 600 free tokens cannot leave.
   The only instructions that clear `in_match` and bring the stake back are
   `settle_match` and `expire_match`. `expire_match` looks like a timeout exit (it only
   works after `match_deadline`), but it, too, requires the operator's signature, and the
   stake it returns sits in the operator-controlled `executor`. No base-layer path exists
   that the player can use alone.
3. The team's match service stops (operator key offline, rollup halted, team gone).
   Neither `settle_match` nor `expire_match` is ever submitted. The player's `in_match`
   flag stays set forever, the 400 staked tokens are stranded in the `executor` account,
   and the 600 free tokens are frozen in the `vault` because `withdraw` is blocked.
   Every player in a match at that moment is locked out the same way.

The root cause is that user funds (not just the transient match state) are handed to the
off-chain executor and can return only through the operator, with no base-layer exit
the user controls.

## Why fixed closes it
The changes are in `src/instructions/`:
- `enter_match` no longer pays the stake out to the `executor`; it only marks `staked`,
  `in_match` and `match_deadline`. The funds stay in the base-layer `vault` (only the
  match *state* is handed to the rollup).
- `settle_match` and `expire_match` no longer pull tokens back from the `executor`; they
  only move the stake from `staked` to the free balance (custody never left the vault).
- `withdraw` gains a base-layer timeout exit: once `now >= match_deadline`, it releases
  the stake back to the free balance and clears `in_match`, so a player can always
  withdraw from the vault without the operator.

Normal play is unchanged: deposit, enter a match, the operator settles it, withdraw.

## Decoys
- The free-balance custody path is correctly built: `deposit` moves tokens into the
  program-owned base-layer `vault`, and the free balance leaves only through the owner's
  own `withdraw` (signed by the player, paid by the arena PDA, no operator involved). It
  is the same custody/exit class as the flaw, done right; only the *staked* funds leave
  the base layer.
- `settle_match` is gated by `arena.operator` (the operator is a legitimately trusted
  referee), which looks like an authority risk but is the intended trust boundary.
- The `vault` and `executor` token accounts are pinned by `address = arena.*` and user
  token accounts by `token::authority`, so the account wiring is correctly constrained.

## Trusted roles
- **Arena authority** (`Arena.authority`): creates the arena. No custody power at
  runtime.
- **Operator** (`Arena.operator`): the match service that submits `settle_match` (and
  `expire_match` after a deadline) to report results. It is trusted to referee matches.
  In the fixed version it moves no funds (custody stays in the vault); in the vulnerable
  version it holds the staked funds in the `executor` account until it settles.
