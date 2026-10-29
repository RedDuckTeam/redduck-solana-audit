---
id: c03
crate: note_market
pattern: checklist-arithmetic
class: exploit
group: g3
vulnerable: b
role: dev
author: case-author
canary: RDSAF81C11F051B132899533
---
# Case c03

## Attack scenario
`note_market` is a yield-bearing vault. Depositors bring an asset token and receive
`notes`. The authority periodically calls `accrue_yield`, which adds assets to the
vault without minting notes, so the assets-per-note ratio grows above 1.

1. Honest users deposit a total of `1_000` assets and receive `1_000` notes
   (`total_notes = 1_000`, `total_assets = 1_000`).
2. Normal operation: the authority calls `accrue_yield(999_000)`. Now
   `total_assets = 1_000_000`, `total_notes = 1_000`; one note is worth 1_000 assets, and
   the 999_000 of yield belongs to the honest note holders.
3. The attacker calls `open_position` but **never deposits**, so their position holds
   `0` notes (and `unlock_ts = 0`, so the hold period does not apply to them).
4. The attacker calls `withdraw(a)` with `a = floor((total_assets - 1) / total_notes)`,
   i.e. just under one note's worth. The burn is
   `notes_to_burn = a * total_notes / total_assets`, which truncates to **0** whenever
   `a * total_notes < total_assets`. The check `position.notes >= notes_to_burn` is
   `0 >= 0`, so the handler burns nothing and transfers `a` assets to the attacker. The
   first call takes 999 assets, the next ~998, and so on.
5. Repeating step 4 with the request recomputed each time (several withdrawals per
   transaction), the attacker needs about 7_500 calls (7_481 exactly) to bring
   `total_assets` from 1_000_000 down to 1_000, at which point `a` becomes 0 and the
   drain stops. The attacker walks away with all **999_000** assets of yield while never
   holding a note.

Affected party: the honest note holders lose the entire accrued yield; their 1_000 notes
are now backed by only 1_000 assets (1 per note), so depositors who joined at 1:1 keep
their principal but nothing more, and anyone who deposited at a higher ratio (after an
earlier accrual) also loses principal.

The root cause is that `withdraw` rounds the number of notes to burn **down**. The
charge to the user must always round **up**, so any positive withdrawal costs at least
one note and an empty position can withdraw nothing.

## Why fixed closes it
In `src/instructions/withdraw.rs` the division that computes `notes_to_burn` changes
from floor (`scaled / total_assets`) to ceiling
(`(scaled + total_assets - 1) / total_assets`). With the fix, `withdraw(999)` requires
`ceil(999 * 1_000 / 1_000_000) = 1` note, so the empty-position attacker fails at
`position.notes (0) >= 1` and a real depositor burns at least the notes their withdrawal
is worth. Normal flow is unchanged: a depositor who withdraws their full share burns
exactly the notes they hold, and `redeem` (note-denominated exit) is untouched.

## Decoys
- `deposit` computes minted notes with the same `a * b / c` shape and also rounds
  **down** — but that is correct, because under-issuing notes to a depositor rounds in
  the vault's favour.
- `redeem` converts notes to assets with floor division as well, which is correct for
  the same reason (a redeemer should never get more assets than their notes are worth).
  Two of the three division sites round down correctly; only the burn side in
  `withdraw` is wrong, because the charge to the user must round up.
- The per-deposit hold period (`unlock_ts`) is an ordinary anti-sniping control checked
  in `withdraw` and `redeem`; it does not bear on the rounding defect.

## Trusted roles
- **Vault authority** (`Vault.authority`): the account that created the vault. It may
  call `accrue_yield` to add assets to the vault. It cannot mint notes or move user
  funds beyond adding assets. It calls `accrue_yield` at least once every `HOLD_SECS`, so a
  deposit that must stay for `HOLD_SECS` is exposed for at least as long as the period
  any single accrual covers (no just-in-time capture of a long accrual).
