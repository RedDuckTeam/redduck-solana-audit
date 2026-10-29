# Solana audit report: affiliate_rewards

## Scope
`program/src/lib.rs` (Anchor 0.31.1, SPL Token). Instructions: create_campaign, fund, publish_epoch, register, claim. No toolchain was run; no exploit tests.

## Summary
0 Critical, 0 High, 3 Medium, 2 Low, 1 Info. Main issues: the authority fully controls payouts, and old roots can never be revoked.

## Findings

### F-1: Old epoch roots stay claimable forever
Severity: Medium
Location: src/lib.rs:221
Mechanism: `claim` accepts any epoch of the campaign, and epochs can't be revoked or superseded. If the authority publishes a corrected root (wrong or fraudulent earnings lowered or removed), the affiliate can still claim against the older, larger root.

Pattern: one-way-state
Test: not attempted
Verification: not required (Medium)
Fix: Only accept the latest epoch (`epoch.index == campaign.epochs - 1`), or add a revoked flag the authority can set.

### F-2: Claim does not check vault solvency; claims are first-come-first-served
Severity: Low
Location: src/lib.rs:102
Mechanism: Roots are not checked against the vault balance, so over-committed roots make later claims fail on the token transfer while earlier claimants are paid in full.

Pattern: defensive-computation
Test: not attempted
Fix: Track funded totals and require the root's committed total (published with the root) to be covered by the vault.

### F-3: Only the legacy SPL Token program is supported; mint freeze authority can freeze the vault
Severity: Low
Location: src/lib.rs:165
Mechanism: `mint` is any mint chosen by the creator, with no check of freeze authority. A freezable mint lets its freeze authority freeze the vault and block every claim.

Pattern: checklist-token-2022
Test: not attempted
Fix: Reject mints with a freeze authority, or document it as a trust assumption.

### F-4: Redundant `owed > 0` branch
Severity: Info
Location: src/lib.rs:93
Mechanism: `require!(earned_total > claimed)` at line 89 already guarantees `owed > 0`, so the branch is dead code. No security impact.

Pattern: checklist-arithmetic
Test: not attempted

## Design findings

### F-5: Authority can publish any root and drain the vault; no recovery path or key rotation
Severity: Medium
Location: src/lib.rs:45
Mechanism: `publish_epoch` accepts any root with no cap, timelock or dispute window, so the authority (or an attacker holding its key) can add a leaf for itself worth the whole vault and claim it, taking the funds other parties deposited through the permissionless `fund`. The campaign PDA is seeded by the authority (line 152) and there is no authority transfer. A lost key stops new epochs, and there is no instruction to withdraw or close the vault, so leftover funds can get stuck.

Pattern: dependency-independency
Scenario:
1. Third parties fund the vault.
2. Authority publishes a root with one leaf (campaign, authority, vault balance).
3. Authority registers and claims with an empty proof, and the vault pays out.
Test: not attempted
Verification: downgraded by verifier from High: the mechanism is real, but it needs the campaign authority's key, which is the payout oracle by design, and loss is bounded to one campaign's vault. Kept at Medium because third-party funders have no refund path.
Fix: Timelock plus cap on newly committed totals, a funder-side refund/close path after expiry, and authority rotation (for example a multisig).

### F-6: Liveness depends on one off-chain publisher
Severity: Medium
Location: src/lib.rs:45
Mechanism: Affiliates can only claim earnings that the single authority has published in a root. If the authority stops publishing, new earnings can't be paid and nobody else can publish.

Pattern: untrusted-executor
Test: not attempted
Verification: not required (Medium)
Fix: Document and use a multisig or replaceable publisher, with a way to move the campaign to a new authority.

## Coverage

| Item | Verdict | Evidence |
|---|---|---|
| root-of-trust | clear | Q: chain ends at campaign PDA seeds (152), has_one authority (185), vault address check (230); epoch tied by has_one campaign (220) |
| identity-rejection | n/a | no allow/deny lists |
| untrusted-executor | found | F-6 |
| role-inversion | n/a | the authority pushes roots, but users bring proofs, which is already the flipped design |
| lazy-loading | clear | per-affiliate `claimed` marker, no iteration (89-91) |
| pay-as-you-act | clear | no batch step |
| divide-and-conquer | clear | per-affiliate and per-epoch PDAs |
| complexity-asymmetry | clear | proof checked in a loop over the caller's own proof (134-144); only the caller pays |
| commit-execution-split | n/a | no randomness or previewable outcomes |
| action-as-data | n/a | no stored actions |
| virtual-execution | n/a | no obligations |
| one-way-state | found | F-1 (`claimed` is monotonic, line 91) |
| boundary-of-consent | n/a | claim pays a deterministic amount |
| internal-oracle | n/a | no derived prices |
| defensive-computation | found | F-2 |
| fail-fast | n/a | no external oracle |
| execution-layer-shift | n/a | none |
| dependency-independency | found | F-5 |
| checklist-arithmetic | clear | checked_add (53, 117); subtraction guarded at 89; F-4 |
| checklist-duplicate-accounts | clear | no two mutable accounts of the same type can alias harmfully |
| checklist-unsafe-and-panics | clear | no unsafe code or unwraps |
| checklist-reload-after-cpi | clear | no state read after CPI depends on token balances |
| checklist-token-2022 | found | F-3 |
| checklist-closing | n/a | no close instructions (see F-5 for lack of a recovery path) |
| checklist-seed-collisions | clear | distinct prefixes, fixed-size seeds (152, 159, 191, 207); leaf/node prefixes at 7-8 |
| checklist-init-frontrun | clear | campaign PDA is per authority; epoch only by authority; affiliate needs the owner's signature |
| checklist-realloc | n/a | no realloc |
| checklist-authority-transfer | found | F-5 (no transfer) |
| checklist-signers-bumps-cpi | clear | owner signer (234), stored bump used (227), vault PDA signs through the campaign seeds (96-101) |
