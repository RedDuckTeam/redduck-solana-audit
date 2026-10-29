---
id: c19
crate: affiliate_rewards
pattern: one-way-state
class: exploit
group: g2
vulnerable: b
role: dev
author: case-author
canary: RDSAF81C11F051B132899533
---
# Case c19

`affiliate_rewards` pays affiliate commissions from a campaign vault. The campaign authority publishes a Merkle root for every epoch (`publish_epoch`). Each leaf is `(campaign, affiliate, earned_total)`, where `earned_total` is the affiliate's total earnings since the campaign started. Every affiliate has an `Affiliate` account whose `claimed` field records how much was paid. `claim` accepts a proof against any published epoch and pays `earned_total - claimed`.

## Attack scenario
1. Campaign C has a vault funded with 50,000 tokens owed to many affiliates. The attacker is a real, registered affiliate. Epoch 0's root has the leaf `(C, attacker, 100)`, and epoch 1's root has `(C, attacker, 150)`. Both proofs come from the off-chain tree the campaign publishes. `affiliate.claimed = 0`.
2. The attacker calls `claim(earned_total = 150, proof1)` with `epoch = E1`. The proof is valid, `owed = 150 - 0 = 150`, and `claimed = 150`. 150 tokens are paid. So far this is correct.
3. The attacker calls `claim(earned_total = 100, proof0)` with `epoch = E0`. The proof is valid against the older root. `owed = 100.saturating_sub(150) = 0`, and then `claimed = earned_total = 100`. The counter moves backwards and nothing is paid.
4. The attacker calls `claim(150, proof1)` with `E1` again. `owed = 150 - 100 = 50`, and 50 tokens are paid. `claimed = 150`.
5. Steps 3 and 4 can be repeated, several pairs per transaction. Each pair pays another 50 tokens, until the vault, which holds every other affiliate's earnings, is empty.

## Why fixed closes it
In the fixed program (`src/lib.rs:88-89`), `let owed = earned_total.saturating_sub(affiliate.claimed);` is replaced by
`require!(earned_total > affiliate.claimed, RewardsError::NothingToClaim);` and `let owed = earned_total - affiliate.claimed;`. The error variant `NothingToClaim` is also added (fixed `src/lib.rs:289-290`).

`claimed` can now only move up. Step 3 fails, because 100 is not above 150, and `claimed` stays at 150. In step 4, 150 is not above 150, so it fails too. The order of claims no longer matters, and a leaf that was already paid cannot pay again.

The normal flow still works. An affiliate claims with the newest (or any) epoch whose total is above what they have been paid, and gets exactly the difference. A claim prepared against an older root that lands after a newer root still pays the right difference, as long as its total is above `claimed`. A duplicate or stale claim fails instead of paying. The diff is 1 line replaced by 2, plus 2 added lines for the error.

## Decoys
- The `epoch` account in `Claim` is only constrained with `has_one = campaign`, so any published epoch is accepted. This is deliberate and safe once `claimed` only moves forward, because the leaves are cumulative totals.
- `fund` is open to everyone. Donating tokens to the vault cannot hurt anyone.
- The Merkle check (`verify_proof`) hashes sorted pairs with separate leaf and node prefixes (`0`/`1`), and the leaf includes the campaign key. This prevents proofs from being reused across campaigns and prevents second-preimage tricks.
- `publish_epoch` derives the epoch PDA from the `campaign.epochs` counter and is restricted to the authority with `has_one = authority`, so roots cannot be overwritten or forged.
- `owner_token` in `Claim` is any token account of the campaign mint. The affiliate signs and chooses where their own payout goes.

## Trusted roles
- **Campaign authority** (`campaign.authority`): publishes one Merkle root per epoch with `publish_epoch`. It is trusted to put every affiliate's correct cumulative earnings in each root and to keep the vault funded. It cannot overwrite or delete a published root, move tokens out of the vault, or change any affiliate's `claimed` counter.
- Nobody else is trusted. Anyone may add tokens with `fund`. An affiliate can only claim for itself, with a proof against a published root.
