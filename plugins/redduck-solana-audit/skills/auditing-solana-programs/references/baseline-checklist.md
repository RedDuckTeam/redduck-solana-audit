# Baseline checklist

These are the classic Solana bug classes that the 18 patterns do not cover. Each item has a signal to look for, the question to answer, and what is not a bug. In the coverage table each item appears as `checklist-<item>`.

## Contents
1. arithmetic
2. duplicate-accounts
3. unsafe-and-panics
4. reload-after-cpi
5. token-2022
6. closing
7. seed-collisions
8. init-frontrun
9. realloc
10. authority-transfer
11. signers-bumps-cpi

## 1. arithmetic
- **Signal:** `wrapping_*`, `as u32` / `as u64` / `as i64` casts of larger or signed values, `saturating_*` on balances, division before multiplication, a crate without `overflow-checks = true` in `[profile.release]`, and fixed-point math with rounding in the user's favour.
- **Question:** Can a cast truncate, a subtraction wrap, or rounding let a user withdraw more than they deposited, or pay zero for something?
- **Not a bug if:** checked or overflow-checked arithmetic with explicit errors is used and rounding goes against the user. A panic that only fails the attacker's own transaction is at most Low.

## 2. duplicate-accounts
- **Signal:** two writable accounts of the same type in one instruction (`from` and `to`, `vault_a` and `vault_b`, `user` and `counterparty`) with no `constraint = a.key() != b.key()`.
- **Question:** If the same account is passed twice, does the logic double-credit, skip a debit, or bypass a check that compares the two?
- **Not a bug if:** the accounts are PDAs with distinct seeds, a key-inequality check exists, or the operation is a harmless no-op on equal accounts.

## 3. unsafe-and-panics
- **Signal:**
  - `unsafe` blocks;
  - Pinocchio or zero-copy `borrow_*_unchecked`, `from_bytes_unchecked` and pointer casts;
  - unchecked slice indexing and `unwrap()` / `expect()` on account data or user input;
  - `RefCell` borrows held across CPIs;
  - alignment assumptions in zero-copy structs.
- **Question:** Can attacker-controlled data cause undefined behaviour, read out of bounds, or panic an instruction that others depend on, such as a crank or a liquidation?
- **Not a bug if:** lengths and owners are validated before unchecked access, and a panic can only hit the caller's own transaction without blocking shared progress.

## 4. reload-after-cpi
- **Signal:** an Anchor `Account<T>` or a deserialized struct read **after** a CPI that may modify the same account, such as a token transfer into a vault followed by reading `vault.amount`, with no `.reload()?` in between.
- **Question:** Does the program act on stale balances or state after the CPI changed them?
- **Not a bug if:** `.reload()` is called, or the value is recomputed from the CPI's known effect.

## 5. token-2022
- **Signal:**
  - `Interface<TokenInterface>` / `InterfaceAccount` accepting any mint;
  - `transfer` instead of `transfer_checked`;
  - accounting based on the amount sent rather than the amount received;
  - no extension allowlist.
- **Question:** With a mint that has a transfer fee, transfer hook, permanent delegate, non-transferable flag, mint close authority or default-frozen state, can the program's accounting break, can funds be pulled out by the delegate, or can a hook fail every transfer (freezing the token in the vault) or run logic the accounting does not expect? A hook cannot re-enter the calling program: that would be reentrancy through another program, which the runtime rejects.
- **Not a bug if:** only the classic Token program is accepted, or mints are restricted to an allowlist, or the extensions are checked and accounting uses post-transfer balances.

## 6. closing
- **Signal:** manual closing that only moves lamports out without zeroing data and reassigning the owner, closing an account that still holds tokens or obligations, and a close instruction callable by someone other than the owner.
- **Question:** Can a closed account be reused in the same transaction, can its rent go to the wrong party, and can closing strand other users' funds?
- **Not a bug if:** Anchor's `close = recipient` is used on an account with no remaining obligations, and `recipient` is constrained.

## 7. seed-collisions
- **Signal:**
  - PDA seeds built from variable-length user input concatenated without separators or fixed lengths;
  - two account types derived from the same seed prefix;
  - one PDA used as authority for many users' assets, where any user's instruction can sign for all.
- **Question:** Can two different logical records map to the same address, or can one user's instruction exercise a PDA's authority over another user's assets?
- **Not a bug if:** seeds use fixed-length keys or distinct prefixes per type, and per-user authorities are separate PDAs.

## 8. init-frontrun
- **Signal:**
  - an `initialize` or config instruction with no `has_one` / upgrade-authority / hardcoded-key check;
  - PDAs created with `init` at addresses an attacker can predict and pre-fund or pre-create;
  - `system_program::create_account` on an address that may already hold lamports.
- **Question:** Can an attacker initialize the global config first and become its admin, or block a user's `init` by squatting or pre-funding the address?
- **Not a bug if:** initialization is gated by the program's upgrade authority or a hardcoded key, or account creation tolerates pre-funded lamports (allocate and assign instead of `create_account`).

## 9. realloc
- **Signal:** `realloc` / `resize` on growing accounts, growth driven by user actions, shrinking then growing again, and per-call growth over 10 KiB.
- **Question:** Can stale bytes reappear after a shrink and regrow, can a user grow an account until it is unusable, and is rent paid by the party that causes the growth?
- **Not a bug if:** new space is zero-initialized (`realloc::zero = true`, or Anchor's resize behaviour in the version used), growth is bounded, and the payer is the actor.

## 10. authority-transfer
- **Signal:** `set_authority` / `update_admin` that writes the new key in one step, upgrade authority held by a single EOA, and no timelock on critical parameter changes.
- **Question:** Can a typo or a compromised key permanently lose or take control? Is there a two-step propose-and-accept flow?
- **Not a bug if:** the transfer is two-step (pending authority plus accept signed by the new key), or the authority is a multisig with a timelock for high-impact changes.

## 11. signers-bumps-cpi
- **Signal:**
  - native and Pinocchio code without `is_signer` checks on authorities;
  - `create_program_address` with a caller-supplied bump;
  - bumps not stored or not canonical;
  - `invoke` / `invoke_signed` into a program id taken from an account the caller passed;
  - `remaining_accounts` used without validation.
- **Question:** Can an authority be impersonated, can a non-canonical bump create a second valid address, or can the program be tricked into calling, and signing for, an attacker's program?
- **Not a bug if:** Anchor `Signer`, `seeds` with `bump` (canonical), and `Program<'info, T>` or an explicit program-id check are used, and `remaining_accounts` are validated like declared accounts.
