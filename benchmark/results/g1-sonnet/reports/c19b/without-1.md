# Solana Audit Report: affiliate_rewards

Severity: Critical
Location: program/src/lib.rs:90
Mechanism: `claim` accepts any published epoch and overwrites `affiliate.claimed = earned_total` even when the value is lower than the stored one. An affiliate can claim with the latest epoch (claimed = 100), then replay a proof from an older epoch (earned_total = 50, owed = 0, claimed reset to 50), then claim the latest again and be paid 50 again. Repeating this drains the vault. Fix: only accept the latest epoch, or set `claimed = max(claimed, earned_total)` and require `earned_total > claimed`.

Severity: High
Location: program/src/lib.rs:45
Mechanism: `publish_epoch` lets the single campaign authority publish any root with no check against vault balance or the previous epoch's totals. A malicious or compromised authority can add a leaf for themselves with a huge `earned_total` and drain every token that third parties deposited through the open `fund` instruction. There is no withdraw, so the risk is trust-only, with no on-chain limit.

Severity: Medium
Location: program/src/lib.rs:74
Mechanism: Epoch roots are never revoked or expired, and every historical epoch stays claimable forever. A wrong root (a bug or a bad leaf) cannot be fixed, because later roots do not invalidate it. Combined with the finding at line 90, all past roots stay live attack surface.

Severity: Medium
Location: program/src/lib.rs:101
Mechanism: Claims are paid first-come-first-served from the vault, and the program does not track total obligations against the vault balance. If published roots promise more than the vault holds, late claimants' transfers fail and the earliest claimants take the funds. Their `claimed` update then reverts with the failed transfer, so honest users are blocked until someone funds more.

Severity: Low
Location: program/src/lib.rs:16
Mechanism: Authority is a single key fixed at creation, with no rotation or multisig. If the key is lost, no further epochs can be published and rewards stall, so liveness depends on that one key. Compromise is covered by the High finding.

Severity: Low
Location: program/src/lib.rs:186
Mechanism: Each epoch creates a permanent `EpochRoot` PDA that is never closed, so the authority pays rent for every epoch and state grows without bound. Old epochs cannot be closed, which also keeps the stale-root problem alive.

Severity: Info
Location: program/src/lib.rs:3
Mechanism: Only the legacy SPL Token program is supported (`Program<'info, Token>`). Token-2022 mints are rejected. The `mint` is not checked for a freeze authority, which could freeze the vault and block claims.
