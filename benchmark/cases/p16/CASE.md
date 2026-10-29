---
id: p16
crate: prize_rounds
pattern: dependency-independency
class: design
group: g3
vulnerable: a
fixed: none
role: public
author: case-author
canary: RDSAF81C11F051B132899533
source_vulnerable: benchmark/public/programs/p16
---
# Case p16

## Attack scenario
This is a design flaw: players' funds and a core instruction each hang on a single external dependency with no in-program fallback.

1. **No path out of a round but `settle`.** Players move tokens into the vault with `join` / `join_with_swap`, which credit a `Round` pot. The only instruction that moves tokens back out is `settle`, which requires `config.executor` to sign (`has_one = executor`) and pays the whole pot to one winner's entry. There is no refund or withdraw instruction. If the executor never settles a round (the off-chain settler stops, or the executor key is lost or compromised), every stake joined to that round stays in the vault permanently; players cannot recover their own funds without the executor.
2. **Hardwired swap venue.** `join_with_swap` can only CPI into `DEX_PROGRAM_ID`, a `pubkey!` constant in `src/constants.rs`, enforced by `#[account(address = DEX_PROGRAM_ID)]`. There is no instruction to change it. If that venue is deprecated, halted, or compromised, paying with another token stops working until the program itself is upgraded, and the program keeps routing swaps to a venue that can no longer be trusted.

Both dependencies are visible in the code and neither has an exit: a stuck executor locks staked funds, and a bad or dead DEX disables or endangers the swap-join path.

## Source
Template task folder `programs/09-write-the-exit` of https://github.com/RedDuckTeam/redduck-academy-solana-pro-template at commit `782fb2d28eb1ad47ebd6028b1df870706b1133c3`, the version given to students. Renamed the crate, `[lib] name` and `#[program]` module `round_exit` to `prize_rounds`, and removed the Cargo `description`. No comments needed neutralizing. The logic is unchanged and `declare_id!` is kept.

## Trusted roles
- **admin**: runs `initialize` (sets executor, mint) and opens rounds with `open_round`. May set round deadlines.
- **executor**: `config.executor`, the off-chain settler. It is the only key that can end a round and release the pot to the declared winner. It is trusted to settle every round; the program gives players no way to act if it does not.
- Players and callers of `join` / `join_with_swap` are untrusted.
