# Solana audit report: group_buy (Anchor)

Scope: `program/src` (lib.rs, state.rs, instructions/buyer.rs, instructions/merchant.rs, instructions/mod.rs) plus `program/ARCHITECTURE.md`.
No Critical or High findings. Token flows (commit, refund, claim_proceeds) are guarded correctly: the escrow is a PDA owned by the deal, `address = deal.escrow` is enforced, refunds are single-use, and arithmetic is checked.

## Findings

Severity: Medium
Location: program/src/instructions/buyer.rs:26
Mechanism: `join_deal` is free and permissionless, pushes a zero-unit entry into a Vec capped at 100, and nothing ever removes entries. An attacker with 100 throwaway wallets fills the list, so no real buyer can join or commit (`commit` requires a prior join) and the deal fails at its deadline.
Verification: reviewed by the finding-verifier agent, which downgraded the finding from High to Medium. Funds are not at risk, because a failed deal refunds through `refund`. Not tested with an exploit test (no toolchain).

Severity: Medium
Location: program/src/instructions/merchant.rs:73
Mechanism: `create_deal` does not require `min_units <= MAX_BUYERS * max_units_per_buyer`. With the documented parameters (for example 2,000 units, minimum 1,000, limit 5 per buyer) at most 500 units can ever be committed. Such a deal can never succeed, and buyers' funds are tied up until the deadline.
Verification: confirmed by the finding-verifier agent.

Severity: Medium
Location: program/src/instructions/merchant.rs:137
Mechanism: Only `deal.authority` can call `claim_proceeds`, and a succeeded deal cannot be refunded. If the merchant key is lost or the merchant stops cooperating, the escrow is locked forever. The authority cannot be rotated, and no one else can trigger settlement or pay out.

Severity: Low
Location: program/src/instructions/merchant.rs:48
Mechanism: The merchant chooses any SPL mint without an allow-list. A mint with a freeze authority, or a worthless mint, lets the merchant freeze buyer or escrow token accounts, or take real payment in a worthless token. This blocks refunds or claims for that deal.

## Design findings

Severity: Medium
Location: program/src/state.rs:11
Mechanism: All buyers are stored in one `Vec<Commitment>` inside the single Deal account, capped at `MAX_BUYERS = 100`. Every buyer instruction writes this one hot account, and the deal's capacity is bounded by it. Per-buyer PDA accounts with a running total on the deal would remove both the cap and the squatting problem.

Severity: Info
Location: program/src/instructions/buyer.rs:27
Mechanism: The off-chain fulfilment service reads `buyers` after the claim. The list also holds zero-unit entries and refunded entries, so the service must filter on `units > 0` and `!refunded`. Otherwise it ships to non-buyers.

Severity: Info
Location: program/src/instructions/merchant.rs:52
Mechanism: Deal and escrow accounts are never closed. The merchant's rent deposit stays locked after the deal ends, and state grows without bound.

## Coverage

The full 29-row pattern and checklist coverage table was not produced. The checks run were root-of-trust on each instruction's accounts, the arithmetic and refund-reuse review, and the executor, liveness and scalability review described above.
