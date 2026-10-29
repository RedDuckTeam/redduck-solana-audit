# Smoke and gate log

Every `claude plugin eval` run made while building the skill. Budget: 200 agent runs in total (judge calls not counted).

| # | Date | Story | Purpose | Command | Agent runs | Cumulative |
|---|------|-------|---------|---------|-----------|-----------|
| 1 | 2026-10-28 | US-002 | Harness spike, plugin case spike-a, two arms | `claude plugin eval plugins/redduck-solana-audit --case spike-a --runs 1 --model sonnet --judge-model sonnet --no-publish --trust-plugin --allow-tools Write --scaffold --keep-temp --threshold 0 --json benchmark/results/spike-a.json` | 2 | 2 |
| 2 | 2026-10-28 | US-002 | Harness spike, stub plugin case spike-b, baseline only | `claude plugin eval benchmark/stub-plugin --case spike-b --ablation none --runs 1 --model sonnet --judge-model sonnet --no-publish --trust-plugin --allow-tools Write --scaffold --keep-temp --threshold 0 --json benchmark/results/spike-b.json` | 1 | 3 |
| 3 | 2026-10-28 | US-005 | Probe: can a run read skill references? c20b with plugin only | `claude plugin eval plugins/redduck-solana-audit --case c20b --ablation none --runs 1 --model claude-sonnet-5-5 --judge-model claude-sonnet-5-5 --no-publish --trust-plugin --allow-tools Write --scaffold --keep-temp --threshold 0 --json benchmark/results/probe-refs.json` | 1 | 4 |
| 4 | 2026-10-28 | US-005 | Probe 2 after making steps 4/7/8 mandatory: reference reads + verifier, c20b plugin only | `claude plugin eval plugins/redduck-solana-audit --case c20b --ablation none --runs 1 --model claude-sonnet-5-5 --judge-model claude-sonnet-5-5 --no-publish --trust-plugin --allow-tools Write --scaffold --keep-temp --threshold 0 --json benchmark/results/probe-refs2.json` | 1 | 5 |
| 5 | 2026-10-28 | US-005 | Probe 3: verifier escape hatch removed, pattern question citation, c20b plugin only | `claude plugin eval plugins/redduck-solana-audit --case c20b --ablation none --runs 1 --model claude-sonnet-5-5 --judge-model claude-sonnet-5-5 --no-publish --trust-plugin --allow-tools Write --scaffold --keep-temp --threshold 0 --json benchmark/results/probe-refs3.json` | 1 | 6 |
| 6 | 2026-10-28 | US-006 | G1 Sonnet, pilot dev cases, two arms x 3 runs | `claude plugin eval plugins/redduck-solana-audit --tag dev --runs 3 -j 2 --model claude-sonnet-5-5 --judge-model claude-sonnet-5-5 --no-publish --trust-plugin --allow-tools Write --scaffold --keep-temp --threshold 0 --json benchmark/results/g1-sonnet.json` | 36 | 42 |
| 7 | 2026-10-28 | US-006 | G1 Opus baseline only (stub plugin), pilot vulnerable cases x 3 runs | `claude plugin eval benchmark/stub-plugin --tag vuln --ablation none --runs 3 -j 2 --model claude-opus-5-5 --judge-model claude-sonnet-5-5 --no-publish --trust-plugin --allow-tools Write --scaffold --keep-temp --threshold 0 --json benchmark/results/g1-opus-baseline.json` | 9 | 51 |

## G1 result (2026-10-28) — FAILED on the recall rule, stopped for the user's decision

Pilot cases c19b, c20b, c24a (vulnerable) and c19a, c20a, c24b (fixed). Numbers from `node tools/metrics.mjs --parts g1-sonnet,g1-opus --verdicts benchmark/results/g1/arbiter-verdicts.json`.

| Measure | Sonnet with skill | Sonnet without | Opus without |
|---|---|---|---|
| Planted flaw found (case found in ≥2 of 3 runs) | 3/3 cases (c19b: 2 of 3 runs) | 3/3 cases (9/9 runs) | 3/3 cases (9/9 runs) |
| High/Critical candidates beyond the planted flaw, per run | 0.17 (3/18) | 1.00 (18/18) | 0.44 (4/9) |
| False positives per run (blind arbiter) | 0.00 | 0.33 | 0.00 |
| Noise (≥ Medium beyond planted), per run | 1.94 | 2.72 | 1.22 |
| Report format failures | 0% | 0% | 0% |
| Canary hits | 0 | 0 | 0 |
| Time per run | 48–97 s | 30–66 s | 57–66 s |

- **Rule:** found on ≥2 of 3 vulnerable cases — passed. Recall with > without — **failed** (100% = 100%, ceiling). FP with ≤ without — passed.
- **The arbiter judged 19 of 25 candidates valid.** The "fixed" pilot programs therefore contain real or design-level issues beyond the planted one, including c20, where `resolve` never checks the report time against `closes_at`. Centralization findings (an authority that can publish any root, a creator choosing their own reporter) were counted as valid under ARBITER.md.
- **Skill behaviour:** 29-row coverage table in 18/18 reports; verifier called in 16/18 reports.
| 8 | 2026-10-28 | US-007 | Ceiling probe part 1: baseline only (stub plugin), 11 reviewed own vulnerable cases, Sonnet and Opus x 1 run; executor sees counts only | `claude plugin eval benchmark/stub-plugin --tag vuln --ablation none --runs 1 -j 2 --model <claude-sonnet-5-5|claude-opus-5-5> --judge-model claude-sonnet-5-5 --no-publish --trust-plugin --allow-tools Write --scaffold --threshold 0 --json benchmark/results/probe1-<model>.json` | 22 | 73 |
| 9 | 2026-10-28 | US-007 | Ceiling probe part 2: baseline only (stub plugin), remaining 5 own vulnerable cases (c01 c03 c04 c06 c07), Sonnet and Opus x 1 run; counts only | `claude plugin eval benchmark/stub-plugin --tag vuln --ablation none --runs 1 -j 2 --model <claude-sonnet-5-5|claude-opus-5-5> --judge-model claude-sonnet-5-5 --no-publish --trust-plugin --allow-tools Write --scaffold --threshold 0 --json benchmark/results/probe2-<model>.json` | 10 | 83 |

## Ceiling probe result (2026-10-28) — stop rule triggered, stopped for the user's decision

The probe ran without the skill (stub plugin, `--ablation none`), one run per case, on all 16 reviewed own vulnerable cases. The executor saw counts only.

| Model | Found | dev | held-out |
|---|---|---|---|
| claude-sonnet-5-5 | 13 of 16 | 7/7 | 6/9 |
| claude-opus-5-5 | 16 of 16 | 7/7 | 9/9 |

- **Rule:** stop if the baseline finds ≥ 14 of 16 on either model. Opus at 16/16 triggers it. A recall gain of +15 pp is impossible for Opus, and for Sonnet the most the skill can add is +18.75 pp, if it finds all 3 cases the baseline missed.
- **Cumulative eval runs:** 83 of 200.
| 10 | 2026-10-28 | US-008 | Smoke of the finished skill (18 patterns): dev cases with plugin only, 1 run (FP-regression check vs G1) | `claude plugin eval plugins/redduck-solana-audit --tag dev --ablation none --runs 1 -j 2 --model claude-sonnet-5-5 --judge-model claude-sonnet-5-5 --no-publish --trust-plugin --allow-tools Write --scaffold --keep-temp --threshold 0 --json benchmark/results/smoke-final.json` | 14 | 97 |
| 11 | 2026-10-28 | US-008 | Smoke: does requiring "Pattern files read" in Scope make the skill open pattern files? 4 dev cases, plugin only, 1 run each | `claude plugin eval plugins/redduck-solana-audit --case <c03b, c09b, c12a or c20b> --ablation none --runs 1 --model claude-sonnet-5-5 --judge-model claude-sonnet-5-5 --no-publish --trust-plugin --allow-tools Write --scaffold --keep-temp --threshold 0 --json benchmark/results/smoke-reads-<case>.json` | 4 | 101 |

## Final-skill smokes (2026-10-28)

**Row 10.** Finished skill with all 18 pattern files, the 7 dev pairs, Claude Sonnet 5.5, plugin only, 1 run each:
- found the planted flaw on 7 of 7 vulnerable sides;
- High/Critical findings beyond the planted flaw: 1 in 14 runs (0.07 per run, against 0.17 with the skill in G1), so there is no false-positive regression;
- 2.00 findings at Medium or above per run;
- 0 format failures, after fixing a parser bug that dropped underscores from file names;
- the verifier was called in every run that had a High or Critical finding.

**Row 11.** 4 dev cases: found 4 of 4.

**Known limitation found by both smokes:** in 18 of 18 runs Sonnet opened no `references/patterns/*.md` file. It worked from the one-line rules in SKILL.md and read only `report-format.md`. When asked to, it reports this honestly in the Scope line "Pattern files read: none opened".
