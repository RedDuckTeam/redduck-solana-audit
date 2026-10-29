---
id: c08
canary: RDSAF81C11F051B132899533
---
## Mechanism
`cancel_action` marks an action `cancelled`, but `execute_action` never checks that flag. It checks only `queued_at`, `executed` and the timelock window. The cancelled check lives in `require_pending`, which execute does not use. A guardian cancellation made after queueing, the veto the timelock exists for, does not stop anyone from executing the action through the treasury signer after the delay.

## Locations
- src/instructions/execute.rs:9-15
- src/instructions/execute.rs:17-31
- src/instructions/timelock.rs:45-61
- src/instructions/mod.rs:16-22
- src/state.rs:55-60

## Judge criteria
- PASS: the finding says that a cancelled action, typically one cancelled by the guardian after it was queued, can still be executed because `execute_action` does not check `cancelled`. Saying that the cancellation or guardian veto is ineffective and executes anyway is enough.
- FAIL: a finding about re-execution, account or program substitution, or flag escalation in `stored_instruction`. These are correctly guarded.
- FAIL: a finding that `execute_action` is permissionless, or that the guardian is too powerful. Both are intended.
- FAIL: generic "add state checks to execute" without naming cancellation.
