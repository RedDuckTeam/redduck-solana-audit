---
id: c06
canary: RDSAF81C11F051B132899533
---
## Mechanism
`cancel` refunds and closes an open commitment with no timing restriction. Once the
randomness slot (`reveal_slot`) has passed, the outcome is already determined and
readable from `SlotHashes` before any transaction is sent, so a player reveals winners
and cancels losers for a full refund — free re-rolls that drain the prize pool. A
committed (randomness-assigned) crate must not be abandonable for a refund.

## Locations
- src/instructions/cancel.rs:33-53
- src/instructions/cancel.rs:6-31

## Judge criteria
- A hit must identify that a losing commitment can be cancelled/refunded after the
  outcome is knowable (no check that `slot < reveal_slot`), enabling free re-rolls / only
  revealing wins. Naming the missing timing guard on `cancel`, or the ability to abandon
  a determined bet, is required.
- A finding that only flags `reveal` (same-tx resolution, user-controlled randomness) is
  a FAIL: reveal is correctly split and permissionless, sourced from the committed slot
  hash.
- A finding that only says "randomness is weak / use a VRF" without identifying the
  refundable-cancel abandonment is a FAIL for this case.
- Flagging the `slot_hashes` unchecked account or the house authority is a FAIL.
