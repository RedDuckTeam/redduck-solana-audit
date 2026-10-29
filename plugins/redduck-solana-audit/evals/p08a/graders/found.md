---
type: llm
focus: { source: file, path: solana-audit-report.md }
---
You are grading a security audit report of a Solana program.

The program contains this flaw: `withdraw` authorizes a payout using accounts the caller supplies, with no root in program state. The `instructions` account is never checked against the instructions sysvar address before `load_instruction` parses it. The `allowlist` is any program-owned `Allowlist`, and anyone can create one with `create_allowlist`; it is not bound to `config`. So anyone can make the signature check pass and drain the vault.

Acceptable places for the finding (paths are relative to the program folder; a report may prefix them with "program/"):
- src/instructions/withdraw.rs lines 6-23
- src/instructions/withdraw.rs lines 34-46
- src/sysvar.rs lines 6-56
- src/instructions/create_allowlist.rs lines 5-22
- src/state/allowlist.rs lines 5-11
- src/state/config.rs lines 3-9

PASS if the report contains a finding that identifies this flaw (the same root cause, not just a nearby symptom or a generic best-practice remark) and ties it to one of the acceptable places, either by a line inside a listed range or by naming the function, instruction or account struct at that place.
The severity of the finding and the section it appears in do not matter.
FAIL otherwise.

Additional guidance: HIT if the finding says that an untrusted caller can get `withdraw` to pay without a real oracle signature, and names at least one of the two caller-controlled trust anchors:
- the unchecked `instructions` account: no comparison with the sysvar id, so a fake account with sysvar-shaped data is accepted;
- the allowlist: any account created through `create_allowlist` is accepted, and it is not stored in or derived from `config`.

Naming both is ideal, but either one is a HIT because each is enough to drain the vault.

FAIL:
- Replay of a genuine signed withdrawal message (it has no nonce). That is a separate issue, not this mechanism.
- Generic "validate accounts" advice that names neither the instructions account nor the allowlist.
- Findings about `deposit`, `initialize` or the Ed25519 offset parsing alone.
- Other issues in this program do not count as a HIT: a genuine signed withdrawal message `(vault, recipient, amount)` can be resent because it has no nonce, expiry or consumed flag, and `initialize` can be front-run by the first caller of the `config` PDA.

<!-- RDSAF81C11F051B132899533 -->
