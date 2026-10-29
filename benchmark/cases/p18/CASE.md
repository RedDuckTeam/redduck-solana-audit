---
id: p18
crate: treasury_council
pattern: commit-execution-split
class: exploit
group: g2
vulnerable: a
fixed: none
role: public
author: case-author
canary: RDSAF81C11F051B132899533
source_vulnerable: benchmark/public/programs/p18
---
# Case p18

## Attack scenario
A proposal stores a future instruction (program id, account metas, data). Members approve it, then `execute` runs the stored instruction signed by the treasury vault PDA. `execute` checks `approvals >= threshold`, performs the CPI, and sets `proposal.executed = true`. It never reads `proposal.executed` before running, so the flag records execution but does not prevent it.

1. The members create and approve a proposal that transfers tokens from the treasury (vault-owned token account) to some recipient. This reaches the threshold once, legitimately.
2. The admin (or, since the scenario is about re-use, whoever can sign `execute`) calls `execute`. The transfer runs and `executed` is set to `true`.
3. `execute` is called again on the **same** proposal account. `approvals` is still `>= threshold`, the `executed` flag is not checked, so the stored transfer runs a second time, moving tokens again.
4. Repeating this drains the treasury: a single-approval transfer proposal can be replayed as many times as the treasury has balance.

## Source
Template task folder `programs/06-proposal-anyone-can-execute` of https://github.com/RedDuckTeam/redduck-academy-solana-pro-template at commit `782fb2d28eb1ad47ebd6028b1df870706b1133c3`, the version given to students. Renamed the crate, `[lib] name` and `#[program]` module `open_multisig` to `treasury_council`, and removed the Cargo `description`. Replaced two doc comments: on `Execute` (src/instructions/execute.rs:7) and on `Proposal` (src/state/proposal.rs:16). The logic is unchanged and `declare_id!` is kept.

## Trusted roles
- **admin**: runs `initialize` (sets members and threshold) and, in this program, is the signer of `execute`. The admin is trusted to decide when an approved proposal runs, not to run it more than once.
- **members**: the keys in `config.members`. They may create proposals and approve them. A proposal needs `threshold` distinct member approvals.
- Proposers and executors otherwise are untrusted; approval by members is the only gate on what the treasury does.
