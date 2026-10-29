# Solana Audit Report: affiliate_rewards

Scope: program/src/lib.rs (single file, Anchor 0.31.1).

Severity: High
Location: program/src/lib.rs:45
Mechanism: The campaign authority alone publishes Merkle roots, with no cap, timelock or check against vault balance or funded amount. A compromised or malicious authority can publish a root that assigns itself (or a sybil affiliate) an `earned_total` equal to the whole vault and drain all funds that third parties deposited via `fund`.

Severity: High
Location: program/src/lib.rs:74
Mechanism: `claim` accepts any epoch ever published, and there is no way to revoke or close an epoch. A root that is wrong or malicious (bad off-chain computation, key compromise) stays claimable forever, so it cannot be corrected, only out-published by cumulative totals that cannot lower already-claimed amounts.

Severity: Medium
Location: program/src/lib.rs:89
Mechanism: Nothing ties the sum of leaf totals to funded tokens, and `claim` never checks vault solvency. If roots over-promise, claims are first-come-first-served and later honest affiliates hit a failing token transfer, so the early claimers win.

Severity: Medium
Location: program/src/lib.rs:12
Mechanism: There is no instruction to withdraw or recover vault funds, close a campaign, or rotate or transfer the authority. If an affiliate's key is lost, their share is locked forever. If the authority key is lost, no new roots can be published, so rewards stop (a liveness failure). Any tokens sent to the vault by mistake are unrecoverable.

Severity: Low
Location: program/src/lib.rs:165
Mechanism: The mint's freeze authority is not checked, and only the legacy SPL Token program is supported. A mint with a freeze authority lets that party freeze the vault or an owner's token account and block all claims. Mints with a close authority or other extensions are not handled.

Severity: Low
Location: program/src/lib.rs:187
Mechanism: Every epoch creates a new permanent `EpochRoot` account whose rent is paid by the authority and can never be reclaimed (no close instruction). Because the leaves hold cumulative totals, only the latest epoch is needed, so the accounts add state bloat and cost for no benefit. The full affiliate tree must also be rebuilt off-chain and re-published each epoch.

Severity: Info
Location: program/src/lib.rs:134
Mechanism: Merkle verification is sound: leaf and node hashes use separate prefixes, which prevents second-preimage attacks, and the leaf commits to campaign, owner and amount. The proof length is unbounded, but this is limited by compute budget and is not exploitable. Claim state is updated before the CPI and `owner` must sign, so no reentrancy or third-party claim issue was found.
