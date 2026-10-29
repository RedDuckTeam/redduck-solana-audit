# Solana Audit Report: affiliate_rewards (program/src/lib.rs)

Severity: High
Location: program/src/lib.rs:45
Mechanism: A single authority key publishes arbitrary Merkle roots with no cap, timelock or review. It can add a leaf for itself (or a sybil) with an `earned_total` equal to the whole vault, then claim it. Affiliates and funders must trust this key completely, and the vault is not protected from it.

Severity: High
Location: program/src/lib.rs:74
Mechanism: `claim` accepts the root of any published epoch, and epochs can never be revoked or superseded. Leaves hold cumulative totals, so a root that is wrong, malicious or later corrected (for example after fraud is found and an affiliate's total is lowered) stays claimable. An affiliate can claim against the older, higher-value epoch forever. A bad root cannot be undone.

Severity: Medium
Location: program/src/lib.rs:102
Mechanism: Nothing ties the sum of published entitlements to the vault balance. `affiliate.claimed` is updated before the transfer, so if the vault is underfunded the transfer fails, and payouts depend on who claims first. Late claimers get nothing until someone tops up the vault. Because the authority can promise more than the vault holds, this is a liveness and fairness problem.

Severity: Medium
Location: program/src/lib.rs:146
Mechanism: The campaign has no authority rotation, transfer, pause or close instruction, and `authority` is a plain key used in the PDA seeds. If the key is lost or compromised, no new epochs can be published, so affiliates cannot be paid for new earnings. Because the key is in the seeds, the campaign cannot be handed to a new key. A compromised key makes the first finding permanent.

Severity: Low
Location: program/src/lib.rs:187
Mechanism: Each epoch creates a permanent `EpochRoot` account that is never closed, and old epochs stay valid, so rent grows without bound and cannot be reclaimed. Affiliate accounts (line 203) also cannot be closed. This is an architecture and scalability cost. It also keeps the stale-root problem above alive.

Severity: Low
Location: program/src/lib.rs:152
Mechanism: The campaign PDA is derived only from (authority, mint), so one authority can run only one campaign per mint. A second program or campaign needs a new key. There is also no way to move leftover vault funds out, so a mis-funded or abandoned campaign locks tokens, apart from the authority publishing a root that pays itself.

Severity: Info
Location: program/src/lib.rs:93
Mechanism: `if owed > 0` is always true because line 89 already requires `earned_total > claimed`. Proof length is unbounded, but only the caller's own compute cost grows with it, so this is not exploitable. The Merkle leaf and node hashing is domain-separated and the sorted-pair check looks sound. Only the classic SPL Token program is supported, not Token-2022.
