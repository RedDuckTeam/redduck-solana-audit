# Solana audit report: affiliate_rewards

Severity: Critical
Location: program/src/lib.rs:90
Mechanism: `affiliate.claimed = earned_total` is written unconditionally, even when `earned_total` is lower than the stored `claimed`, and `claim` accepts any epoch. An affiliate can claim against the newest epoch (for example 200), then claim against an older epoch with a smaller total (for example 100). That pays nothing but resets `claimed` to 100. Re-claiming against the newest epoch then pays 100 again. Repeating this drains the whole vault. The fix is to require `earned_total > claimed` or to use `max`.

Severity: High
Location: program/src/lib.rs:73
Mechanism: Every published epoch stays claimable forever, and epochs are immutable and cannot be revoked or superseded. If the authority publishes a wrong or inflated root, or its key is compromised, that root can be drained permanently. Claims only need to be monotonic across epochs, and this is not enforced (see the Critical finding).

Severity: Medium
Location: program/src/lib.rs:45
Mechanism: The single `authority` key alone decides how much every affiliate is owed. Roots are not checked against vault balance or total funding, and there is no timelock, multisig or dispute path. A malicious or compromised authority can publish a root that pays itself, as an affiliate, the entire vault.

Severity: Medium
Location: program/src/lib.rs:101
Mechanism: Nothing ties the amounts promised in roots to the vault balance. If a root over-allocates, claims are first-come-first-served. Late claimers hit a failing transfer, and their `claimed` update is rolled back, so they are stuck until someone tops up the vault. This is a liveness and fairness problem.

Severity: Medium
Location: program/src/lib.rs:16
Mechanism: `campaign.authority` is fixed at creation. There is no authority rotation, no withdrawal of unclaimed or leftover funds, and no close or pause instruction. If the authority key is lost, or it stops publishing, deposited funds and future rewards are locked or stalled. Funds sent by mistake can never be recovered.

Severity: Low
Location: program/src/lib.rs:190
Mechanism: Each epoch creates a new permanent `EpochRoot` account, paid for by the authority, with no close instruction. Rent grows linearly with epochs and the accounts are never reclaimed. Old epochs also cannot be retired, which is the same cause as the High finding.

Severity: Low
Location: program/src/lib.rs:63
Mechanism: `register` requires a separate `Affiliate` account (rent paid by the owner) before the first claim. Every affiliate therefore needs a registration transaction, which adds friction at scale. The `owner_token` account is also only checked for mint, not owner. This is harmless because the owner signs, but it makes it easy to send rewards to an unintended account.

Severity: Info
Location: program/src/lib.rs:3
Mechanism: The program only supports the classic SPL Token program (`Program<'info, Token>`). Token-2022 mints (transfer fees, hooks) are not supported. A mint with a freeze authority can freeze the vault or claimant accounts and block claims.
