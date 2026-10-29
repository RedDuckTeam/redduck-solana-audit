# Security

## What the plugin runs

The installed plugin is the folder `plugins/redduck-solana-audit/`.

- **Skill and agent.** `skills/` and `agents/` are Markdown instructions only. They contain no executable code, no hooks, no MCP servers, and make no network calls. The skill may ask Claude to run Solana build and test commands (`anchor build`, `cargo build-sbf`, `cargo test`) when you have a toolchain and an exploit test is worth writing. Claude Code asks for your permission as usual.
- **Eval cases.** `evals/` holds the benchmark's eval cases: prompts, graders, and one `stage.sh` per case. Each script copies that case's program from `benchmark/fixtures/` into the eval's temporary workspace. The scripts run only when you pass `--scaffold` to `claude plugin eval` yourself. Outside a clone of this repository they exit with an error.

## What is not shipped

`benchmark/`, `tools/`, `examples/` and `drafts/` stay in the repository. `tools/` contains Node scripts used to build and score the benchmark. Nothing in the plugin calls them.

## Reporting a problem

Report a security problem in the plugin itself, such as an instruction that could make Claude do something harmful, through a private GitHub security advisory on this repository, not a public issue. Findings that the skill produces about your own programs are yours, and are not issues in this repository.

## Limits

The skill helps a reviewer, and it does not replace an audit. It can miss flaws and it can report flaws that are not there. Every finding carries the evidence for you to check.
