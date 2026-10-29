# Draft: PR to trailofbits/skills-curated

Trail of Bits reads every line of submitted hooks and scripts. This skill has none in its skill folder. The plugin's `evals/*/stage.sh` scripts only copy fixtures and run only with `--scaffold`.

**Title:** Add auditing-solana-programs (RedDuck): design-level Solana audit skill

**PR body:**

> Submitting one skill: `plugins/redduck-solana-audit/skills/auditing-solana-programs` from https://github.com/RedDuckTeam/redduck-solana-audit (MIT, v0.1.0).
>
> - **What it adds over existing Solana scanners:** a root-of-trust pass on every instruction, and routing by attack surface into 18 design patterns: liveness through untrusted executors, ordering and replay through one-way state, oracle handling through defensive computation and fail-fast, and dependency exits. An 11-item checklist covers the classic bug classes.
> - **Contents:** Markdown only, with no scripts, hooks or network calls. An optional verifier agent ships in the plugin, and the skill falls back to an in-session verification procedure without it.
> - **Evidence:** a reproducible with/without suite for `claude plugin eval`: 16 program pairs written by isolated authors (9 held out, 4 of them fully clean) plus 6 public cases.
>   - Preliminary numbers: plain Sonnet and Opus already find most single planted flaws. In the pilot, the skill reduced extra High/Critical findings, partly by capping trusted-role risks at Medium, and blind-judged false positives.
>   - Limits, stated in the README: detection only, programs not executed, full run not yet made.
