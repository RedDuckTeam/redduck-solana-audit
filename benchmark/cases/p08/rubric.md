---
id: p08
canary: RDSAF81C11F051B132899533
---
## Mechanism
`withdraw` authorizes a payout using accounts the caller supplies, with no root in program state. The `instructions` account is never checked against the instructions sysvar address before `load_instruction` parses it. The `allowlist` is any program-owned `Allowlist`, and anyone can create one with `create_allowlist`; it is not bound to `config`. So anyone can make the signature check pass and drain the vault.

## Locations
- src/instructions/withdraw.rs:6-23
- src/instructions/withdraw.rs:34-46
- src/sysvar.rs:6-56
- src/instructions/create_allowlist.rs:5-22
- src/state/allowlist.rs:5-11
- src/state/config.rs:3-9

## Judge criteria
HIT if the finding says that an untrusted caller can get `withdraw` to pay without a real oracle signature, and names at least one of the two caller-controlled trust anchors:
- the unchecked `instructions` account: no comparison with the sysvar id, so a fake account with sysvar-shaped data is accepted;
- the allowlist: any account created through `create_allowlist` is accepted, and it is not stored in or derived from `config`.

Naming both is ideal, but either one is a HIT because each is enough to drain the vault.

FAIL:
- Replay of a genuine signed withdrawal message (it has no nonce). That is a separate issue, not this mechanism.
- Generic "validate accounts" advice that names neither the instructions account nor the allowlist.
- Findings about `deposit`, `initialize` or the Ed25519 offset parsing alone.
- Other issues in this program do not count as a HIT: a genuine signed withdrawal message `(vault, recipient, amount)` can be resent because it has no nonce, expiry or consumed flag, and `initialize` can be front-run by the first caller of the `config` PDA.
