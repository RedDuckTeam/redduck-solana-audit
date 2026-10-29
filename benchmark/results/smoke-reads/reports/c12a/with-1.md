# Solana audit report: group_buy (Anchor)

## Scope
Framework: Anchor. Instructions: register_merchant, create_deal, cancel_deal, claim_proceeds, join_deal, commit, refund. Files read: all of `program/src` and `program/ARCHITECTURE.md`.
Surfaces found: per-user list stored in one account, permissionless registration, escrow PDA payouts, and a time-based deal lifecycle. Not every pattern reference file was opened. The coverage table below is a condensed summary, not the full 29-row table.
No Solana toolchain was used, so nothing was exploit-tested.

## Findings

Severity: Medium
Location: program/src/instructions/buyer.rs:26
Mechanism: `join_deal` is free and permissionless and appends a zero-unit slot to a buyer list capped at 100 (`state.rs:11`). Slots are never freed. Anyone with 100 throwaway keys, or the app's own page views (ARCHITECTURE.md: opening a deal page sends `join_deal`), can fill the list, so real buyers get `NotJoined` on `commit` and the deal fails. Funds are not lost because buyers who committed get refunded.
Verification: finding-verifier confirmed reachability and downgraded it from High to Medium (availability only, principal is refundable).

Severity: Medium
Location: program/src/instructions/merchant.rs:73
Mechanism: `create_deal` never checks that `min_units` can be reached with `MAX_BUYERS (100) * max_units_per_buyer`. With the documented parameters (300 to 2,000 units, a minimum of about half, 1 to 5 units per buyer), many deals can never succeed even with no attacker. They always end in refunds.
Verification: finding-verifier confirmed.

Severity: Low
Location: program/src/instructions/merchant.rs:128
Mechanism: The merchant can cancel any open deal before the deadline, even after the minimum is reached. This lets it back out of a sale that has succeeded, or hold buyers' funds until the cancel and refund steps are done. Buyers can still refund their principal.

Severity: Info
Location: program/src/instructions/merchant.rs:48
Mechanism: The merchant picks any SPL mint for a deal, and the program does not check it against an allow list. The operator's UI has to enforce USDC. A fake or freezable mint could trap buyer funds, because the mint authority can freeze the escrow or buyer accounts.

## Design findings

Severity: Medium
Location: program/src/state.rs:59
Mechanism: The buyer list is stored inside the single `Deal` account with a hard cap of 100 entries. Every buyer action writes this one account and scans it linearly. This limits scale: the cap makes deals of 300 to 2,000 units practical only with large per-buyer limits, and it is what makes the slot-exhaustion attack above possible. Using a per-buyer PDA instead would remove the cap and the contention.

Severity: Info
Location: program/ARCHITECTURE.md:6
Mechanism: The operator's fulfilment service relies on the on-chain buyer list. The list also holds zero-unit joiners, so it must filter on `units > 0`. Fulfilment is an off-chain dependency and does not affect fund safety.

## Checked and clear
- Escrow signing: the escrow is tied to the deal by `address = deal.escrow` (buyer.rs:42, buyer.rs:93, merchant.rs:139), and the payout is signed by the deal PDA through `pay_from_escrow`.
- Double claim: `claim_proceeds` requires `status == Open` and then sets `Claimed` (merchant.rs:151-155).
- Double refund: `refunded` is set before the transfer (buyer.rs:108-109).
- `claim_proceeds` and `refund` cannot both succeed, because their outcomes are mutually exclusive (state.rs:66-74).
- Arithmetic is checked, and `create_deal` bounds `price * max_units` (merchant.rs:80-83).
- Authority checks are present (`has_one = authority`). Merchant and deal PDAs use seeds.

## Coverage summary
Found: complexity-asymmetry / divide-and-conquer (buyer list cap), fail-fast (min_units reachability), boundary-of-consent (merchant cancel).
Clear: root-of-trust (PDAs and `address` constraints), one-way-state (status and refunded flags), checklist-arithmetic, checklist-duplicate-accounts, checklist-signers-bumps-cpi, checklist-closing.
N/A: oracle, internal-oracle, defensive-computation, virtual-execution, execution-layer-shift, commit-execution-split, action-as-data, lazy-loading, pay-as-you-act, checklist-realloc, checklist-token-2022 (classic SPL token only), checklist-unsafe-and-panics, checklist-reload-after-cpi, checklist-seed-collisions, checklist-init-frontrun, checklist-authority-transfer, identity-rejection, untrusted-executor, role-inversion, dependency-independency.
