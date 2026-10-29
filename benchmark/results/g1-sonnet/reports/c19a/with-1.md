# Solana audit report: affiliate_rewards

## Scope
`program/src/lib.rs` (Anchor 0.31.1, single file). Instructions: create_campaign, fund, publish_epoch, register, claim. Off-chain tree building is out of scope. No toolchain was run, so no exploit tests.

## Summary
0 Critical, 0 High, 3 Medium, 3 Low, 1 Info. No unprivileged-user exploit found. Account validation, the Merkle proof (domain-separated leaves and nodes) and the cumulative-claim accounting are sound. The main risk is trust in the campaign authority, with no revocation and no solvency check.

## Findings

### F-1: Unbounded root publication lets the authority drain the vault (including third-party funds)

Severity: Medium
Location: src/lib.rs:45
Mechanism: `publish_epoch` accepts any root with no link to the vault balance or funded amount, and `claim` pays any leaf proving against any epoch, so the authority can publish a one-leaf root for its own wallet worth the whole vault and claim it, including tokens added by third parties through the permissionless `fund`.

Pattern: dependency-independency
Scenario:
1. Third parties fund the vault.
2. Authority publishes root = leaf hash(campaign, authority, vault balance).
3. Authority registers and calls `claim` with an empty proof, and the vault pays out.
Test: not attempted
Verification: downgraded by verifier from High: the authority already decides every payout by design (line 15), so this is a trust-model limitation. Medium is kept because `fund` is open to third parties.
Fix: Document the trust model. Optionally cap cumulative committed rewards against funded amounts, or add a timelock before new roots become claimable.

### F-2: Published roots can never be revoked, and every old epoch stays claimable

Severity: Medium
Location: src/lib.rs:221
Mechanism: `claim` only checks that the epoch belongs to the campaign (`has_one = campaign`), and there is no revoke, expiry or latest-epoch rule, so a root published with a wrong or inflated total stays claimable forever and the authority cannot correct it.

Pattern: one-way-state
Scenario:
1. Authority publishes a root with an erroneous inflated leaf.
2. A corrected root is published next.
3. The holder of the bad leaf still claims against the old epoch.
Test: not attempted
Verification: n/a (Medium)
Fix: Add an authority-callable `revoke_epoch` (or a revoked flag checked in `claim`), or accept only the latest epoch plus a grace window.

### F-3: No solvency accounting; claims are first-come against the vault balance

Severity: Medium
Location: src/lib.rs:102
Mechanism: Roots promise totals without checking them against the vault balance, and `total_claimed` is only written (line 117) and never enforced. If the roots over-commit, early claimants drain the vault and later valid claims fail on the token transfer until someone tops it up.
Pattern: virtual-execution
Scenario:
1. Roots total 1000 tokens but the vault holds 600.
2. The first claimants take 600.
3. Remaining affiliates hold valid proofs but their claims revert.
Test: not attempted
Verification: n/a (Medium)
Fix: Track `funded` and `committed` on the campaign and require `committed <= funded` at publish time.

## Design findings

### D-1: Authority key is fixed and there is no recovery path

Severity: Low
Location: src/lib.rs:18
Mechanism: `authority` is set once with no transfer or rotation, so if the key is lost, no new epochs can be published and affiliates cannot claim earnings accrued after the last root. A compromised key cannot be replaced either.
Pattern: checklist-authority-transfer
Test: not attempted
Fix: Add a two-step authority transfer.

### D-2: Funds cannot be withdrawn or refunded

Severity: Low
Location: src/lib.rs:28
Mechanism: The vault has no outflow other than `claim`, so surplus or mistaken deposits, and everything in an abandoned campaign, are locked permanently.
Pattern: dependency-independency
Test: not attempted
Fix: Add an authority withdrawal of unclaimed surplus, ideally after a delay or only above committed amounts.

### D-3: A claim leaf is bound to the affiliate's owner key only, and the proof length is unbounded

Severity: Low
Location: src/lib.rs:74
Mechanism: `proof: Vec<[u8; 32]>` has no length cap, so a caller can submit a very long proof that wastes compute. This only hurts the caller's own transaction, and the real risk is the transaction-size limit at large trees, which the off-chain builder must respect.
Pattern: complexity-asymmetry
Test: not attempted
Fix: Cap the proof length (for example 32).

### D-4: One campaign per (authority, mint), and epoch accounts never close

Severity: Info
Location: src/lib.rs:152
Mechanism: The campaign PDA is seeded by authority and mint only, so an authority cannot run a second campaign on the same mint, and epoch accounts (line 191) never have their rent reclaimed.
Pattern: divide-and-conquer
Test: not attempted
Fix: Add a campaign id seed and a `close_epoch` instruction.

## Coverage

| Item | Verdict | Evidence |
|---|---|---|
| root-of-trust | clear | Q1: chains end at campaign PDA (152), the stored authority signer (185), and `has_one`/`address` checks on epoch, affiliate and vault (220-231); epoch roots exist only as authority-created PDAs (191) |
| identity-rejection | n/a | no allow lists or per-wallet limits |
| untrusted-executor | found | D-1 (publishing is authority-only; claim is owner-signed) |
| role-inversion | n/a | the claimant brings the proof; the authority publishes only roots |
| lazy-loading | n/a | no iteration or accrual loops; cumulative totals are claimed lazily |
| pay-as-you-act | n/a | no aggregate or settlement step |
| divide-and-conquer | found | D-4 |
| complexity-asymmetry | found | D-3 |
| commit-execution-split | n/a | no randomness or previewable outcome |
| action-as-data | n/a | no stored executable actions |
| virtual-execution | found | F-3 |
| one-way-state | found | F-2 (`affiliate.claimed` is monotonic, lines 89-91, which is correct) |
| boundary-of-consent | n/a | claims pay the exact proven amount, with no rate to bound |
| internal-oracle | n/a | no derived prices |
| defensive-computation | n/a | no valuation or rounding |
| fail-fast | n/a | no oracle inputs |
| execution-layer-shift | n/a | no rollup or off-chain layer on chain |
| dependency-independency | found | F-1, D-2 |
| checklist-arithmetic | clear | checked_add at 53 and 119; subtraction guarded at 89-90 |
| checklist-duplicate-accounts | clear | the vault is pinned by address (175, 230) and the mutable accounts have distinct roles |
| checklist-unsafe-and-panics | clear | no unsafe code or unwraps |
| checklist-reload-after-cpi | clear | no account data is read after the CPI except `total_claimed`, which is not stale |
| checklist-token-2022 | clear | `Program<Token>` pins classic SPL Token (168, 235) |
| checklist-closing | n/a | no account closing (D-4) |
| checklist-seed-collisions | clear | distinct literal prefixes (campaign, vault, epoch, affiliate) with fixed-size keys |
| checklist-init-frontrun | clear | the campaign PDA is seeded by the authority, and the affiliate PDA by its owner signer (207) |
| checklist-realloc | n/a | no realloc |
| checklist-authority-transfer | found | D-1 |
| checklist-signers-bumps-cpi | clear | stored bump used for signing (95, 227); claim seeds match the campaign derivation (97-100) |
