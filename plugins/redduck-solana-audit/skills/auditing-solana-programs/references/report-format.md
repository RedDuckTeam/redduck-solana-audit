# Report format

## Contents
- File and layout
- Finding block
- Severity
- Design findings
- Coverage table
- Final message

## File and layout

Write `solana-audit-report.md` in the working directory unless the user names another path:

```markdown
# Solana audit report: <program name>

## Scope
Programs, commit or folder, framework, instructions reviewed, and anything out of scope. Then two lines:
Surfaces: <attack surfaces found in step 4>
Pattern files read: <the references/patterns files opened for them>

## Summary
Counts per severity, and the one or two findings that matter most.

## Findings
<finding blocks, most severe first>

## Design findings
<finding blocks for liveness, centralization and scalability problems>

## Coverage
<coverage table>
```

## Finding block

Each finding starts with the three machine-readable lines, each on its own line and in this order. Tools parse them, so do not wrap them in tables or bullets.

```markdown
### F-1: <short title naming the root cause>

Severity: High
Location: src/instructions/withdraw.rs:42
Mechanism: The vault account is only compared with the caller-supplied config, so an attacker can pass a config they created and withdraw from any vault.

Pattern: root-of-trust
Scenario:
1. Attacker creates config X with `authority = attacker`.
2. Calls `withdraw` with the victim's vault and config X.
3. The `has_one = config` check passes, and the vault pays the attacker.
Test: confirmed | refuted | unconfirmed (no toolchain) | not attempted
Verification: confirmed by verifier | downgraded by verifier from Critical: <reason> | self-checked, verifier unavailable: "<error the Agent tool returned>" | self-checked, verifier unavailable: no Agent tool
Fix: Derive the config as a PDA with seeds [b"config"] and check the vault's seeds against it.
```

- **Location** points at the line where the missing or wrong check belongs. That is usually the account constraint or the handler line, not the place where the damage shows up. Paths are relative to the program root. If the fix spans several places, give the most specific one, and name the others in the scenario.
- **Mechanism** states the root cause and how it is abused, in one or two sentences.
- **Pattern** is a pattern slug or `checklist-<item>`.

## Severity

| Severity | Meaning |
|---|---|
| Critical | Direct loss or theft of funds, or unauthorized minting, by any user and with no special conditions |
| High | Loss of funds under realistic conditions, permanent freeze of user funds, or full protocol takeover |
| Medium | Bounded loss, temporary freeze, griefing that blocks users, or a liveness failure that halts the protocol until someone intervenes |
| Low | Minor value leaks, edge cases with small impact, missing defence in depth |
| Info | Code quality and documentation, without a security impact |

Judge severity on impact and likelihood together.

**Trusted roles.** Some roles the protocol deliberately trusts: an admin, an upgrade authority, an oracle or root signer, a reporter that users can see before they commit funds. The risk that such a role misuses its power is a design finding, at Medium at most. Say who holds the role and what it can do. It becomes High or Critical only when the role can do more than its stated purpose. One example is a fee admin that can also withdraw user deposits. Another is an instruction that lets anyone take over the role.

The cap applies only when the harm requires the role to misbehave. If an ordinary user can profit while the role behaves honestly, rate the finding by its impact. An example is a report the program accepts too early, so that results already known can still be bet on.

## Design findings

Liveness, centralization and scalability problems use the same block under `## Design findings`. Examples:
- a settlement only the operator can run;
- an oracle with no way to switch away;
- a loop that stops fitting in a transaction as users grow.

Their severity follows the table. "Users cannot withdraw while the operator is offline" is at least Medium, and High when nothing else can unlock the funds.

## Coverage table

Every pattern and every checklist item gets one row:

```markdown
| Item | Verdict | Evidence |
|---|---|---|
| root-of-trust | clear | vault PDA seeds [b"vault", mint] checked at src/lib.rs:31 |
| untrusted-executor | found | F-2 |
| execution-layer-shift | n/a | no rollup, delegation or off-chain matching |
| checklist-arithmetic | clear | checked_mul / checked_div in src/math.rs:10-24 |
```

- `found` names the finding id.
- `clear` names the protection and its line.
- `n/a` gives a one-clause reason.
- For each pattern whose file you read in step 4, start the evidence with the question that decided the verdict, for example "Q2: chain ends at config PDA, src/lib.rs:31".

Medium and lower findings need no verifier call. High and Critical findings always carry a `Verification:` line from step 7.

## Final message

Repeat the list of findings in your final message: each finding's id, title and its three lines. Then add one sentence on the overall risk.
