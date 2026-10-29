# Draft: PR to solana-foundation/solana-dev-skill

This repository already links third-party security resources in `references/resources.md`. Propose one link there. Do not propose copying content. Open the PR after the benchmark results are published.

**Title:** resources: add redduck-solana-audit (architecture-level audit skill)

**Change** in `references/resources.md`, in the security section next to the existing audit resources:

```markdown
- [redduck-solana-audit](https://github.com/RedDuckTeam/redduck-solana-audit): an audit skill that reviews programs against 18 design patterns (root of trust, untrusted executor, one-way state, oracle defensive computation…) plus a classic-bug checklist; ships a reproducible with/without eval suite (16 isolated program pairs + 6 public cases).
```

**PR body:**

> Adds a pointer to an MIT-licensed audit skill that complements the security guidance here at the design level. It has no dependencies, hooks or MCP servers, and is Markdown only. Preliminary results and limits: README, section Benchmark.
