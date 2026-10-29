# Solana audit report: affiliate_rewards

## Scope
program/src/lib.rs (Anchor 0.31.1, SPL Token only). Instructions: create_campaign, fund, publish_epoch, register, claim. Single-file program, no tests or off-chain code reviewed.

## Summary
1 Critical, 1 High, 1 Medium, 2 Low. The most important finding is F-1: `claim` lets an affiliate lower their `claimed` counter by replaying an older epoch, then be paid again, which drains the vault. F-2 follows: the authority's roots can never be revoked.

## Findings

### F-1: claim overwrites `claimed` with a possibly lower value and accepts any epoch

Severity: Critical
Location: src/lib.rs:90
Mechanism: `claim` accepts any published epoch (line 219) and sets `affiliate.claimed = earned_total` unconditionally, so claiming with an older, smaller leaf lowers `claimed`. A following claim with the newest leaf pays the difference again, and the loop drains the shared vault, including other affiliates' rewards.

Pattern: one-way-state
Scenario:
1. Affiliate has total 100 in epoch 0 and 200 in epoch 1.
2. Claims with epoch 1: paid 200, claimed=200.
3. Claims with epoch 0: owed 0, claimed=100.
4. Claims with epoch 1: paid 100, claimed=200. Repeat steps 3-4 until the vault is empty.
Test: not attempted
Verification: confirmed by verifier
Fix: Require `earned_total > affiliate.claimed` (or set `claimed = max(claimed, earned_total)`), and preferably accept only the latest epoch.

### F-2: Published roots are irrevocable and nothing ties them to vault funds

Severity: High
Location: src/lib.rs:45
Mechanism: `publish_epoch` lets the authority publish any root with no cap against vault balance, and `claim` accepts every old epoch forever, so a wrong or malicious root can never be withdrawn. A malicious or compromised authority can publish a root granting itself the whole vault.

Pattern: dependency-independency
Scenario:
1. Authority (or an attacker holding its key) publishes a root with a leaf for itself worth the vault balance.
2. It registers and claims, taking all funded tokens.
Test: not attempted
Verification: self-checked, not sent to the verifier (design finding, trust in the authority is by design)
Fix: Cap cumulative promised totals against funded amounts, add a challenge delay before a root becomes claimable, and allow revoking or superseding roots (claim against the latest epoch only).

### F-3: Insolvency is not handled and claims are first come, first served

Severity: Medium
Location: src/lib.rs:101
Mechanism: Roots are not checked against vault balance, so total entitlements can exceed funds; the transfer then fails for late claimants, who cannot claim until someone calls `fund`.

Pattern: fail-fast
Scenario:
1. Roots promise more than the vault holds.
2. Early claimers drain the vault; later claims revert on insufficient balance.
Test: not attempted
Fix: Track total committed per root and reject publication above funded balance, or pro-rate payouts.

## Design findings

### F-4: Single authority key, no rotation, no funds recovery, no root expiry

Severity: Medium
Location: src/lib.rs:184
Mechanism: `Campaign.authority` is fixed at creation (it is also a PDA seed) with no transfer path, so if the key is lost no new epochs are published and rewards stop accruing, and the campaign cannot be migrated. Funders also have no way to withdraw leftover tokens.

Pattern: untrusted-executor
Test: not attempted
Fix: Add a two-step authority transfer (or a multisig), and a time-locked sweep of unclaimed funds.

### F-5: Epoch accounts accumulate without a way to close them

Severity: Low
Location: src/lib.rs:186
Mechanism: Each `publish_epoch` creates a permanent EpochRoot paid by the authority and never closed, so rent grows linearly. Because every old epoch stays claimable (see F-1), old state also cannot be retired.

Pattern: divide-and-conquer
Test: not attempted
Fix: Keep only the latest root, or close epochs after an expiry.

### F-6: Mint properties not validated; only classic SPL Token supported

Severity: Low
Location: src/lib.rs:164
Mechanism: `create_campaign` accepts any mint, including one with a freeze authority, which can freeze the vault and block all claims. Token-2022 mints are rejected by `Program<Token>`.

Pattern: checklist-token-2022
Test: not attempted
Fix: Reject mints with a freeze authority or document the trust, and decide on Token-2022 support explicitly.

## Coverage

| Item | Verdict | Evidence |
|---|---|---|
| root-of-trust | clear | campaign PDA (line 151), vault address = campaign.vault (229), epoch/affiliate has_one campaign (219, 223); roots trusted from authority, see F-2 |
| identity-rejection | n/a | no allow/deny lists or per-wallet rules |
| untrusted-executor | found | F-4 |
| role-inversion | n/a | the operator publishes roots, but affiliates bring their own proofs |
| lazy-loading | clear | per-affiliate claimed marker (88-90), no loops over holders |
| pay-as-you-act | clear | no global batch step, claims are per affiliate |
| divide-and-conquer | found | F-5 |
| complexity-asymmetry | clear | Merkle proof verified on-chain at line 133; proof length bounded by the transaction size |
| commit-execution-split | n/a | no randomness or previewable outcomes |
| action-as-data | n/a | no stored executable actions |
| virtual-execution | n/a | claims pay out immediately |
| one-way-state | found | F-1 |
| boundary-of-consent | n/a | amounts come from the signed-for leaf, no computed rates |
| internal-oracle | n/a | no derived prices |
| defensive-computation | n/a | no valuation or rounding |
| fail-fast | found | F-3 |
| execution-layer-shift | n/a | no other layer |
| dependency-independency | found | F-2 |
| checklist-arithmetic | clear | checked_add at lines 53 and 116; saturating_sub at 89 is the cause of F-1 |
| checklist-duplicate-accounts | clear | no two mutable accounts of the same type that could alias harmfully |
| checklist-unsafe-and-panics | clear | no unsafe, unwrap or indexing |
| checklist-reload-after-cpi | clear | no reads of the vault after the CPI |
| checklist-token-2022 | found | F-6 |
| checklist-closing | n/a | no account closing (see F-5) |
| checklist-seed-collisions | clear | distinct prefixes b"campaign", b"vault", b"epoch", b"affiliate"; leaf and node hashes use different prefixes (lines 8-9) |
| checklist-init-frontrun | clear | campaign PDA is seeded with the authority (151), affiliate with the owner (206) |
| checklist-realloc | n/a | no realloc |
| checklist-authority-transfer | found | F-4 |
| checklist-signers-bumps-cpi | clear | owner and authority are Signers (233, 195); stored bump used (226); token program is typed `Program<Token>` |
