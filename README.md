# redduck-solana-audit

An Agent Skill that reviews Solana programs (Anchor, native `solana-program`, Pinocchio) at the level of architecture, not only line by line.

Most Solana audit checklists list bug signatures: missing signer check, missing owner check, non-canonical bump. This skill starts one level higher. It asks whether the design holds up when the caller, the operator, the oracle or a dependency behaves in the worst way it can. The answers come from **18 design patterns** that RedDuck Academy distilled from the top 50 Solana protocols, taught in the [Solana PRO course](https://academy.redduck.io). A compact checklist covers the classic bug classes the patterns do not.

It complements the signature-based scanners rather than replacing them.

## What it checks

| Group | Patterns |
|---|---|
| Trust and identity | root of trust, identity rejection |
| Permissionless execution | untrusted executor, role inversion |
| Cost distribution | lazy loading, pay-as-you-act |
| State partitioning | divide and conquer, complexity asymmetry |
| Deferred execution | commit-execution split, action as data, virtual execution |
| Order and consent | one-way state, boundary of consent |
| Outside data | internal oracle, defensive computation, fail-fast |
| Dependencies and exits | execution layer shift, dependency independency |

**Baseline checklist:** arithmetic and casts, duplicate accounts, `unsafe` and panics, reloading after CPI, Token-2022 extensions, closing accounts, seed collisions, initialization front-running, `realloc`, authority transfer, signers, bumps and CPI targets.

## How it works

1. **Map the program.** List every instruction and every account, and note who can create each account.
2. **Trace the root of trust.** For every instruction, follow each check to the point where it ends.
3. **Route by attack surface.** Oracle reads, operator-only actions, loops, signed messages and the like decide which patterns to apply in depth.
4. **Run the baseline checklist.**
5. **Verify.** Each High or Critical finding goes to a verifier subagent that tries to refute it. When a Solana toolchain is available, an exploit test is written too.
6. **Report.** Write `solana-audit-report.md`: findings with `Severity` / `Location` / `Mechanism`, a separate section for design findings (liveness, centralization, scalability), and a coverage table that gives every pattern a verdict.

See [examples/sample-report.md](examples/sample-report.md) for the output.

## Install

**Claude Code** (plugin, includes the verifier agent):

```
claude plugin install redduck-solana-audit --marketplace RedDuckTeam/redduck-solana-audit
```

Or, from inside a session:

```
/plugin marketplace add RedDuckTeam/redduck-solana-audit
/plugin install redduck-solana-audit@redduck
```

**skills.sh** (skill only):

```
npx skills add RedDuckTeam/redduck-solana-audit
```

**Other agents that support Agent Skills** (Codex, Cursor and others): copy `plugins/redduck-solana-audit/skills/auditing-solana-programs/` into the agent's skills folder. Without the plugin's agent, the skill runs the verification step in the same session.

## Use

Ask for an audit in plain words, for example "Audit the program in programs/vault for security issues". You can also invoke the skill directly with `/redduck-solana-audit:auditing-solana-programs`.

## Where it has been checked

| Agent | Status |
|---|---|
| Claude Code (plugin) | Preliminary benchmark: see below |
| Claude Code, exploit-test path with a Solana toolchain | not yet verified |
| skills.sh install (nested `plugins/*/skills/` path) | not yet verified |
| Codex | not yet verified |
| Cursor | not yet verified |

## Benchmark

**Status: preliminary.** The full with/without run was not made for v0.1. Below are the two measurements that were made while building it. The complete suite is included so anyone can run the full comparison: see [benchmark/RUNNING.md](benchmark/RUNNING.md).

**What we found:** current frontier models already find a single planted flaw in a small program on their own. In our cases, what the skill measurably changed was how many findings were rated High or Critical, not detection. Part of that drop is severity calibration rather than removed noise; see the note under G1.

**Pilot gate G1.** The skill version used here had the workflow and 3 of the 18 pattern files. The test covered 3 program pairs (one flawed version, one fixed version each) with Claude Sonnet 5.5 and 3 runs per arm.

| | With skill | Without skill |
|---|---|---|
| Planted flaw found | 3 of 3 | 3 of 3 |
| High/Critical findings beyond the planted flaw, per run | 0.17 | 1.00 |
| False positives per run (blind arbiter) | 0.00 | 0.33 |
| Findings at Medium or above beyond the planted flaw, per run | 1.94 | 2.72 |

Claude Opus 5.5 without the skill also found 3 of 3.

**How to read the drop.** A blind arbiter judged 12 of the 18 extra High/Critical findings made without the skill to be real issues. This used the arbiter rules of the time, before the trusted-role rule was added.
- Most of the 12 were centralization risks, which the skill caps at Medium by design.
- One concerned behaviour that the case author documents as intended.
- One was a real second flaw in a pilot program. The skill reported it only at Medium or Low, which is why the trusted-role cap now applies only when the harm needs the role to misbehave.

c20 has since been fixed.

**Ceiling probe.** This was a run without the skill only, once per case, on the 16 vulnerable programs of the own case set. Claude Sonnet 5.5 found 13 of 16 and Claude Opus 5.5 found 16 of 16. Opus left no room for a recall gain on this set. For Sonnet, the skill could have added at most the 3 missed cases, +18.75 points. The full run was not made.

The case set:
- **16 own pairs.** Each pair is a program with one planted flaw and its fixed twin, covering 14 patterns and 2 checklist items (arithmetic, duplicate accounts). Most programs have 300–600 lines across several files, and the flaw sits in how instructions interact. Separate authors who never saw the skill wrote them, and an independent reviewer checked each pair. 9 pairs are held out. For 5 of them (c01, c02, c04, c05, c07), a one-line summary of the planted flaw reached the skill's author through an author's report (recorded in `benchmark/LEDGER.md`). Only c08, c10, c11 and c13 are fully clean, and results must report the two groups separately.
- **6 public cases,** from the RedDuck Academy Solana PRO homework. They are reported separately because models may have seen them.
- **Not covered by a case:** complexity asymmetry, virtual execution, boundary of consent, fail-fast, and the checklist items for reloading after CPI, closing accounts, Token-2022, and `unsafe` code.

Metrics:
- **Recall:** the share of planted flaws found in at least 2 of 3 runs.
- **False positives:** High or Critical findings that a blind arbiter judged wrong. They are counted on both the vulnerable and the fixed versions.

Read the numbers with these limits in mind:
- **Small samples.** G1 covered 3 pairs. Treat it as a direction, not a result.
- **Detection only.** Runs read the code and write a report; they cannot build or execute anything.
- **Programs were not built or executed.** Each pair was checked with `cargo check` and by the reviewer. An exploit was not run.
- **The exploit-test step of the skill is not covered by the benchmark.**
- **The eval cases in `evals/` run only from a clone of this repository**, because the programs they stage live in `benchmark/`.

Method, cases and raw results: [benchmark/](benchmark/).

## Limits

This skill helps a reviewer. It does not replace an audit. It can miss flaws and it can report flaws that are not there, and every finding carries the evidence for you to check.

Known limitation in v0.1: in our test runs, Claude Sonnet relied on the one-line pattern rules inside the skill and did not open the per-pattern reference files, even when instructed to. The report's Scope section states which pattern files were read, so you can see when this happens. Making the deeper checks reach the model reliably is planned for the next version.

## License

MIT. See [LICENSE](LICENSE).
