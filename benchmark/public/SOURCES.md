# Public case sources

The public cases are sanitized copies of the RedDuck Academy Solana PRO homework
programs (RedDuck-owned, MIT), taken as given to students. Each program lives under
`benchmark/public/programs/<id>/` and is referenced by its case's `source_vulnerable`.

- Template repository: https://github.com/RedDuckTeam/redduck-academy-solana-pro-template
- Template commit: `782fb2d28eb1ad47ebd6028b1df870706b1133c3` (see `vendor/solana-pro-template/COMMIT`)

| Public id | Template task folder | Commit | Renamed |
|-----------|----------------------|--------|---------|
| p08 | `programs/02-find-the-root-of-trust` | `782fb2d28eb1ad47ebd6028b1df870706b1133c3` | crate / `[lib] name` / `#[program]` `oracle_vault` → `payout_vault`; helper `load_instruction_unchecked` → `load_instruction`; dropped Cargo `description`; neutralized 2 narrating comments |
| p06 | `programs/03-run-it-without-the-operator` | `782fb2d28eb1ad47ebd6028b1df870706b1133c3` | crate / `[lib] name` / `#[program]` `operator_free` → `reward_market`; dropped Cargo `description`; neutralized 5 narrating comments |
| p20 | `programs/04-rewards-without-a-loop` | `782fb2d28eb1ad47ebd6028b1df870706b1133c3` | crate / `[lib] name` / `#[program]` `lazy_pool` → `stake_rewards`; dropped Cargo `description`; neutralized 1 narrating comment |
| p18 | `programs/06-proposal-anyone-can-execute` | `782fb2d28eb1ad47ebd6028b1df870706b1133c3` | crate / `[lib] name` / `#[program]` `open_multisig` → `treasury_council`; dropped Cargo `description`; neutralized 2 narrating comments |
| p03 | `programs/07-claims-in-any-order` | `782fb2d28eb1ad47ebd6028b1df870706b1133c3` | crate / `[lib] name` / `#[program]` `total_claims` → `device_rewards`; dropped Cargo `description`; neutralized 2 narrating comments |
| p16 | `programs/09-write-the-exit` | `782fb2d28eb1ad47ebd6028b1df870706b1133c3` | crate / `[lib] name` / `#[program]` `round_exit` → `prize_rounds`; dropped Cargo `description` |

In every case only the crate name, `[lib] name`, `#[program]` module name (and, for p08,
one helper function name) were changed, the Cargo `description` was removed, and comments
that narrated the flaw or named a course pattern were replaced with neutral API wording.
No program logic was changed, and each `declare_id!` is kept.

## Notes

- Only the versions given to students are used. The homework solutions (the fixed
  versions) are not published, so every public case is a single vulnerable program with
  `fixed: none`.
- `TASK.md` and `SOLUTION.md` were never copied: only `src/`, `Cargo.toml` and `Xargo.toml` were taken. The template repository is MIT-licensed (its `LICENSE`, "Copyright (c) 2026 RedDuck Limited").
- `08-value-the-position` is excluded because its instruction handlers are `todo!()`
  stubs: there is no implemented logic for an auditor to review.
- `05-orderbook-with-local-checks` is excluded: as given, the program is a correct
  implementation whose only shortcoming is pedagogical (it walks the order list instead of
  using the supplied hint). The list is a fixed 64-cell array, so the walk is bounded and
  cheap, and no concrete security or design consequence of at least Medium severity follows
  from the code as written.
- sealevel-attacks (coral-xyz) was considered as an additional public source but excluded:
  its repository has no license file, so its code cannot be redistributed here, and its
  programs are 10–30-line examples that are already widely published.
