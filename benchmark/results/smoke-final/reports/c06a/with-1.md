# Solana audit report: loot_crate (program/)

## Findings

### F1
Severity: Critical
Location: program/src/instructions/cancel.rs:33
Mechanism: `cancel` has no slot or state restriction and refunds the full `paid` cost, while the outcome hash for `reveal_slot` is public once that slot passes. A player computes the roll off-chain, calls `reveal` if it wins and `cancel` if it loses, so they win every time with no downside and drain the vault.
Verification: confirmed by finding-verifier (no toolchain exploit test run).

### F2
Severity: High
Location: program/src/instructions/open.rs:50
Mechanism: The solvency check counts the vault balance, which includes players' pending `paid` costs that `cancel` refunds and `withdraw` can take, as backing for `reserved` prizes. With a small `fund_prizes` deposit, many crates can be opened whose combined prizes exceed real funds, so winning `reveal` transfers (reveal.rs:81) fail or the vault becomes insolvent.

### F3
Severity: Medium
Location: program/src/instructions/withdraw.rs:29
Mechanism: `free = vault - reserved` ignores that pending commitments' `paid` amounts are refundable via `cancel`. The authority can withdraw down to `reserved`, after which cancels or later reveals can fail for lack of funds.

### F4
Severity: Medium
Location: program/src/instructions/reveal.rs:75
Mechanism: If the SlotHashes entry is missing (skipped slot, or older than about 512 slots), the crate silently settles as a loss. A skipped slot, or a delay before anyone reveals, makes a legitimate player lose their cost without any random outcome. Reveal is also permissionless, so the timing is controlled by whoever calls it first.

### F5
Severity: Low
Location: program/src/instructions/reveal.rs:98
Mechanism: `saturating_sub` on `reserved` (also cancel.rs:51) masks accounting errors instead of failing, so drift in `reserved` goes undetected and can later over-permit `withdraw`.

### F6
Severity: Low
Location: program/src/instructions/init.rs:25
Mechanism: The house PDA is seeded only by the mint and `init_house` is permissionless. Anyone can create the house for a mint first with their own authority and parameters, which blocks the intended operator and can lure players into an attacker-run house.

### F7
Severity: Low
Location: program/src/instructions/init.rs:51
Mechanism: The authority is a single key with no transfer or rotation instruction. The authority can change `odds_bps` and `prize` at will (set_odds.rs:19) and withdraw all free funds (withdraw.rs:27). Loss of the key strands the house's funds permanently, and a compromised key gives full control.

## Design findings

- Randomness comes from a recent slot hash (reveal.rs:75), which is influenceable by validators and not suitable for high-value outcomes. It must be paired with a commit that cannot be abandoned (see F1).
- Every crate opens against a single shared vault and `House` account (open.rs:12), so all players write to one hot account. This limits throughput and serializes transactions.
- Only one crate can be open per player per house (PDA seeds at open.rs:19), which is a minor usability limit.

## Coverage

| Item | Verdict |
|---|---|
| commit-execution-split | found (F1) |
| boundary-of-consent | found (F4; players cannot bound the cost or odds they get, since they are read from mutable `House` state at open.rs:35) |
| defensive-computation | found (F2) |
| one-way-state | found (F1, F5) |
| dependency-independency | found (F7) |
| untrusted-executor | clear (reveal is permissionless, reveal.rs:11) |
| root-of-trust | clear (PDAs with seeds, `address = house.vault`) |
| checklist-init-frontrun | found (F6) |
| checklist-authority-transfer | found (F7) |
| checklist-arithmetic | clear (checked_add in open.rs:50) except F5 |
| other patterns and checklist items | n/a or clear; not exercised in this small program |
