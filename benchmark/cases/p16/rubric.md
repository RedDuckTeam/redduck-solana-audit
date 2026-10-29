---
id: p16
canary: RDSAF81C11F051B132899533
---
## Mechanism
Staked funds leave the vault only through `settle`, which requires `config.executor`; there is no refund path, so an executor that never settles (stopped service or lost key) locks every stake in that round permanently. Separately, `join_with_swap` is pinned to the `DEX_PROGRAM_ID` constant with no instruction to change it, so a dead or compromised venue cannot be replaced without a program upgrade.

## Locations
- src/instructions/settle.rs:6-31
- src/instructions/join.rs:32-57
- src/instructions/join_with_swap.rs:8-29
- src/instructions/join_with_swap.rs:31-73
- src/state/config.rs:5-10
- src/constants.rs:3
- src/state/round.rs:5-11
- src/lib.rs:16-42

## Judge criteria
HIT if the finding names at least one of these dependencies with its consequence:
- the only exit from a round is executor-signed `settle` and there is no refund/withdraw, so a round the executor never settles leaves player stakes locked forever (loss of availability of funds); or
- the swap venue is a hardcoded constant with no change mechanism, so a failed/compromised DEX breaks or endangers `join_with_swap` until a program upgrade.

Either is a HIT; naming both is ideal.

FAIL:
- Claiming `join_with_swap` is exploitable because it CPIs an arbitrary program (the `#[account(address = DEX_PROGRAM_ID)]` constraint fixes the callee; that is a correct decoy).
- Generic "add admin recovery" with no named dependency or consequence.
- A pure off-chain upgrade-authority / multisig observation with no in-code dependency.
- Other issues in this program do not count as a HIT: the executor choosing any `Entry` of the round as winner, `open_round` accepting any deadline (both trusted-role powers), and `initialize` being front-runnable by the first caller of the `config` PDA.
