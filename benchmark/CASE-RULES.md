# Rules for benchmark cases

These rules are for the case author and the case reviewer. A case is a pair of small Solana programs: a vulnerable version that contains one planted flaw, and a fixed version that removes it. The benchmark measures whether an auditor finds the planted flaw, so a case must not give the answer away.

## Isolation

- Do not open anything under `plugins/`. The case author must not see the skill being tested.
- Do not open other cases unless the task names them.
- Held-out cases are shown to nobody except the case author and the case reviewer. When reporting on them, report only ids and pass/fail.

## Ids and groups

- Pick an id with `node tools/pick-id.mjs` (own cases `c01`..`c24`). Public cases use `--prefix p`.
- Pick the vulnerable side at random: `node -e "console.log(Math.random() < 0.5 ? 'a' : 'b')"`.
- `group` is set by the pattern or checklist item:
  - `g1`: root-of-trust, identity-rejection, untrusted-executor, role-inversion, lazy-loading, pay-as-you-act;
  - `g2`: divide-and-conquer, complexity-asymmetry, commit-execution-split, action-as-data, virtual-execution, one-way-state, boundary-of-consent;
  - `g3`: internal-oracle, defensive-computation, fail-fast, execution-layer-shift, dependency-independency, and every checklist case.

## Files

```
benchmark/cases/<id>/CASE.md
benchmark/cases/<id>/rubric.md
benchmark/workspaces/vulnerable/programs/<crate>/{Cargo.toml, Xargo.toml, src/**, ARCHITECTURE.md?}
benchmark/workspaces/fixed/programs/<crate>/{same files}
```

The crate name is a realistic domain name (`escrow_vault`, `reward_pool`, `ticket_raffle`). It is the same in both workspaces, and it must not be already used.

`CASE.md`:

```markdown
---
id: c07
crate: escrow_vault
pattern: root-of-trust            # pattern slug, or checklist-<item> for checklist cases
class: exploit                    # exploit = loss of funds or state; design = liveness, centralization, scalability
group: g1
vulnerable: b                     # which eval side (a or b) is the vulnerable program
role: dev                         # dev | heldout, as assigned in the task
author: case-author
canary: <contents of tools/canary.txt>
---
# Case c07

## Attack scenario
Numbered steps: which accounts the attacker creates or passes, which instruction they call with which arguments, and what they get. For design cases: the concrete failure, for example "the team's server stops, so no round can be settled and deposits stay locked".

## Why fixed closes it
Which lines changed in the fixed program and why the scenario no longer works. Confirm the normal user flow still works.

## Decoys
The code that looks suspicious but is correct, and why.

## Trusted roles
Each role the program deliberately trusts, and exactly what it may do.
```

`rubric.md`:

```markdown
---
id: c07
canary: <contents of tools/canary.txt>
---
## Mechanism
One or two sentences a judge can compare a finding against: the root cause, not the symptom.

## Locations
- src/lib.rs:40-58
- src/instructions/withdraw.rs:12-30

## Judge criteria
Anything a judge needs to tell a real hit from a near miss, such as "a finding that only says 'add more validation' without naming the vault account is a FAIL".
```

List **every** place an auditor could reasonably point at: the accounts struct and the handler, for example. Paths are relative to the program folder.

## The programs

- **Anchor 0.31.1.** Copy the `Cargo.toml` layout of an existing program (`[lib] crate-type = ["cdylib", "lib"]`, the standard features, `anchor-lang = "0.31.1"`, plus `anchor-spl = "0.31.1"` when tokens are needed). Use **no** `description` line. Take ids from `node tools/new-program-id.mjs`. Add the program to `[programs.localnet]` in both `Anchor.toml` files.
- **Pinocchio and native cases** are allowed where the checklist item needs them. Keep them inside the same workspaces.
- **Size.** 100–300 lines of Rust per program for the three pilot cases. Every case written after the pilot follows "Difficulty" below.
- **One planted flaw** in the vulnerable version, matching the case's pattern or checklist item. The rest of the program must be correct. An accidental second bug ruins the false-positive measurement.
- **At least one decoy:** correct code that looks like it could be wrong, such as an `UncheckedAccount` that is validated by address, or arithmetic that is checked.
- **Minimal diff.** The fixed version differs only where the flaw is. Aim for 15 changed lines or fewer; design cases may need up to 30, and say why in `CASE.md`.
- **No hints anywhere** in code, comments, names, paths, `ARCHITECTURE.md` or manifests: none of `vuln`, `bug`, `exploit`, `insecure`, `unsafe_`, `todo`, `fixme`, `missing check`, and no pattern names (`root of trust`, `one-way state`, `untrusted executor` and the rest). Comments should read like a normal codebase.
- **Do not copy famous exploits.** Wormhole, Cashio, Mango and the like are in every model's training data. Use the same class of flaw in a different domain with different names and structure.
- **Design cases** add `ARCHITECTURE.md` to the program folder: deployment facts only, written neutrally, without judging them. For example: "A service run by the team calls `close_round` every hour with the operator key." The fixed version's `ARCHITECTURE.md` describes the corrected operation.

## Difficulty (every case after the pilot)

The pilot showed that strong models find a flaw sitting in one account constraint without help. Make later cases harder **by construction**. Never run a model on a case, and never keep or drop a case based on how a model did on it.

- **A real project layout.** 300–600 lines of Rust in at least three files: `lib.rs`, `state.rs`, `errors.rs`, and `instructions/*.rs` with one file per instruction group. Add events, helper functions and a realistic number of instructions (5–9).
- **The flaw lives in an interaction, not in a single line.**
  - The state that one instruction writes is misused by another.
  - A check exists but is placed where it does not cover every path.
  - An order of calls breaks an invariant.
  - An economic step goes wrong over several transactions.

  Reading one accounts struct in isolation should not reveal it.
- **At least two decoys.** One must look like the same flaw class elsewhere in the program, but be correctly guarded there.
- **No narrating comments near the flaw.** Do not write doc comments that describe the accepted inputs, ordering or trust at the vulnerable spot, such as "any published epoch is accepted". Normal API docs elsewhere are fine.
- **Name the trusted roles.** List them in `CASE.md` under `## Trusted roles` (admin, authority, oracle signer and so on) with what each may do. Neither version may give a role more power than listed, and no untrusted party may obtain one.
- **The fixed version must be clean.** It has no flaw of Medium or higher apart from the trusted roles you listed, and the reviewer checks this.

## Checks before handing a case over

```
node tools/make-fixtures.mjs --case <id>
node tools/check-cases.mjs --role <role>        # must show check-ok for your case
node tools/ledger.mjs add <id> --author case-author
node tools/verify.mjs --regression              # stop-list, canary, hashes
```

If you edit a case after `ledger add`, run `make-fixtures` again, then `node tools/ledger.mjs rehash <id> --by case-author --note "<why>"`.

## Reviewer

The case reviewer reads `CASE.md`, `rubric.md` and both programs, then checks five things:

1. The attack scenario works against the vulnerable program as written.
2. The fixed program closes it.
3. The normal flow still works in the fixed program.
4. No second flaw exists in either program.
5. No hint leaks.

Record the verdict with `node tools/ledger.mjs review <id> pass|fail --by case-reviewer --note "<one line>"`. A fail goes back to the case author with the reason.
