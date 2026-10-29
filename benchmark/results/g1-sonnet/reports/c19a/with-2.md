# Solana audit report: affiliate_rewards

## Scope
`program/src/lib.rs` (Anchor 0.31.1, single program). Instructions reviewed: `create_campaign`, `fund`, `publish_epoch`, `register`, `claim`. No exploit tests were run (no toolchain attempted).

## Summary
No account-validation or arithmetic bugs found. Four design findings (all Medium or Low, one Info). The main one is that the campaign authority is fully trusted over third-party funds: it can publish any root and drain the vault, and nothing can revoke a root or recover funds.

## Findings
No findings outside the design section below. Account validation, the PDA chains, the Merkle leaf and node domain separation, and the cumulative `claimed` accounting are sound.

## Design findings

### F-1: Authority can publish an arbitrary root and drain the vault

Severity: Medium
Location: src/lib.rs:45
Mechanism: `publish_epoch` accepts any root from the authority with no timelock or cap against the vault balance, and `fund` is open to anyone, so the authority can publish a one-leaf root for itself, register and claim the whole vault in one transaction, taking funders' tokens.

Pattern: dependency-independency
Scenario:
1. Authority calls `publish_epoch` with root = hash of leaf (campaign, authority, vault balance).
2. Authority calls `register`.
3. Authority calls `claim(balance, [])`. `verify_proof` with an empty proof returns `leaf == root`, so the vault pays out.
Test: not attempted
Verification: downgraded by verifier from High: needs the privileged authority key; funders choose to trust it, so it is a trust and centralization finding.
Fix: Add a timelock or challenge window between publishing and claimability, cap `total_claimed` at total funded, and consider a multisig authority.

### F-2: Published epochs can never be revoked, and every old epoch stays claimable

Severity: Medium
Location: src/lib.rs:221
Mechanism: `claim` accepts any epoch of the campaign and `EpochRoot` has no revoked or active flag, so a wrong or malicious root stays valid forever and a later root cannot supersede it.

Pattern: one-way-state
Scenario:
1. Authority publishes a root with an erroneous inflated leaf.
2. The next epoch corrects it, but the bad epoch is still accepted by `claim`.
3. The holder of the inflated leaf claims against the old epoch.
Test: not attempted
Verification: n/a (Medium)
Fix: Accept only the latest epoch, or add a revoke instruction for the authority that is itself delay-bound.

### F-3: No withdrawal or authority transfer, so surplus funds are locked and the key is a single point of failure

Severity: Medium
Location: src/lib.rs:240
Mechanism: `Campaign.authority` is set once and the program has no withdraw, close or authority-transfer instruction, so overfunded or unallocated tokens are locked permanently, and a lost authority key stops all new epochs, leaving later earnings unclaimable.

Pattern: untrusted-executor
Scenario:
1. Funders deposit more than the published leaves cover, or the authority key is lost.
2. No instruction can move the remaining tokens or publish a new root.
Test: not attempted
Verification: n/a (Medium)
Fix: Add authority rotation and an authority-gated, delayed sweep of unallocated funds.

### F-4: No solvency link between roots and vault balance, so claims are first come first served

Severity: Low
Location: src/lib.rs:90
Mechanism: Nothing checks that published totals are covered by the vault, so if roots promise more than was funded, early claimers are paid and later affiliates' `token::transfer` fails.

Pattern: virtual-execution
Scenario:
1. Roots total 1,000 owed while the vault holds 600.
2. Early claimers drain the vault and later valid claims revert.
Test: not attempted
Verification: n/a (Low)
Fix: Track total committed per epoch and require it to be at most total funded when publishing.

### F-5: Redundant zero check in claim

Severity: Info
Location: src/lib.rs:93
Mechanism: `owed > 0` is always true after `require!(earned_total > affiliate.claimed)` at line 89, so the guard is dead code with no security impact.

Pattern: checklist-arithmetic
Test: not attempted
Verification: n/a (Info)
Fix: Remove the condition.

## Coverage

| Item | Verdict | Evidence |
|---|---|---|
| root-of-trust | clear | Q1: campaign PDA seeds src/lib.rs:152, vault `address = campaign.vault` :230, epoch `has_one = campaign` :220, affiliate seeds :226; chains end at PDAs and the authority signer |
| identity-rejection | n/a | no allow lists or per-wallet rules |
| untrusted-executor | found | F-3 |
| role-inversion | n/a | operator pushes roots, but consumers already bring proofs; trust issue is F-1 |
| lazy-loading | clear | per-affiliate `claimed` marker, no iteration, :88-91 |
| pay-as-you-act | clear | cumulative totals in leaves, no bulk step |
| divide-and-conquer | clear | per-affiliate and per-epoch PDAs :190, :207 |
| complexity-asymmetry | clear | proof checked in full at :134; length bounded only by transaction size |
| commit-execution-split | n/a | no randomness or previewable outcome |
| action-as-data | n/a | no stored actions |
| virtual-execution | found | F-4 |
| one-way-state | found | F-2 (`claimed` itself is monotonic, :89-91) |
| boundary-of-consent | n/a | amounts are exact leaf totals, no computed rates |
| internal-oracle | n/a | no derived prices |
| defensive-computation | n/a | no valuation or rounding |
| fail-fast | n/a | no oracle inputs |
| execution-layer-shift | n/a | no other layer |
| dependency-independency | found | F-1 |
| checklist-arithmetic | clear | checked_add :53, :119; subtraction guarded by :89 |
| checklist-duplicate-accounts | clear | no two mutable accounts of the same type that can alias harmfully |
| checklist-unsafe-and-panics | clear | no unsafe, no unwrap |
| checklist-reload-after-cpi | clear | no reads of token balances after CPI |
| checklist-token-2022 | clear | `Program<Token>` only, :168 |
| checklist-closing | n/a | no account closing (affiliate and epoch accounts are never closed) |
| checklist-seed-collisions | clear | distinct prefixes b"campaign", b"vault", b"epoch", b"affiliate"; leaf and node prefixes :8-9 |
| checklist-init-frontrun | clear | campaign PDA seeded by authority, so no one else can squat it :152 |
| checklist-realloc | n/a | no realloc |
| checklist-authority-transfer | found | F-3 |
| checklist-signers-bumps-cpi | clear | bump stored :23 and used :95; Signer on authority, owner, funder |
