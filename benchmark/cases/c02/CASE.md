---
id: c02
crate: commission_pool
pattern: lazy-loading
class: exploit
group: g1
vulnerable: a
role: heldout
author: case-author
canary: RDSAF81C11F051B132899533
---
# Case c02

## Attack scenario
`commission_pool` shares a reward token among delegators by stake weight. It keeps a
running accumulator `acc_reward_per_share`; each stake account stores a `reward_debt`
marker, and the rewards owed are `staked * acc - reward_debt`. The operator grows the
accumulator with `distribute`.

1. Honest delegators stake, then the operator `distribute`s rewards, raising
   `acc_reward_per_share` to some value `A`. The accumulator now represents rewards that
   belong to the existing stakers.
2. The attacker calls `open_stake` (staked = 0, `reward_debt` set to `0 * A = 0`).
3. The attacker calls `deposit(X)`. The handler adds `X` to `staked` **first**, then
   computes the pending reward as `staked * A - reward_debt = X * A - 0` and pays it out
   immediately.
4. The attacker instantly receives `X * A / SCALE` reward tokens — the share of the
   accumulator that accrued **before they joined** — stealing it from the delegators who
   were staked when `distribute` ran.

The root cause is that `deposit` settles pending rewards using the balance **after** the
new stake is added, so freshly added stake is credited with the entire accumulator
history.

## Why fixed closes it
In `src/instructions/deposit.rs` the settlement is reordered: pending is computed and
paid against the **old** `staked` value, and only then is the stake increased and
`reward_debt` reset from the new balance. A brand-new depositor has `staked = 0` at
settle time, so pending is `0` and they earn only on rewards distributed after their
deposit. Normal deposits/withdrawals/claims pay exactly the rewards earned while staked.

## Decoys
- `withdraw` uses the same accumulator but settles correctly: it computes and pays
  pending on the current stake **before** decrementing, then resets `reward_debt`.
- `claim` also settles correctly (pending computed on the current stake, then
  `reward_debt` advanced to the accumulator). Both look like the deposit path but get
  the ordering right, so only `deposit` captures pre-join rewards.

## Trusted roles
- **Pool authority / operator** (`Pool.authority`): calls `distribute` to fund the
  reward vault and advance `acc_reward_per_share`. It cannot touch delegator stake or
  reward markers.
