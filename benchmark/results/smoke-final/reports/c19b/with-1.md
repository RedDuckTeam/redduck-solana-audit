# Solana audit report: affiliate_rewards

## Scope
Single Anchor 0.31.1 program, `program/src/lib.rs` (instructions: create_campaign, fund, publish_epoch, register, claim). SPL Token only. No toolchain run; no tests executed.

## Summary
1 Critical, 0 High, 2 Medium, 2 Low. The key issue is F-1: `claim` lets an affiliate lower their `claimed` counter with an old epoch and be paid again, which drains the vault.

## Findings

### F-1: `claimed` can be moved backwards by claiming against an older epoch

Severity: Critical
Location: program/src/lib.rs:89
Mechanism: `claim` accepts any epoch of the campaign and sets `affiliate.claimed = earned_total` unconditionally. Claiming with an older, smaller cumulative total lowers `claimed`, so the next claim with the newer total pays the difference again; looping this drains the vault.

Pattern: one-way-state
Scenario:
1. Affiliate has leaf total 50 in epoch 0 and 100 in epoch 1.
2. claim(epoch1, 100) pays 100, claimed=100.
3. claim(epoch0, 50): owed=0, claimed=50.
4. claim(epoch1, 100): owed=50 is paid. Repeat steps 3-4 until the vault is empty.
Test: not attempted
Verification: confirmed by verifier
Fix: Use `affiliate.claimed = affiliate.claimed.max(earned_total)`, or require `earned_total > claimed`. Optionally only accept the latest epoch.

### F-2: Epoch roots are unconstrained and cannot be corrected or revoked

Severity: Medium
Location: program/src/lib.rs:45
Mechanism: The campaign authority publishes arbitrary roots with no cap tied to vault balance, and no instruction can revoke or replace a wrong root. A mistaken or malicious root pays out whatever it states, up to the vault balance, and stays claimable forever, since all old epochs stay valid.

Pattern: role-inversion
Scenario:
1. Authority publishes a root with inflated totals for its own affiliate account.
2. The affiliate claims and takes funds that other funders deposited.
Test: not attempted
Fix: Add an epoch revocation or a challenge window, and use the latest epoch only. Document the trust in the authority.

## Design findings

### F-3: No authority rotation, and no way to recover or close funds

Severity: Medium
Location: program/src/lib.rs:18
Mechanism: `authority` is set once and is part of the campaign PDA seeds. If that key is lost, no more epochs can be published and affiliates cannot earn new rewards. Funds that nobody can claim stay in the vault forever.

Pattern: dependency-independency
Scenario:
1. The authority key is lost.
2. Publishing stops and the remaining vault balance is stuck.
Test: not attempted
Fix: Add a two-step authority transfer, and an expiry or sweep path for unclaimed funds.

### F-4: Vault solvency is never checked against published obligations

Severity: Low
Location: program/src/lib.rs:45
Mechanism: `publish_epoch` does not compare cumulative totals with the vault balance, and `total_claimed` is never used. Affiliates can end up with valid proofs that cannot be paid, and the first claimers win.

Pattern: fail-fast
Scenario:
1. Roots promise more than the vault holds.
2. Early claims drain the vault and later valid claims fail.
Test: not attempted
Fix: Track total committed amounts in the root and check them against funding.

### F-5: Mint freeze authority can freeze the vault

Severity: Low
Location: program/src/lib.rs:163
Mechanism: `create_campaign` accepts any mint without checking `freeze_authority`. The mint's freeze authority could freeze the vault and block all claims.

Pattern: dependency-independency
Scenario:
1. A campaign is created with a freezable mint.
2. The freeze authority freezes the vault and all claims fail.
Test: not attempted
Fix: Reject mints that have a freeze authority, or document the risk.

## Coverage

| Item | Verdict | Evidence |
|---|---|---|
| root-of-trust | clear | campaign and affiliate PDAs, vault address = campaign.vault (lib.rs:173, 224, 228); epoch tied to campaign via has_one (lib.rs:218) |
| identity-rejection | n/a | no allow/deny lists |
| untrusted-executor | n/a | claims are permissionless per owner; no operator-only settlement |
| role-inversion | found | F-2 |
| lazy-loading | clear | per-affiliate claim against a cumulative total, no loops |
| pay-as-you-act | n/a | no aggregate processing |
| divide-and-conquer | clear | one account per affiliate and per epoch |
| complexity-asymmetry | clear | proof loop is bounded by transaction size (lib.rs:134) |
| commit-execution-split | n/a | no randomness or previewable outcome |
| action-as-data | n/a | no stored actions |
| virtual-execution | n/a | no deferred obligations |
| one-way-state | found | F-1 |
| boundary-of-consent | n/a | amounts are fixed by the proof |
| internal-oracle | n/a | no internal price |
| defensive-computation | n/a | no pricing; saturating_sub at lib.rs:88 |
| fail-fast | found | F-4 |
| execution-layer-shift | n/a | no other layer |
| dependency-independency | found | F-3, F-5 |
| checklist-arithmetic | clear | checked_add at lib.rs:53, 115 |
| checklist-duplicate-accounts | clear | no two mutable accounts of the same type can be swapped harmfully |
| checklist-unsafe-and-panics | clear | no unsafe or unwrap |
| checklist-reload-after-cpi | clear | no account data is read after the CPI |
| checklist-token-2022 | n/a | uses the SPL Token program only (`Program<Token>`) |
| checklist-closing | n/a | no account closing |
| checklist-seed-collisions | clear | distinct prefixes; leaf and node prefixes at lib.rs:8-9 |
| checklist-init-frontrun | clear | PDAs keyed by authority/owner |
| checklist-realloc | n/a | no realloc |
| checklist-authority-transfer | found | F-3 |
| checklist-signers-bumps-cpi | clear | stored bump used for signing (lib.rs:93-98); owner is a Signer (lib.rs:232) |
