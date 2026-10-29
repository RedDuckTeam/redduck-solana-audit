# wager_arena deployment

## Overview
`wager_arena` holds player deposits for a ranked 1v1 game. Match play itself is too
latency-sensitive for the base layer, so matches are run on an ephemeral rollup operated
by the team. The base-layer program keeps the custody and the player records; the rollup
runs the live match.

## Accounts
- **Arena**: one per token mint. Holds the authority key, the operator key, the base-layer
  `vault` (a program-owned token account), and the `executor` token account.
- **Player**: one per wallet per arena. Tracks the free `balance`, the `staked` amount,
  the `in_match` flag, `match_deadline`, and a `wins` counter.

## Operation
- The `executor` token account is controlled by the team's match service (its authority is
  the operator key). It is the rollup's settlement account on the base layer.
- A player calls `deposit` to move tokens into the base-layer `vault`.
- When a player calls `enter_match`, the staked amount is moved from the `vault` into the
  `executor` account and the player is flagged `in_match`. The match then runs on the
  rollup.
- A service run by the team watches matches and, when one finishes, submits `settle_match`
  with the operator key. `settle_match` moves the stake back from the `executor` account to
  the `vault`, clears `in_match`, and records the result. If a match runs past its
  `match_deadline`, the operator may instead submit `expire_match`, which returns the stake
  and clears `in_match` without a result.
- `withdraw` moves free balance from the `vault` back to the player's wallet.

## Keys
- **authority**: creates the arena. Team-held.
- **operator**: the match service key that submits `settle_match` and `expire_match`.
  Team-held.
