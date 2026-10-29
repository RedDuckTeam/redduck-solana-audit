---
id: c11
crate: spend_card
pattern: dependency-independency
class: design
group: g3
vulnerable: a
role: heldout
author: case-author
canary: RDSAF81C11F051B132899533
---
# Case c11

## Attack scenario
Design failure (liveness): card balances can leave the vault only through a signature from one external key that is compiled into an immutable program.

1. Card holders top up USDC with `top_up`; the vault holds the sum of all balances (`config.total_balance`).
2. The external card processor stops operating its signing service: the company winds down, the contract ends, or the key held by the service is lost.
3. A holder calls `request_withdrawal(amount)` and then `complete_withdrawal`. `CompleteWithdrawal` requires `processor: Signer` with `address = PROCESSOR`, a constant in `src/state.rs`. Nobody can produce that signature, so the call fails with `NotProcessor`.
4. No other path returns funds: `settle_spend` also requires `PROCESSOR` and only pays the settlement account; `queue_config`/`apply_config` can change only `config.settlement`; there is no admin or emergency withdrawal; the program has no upgrade authority (ARCHITECTURE.md). Every card balance stays in the vault permanently.

The same dependency also lets a processor that is still running hold any withdrawal indefinitely by not co-signing it.

## Why fixed closes it
27 changed lines (a design case: the fix adds a config field with its timelocked change path and a second way through `complete_withdrawal`, which touches `lib.rs`, `state.rs`, `admin.rs`, `card.rs`, `processor.rs` and `errors.rs`):
- `state.rs`: the `PROCESSOR` constant is replaced by `Config.processor` and `Config.pending_processor`; `EXIT_DELAY = 48h` is added.
- `admin.rs`/`lib.rs`: `initialize` takes the processor key; `queue_config` takes `new_processor` and `apply_config` applies it after the existing 72-hour `CONFIG_DELAY`.
- `processor.rs` and `card.rs`: the processor constraint becomes `address = config.processor`.
- `card.rs`: in `CompleteWithdrawal` the processor becomes `Option<Signer>`; without it the owner can complete the request once `EXIT_DELAY` has passed since `request_withdrawal` (`ExitNotReady` before that).

If the processor disappears, every holder still gets their balance back 48 hours after requesting it, and the admin can point the program at a new processor after 72 hours. Because the exit delay is shorter than the config delay, holders can leave before a queued processor change takes effect. The normal flow is unchanged: with the processor's co-signature `complete_withdrawal` works immediately, and `settle_spend` works as before.

## Decoys
- The settlement account is also an external dependency, but it is held in `Config` and replaced only through `queue_config`/`apply_config` with a 72-hour delay. This is the same flaw class done correctly, and it never holds user funds.
- `settle_spend` takes any `Card` with `has_one = config`; this is intended because the processor settles for every card, and `auth_id > card.last_auth_id` stops the same authorization from being settled twice.
- `complete_withdrawal` pays `min(withdrawal_amount, balance)`, so purchases settled after the request are respected and the vault never pays more than the card holds.
- `destination` is checked only for the mint; the owner signs and chooses where to receive the funds.
- `Config` is a PDA seeded by the admin key, so nobody can front-run `initialize` for someone else's deployment.

## Trusted roles
- `admin` (issuer multisig): creates its `Config` and vault; queues and applies a new settlement account (and in the fixed version a new processor key), effective 72 hours after queuing. It cannot move card balances.
- Card processor (`PROCESSOR` in the vulnerable version, `config.processor` in the fixed one): settles cleared purchases from any card into `config.settlement` with increasing authorization ids, and co-signs withdrawals. In the fixed version its co-signature only shortens the 48-hour exit delay.
