# Solana audit report: affiliate_rewards

## Scope
Folder `program/` (Anchor 0.31.1, single program `affiliate_rewards`, `program/src/lib.rs`). Instructions reviewed: create_campaign, fund, publish_epoch, register, claim. No toolchain was run; no exploit tests.

## Summary
1 Critical, 1 High (design), 1 Medium (design), 2 Low. Most important: the claim replay in F-1 drains the vault, and the authority-controlled Merkle roots in F-2 give one key full control over the vault.

## Findings

### F-1: claim lets `claimed` move backwards, so old epochs can be replayed
Severity: Critical
Location: program/src/lib.rs:90
Mechanism: `claim` accepts any published epoch and sets `affiliate.claimed = earned_total` unconditionally, so claiming with an older, smaller cumulative total lowers `claimed`, and the affiliate can then re-claim the difference against a newer epoch, again and again, until the vault is empty.

Pattern: one-way-state
Scenario:
1. Affiliate has leaf total=10 in epoch 0 and total=100 in epoch 1.
2. claim(100, epoch1) pays 100, claimed=100.
3. claim(10, epoch0) pays 0, claimed=10.
4. claim(100, epoch1) pays 90. Repeat 3-4 until the vault is drained.
Test: not attempted
Verification: confirmed by verifier
Fix: Only update `claimed` when `earned_total > claimed` (or require `earned_total > claimed`), and/or require the latest epoch.

### F-2: Claim has no bound against the vault balance and the epoch root is not checked to be the latest
Severity: Low
Location: program/src/lib.rs:73
Mechanism: Any published epoch is accepted and nothing ties roots to funded amounts, so claims are first-come and a late affiliate can find the vault empty, with no on-chain solvency check at publish time.

Pattern: fail-fast
Scenario:
1. Authority publishes a root whose totals exceed the vault balance.
2. Early claimers drain it; later claims revert at the token transfer.
Test: not attempted
Fix: Track funded vs. committed amounts, or document and enforce root solvency off-chain.

### F-3: `total_claimed` is written but never used
Severity: Info
Location: program/src/lib.rs:116
Mechanism: The counter provides no on-chain guard (for example against the funded total), so it gives false assurance; with F-1 it also overstates real payouts.

Pattern: one-way-state
Test: not attempted
Fix: Use it in a solvency check or remove it.

## Design findings

### D-1: A single authority key controls every payout through the Merkle roots
Severity: High
Location: program/src/lib.rs:45
Mechanism: `publish_epoch` lets `campaign.authority` publish any root with no timelock, cap or review, so a compromised or malicious authority can publish a root granting itself the entire vault and claim it; roots cannot be revoked once published.

Pattern: role-inversion
Scenario:
1. Authority publishes a root with a leaf (authority, vault balance).
2. Registers and claims; the vault is emptied.
Test: not attempted
Verification: design fact (authority trust), not sent to verifier
Fix: Multisig authority, a delay between publish and claimability, per-epoch caps against funded amounts, and a way to revoke an epoch.

### D-2: No authority rotation and no recovery path; liveness depends on one key
Severity: Medium
Location: program/src/lib.rs:18
Mechanism: `authority` is fixed at creation and is part of the campaign PDA seeds, so a lost key means no new epochs can ever be published and later earnings can never be paid; funded tokens cannot be withdrawn either.

Pattern: dependency-independency
Test: not attempted
Fix: Add a two-step authority transfer and a safe sweep or expiry for unclaimed funds.

## Coverage

| Item | Verdict | Evidence |
|---|---|---|
| root-of-trust | clear | Q3: chain ends at authority-signed epoch PDA (lib.rs:186-193) and campaign has_one (lib.rs:184, 219) |
| identity-rejection | n/a | no allow lists or caller-based guards |
| untrusted-executor | found | D-2 |
| role-inversion | found | D-1 |
| lazy-loading | clear | cumulative totals with per-affiliate marker, lib.rs:89 |
| pay-as-you-act | n/a | no aggregate processing |
| divide-and-conquer | clear | per-affiliate PDA lib.rs:221-227 |
| complexity-asymmetry | clear | proof loop is bounded by tx size (lib.rs:135) |
| commit-execution-split | n/a | no randomness or previewable outcome |
| action-as-data | n/a | no stored actions |
| virtual-execution | n/a | no deferred obligations |
| one-way-state | found | F-1, F-3 |
| boundary-of-consent | n/a | amounts are fixed by the proven leaf |
| internal-oracle | n/a | no derived prices |
| defensive-computation | n/a | no pricing or rounding |
| fail-fast | found | F-2 |
| execution-layer-shift | n/a | no rollups or off-chain layers |
| dependency-independency | found | D-1, D-2 |
| checklist-arithmetic | clear | checked_add lib.rs:53,118; saturating_sub lib.rs:89 |
| checklist-duplicate-accounts | clear | vault as owner_token only harms the claimer |
| checklist-unsafe-and-panics | clear | no unsafe or unwrap |
| checklist-reload-after-cpi | clear | no reads of token balances after CPI |
| checklist-token-2022 | clear | classic `Token` program only, lib.rs:167,234 |
| checklist-closing | n/a | no account closing |
| checklist-seed-collisions | clear | distinct prefixes campaign/vault/epoch/affiliate; leaf/node prefixes lib.rs:8-9 |
| checklist-init-frontrun | clear | PDAs seeded by authority/owner, lib.rs:151,206 |
| checklist-realloc | n/a | no realloc |
| checklist-authority-transfer | found | D-2 |
| checklist-signers-bumps-cpi | clear | signer checks lib.rs:166,178,195,233; stored bump lib.rs:226 |
