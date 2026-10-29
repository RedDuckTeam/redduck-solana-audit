---
id: c07
canary: RDSAF81C11F051B132899533
---
## Mechanism
`enter_match` pays the player's stake out of the base-layer vault into the
operator-controlled `executor` account, and the only ways the stake returns and
`in_match` clears are `settle_match` and `expire_match`, both operator-only. `withdraw`
is blocked while `in_match`, and there is no timeout a user can trigger alone, so if the
operator/rollup stops, deposits are stranded. The fix keeps funds in the vault
(delegating only match state) and adds an L1 timeout exit in `withdraw`.

## Locations
- src/instructions/enter.rs:45-46
- src/instructions/enter.rs:33-54
- src/instructions/withdraw.rs:32-37
- src/instructions/settle.rs:30-45
- src/instructions/expire.rs:30-46
- ARCHITECTURE.md:15-28

## Judge criteria
- A hit must identify that user funds are handed to the off-chain / operator-controlled
  executor and can only return via the operator (`settle_match` / `expire_match`), with
  no base-layer exit, so a stalled operator/rollup strands user deposits. Naming the
  missing user-controlled L1 exit (timeout / permissionless withdrawal) or the fact that
  funds rather than only round state are delegated is required.
- A finding that only observes "the operator is trusted / centralized" without
  connecting it to the inability to exit on L1 is a near miss; to pass it must identify
  the stranded-funds / no-exit consequence.
- A finding that only flags the `settle_match` operator check as missing validation is a
  FAIL; the operator is a listed trusted role.
- Reporting a theft/fund-stealing exploit is a FAIL; this is a liveness/exit (design)
  issue.
