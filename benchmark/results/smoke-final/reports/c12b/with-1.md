# Solana audit report: group_buy (Anchor)

No Critical or High findings. Escrow custody, signer checks, PDA derivation and refund/claim state transitions hold. The findings below are Medium or lower, so the verifier step (High/Critical only) was not needed.

## Findings

Severity: Medium
Location: program/src/instructions/buyer.rs:16
Mechanism: `join_deal` is permissionless, takes no deposit beyond rent and has no cap, so anyone can create unlimited zero-unit `Commitment` accounts per deal (one per wallet). The operator's fulfilment service enumerates all of a deal's `Commitment` accounts with `getProgramAccounts` (ARCHITECTURE.md:6), so spam bloats that list, can slow or time out the query and delays shipping. `buyer_count` (buyer.rs:28) is also inflated.

Severity: Low
Location: program/src/instructions/buyer.rs:108
Mechanism: `refund` sets `refunded = true` but never closes the `Commitment`, and no instruction closes `Commitment` or the escrow token account. The buyer-paid rent (buyer.rs:16) is locked forever, and dead accounts stay in the fulfilment service's `getProgramAccounts` results.

Severity: Low
Location: program/src/instructions/buyer.rs:60
Mechanism: The merchant (or sybil wallets it controls) can commit its own funds through `commit` to push `total_units` over `min_units` just before the deadline. This removes real buyers' refund right on a deal that would otherwise have failed, and the merchant then claims everything in `claim_proceeds`. The minimum is therefore not a guarantee of real demand. Per-buyer limits do not stop this because wallets are free.

Severity: Low
Location: program/src/instructions/merchant.rs:48
Mechanism: The merchant picks any `mint`, including one with a freeze authority (for example USDC) or a worthless or fake token. A freeze or a worthless mint can lock escrowed buyer funds or make "payment" meaningless, and the program does not check an allowed-mint list.

## Design findings

Severity: Medium
Location: program/src/instructions/merchant.rs:148
Mechanism: Liveness and centralization: only the merchant `authority` can call `claim_proceeds`, and there is no way to rotate that key (`Merchant.authority` and `Deal.authority` are fixed at state.rs:15 and merchant.rs:88). If the merchant key is lost or the merchant does not claim, the escrow of a succeeded deal is stuck forever, and buyers cannot refund because the outcome is Succeeded. The same applies to the operator-run dashboard flow.

Severity: Info
Location: program/src/instructions/merchant.rs:125
Mechanism: The merchant can cancel any open deal, including one that has already reached its minimum. Buyers are refunded, so there is no loss of funds, but the deal can be withdrawn at will.

## Coverage

| Item | Verdict |
|---|---|
| root-of-trust | clear: deal, commitment and escrow are tied by `has_one`, `address = deal.escrow` and PDA seeds (buyer.rs:45, 47; merchant.rs:139) |
| identity-rejection | clear: no allow or deny lists. Per-buyer limit is wallet-based, see the sybil finding (buyer.rs:60) |
| untrusted-executor | found: claim is merchant-only (merchant.rs:148) |
| role-inversion | n/a: no operator-pushed data |
| lazy-loading | n/a: no accrual |
| pay-as-you-act | clear: `total_units` is updated on every commit (buyer.rs:70) |
| divide-and-conquer | clear: per-buyer `Commitment` accounts |
| complexity-asymmetry | found: unbounded Commitment enumeration (buyer.rs:16) |
| commit-execution-split | n/a: no randomness |
| action-as-data | n/a |
| virtual-execution | n/a |
| one-way-state | clear: the `refunded` flag (buyer.rs:115) and the status transitions (merchant.rs:155) are one-way |
| boundary-of-consent | clear: the price is fixed at creation and cannot change (state.rs:53) |
| internal-oracle | n/a |
| defensive-computation | clear: checked math (state.rs:77) |
| fail-fast | n/a |
| execution-layer-shift | n/a |
| dependency-independency | found: freeze-authority mint (merchant.rs:48) |
| checklist-arithmetic | clear: checked_* throughout (buyer.rs:62, 68) |
| checklist-duplicate-accounts | clear: no two mutable accounts of the same type are interchangeable |
| checklist-unsafe-and-panics | clear: no unsafe code and no unwrap |
| checklist-reload-after-cpi | clear: balances are not read after a CPI |
| checklist-token-2022 | n/a: the legacy `Token` program only (buyer.rs:51) |
| checklist-closing | found: nothing is ever closed (buyer.rs:108) |
| checklist-seed-collisions | clear: distinct seed prefixes (state.rs:5-8) |
| checklist-init-frontrun | clear: seeds include the authority or the merchant counter (merchant.rs:18, 53) |
| checklist-realloc | n/a |
| checklist-authority-transfer | found: no rotation (merchant.rs:148) |
| checklist-signers-bumps-cpi | clear: the deal PDA signs with its stored bump (mod.rs:21-31) |
