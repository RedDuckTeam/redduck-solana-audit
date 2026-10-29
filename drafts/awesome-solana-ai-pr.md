# Draft: PR to solana-foundation/awesome-solana-ai

Open this PR **after** the repository is public and tagged `v0.1.0`.

**Branch:** `add-redduck-solana-audit`

**Title:** Add redduck-solana-audit (architecture-level Solana audit skill)

**Change:** one line in the **Skills** section, in the list's `[name](link) - one sentence` format:

```markdown
- [redduck-solana-audit](https://github.com/RedDuckTeam/redduck-solana-audit) - Agent Skill that audits Anchor, native and Pinocchio programs against 18 design patterns from the top Solana protocols, with a verifier agent and a reproducible with/without eval suite.
```

**PR body:**

> Adds redduck-solana-audit, an MIT-licensed Agent Skill and Claude Code plugin for security review of Solana programs. Unlike signature-only checklists, it routes each program through 18 named design patterns (root of trust, untrusted executor, one-way state, defensive computation and others), drawn from RedDuck Academy's study of the top 50 Solana protocols, plus an 11-item checklist of classic bug classes.
>
> It ships a reproducible eval suite: 16 flawed/fixed program pairs written by authors isolated from the skill, plus 6 public cases, runnable with `claude plugin eval` with and without the skill.
> Preliminary numbers (README, Benchmark): frontier models already find single planted flaws on their own. In the pilot, the skill cut High/Critical findings beyond the planted flaw from 1.00 to 0.17 per run, and blind-judged false positives from 0.33 to 0.00. Part of that drop is severity calibration: the skill caps trusted-role risks at Medium.
>
> Maintained by RedDuck. There is no token and nothing to promote beyond the open-source skill.
