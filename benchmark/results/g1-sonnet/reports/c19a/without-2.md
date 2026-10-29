# Solana Audit Report: affiliate_rewards

Scope: program/src/lib.rs (Anchor 0.31.1). The Merkle verification itself (domain-separated leaf and node hashes, sorted pairs, the campaign and owner bound into the leaf) and the PDA and account constraints looked sound.

Severity: High
Location: program/src/lib.rs:45
Mechanism: The single `authority` publishes roots with no validation, timelock, cap or on-chain check against vault funds, and can publish a root granting itself (or any colluding affiliate) an inflated `earned_total`. It can then drain the entire vault immediately, so the program is fully custodial and trusts one key. Compromise of that key loses all funds.

Severity: High
Location: program/src/lib.rs:220
Mechanism: `claim` accepts any previously published epoch (`epoch` is only checked with `has_one = campaign`, and the doc comment at line 72 says this is intended), and published epochs can never be revoked, superseded or closed. If the authority publishes a corrected root that lowers or removes an affiliate's total (for example after fraud or a miscalculation), the affiliate can still claim against the older, more generous root.

Severity: Medium
Location: program/src/lib.rs:89
Mechanism: Nothing ties the sum of the promised `earned_total` values to the vault balance. A root can promise more than the vault holds, so claims are first-come-first-served and the transfer at line 102 fails once the vault runs dry. Late honest affiliates are then stuck, and fast or MEV-aware claimers can front-run others. Since `claimed` is only updated when a claim succeeds, the failure is a liveness problem, not an accounting loss.

Severity: Medium
Location: program/src/lib.rs:241
Mechanism: `authority` is immutable, with no rotation, multisig or recovery path, and there is no way to withdraw leftover vault funds or close the campaign. Losing the key permanently stops new epochs, so affiliates stop accruing. A malicious or compromised key is also not recoverable.

Severity: Low
Location: program/src/lib.rs:219
Mechanism: Every claim mutably borrows the shared `campaign` account (to update `total_claimed`, line 117) and the shared vault. Solana serializes write-locked accounts, so all claims and `publish_epoch` on a campaign contend for the same locks. This caps claim throughput per campaign and creates a scalability bottleneck. `total_claimed` is informational only and is not used for any check.

Severity: Low
Location: program/src/lib.rs:191
Mechanism: Each epoch creates a permanent `EpochRoot` PDA that is never closed, and the authority pays rent for it. The set of valid roots grows without bound (all of them stay claimable, see line 220). Rent is never reclaimed, and the growing set of valid roots increases the audit and attack surface.

Severity: Info
Location: program/src/lib.rs:152
Mechanism: The campaign PDA is seeded only by `authority` and `mint`, so each authority can run only one campaign per mint. The program is also pinned to the legacy SPL Token program (`Program<'info, Token>`), so Token-2022 mints are unsupported. Mints with a freeze authority can freeze the vault or an affiliate token account and block payouts.
