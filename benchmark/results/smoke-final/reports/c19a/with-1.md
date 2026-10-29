# Solana audit report: affiliate_rewards

## Scope
`program/src/lib.rs` (Anchor 0.31.1, SPL Token only). Instructions: create_campaign, fund, publish_epoch, register, claim. Pattern reference files were not opened; verdicts use the one-line rules from the skill and a full read of the single source file. No toolchain was run, so no exploit tests. No High or Critical findings, so the verifier agent was not called.

## Summary
0 Critical, 0 High, 3 Medium, 3 Low, 0 Info. The most important issues: published roots can never be revoked or corrected, and the vault is not tied to the promised totals, so a bad root or over-promised leaves cause first-come insolvency.

## Findings

### F-1: Published epoch roots are permanent and every old root stays claimable

Severity: Medium
Location: program/src/lib.rs:219
Mechanism: `claim` accepts any `EpochRoot` of the campaign, and no instruction can revoke, replace or expire one. A wrong root (error, compromised authority key, or a later correction that lowers an affiliate's total) stays valid forever, so anyone holding a proof against it can drain the vault.

Pattern: one-way-state
Scenario:
1. Authority publishes epoch 3 with an inflated leaf for affiliate A (or a leaf later found fraudulent).
2. Authority publishes epoch 4 with the corrected, lower total.
3. A claims against epoch 3 and is paid the higher amount, because epoch 3 is still accepted.
Test: not attempted
Verification: not required (Medium)
Fix: Accept only the latest epoch (`epoch.index == campaign.epochs - 1`), or add a revoke/expiry flag and a claim window.

### F-2: Vault solvency is not enforced, so claims are first come first served

Severity: Medium
Location: program/src/lib.rs:45
Mechanism: `publish_epoch` records a root without checking that the leaves' total fits within the vault balance, and there is no per-epoch cap. If promised totals exceed the funds, early claimers are paid and later affiliates' claims fail at the token transfer with no recourse.

Pattern: virtual-execution
Scenario:
1. Vault holds 100 tokens; the root promises 70 to A and 70 to B.
2. A claims 70. B's claim reverts with insufficient funds until someone funds again.
Test: not attempted
Verification: not required (Medium)
Fix: Commit the epoch's total liability in the root account and check `total_promised <= vault.amount + total_claimed` at publish time.

### F-3: Campaign funds have no recovery path and the authority cannot be rotated

Severity: Low
Location: program/src/lib.rs:16
Mechanism: There is no instruction to withdraw unclaimed or surplus tokens, close the campaign, or change `authority`. Tokens from `fund` that are never claimed are locked forever, and a lost authority key stops epoch publishing permanently.

Pattern: dependency-independency
Scenario:
1. Campaign ends with 1,000 unclaimed tokens in the vault, or the authority key is lost.
2. No instruction can move the tokens or restore publishing rights.
Test: not attempted
Verification: not required (Low)
Fix: Add a two-step authority transfer and an authority-only withdrawal of surplus after a claim deadline.

## Design findings

### F-4: Single authority controls all payouts with no delay

Severity: Medium
Location: program/src/lib.rs:45
Mechanism: One signer publishes roots that decide who gets paid from a vault anyone can fund. The authority can publish a root that pays itself the whole vault in one transaction, with no timelock, multisig requirement or dispute window. Funders must trust the authority fully.

Pattern: root-of-trust
Scenario:
1. Authority registers as an affiliate.
2. Publishes a root whose only leaf gives its own key the vault balance.
3. Claims it immediately.
Test: not attempted
Verification: not required (Medium, trusted role)
Fix: Use a multisig or governance authority, and add a delay between publishing and claimability.

### F-5: Unreclaimable per-epoch and per-affiliate accounts grow forever

Severity: Low
Location: program/src/lib.rs:189
Mechanism: Each epoch creates a new `EpochRoot` account paid by the authority, and none can be closed (also `Affiliate` at line 205). Rent accumulates without bound and cannot be reclaimed.

Pattern: divide-and-conquer
Scenario:
1. The authority publishes an epoch per period for years.
2. Each epoch locks rent permanently.
Test: not attempted
Verification: not required (Low)
Fix: Add close instructions for epochs that have expired or been superseded.

### F-6: Mint freeze authority or token-account state can block payouts

Severity: Low
Location: program/src/lib.rs:164
Mechanism: `create_campaign` accepts any mint without checking `freeze_authority`. If the mint's freeze authority freezes the vault, every claim fails and funds are stuck.

Pattern: dependency-independency
Scenario:
1. Campaign is created with a freezable mint.
2. The freeze authority freezes the vault; claims revert.
Test: not attempted
Verification: not required (Low)
Fix: Require `mint.freeze_authority` to be none, or document and accept the risk.

## Coverage

| Item | Verdict | Evidence |
|---|---|---|
| root-of-trust | clear | campaign PDA seeds line 151; vault checked with `address = campaign.vault` (229); epoch tied by `has_one = campaign` (219); root only writable by authority (184) |
| identity-rejection | n/a | no allow/deny lists or per-wallet rules |
| untrusted-executor | found | F-4 (publishing is authority-only; claims are owner-initiated) |
| role-inversion | n/a | the authority pushes roots, but the claimant brings the proof |
| lazy-loading | clear | claim settles one affiliate against a cumulative total (88-90); no iteration |
| pay-as-you-act | n/a | no aggregate accrual |
| divide-and-conquer | found | F-5 |
| complexity-asymmetry | clear | proof length is caller-bounded by tx size; hashing cost is paid by the claimer (133-142) |
| commit-execution-split | n/a | no randomness or previewable outcomes |
| action-as-data | n/a | no stored executable actions |
| virtual-execution | found | F-2 |
| one-way-state | found | F-1 (affiliate.claimed is monotonic at line 88, but old roots stay valid) |
| boundary-of-consent | n/a | no computed rate; the claim amount is fixed by the leaf |
| internal-oracle | n/a | no derived prices |
| defensive-computation | n/a | no valuation or rounding |
| fail-fast | n/a | no external data inputs |
| execution-layer-shift | n/a | no rollup or off-chain layer |
| dependency-independency | found | F-3, F-6 |
| checklist-arithmetic | clear | `checked_add` at 53 and 116; subtraction guarded by line 88 |
| checklist-duplicate-accounts | clear | no two mutable accounts of the same type can be swapped harmfully |
| checklist-unsafe-and-panics | clear | no unsafe, unwrap or indexing |
| checklist-reload-after-cpi | clear | no account data is read after the CPI |
| checklist-token-2022 | n/a | `Program<Token>` only, so Token-2022 is rejected |
| checklist-closing | n/a | no close instructions (see F-5) |
| checklist-seed-collisions | clear | distinct prefixes campaign/vault/epoch/affiliate with fixed-width keys |
| checklist-init-frontrun | clear | campaign PDA is keyed by authority, so a squatter gets their own campaign |
| checklist-realloc | n/a | no realloc |
| checklist-authority-transfer | found | F-3 |
| checklist-signers-bumps-cpi | clear | owner is a `Signer` (233); stored bumps used (226); campaign PDA signs the vault transfer (95-100) |
