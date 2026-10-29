# Solana Audit Report — affiliate_rewards (`program/src/lib.rs`)

## 1. Claim against a stale epoch rolls back `claimed`, so the vault can be drained by alternating claims
Severity: Critical
Location: program/src/lib.rs:90
Mechanism: `claim` accepts any published epoch and unconditionally sets `affiliate.claimed = earned_total`, even when `earned_total` is lower than what has already been paid. An affiliate can claim with the latest root (paid in full), then claim with an older root showing a smaller cumulative total (paid 0, but `claimed` drops back down), then claim the latest root again and be paid the difference again. Repeating this drains the vault. The fix is to require `earned_total > affiliate.claimed` (or only update when it is larger) and/or accept only the latest epoch.

## 2. Old roots stay valid forever, so a bad or compromised root can never be revoked
Severity: High
Location: program/src/lib.rs:219
Mechanism: The `epoch` account is checked only with `has_one = campaign`, and nothing can close, supersede or invalidate an `EpochRoot`. If the authority ever publishes a wrong root (an inflated leaf, a bug in the off-chain tree builder), the bad leaves can be claimed permanently, and publishing a corrected root does not stop that. Combined with finding 1, every historical root is also an attack input.

## 3. A single authority key fully controls payouts, with no timelock or challenge window
Severity: High
Location: program/src/lib.rs:45
Mechanism: `publish_epoch` lets one signer post an arbitrary root that takes effect immediately (`published_at` is recorded but never enforced). A malicious or compromised authority can publish a root that gives an address it controls the whole vault balance, including tokens added by third-party funders, and claim it in the same slot. There is no multisig, delay, cap per epoch, or check that earnings only grow.

## 4. The authority key cannot be rotated or recovered, which blocks future epochs (liveness)
Severity: Medium
Location: program/src/lib.rs:151
Mechanism: `authority` is part of the campaign PDA seeds and no instruction changes it. If the key is lost or must be rotated after a compromise, no new epochs can ever be published for that campaign, and affiliates' later earnings cannot be paid.

## 5. Funds can never be withdrawn and the campaign cannot be closed
Severity: Medium
Location: program/src/lib.rs:28
Mechanism: Tokens can only leave the vault through `claim`. There is no withdraw, sweep or close instruction, so unclaimed or excess funding (including mistaken top-ups from anyone through `fund`) is locked for good. There is also no way to wind a campaign down cleanly.

## 6. No solvency accounting: an underfunded vault is paid out first come, first served
Severity: Medium
Location: program/src/lib.rs:101
Mechanism: Nothing ties the published root totals to the vault balance. If roots promise more than the vault holds, early claimers are paid in full and later claims fail on the token transfer, so affiliates cannot rely on their published earnings being payable.

## 7. Epoch accounts pile up without bound and their rent is never reclaimed (scalability/cost)
Severity: Low
Location: program/src/lib.rs:186
Mechanism: Every epoch creates a new rent-paying PDA that can never be closed, so the authority's cost grows linearly with the number of epochs. Because every epoch also stays claimable (findings 1 and 2), the set of valid roots, and with it the attack surface, keeps growing.

## 8. Affiliates must register first, which adds a rent cost and an extra step before claiming
Severity: Low
Location: program/src/lib.rs:63
Mechanism: An affiliate's claim fails until they create their own `Affiliate` PDA and pay its rent. This makes claiming harder for small earners. It is not exploitable, but the record could instead be created during the first claim (`init_if_needed`).

## 9. Only the legacy SPL Token program is supported, and freezable mints are accepted
Severity: Info
Location: program/src/lib.rs:164
Mechanism: The `Token` program type rules out Token-2022 mints. A mint with a freeze authority can have the vault frozen by its freeze authority, which blocks every claim (a liveness risk outside the campaign's control).

## 10. The proof length is not bounded
Severity: Info
Location: program/src/lib.rs:135
Mechanism: `proof` has no length limit. Oversized proofs only waste the caller's own compute or fail on transaction size, so this is not exploitable, but a maximum tree depth would make failures clearer. Leaf and node hashes are domain-separated by prefix and input length, so second-preimage attacks on the tree are not possible.
