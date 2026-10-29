# Solana audit report: affiliate_rewards (program/src/lib.rs)

Severity: Critical
Location: program/src/lib.rs:90
Mechanism: `affiliate.claimed = earned_total` is written unconditionally, and `claim` accepts any published epoch (line 73). An affiliate can claim with a newer epoch (total 100), then replay an older epoch's smaller leaf (total 50, owed 0), which rolls `claimed` back to 50. Claiming again with the newer epoch pays the 50 difference a second time. Repeating this drains the whole vault, including other affiliates' funds. The fix is to set `claimed = max(claimed, earned_total)` (or reject `earned_total <= claimed`) and/or accept only the latest epoch.

Severity: High
Location: program/src/lib.rs:45
Mechanism: The campaign authority alone chooses each epoch root and nothing validates it. There is no check against vault balance, no cap on cumulative totals, and no timelock or dispute path. A malicious or compromised authority can publish a root with a leaf for its own registered affiliate and claim every token in the vault, including third-party funds from the permissionless `fund`.

Severity: Medium
Location: program/src/lib.rs:45
Mechanism: Epoch roots are immutable and cannot be revoked or closed, and `claim` accepts every historical epoch (line 74). A root published with a mistake, or a leaf that over-credits someone, stays claimable forever. Because totals are cumulative, a corrected later root cannot take back what an earlier root already allows.

Severity: Medium
Location: program/src/lib.rs:101
Mechanism: Solvency is not tracked, so the vault can hold less than the sum of outstanding entitlements. Claims are first-come-first-served, and a claim fails at the token transfer once the vault is short. Late affiliates are left unpaid, which is a liveness and fairness problem. Early claimers can also front-run others when funds are scarce.

Severity: Low
Location: program/src/lib.rs:190
Mechanism: Each epoch creates a permanent PDA account that the authority pays rent for, and the account is never closed. State grows without bound, and rent is locked up for the life of the campaign. Authority key loss also means no more roots can be published, which stops all new accruals. There is no authority rotation instruction.

Severity: Low
Location: program/src/lib.rs:145
Mechanism: The mint's freeze authority is not checked in `create_campaign`. A mint with a freeze authority lets the issuer freeze the vault or an affiliate's token account, which blocks all claims. Mints with a delegate or other extensions are also not restricted.

Severity: Info
Location: program/src/lib.rs:136
Mechanism: The Merkle verification uses domain-separated leaf and node prefixes, binds the campaign and owner in the leaf, and sorts pairs, so second-preimage attacks are not feasible. There is no proof-length bound, but it is limited by the transaction size.
