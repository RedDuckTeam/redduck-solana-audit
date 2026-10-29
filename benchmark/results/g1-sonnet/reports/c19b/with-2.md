# Solana audit report: affiliate_rewards

## Scope
Single Anchor 0.31.1 program, `program/src/lib.rs` (instructions: create_campaign, fund, publish_epoch, register, claim). No README or deployment notes. Single-file review, no toolchain run.

## Summary
0 Critical, 0 High, 3 Medium, 3 Low, 1 Info. Most important: the campaign authority alone decides payouts with no bound, and old roots can never be revoked.

## Findings

### F-1: Superseded roots stay claimable forever
Severity: Medium
Location: program/src/lib.rs:220
Mechanism: `claim` accepts any epoch account of the campaign (only `has_one = campaign`), and epochs cannot be revoked or closed, so if the authority publishes a corrected root that lowers an affiliate's total (for example after fraud), the affiliate claims against the older, higher root and keeps the difference.

Pattern: one-way-state
Scenario:
1. Epoch 3 lists affiliate A at 1000. The authority finds fraud and publishes epoch 4 with A at 100.
2. A calls `claim` with epoch 3 and its proof for 1000.
3. The proof passes and A is paid 1000 (minus what was claimed before).
Test: not attempted
Verification: Medium, not sent to verifier
Fix: Require `epoch.index == campaign.epochs - 1` (latest root only), or add a per-epoch revoked flag the authority can set.

### F-2: Claim payouts are not bounded by funding or any cap
Severity: Medium
Location: program/src/lib.rs:89
Mechanism: `owed` is whatever the Merkle leaf says; nothing ties the sum of leaves to vault funding, so an over-committed root lets early claimers drain the vault and later honest affiliates are left with a failing transfer (revert) until someone tops up.

Pattern: defensive-computation
Scenario:
1. Root promises more in total than the vault holds.
2. First claimers are paid; the vault empties.
3. Remaining claims revert on the token transfer, with no pro-rata or queue.
Test: not attempted
Verification: Medium, not sent to verifier
Fix: Store a per-epoch total/cap and check it against vault funding at publish time; or document that the authority must pre-fund.

### F-3: Epoch accounts and proof length are unbounded
Severity: Low
Location: program/src/lib.rs:133
Mechanism: `verify_proof` loops over a caller-supplied proof of any length; a long proof only wastes the caller's own compute (self-DoS), and epochs accumulate without any close path, locking authority rent forever.

Pattern: complexity-asymmetry
Test: not attempted
Fix: Cap `proof.len()` (e.g. 32) and add a `close_epoch` for old epochs.

### F-4: Mint with freeze authority can freeze the vault
Severity: Low
Location: program/src/lib.rs:164
Mechanism: `create_campaign` accepts any classic SPL mint without checking `freeze_authority`; the mint's freeze authority can freeze the vault, blocking all claims.

Pattern: dependency-independency
Test: not attempted
Fix: Require `mint.freeze_authority.is_none()` or an allow-listed mint.

### F-5: `total_claimed` is write-only
Severity: Info
Location: program/src/lib.rs:116
Mechanism: `total_claimed` is tracked but never read or checked against anything, so it gives no accounting protection.

Pattern: checklist-arithmetic
Test: not attempted
Fix: Use it in a solvency or cap check, or remove it.

## Design findings

### F-6: Authority fully controls payouts, with no timelock, dispute path or key rotation
Severity: Medium
Location: program/src/lib.rs:45
Mechanism: `publish_epoch` lets the single `campaign.authority` key publish any root instantly; the root can pay any amount to any wallet, so a compromised or malicious authority can drain every token in the vault (including third-party `fund` deposits), and funders cannot withdraw. `authority` is also fixed at creation with no transfer, so a lost key permanently stops new epochs (liveness).

Pattern: role-inversion
Scenario:
1. Third parties call `fund` believing payouts follow the earnings rules.
2. Authority publishes a root with a leaf for its own wallet equal to the vault balance.
3. It claims immediately and drains the vault.
Test: not attempted
Verification: downgraded by verifier from High: the mechanism is real but it is the intended trusted-publisher design (an ordinary user cannot exploit it); verifier suggested Low, kept at Medium because open `fund` exposes third-party depositors and a lost key strands vault funds.
Fix: Add a delay between publication and claimability, a funder or guardian veto, a per-epoch cap tied to funding, and a two-step authority transfer. Consider a multisig authority.

### F-7: Earnings data is computed entirely off-chain by one party
Severity: Low
Location: program/src/lib.rs:16
Mechanism: Correctness of every leaf depends on an off-chain service run by the authority and cannot be verified on-chain; if that service stops, affiliates stop earning, and there is no permissionless fallback.

Pattern: untrusted-executor
Test: not attempted
Fix: Document the trust model; publish the leaf data so affiliates can verify.

## Coverage

| Item | Verdict | Evidence |
|---|---|---|
| root-of-trust | clear | Claim chain ends at campaign account owner-checked, affiliate PDA seeds at lib.rs:225, vault `address = campaign.vault` at lib.rs:229; epoch tied to campaign at lib.rs:219 |
| identity-rejection | n/a | no allow/deny lists |
| untrusted-executor | found | F-7 |
| role-inversion | found | F-6 |
| lazy-loading | n/a | no iteration over holders |
| pay-as-you-act | n/a | no aggregates to settle |
| divide-and-conquer | clear | per-affiliate PDA at lib.rs:206 |
| complexity-asymmetry | found | F-3 |
| commit-execution-split | n/a | no randomness or previewable outcome |
| action-as-data | n/a | no stored actions |
| virtual-execution | n/a | no deferred obligations |
| one-way-state | found | F-1 (claimed is monotone via lib.rs:89-90, clear) |
| boundary-of-consent | n/a | payout is fixed by the leaf, no rate |
| internal-oracle | n/a | no derived prices |
| defensive-computation | found | F-2 |
| fail-fast | n/a | no oracle |
| execution-layer-shift | n/a | none |
| dependency-independency | found | F-4 |
| checklist-arithmetic | found | F-5; checked_add at lib.rs:53,116 |
| checklist-duplicate-accounts | clear | vault and owner_token are not exploitable if equal, owner signs (lib.rs:233) |
| checklist-unsafe-and-panics | clear | no unsafe or unwrap |
| checklist-reload-after-cpi | clear | no account data read after CPI |
| checklist-token-2022 | clear | `Program<Token>` at lib.rs:234 rejects Token-2022 |
| checklist-closing | n/a | no close instructions (see F-3) |
| checklist-seed-collisions | clear | distinct prefixes and fixed-size seed parts, lib.rs:151,158,190,206; leaf and node hashes domain-separated lib.rs:8-9 |
| checklist-init-frontrun | clear | campaign seeds include authority, lib.rs:151 |
| checklist-realloc | n/a | no realloc |
| checklist-authority-transfer | found | F-6 (no transfer path) |
| checklist-signers-bumps-cpi | clear | stored bumps used, lib.rs:94,226; signer seeds at lib.rs:95-100 |
