---
id: c24
crate: points_staking
pattern: untrusted-executor
class: design
group: g1
vulnerable: a
role: dev
author: case-author
canary: RDSAF81C11F051B132899533
---
# Case c24

`points_staking` is a staking pool for an off-chain points program. Users stake a token, and to exit they call `request_unstake`. The tokens can be taken out with `withdraw` only after the epoch in which the request was made has been closed (`pool.epoch > request.epoch`). `close_epoch` advances the epoch once `epoch_length` has passed and emits the `EpochClosed` snapshot that the points engine reads. In the vulnerable version `close_epoch` needs the signature of `pool.authority`. `ARCHITECTURE.md` says that this key is the team's operator wallet, used by a keeper on the team's server.

## Attack scenario
This is a design failure, not a theft.
1. A pool holds stake from many users. Some of them have pending `Unbonding` requests made in epoch N.
2. The team's keeper stops. This can be a server outage, a lost or rotated operator key, an empty fee wallet, or the team walking away.
3. `close_epoch` has `#[account(mut, has_one = authority)]` with `authority: Signer` (`src/lib.rs:191-196`), so no one else can advance `pool.epoch`. It stays at N.
4. Every `withdraw` checks `pool.epoch > request.epoch` (`src/lib.rs:81`) and fails with `EpochNotClosed`. No other instruction moves tokens out of the vault, so all requested stake, and all stake whose owners try to exit later, is locked for as long as the keeper is down, possibly forever. The points snapshots also stop.

## Why fixed closes it
- `CloseEpoch` no longer has `has_one = authority`. Any `caller: Signer` can close an epoch once `epoch_length` has passed. The time check is unchanged.
- To pay outside callers, every `request_unstake` sends `CLOSE_TIP_LAMPORTS` from the requester into the pool account (`system_program::transfer`) and adds it to `pool.tip_pot`. `close_epoch` pays the whole `tip_pot` to the caller (`sub_lamports`/`add_lamports`) and resets it. The users who need the next close pay for it, independent bots are paid to send the transaction, and a user waiting to withdraw can also close the epoch themselves.
- The caller cannot change anything else. `close_epoch` only advances the epoch on schedule and emits the snapshot. Withdrawals still need the request owner's signature and go to the owner.
- The rest is unchanged: the import, the constant, the `tip_pot` field and its initialization. The normal flow still works (`stake`, `request_unstake`, then any wallet's `close_epoch`, then `withdraw`), and the team's keeper can keep running as one caller among many.
- The diff is 27 changed lines. That is over the usual 15 because the incentive needs a lamport transfer CPI in `request_unstake` (10 lines), the payout in `close_epoch` (6 lines), and the new field and constant. Removing the signer alone would restore liveness but would add no incentive for executors.

## Decoys
- `withdraw` sends tokens to any token account of the stake mint that the request owner signs for. The owner chooses where their own tokens go.
- In the fixed program anyone can call `close_epoch`. This is safe because it only advances the epoch once `epoch_started_at + epoch_length` has passed and pays out only tips that were deposited for that purpose.
- `pool.epoch_started_at = ends_at` instead of `now` keeps epochs on a fixed schedule. After a pause, several epochs can be closed back to back, which matches the snapshots the points engine expects.
- The pool PDA is seeded with the creator's key and the mint, so nobody can front-run another creator's pool setup. The vault is a PDA token account owned by the pool, and transfers are signed with the pool seeds.
- `request_unstake` uses `checked_sub` on both the staker's and the pool's totals, and one `Unbonding` PDA per owner and pool prevents duplicate requests.

## Trusted roles
- **Pool authority** (`pool.authority`, the team's operator wallet): creates the pool and sets `epoch_length` in `create_pool`. In the vulnerable version it is also the only wallet that can sign `close_epoch`; that is the design flaw of this case. In the fixed version it has no powers after creation. In neither version can it move staked tokens or touch other users' requests.
- Nobody else is trusted. Stakers act only on their own `Staker` and `Unbonding` accounts. In the fixed version any wallet may call `close_epoch`, but only after `epoch_length` has passed, and it receives only the collected tips.
