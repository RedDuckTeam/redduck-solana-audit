# Running the benchmark

This file explains what the benchmark measures, how its harness behaves, and how to run the full with/without comparison yourself. v0.1 was released with preliminary numbers only (README, "Benchmark"). The full run was not made, because the ceiling probe showed too little headroom (SMOKE-LOG.md).

## What the benchmark measures

- **Detection only.** Each run audits one program with read-only tools plus `Write` for the report. No Bash is granted, so nothing is built or executed during a run.
- **Programs are not built or executed at all.** Each pair is checked statically: `cargo check` and an independent case reviewer.
- **Two arms per case:** with the plugin, and with no plugin (`claude plugin eval` ablation).

## Harness facts

Checked on 2026-10-28 with Claude Code 2.1.312 on Windows 11 (`claude --version`). Runs: rows 1–2 of `SMOKE-LOG.md`.

**(a) Staging a program with `stage.sh`.** A per-case `stage.sh`, run with `--scaffold`, copies its own program into `./program`.
- It runs under Git Bash with `pwd=/tmp/claude-eval-<rand>/home/cwd`.
- `BASH_SOURCE` resolves to the real case directory, for example `/e/work/RedDuck-Software/redduck-solana-audit/plugins/redduck-solana-audit/evals/spike-a`. This means the script can find `benchmark/fixtures/<evalId>` by walking up from its own folder (`tools/templates/stage.sh`).
- Both spike cases got their own program. spike-b staged `// spike-b marker program` from the stub plugin folder.
- The docs say the script gets no `EVAL_*` variables, so the eval id is written into each script.

**(b) Case and grader keys that work.**

`case.yaml`:
```yaml
schema_version: "1.1"
name: <evalId>
tags: [...]
context:
  scaffold_script: stage.sh
execution:
  max_turns: <n>
  timeout_seconds: <n>
  allowed_tools: [Read, Glob, Grep, Skill, Agent, Write]
```

- `prompt.md` with no frontmatter supplies the prompt.
- An `llm` grader pointed at the report uses `focus: { source: file, path: solana-audit-report.md }`. Its `evidence` field in `--json` shows the report text.

**(c) Canary grader.** A `regex` grader with `target: trace`, `pattern: <canary>`, `match: not_contains`, `arm: both` passes when the canary is absent ("pattern absent as expected"). The canary is plain alphanumeric, so JSON escaping in the trace cannot break the match.

**(d) The verifier subagent.**
- In the with-plugin arm, `Agent` with `subagent_type: "redduck-solana-audit:finding-verifier"` worked.
- In the no-plugin arm it fails with `Agent type 'redduck-solana-audit:finding-verifier' not found`, as expected, since the skill is absent there too.
- `Agent` only needs to be listed in `allowed_tools`. `Write` also needs `--allow-tools Write`.

**(e) Cost per run.** On a 30-line program with Sonnet:

| Arm | Turns | Time | Cost (list price) |
|---|---|---|---|
| With plugin | 9 | 35 s | $0.089 |
| Without plugin | 8 | 41 s | $0.060 |
| Stub baseline | — | 29 s | $0.055 |

- A judge call costs about $0.005.
- Real cases (100–300 lines) will take longer. The G1 measurements will replace these figures.
- On a subscription the runs count against plan limits, not dollars.

**(f) Baseline-only runs.** `claude plugin eval benchmark/stub-plugin --ablation none --no-publish …` runs a single arm with an empty plugin, which is a no-skill run. `--ablation` accepts only `none` and `with-without`.

**(g) Reading outside the workspace is blocked.** Asked to read `benchmark/spike-bait.txt` by absolute path, both arms got `Permission to use Read has been denied because Claude Code is running in don't ask mode`, and the canary grader passed. The docs also say a run cannot read the eval directory. The canary stays in every answer file anyway, as defence in depth.

**(h) Finding each run's report.** Under `--json <file>`, every run entry `cases[].arms.{with,without}[]` carries `tracePath` = `<keptDir>\out\trace.jsonl`.
- The kept directory is `dirname(dirname(tracePath))`.
- The report is `<keptDir>/home/cwd/solana-audit-report.md`.
- `tools/collect-reports.mjs` uses this mapping.
- On Windows `--keep-temp` warns that it cannot seal the directory. Read only the report from it, and delete the directory after collecting.

**Model IDs.** The `sonnet` alias resolved to `claude-sonnet-5-5` (from the run trace). Full runs pin `--model claude-sonnet-5-5` / `--model claude-opus-5-5` and `--judge-model claude-sonnet-5-5`.

## Environment probes

- **Native `cargo check` works.** `cargo check --locked` of the template program `02-find-the-root-of-trust` (Anchor 0.31.1, `pyth-solana-receiver-sdk`), with the host toolchain `stable-x86_64-pc-windows-gnu` and Rust 1.97.1, finished in 40 s with warnings only. Pairs are therefore compile-checked with `tools/check-cases.mjs`. The target dir is kept outside the repository.
- **Exploit-test stack.** The solana-bankrun README is titled "Bankrun (deprecated)" and says "DEPRECATED: use LiteSVM instead" (https://github.com/kevinheavey/solana-bankrun, read 2026-10-28). `references/exploit-tests.md` therefore recommends LiteSVM (Rust crate `litesvm`, Node bindings under https://github.com/LiteSVM/litesvm/tree/master/crates/node-litesvm) for new tests, and bankrun only where a repository already uses it. Benchmark pairs carry written attack scenarios instead of executable tests, by the user's decision.

## Reproducing the full comparison

Run everything from a clone of the repository root. Each `claude plugin eval` run is a real model session and counts against your plan limits.

1. Generate the eval cases for every role (own dev, held-out and public):
   ```bash
   node tools/make-evals.mjs --role dev --role heldout --role public
   ```
2. Run each model separately. `-j 2` halves the wall time and shares one rate limit. Every command keeps the HTML report local:
   ```bash
   claude plugin eval plugins/redduck-solana-audit --runs 3 -j 2 --model claude-sonnet-5-5 --judge-model claude-sonnet-5-5 --no-publish --trust-plugin --allow-tools Write --scaffold --keep-temp --threshold 0 --json benchmark/results/full-sonnet.json
   claude plugin eval plugins/redduck-solana-audit --runs 3 -j 2 --model claude-opus-5-5 --judge-model claude-sonnet-5-5 --no-publish --trust-plugin --allow-tools Write --scaffold --keep-temp --threshold 0 --json benchmark/results/full-opus.json
   ```
   To split the work across days, add `--tag g1`, `--tag g2` or `--tag g3`.
3. Collect the reports and build the false-positive candidate list:
   ```bash
   node tools/collect-reports.mjs --json benchmark/results/full-sonnet.json --part full-sonnet --model claude-sonnet-5-5
   node tools/collect-reports.mjs --json benchmark/results/full-opus.json --part full-opus --model claude-opus-5-5
   mkdir -p benchmark/results/full
   node tools/metrics.mjs --parts full-sonnet,full-opus --arbiter-input benchmark/results/full/arbiter-input.json
   ```
4. Have a blind arbiter judge the candidates by `benchmark/ARBITER.md`. That can be a person or a separate model session that never sees `arbiter-input.key.json`. Then compute the results:
   ```bash
   node tools/metrics.mjs --parts full-sonnet,full-opus --verdicts benchmark/results/full/arbiter-verdicts.json --out benchmark/results/full/metrics.json
   ```

**Size:** 32 own eval cases (16 pairs) plus 6 public cases, × 2 arms × 3 runs = 228 agent runs per model. G1 took about 30–100 seconds per run.

**Held-out groups:** `metrics.mjs` reports the held-out cases in two groups.
- **Clean:** c08, c10, c11, c13.
- **Leaked:** c01, c02, c04, c05, c07. The case author's report gave the skill's author a one-line summary of each planted flaw, recorded as `LEAK` in `LEDGER.md`.

**What to retry:**
- Re-run runs that ended on an API error or a usage limit (`error` set in the JSON).
- Re-run runs whose canary grader failed.
- Do not re-run runs whose report did not parse. `metrics.mjs` counts them as format failures.

## Hand-off: what is left for a person

These steps follow the manual checklist in the project spec. Items 1, 5 and 6 (toolchain, full run, re-runs) were cancelled by decision.

2. Confirm inside the team that the course-derived text ships under MIT. Course authors: Ilya Kubariev, Mark, MrJeleika, borseno, nikita-kryvoruk-rd.
3. Review the case pairs in `benchmark/cases/` and `benchmark/workspaces/`: the dev cases, plus a sample of held-out ones.
4. Approve the names or change them before publishing: plugin `redduck-solana-audit`, skill `auditing-solana-programs`, marketplace `redduck`.
- Add a square PNG icon, 512–2048 px and under 2 MB, at `plugins/redduck-solana-audit/.claude-plugin/icon.png`. The directory validator asks for it, and the first icon you submit becomes the listing icon permanently.
7. Create the public repository `RedDuckTeam/redduck-solana-audit`, then `git init`, make the first commit and push. `.gitignore` already excludes vendor copies, logs and traces.
8. Tag `v0.1.0` and create a GitHub release from `CHANGELOG.md`.
9. Check that the install works:
   - Claude Code: `claude plugin install redduck-solana-audit --marketplace RedDuckTeam/redduck-solana-audit`
   - skills.sh: `npx skills add RedDuckTeam/redduck-solana-audit`, which also tests the nested `plugins/*/skills/` path
   - Codex and Cursor: copy the skill folder into each and run one audit.

   Update the "Where it has been checked" table in README and bump the version if anything changes.
10. PR to `solana-foundation/awesome-solana-ai`. Text: `drafts/awesome-solana-ai-pr.md`.
11. PR to `solana-foundation/solana-dev-skill`. Text: `drafts/solana-dev-skill-pr.md`.
12. Submit to the Anthropic directory at claude.ai/directory/manage. Text: `drafts/anthropic-directory-submission.md`. Run `claude plugin validate --strict --profile directory plugins/redduck-solana-audit` first.
13. PR to `trailofbits/skills-curated`. Text: `drafts/tob-skills-curated-pr.md`.
14. 14 or more days after the first commit, submit to the awesome lists. Text: `drafts/awesome-lists.md`.
15. Optional: Cursor Marketplace.
16. After the first installs, check the skills.sh page and its security badges.

Optional for v0.2:
- build and execute the case programs;
- run the full comparison with "Reproducing the full comparison" above;
- add cases for the 8 uncovered patterns and checklist items;
- address the pattern-file limitation noted in README.
