# Changelog

Every change to the plugin, however small, raises `version` in `plugins/redduck-solana-audit/.claude-plugin/plugin.json`. Claude Code compares versions to decide whether installed copies need an update, so a change shipped without a version bump never reaches existing users.

Versions follow semantic versioning:

- **Patch:** wording fixes and false-positive notes.
- **Minor:** new patterns, checklist items or workflow steps.
- **Major:** changes to the report format that break tools reading it.

## 0.2.0

Material from the RedDuck Academy course Development on Solana, folded into the existing skill. The coverage table keeps its 29 rows.

- Checklist items extended:
  - `init-frontrun`: `init_if_needed` reinitialization, manual init paths, ATA squatting.
  - `closing`: lamport withdrawals that ignore the rent reserve and implicitly close an account.
  - `token-2022`: mint and freeze authorities on classic and Token-2022 mints, share mint authority, one claim per deposit for receipt tokens, push payouts, UI amounts.
  - `realloc`: account layout changes across program upgrades.
  - `signers-bumps-cpi`: `has_one` without `Signer`, type confusion in native code, PDA signatures forwarded to external programs.
- Patterns extended:
  - `commit-execution-split`: Jito bundles, leader windows, VRF callback authentication and liveness.
  - `complexity-asymmetry`: weighted-draw ticket ranges, merkle claim namespaces and leaf encoding.
  - `fail-fast`: Pyth feed identity by model, unverified `/// CHECK` comments, legacy feeds, threshold reference points.
  - `defensive-computation`: exponent casts, cross rates.
  - `root-of-trust`: NFT collection gates and NFT-held positions.
  - `lazy-loading`: lock start on top-ups.
- New reference `archetypes/amm.md`: constant-product and concentrated-liquidity invariants, routed from step 4.
- Seven new misreports: rent-exempt rejection, the 200,000 CU default, CPI privilege escalation, `init_if_needed` pre-creation, forwarded `UncheckedAccount`, mint authority, missing events. The overflow entry now depends on the workspace-root profile, here and in the verifier agent. Misreports are renumbered, so cite them by title.
- `exploit-tests.md`: what a test may forge, moving the clock, reading why a transaction failed, compute-limit findings.
- Step 1 records whether the program is already deployed.

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
