# redduck-solana-audit

A security review skill for Solana programs written in Rust: Anchor, native `solana-program` and Pinocchio. It audits at the level of architecture, using 18 design patterns that RedDuck Academy distilled from the top Solana protocols, plus a compact checklist of classic Solana bug classes. It writes `solana-audit-report.md` with findings, design findings and a coverage table. A verifier agent tries to refute each serious finding before it is reported.

Install:

```
claude plugin install redduck-solana-audit --marketplace RedDuckTeam/redduck-solana-audit
```

Use: ask Claude to audit a Solana or Anchor program, or run `/redduck-solana-audit:auditing-solana-programs`.

The method, the benchmark against plain Claude and its limits are documented at https://github.com/RedDuckTeam/redduck-solana-audit. This skill does not replace an audit.
