# Solana Audit Report: affiliate_rewards (./program)

Scope: `program/src/lib.rs` (Anchor 0.31.1). Each leaf in the Merkle tree is cumulative: `(campaign, owner, earned_total)`. A claim pays `earned_total - affiliate.claimed` against any published epoch root.

## Findings

### 1. Claiming against an old epoch resets `claimed`, so rewards can be paid out again
Severity: Critical
Location: program/src/lib.rs:90
Mechanism: `claim` accepts any published epoch and overwrites `affiliate.claimed = earned_total` instead of taking the max. An affiliate claims with the latest root (for example total 100). They then claim with an older root that has a lower total (for example 10), which pays 0 but sets `claimed` back to 10. A third claim with the latest root pays 90 again. Repeating this drains the whole vault.

### 2. Epoch roots can never be revoked, so a wrong or malicious root stays claimable forever
Severity: High
Location: program/src/lib.rs:219
Mechanism: The `Claim` accounts struct accepts every `EpochRoot` of the campaign, and the leaf does not include the epoch index. Publishing a corrected root does not supersede an earlier root that was wrong, inflated or leaked. Anyone listed in a bad root with a higher `earned_total` can claim against it at any time, and there is no instruction to invalidate it.

### 3. A single authority key can publish any root and drain the vault, with no timelock
Severity: High
Location: program/src/lib.rs:184
Mechanism: The `PublishEpoch` accounts struct only requires the one `authority` signer. The new root takes effect immediately: there is no timelock, dispute window, multisig, or check against the funded amount (`published_at` is stored but never used). A compromised or malicious authority can publish a root with a leaf crediting itself the full vault balance and claim it in the next transaction, taking all funds that third parties deposited through `fund`.

### 4. No authority rotation, withdrawal, close or pause path, so funds can get stuck
Severity: Medium
Location: program/src/lib.rs:16
Mechanism: `Campaign.authority` is fixed when the campaign is created, and there is no instruction to rotate it, pause claims, close the campaign, or recover unclaimed vault tokens. If the authority key is lost, no new epochs can be published. In that case the remaining funds, including third-party top-ups, are locked permanently. During an incident there is also no way to stop claims.

### 5. Nothing checks that the vault can cover the published totals
Severity: Medium
Location: program/src/lib.rs:101
Mechanism: Nothing ties the sum of `earned_total` values in a root to the vault balance or the amount funded. If the vault is underfunded, payouts become first-come-first-served and later affiliates' claims fail. The `total_claimed` counter is updated but never checked against anything.

### 6. Epoch and affiliate accounts can never be closed, so rent and account count only grow
Severity: Low
Location: program/src/lib.rs:186
Mechanism: Every `publish_epoch` creates a new rent-paying PDA that can never be closed, and all of them stay valid for claims (see finding 2). Over a long-running campaign, the authority's rent cost and the set of live roots both grow without limit. `Affiliate` accounts can't be closed either.

### 7. The mint's freeze authority and non-standard token behaviour are not checked
Severity: Low
Location: program/src/lib.rs:164
Mechanism: Any SPL mint is accepted. If the mint has a freeze authority, that party can freeze the vault and block every claim and funding transfer. The program also hard-codes the legacy Token program, so Token-2022 mints (with transfer fees or hooks) are unsupported.

### 8. Each authority can create only one campaign per mint
Severity: Info
Location: program/src/lib.rs:151
Mechanism: The campaign PDA seeds are `[b"campaign", authority, mint]`, so an authority cannot run a second campaign for the same mint. To do so it has to use a new key, which also complicates the centralization and key-rotation issues in findings 3 and 4.

### 9. Claims are silently accepted when `earned_total` is below what was already claimed
Severity: Info
Location: program/src/lib.rs:89
Mechanism: `saturating_sub` turns a stale or lower `earned_total` into a successful claim that pays 0, instead of rejecting it. This hides the regression that enables finding 1. The claim should fail when `earned_total <= affiliate.claimed`.

## Recommended fixes
- Only advance `claimed`: `require!(earned_total > affiliate.claimed)` and set `claimed = earned_total` only when it increases. Better still, only accept the latest epoch (`epoch.index == campaign.epochs - 1`), or include the epoch index in the leaf.
- Add a timelock or dispute window on new roots (using `published_at`), a multisig authority, and an instruction to rotate the authority and revoke roots.
- Add pause, close and withdraw-unclaimed instructions, with clearly defined permissions.
- Make it possible to close superseded epoch accounts.
