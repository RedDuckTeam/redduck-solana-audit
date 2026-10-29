# Rules for the false-positive arbiter

The arbiter decides whether each High or Critical finding the auditors reported is a real issue in the program. It works blind: candidates arrive without the arm (with or without the skill) and without the model, in a shuffled order. It sees only the three lines of each finding and the program source.

## Input

`benchmark/results/<gate>/arbiter-input.json` is a list of candidates:

```json
{ "cid": "3f9a1c2b7d4e", "evalId": "c20a", "program": "benchmark/fixtures/c20a/program",
  "severity": "high", "location": "src/lib.rs:148", "mechanism": "..." }
```

Do not open the matching `.key.json`, `runs.json`, the reports, `CASE.md`, `rubric.md` or anything under `plugins/`. The verdict must come from the program alone.

## Verdict

For each `cid`, read the program at `program` and decide:

- **`valid`:** the mechanism describes a real flaw in this program that an ordinary user, or the party named in the finding, can trigger, with an impact of at least Medium. A design finding (liveness, centralization) is valid when the dependency it names really exists in the code or in `ARCHITECTURE.md`, and the consequence it claims really follows.
- **`false`:** a constraint or check in the program blocks the scenario, the claim contradicts Solana runtime behaviour (atomic transactions, a failing CPI aborts the transaction, no reentrancy through other programs, owner-only writes), the impact is negligible, or the "issue" is the program working as designed with nothing at stake.

**Trusted roles.** Some findings only work if a role the program deliberately trusts misuses its power: the admin, an authority, an oracle or root signer, or a reporter that stakers can see before they commit. Such a finding is `false` as a High or Critical claim. It is `valid` only when the role can do more than its stated purpose, or when an untrusted party can obtain the role. If an ordinary user can profit while the role behaves honestly, the trusted-role rule does not apply: judge the finding on its impact. The same rule is in the skill's report format (`Trusted roles`), so a skill-guided report and a plain report are judged alike.

Judge each finding on its own text. Two findings that describe the same flaw must get the same verdict.

## Output

Write `benchmark/results/<gate>/arbiter-verdicts.json` as `{ "<cid>": "valid" | "false" }`, and `arbiter-notes.md` with one line per `cid`: the verdict and the `file:line` that decided it.

## Spot-check

Pick 10% of the `found` grader verdicts with `node -e` using the seed recorded in `RESULTS.md`, read the report and the rubric, and note agreement or disagreement in `arbiter-notes.md`. This is the one place where the arbiter reads rubrics, so do it after the false-positive verdicts are written.
