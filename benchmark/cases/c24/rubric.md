---
id: c24
canary: RDSAF81C11F051B132899533
---
## Mechanism
Only `pool.authority` can call `close_epoch`, and the deployment describes that key as the team's keeper on the team's server. `withdraw` requires the request's epoch to be closed. So every exit depends on one operator key and server: if the keeper stops, `pool.epoch` never advances and unstaked funds stay locked. Epoch closing should be open to anyone once the time has passed, with a reward for the caller.

## Locations
- src/lib.rs:191-196
- src/lib.rs:104-124
- src/lib.rs:78-102
- ARCHITECTURE.md:6
- ARCHITECTURE.md:3-3
- src/lib.rs:201-201

## Judge criteria
- PASS: the finding says that advancing or closing the epoch (`close_epoch`) is restricted to the authority or operator, or depends on the team's keeper, and that `withdraw` therefore depends on it. If that key or service is unavailable, users cannot withdraw (liveness failure, funds locked). A recommendation to make `close_epoch` permissionless after `epoch_length`, ideally with a fee or tip for the caller, is expected but not required.
- PASS: the finding is framed as centralization or censorship ("the authority can stop withdrawals by never closing epochs"), as long as it names `close_epoch` gating `withdraw`.
- FAIL: generic remarks such as "admin key is a single point of failure" or "consider a multisig" that do not tie the authority to `close_epoch` and locked withdrawals.
- FAIL: the finding only questions the cooldown semantics or epoch timing (for example "requests made late in an epoch unlock early") without the dependency on the operator.
