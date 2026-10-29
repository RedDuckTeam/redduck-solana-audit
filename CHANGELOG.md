# Changelog

Every change to the plugin, however small, raises `version` in `plugins/redduck-solana-audit/.claude-plugin/plugin.json`. Claude Code compares versions to decide whether installed copies need an update, so a change shipped without a version bump never reaches existing users.

Versions follow semantic versioning:

- **Patch:** wording fixes and false-positive notes.
- **Minor:** new patterns, checklist items or workflow steps.
- **Major:** changes to the report format that break tools reading it.

## 0.1.0

First release.

- Skill `auditing-solana-programs`:
  - an 8-step workflow;
  - attack-surface routing to 18 design patterns;
  - an 11-item checklist of classic Solana bug classes;
  - a misreport list;
  - the report format with a coverage table.
- Agent `finding-verifier`, which tries to refute each High or Critical finding. When the agent is unavailable, the skill falls back to the same procedure in-session.
- Benchmark tooling and cases under `benchmark/` (not shipped with the plugin): 16 own program pairs, 6 public cases, preliminary results in README.

Known limitations:
- In test runs, Claude Sonnet did not open the per-pattern reference files and worked from the in-skill summaries. The report's Scope line shows when this happens.
- No case covers complexity asymmetry, virtual execution, boundary of consent, fail-fast, or the checklist items for reloading after CPI, closing, Token-2022 and `unsafe` code.
- The full with/without benchmark was not run.
