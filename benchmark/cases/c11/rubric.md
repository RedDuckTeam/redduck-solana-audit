---
id: c11
canary: RDSAF81C11F051B132899533
---
## Mechanism
The only way to get a card balance out of the vault is `complete_withdrawal`, which requires a co-signature from `PROCESSOR`, a key hardcoded in an immutable program; there is no way to replace it and no exit without it, so if the external processor's signing service stops, all user balances are locked permanently.

## Locations
- src/state.rs:7-8
- src/instructions/card.rs:109-154
- src/instructions/processor.rs:13-15
- src/instructions/admin.rs:49-98

## Judge criteria
- PASS: the finding says withdrawals depend on the hardcoded processor key (or its co-signature) with no rotation path and no direct or emergency exit, so users' funds are locked if the processor key is lost or its service stops (or the processor can block withdrawals indefinitely). It must name `complete_withdrawal`, `CompleteWithdrawal` or the `PROCESSOR` constant.
- PASS: "the processor is a single point of failure for withdrawals; add a timelocked way to change it and a delayed self-service exit" when it is tied to user funds being stuck.
- FAIL: only "the processor is trusted and can drain cards through `settle_spend`" (a listed trusted role) without the locked-funds failure.
- FAIL: only "the admin cannot be rotated" or "the settlement account can be changed" (it is timelocked).
- FAIL: generic centralization remarks that do not identify the withdrawal path.
