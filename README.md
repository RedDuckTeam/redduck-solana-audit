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

**Baseline checklist:** arithmetic and casts, duplicate accounts, `unsafe` and panics, reloading after CPI, Token-2022 extensions and mint authorities, closing accounts and rent, seed collisions, initialization front-running and `init_if_needed`, `realloc` and layout migration, authority transfer, signers, bumps and CPI targets. Checklist details, the AMM and CLMM invariants and several misreports come from the academy's Development on Solana course.

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

## Limits

This skill helps a reviewer. It does not replace an audit. It can miss flaws and it can report flaws that are not there, and every finding carries the evidence for you to check.

Known limitation in v0.1: in our test runs, Claude Sonnet relied on the one-line pattern rules inside the skill and did not open the per-pattern reference files, even when instructed to. The report's Scope section states which pattern files were read, so you can see when this happens. Making the deeper checks reach the model reliably is planned for the next version.

## License

MIT. See [LICENSE](LICENSE).
