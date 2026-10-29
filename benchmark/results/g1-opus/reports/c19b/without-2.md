# Solana Audit Report: `affiliate_rewards` (program/src/lib.rs)

Scope: `program/src/lib.rs` (Anchor 0.31.1). Review was manual; nothing was compiled or tested.

---

## 1. Replaying stale epochs resets `claimed` and drains the vault

Severity: Critical
Location: program/src/lib.rs:90
Mechanism: `claim` takes any published epoch and overwrites `affiliate.claimed = earned_total` without checking that the value never goes down. An affiliate can claim against the latest root (total 100, paid 100), then claim against an older root (total 50, paid 0, `claimed` reset to 50), then claim the latest root again (paid 50), and repeat this until the vault is empty.

## 2. Old roots can never be retired, so a wrong or compromised root stays claimable forever

Severity: High
Location: program/src/lib.rs:219
Mechanism: The `Claim` context accepts any `EpochRoot` for the campaign, and there is no instruction to invalidate, supersede or close an epoch. A root published by mistake (or with inflated totals) stays a valid claim source permanently, and later corrective roots cannot lower anyone's entitlement. Combined with finding 1, they actually let affiliates claim the same amount again.

## 3. Centralized, unchecked root publishing lets the authority take all vault funds

Severity: High
Location: program/src/lib.rs:45
Mechanism: One `authority` key publishes arbitrary Merkle roots instantly. There is no timelock or challenge window (`published_at` is stored but never used), no multisig, and no on-chain check of totals against funding. A compromised or malicious authority can publish a root that gives itself (or any address) the whole vault, including tokens deposited by third-party funders through `fund`.

## 4. No withdraw, close or recovery path; funds are stuck if the authority goes away

Severity: Medium
Location: program/src/lib.rs:172
Mechanism: Anyone can deposit through `fund`, but no instruction ever returns unclaimed tokens or closes the campaign or vault. If the authority key is lost or stops publishing, affiliates' newer earnings can never be claimed and leftover vault tokens are locked forever (a liveness failure).

## 5. Authority cannot be rotated because it is baked into the campaign PDA seeds

Severity: Low
Location: program/src/lib.rs:151
Mechanism: The campaign PDA is derived from `authority` and there is no `set_authority` instruction, so a key that is suspected to be compromised cannot be replaced. The only option is a new campaign, which strands the old vault (see finding 4).

## 6. Vault solvency is not tracked; underfunding makes claims fail all-or-nothing

Severity: Low
Location: program/src/lib.rs:101
Mechanism: Nothing ties the published totals to the vault balance. When the vault is short, the transfer reverts the whole claim (there is no partial payout), and payment goes first-come-first-served, so early claimers can empty the vault and leave other affiliates with nothing.

## 7. Per-epoch rent accounts grow without limit and are never reclaimed

Severity: Low
Location: program/src/lib.rs:186
Mechanism: Each epoch creates a new rent-paying `EpochRoot` PDA that can never be closed, so the authority's rent cost and the number of on-chain accounts grow linearly forever. Because every root stays valid, this also widens the attack surface described in findings 1 and 2. The design only needs the latest cumulative root.

## 8. Mint freeze authority can freeze the vault

Severity: Low
Location: program/src/lib.rs:164
Mechanism: `create_campaign` accepts any SPL mint without checking its `freeze_authority`. The mint's freeze authority can freeze the vault token account and block every claim (and every `fund`) indefinitely.

## 9. `total_claimed` is bookkeeping only and is never checked

Severity: Info
Location: program/src/lib.rs:116
Mechanism: `campaign.total_claimed` is updated but never compared against anything, such as total funding or the epoch totals. Because of finding 1 it can also grow past what was legitimately earned, so off-chain monitoring should not rely on it.

## 10. The destination token account is not restricted to the owner

Severity: Info
Location: program/src/lib.rs:232
Mechanism: `owner_token` only checks the mint, not that the account belongs to `owner`. This is not exploitable because `owner` must sign, but it lets the signer send rewards to any account, which can confuse off-chain indexing and accounting.
