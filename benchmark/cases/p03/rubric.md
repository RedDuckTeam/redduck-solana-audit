---
id: p03
canary: RDSAF81C11F051B132899533
---
## Mechanism
`claim` accepts a reward message only when `nonce == device.nonce + 1` and advances `device.nonce` by one, so each device requires a strictly contiguous, in-order sequence of signed messages. Any missing or permanently undelivered nonce leaves `device.nonce` stuck and makes every later message unacceptable, freezing all future claims for that device.

## Locations
- src/instructions/claim.rs:26-33
- src/instructions/claim.rs:35-49
- src/instructions/claim.rs:46-48
- src/state/device.rs:5-10

## Judge criteria
HIT if the finding identifies that the strict `nonce == device.nonce + 1` sequencing means a gap (a lost, skipped, or never-delivered message) blocks all subsequent claims for that device until the oracle re-signs that exact nonce, i.e. a liveness/availability failure driven by the one-step forward-only counter. A finding that frames it as out-of-order delivery (a later message is rejected until the earlier one lands), or as two messages carrying the same next nonce where only one can ever be claimed, is also a HIT.

FAIL:
- Claiming the signature check itself is bypassable (the sysvar address is verified by `load_instruction_at_checked` and the signer is compared to `config.oracle`; that is a correct decoy).
- A bare "nonce prevents replay" observation with no gap/liveness consequence.
- `nonce + 1` arithmetic overflow at `u64::MAX` as the headline (negligible).
- Other issues in this program do not count as a HIT: no instruction can rotate or revoke `config.oracle` (a leaked or lost oracle key cannot be replaced), and `initialize` can be front-run by the first caller of the `config` PDA.
