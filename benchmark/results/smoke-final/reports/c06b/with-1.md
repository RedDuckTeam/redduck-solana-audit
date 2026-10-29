# Solana audit report: loot_crate

## Scope
Anchor program in `program/` (7 instructions: init_house, fund_prizes, set_odds, open_crate, reveal, cancel, withdraw; accounts House and Commitment). No tests or toolchain were run; findings come from source review.

## Summary
0 Critical, 0 High, 3 Medium, 3 Low. No path was found for a normal user to steal vault funds. Reserved-prize accounting, PDA seeds and signer checks hold. The weak points are the randomness source and the unbounded authority power over odds.

## Findings

### F-1: Outcome depends on a slot hash that the slot's leader can influence or skip

Severity: Medium
Location: src/instructions/reveal.rs:75
Mechanism: The outcome is the SlotHashes entry of `reveal_slot` (open slot + 1), and a missing entry settles as a loss. The leader of that slot can grind its block contents or skip the slot, so a validator, or a player colluding with one, can bias outcomes. A skipped slot is a guaranteed house win.

Pattern: commit-execution-split
Scenario:
1. A leader-controlled player opens a crate in the slot before its own leader slot.
2. The leader checks the resulting hash and drops or reorders transactions, or skips the slot, to get a favourable result.
3. The house loses prizes, or players lose their stake with no refund.
Test: not attempted
Verification: not required (Medium)
Fix: Use a VRF or oracle randomness, or at least a reveal slot several slots ahead. Refund the player on a missing hash instead of settling a loss.

### F-2: Player cannot bound the odds, prize or cost that the authority can change before the open

Severity: Medium
Location: src/instructions/open.rs:34
Mechanism: `open_crate` takes no expected odds, prize or cost, and reads them from `House` at execution time. The authority can call `set_odds` (set_odds.rs:24) between a player's signing and landing and drop odds to 1 bps. The player pays the full cost for a nearly impossible prize.

Pattern: boundary-of-consent
Scenario:
1. A player signs `open_crate` against odds of 5000 bps.
2. The authority front-runs with `set_odds(1, prize)`.
3. The player's open lands with the new odds, and the commitment snapshots them.
Test: not attempted
Verification: not required (Medium)
Fix: Add `min_odds_bps`, `min_prize` and `max_cost` arguments to `open_crate` and check them against the House.

### F-3: Anyone can create the House for a mint, and the authority can never be rotated

Severity: Low
Location: src/instructions/init.rs:25
Mechanism: The House PDA is seeded only by the mint, so the first caller of `init_house` becomes its permanent authority. A squatter can pre-empt the intended operator for a mint. No instruction transfers authority, so a lost or compromised key cannot be replaced.

Pattern: checklist-init-frontrun
Scenario:
1. An attacker calls `init_house` for a popular mint with their own parameters.
2. The real operator cannot create a house for that mint.
Test: not attempted
Verification: not required (Low)
Fix: Add an authority or a house id to the seeds, or gate init to a known deployer. Add a two-step authority transfer.

### F-4: Reserved-prize accounting uses saturating_sub

Severity: Low
Location: src/instructions/reveal.rs:98
Mechanism: `reserved` is decreased with `saturating_sub`, here and at cancel.rs:56. An accounting drift would be silently clamped at zero instead of failing. Over-reserving would lock authority withdrawals, and under-reserving would let the authority withdraw funds owed to winners.

Pattern: fail-fast
Scenario: Any future bug that desynchronises `reserved` stays hidden until a winner's transfer fails or the authority withdraws too much.
Test: not attempted
Verification: not required (Low)
Fix: Use `checked_sub` and return an error.

## Design findings

### F-5: Single authority controls odds and prize with no limits, timelock or rotation

Severity: Medium
Location: src/instructions/set_odds.rs:19
Mechanism: One key sets odds (any value from 1 to 10000 bps) and prize immediately, with no timelock, bounds or event. It can make the game unwinnable for players. A stolen key does the same. It also withdraws all unreserved vault funds (withdraw.rs:27), which is its stated purpose.

Pattern: dependency-independency
Scenario: The authority sets odds to 1 bps. Players keep paying `cost` for an effectively impossible prize (see F-2).
Test: not attempted
Verification: not required (Medium)
Fix: Use a timelock or a multisig authority, a minimum odds floor, and emit an event on change.

### F-6: Every open, reveal and cancel writes the single House account

Severity: Low
Location: src/instructions/open.rs:12
Mechanism: `house` is mutable in `open_crate`, `reveal` and `cancel` because of the shared `reserved` counter. All players serialise on one writable account, which limits throughput under load. Pending commitments are also one per player and house (open.rs:19), so a player is blocked until their commitment is revealed or cancelled.

Pattern: divide-and-conquer
Scenario: A busy house sees transaction contention and failed opens.
Test: not attempted
Verification: not required (Low)
Fix: Accept the limit, or shard the reserve counter.

## Coverage

| Item | Verdict | Evidence |
|---|---|---|
| root-of-trust | clear | Q: chain ends at House PDA seeds and `has_one`/`address = house.vault`, open.rs:12-24 |
| identity-rejection | n/a | no allow lists or per-wallet rules beyond the one-commitment PDA |
| untrusted-executor | clear | `reveal` is callable by any signer and pays only the recorded player, reveal.rs:26,35 |
| role-inversion | n/a | no operator-pushed data |
| lazy-loading | n/a | no loops over holders or accrual |
| pay-as-you-act | clear | `reserved` updated in open, reveal and cancel |
| divide-and-conquer | found | F-6 |
| complexity-asymmetry | n/a | the SlotHashes scan is bounded by the sysvar's 512 entries |
| commit-execution-split | found | F-1 |
| action-as-data | n/a | no stored executable instructions |
| virtual-execution | n/a | prizes are paid at reveal, with the obligation reserved at open |
| one-way-state | clear | commitment closed on reveal and cancel (reveal.rs:18, cancel.rs:17), so no double settle |
| boundary-of-consent | found | F-2 |
| internal-oracle | n/a | no derived prices |
| defensive-computation | n/a | no valuation or oracle math |
| fail-fast | found | F-4 |
| execution-layer-shift | n/a | no rollup or delegation |
| dependency-independency | found | F-5 |
| checklist-arithmetic | clear | checked_add in open.rs:50,57,64 |
| checklist-duplicate-accounts | clear | no instruction takes two mutable accounts of the same type that could be swapped to exploit it |
| checklist-unsafe-and-panics | clear | the `unwrap`s in reveal.rs:55,76 are on fixed-size slices after a bounds check |
| checklist-reload-after-cpi | clear | `vault.reload()` at open.rs:49 |
| checklist-token-2022 | n/a | only the classic `Token` program is accepted |
| checklist-closing | clear | Anchor `close = player` in reveal.rs:18 and cancel.rs:17 |
| checklist-seed-collisions | clear | distinct seed prefixes `house`, `vault`, `crate` |
| checklist-init-frontrun | found | F-3 |
| checklist-realloc | n/a | no realloc |
| checklist-authority-transfer | found | F-3 |
| checklist-signers-bumps-cpi | clear | stored bumps are used and the token program is `Program<Token>`, open.rs:30 |
