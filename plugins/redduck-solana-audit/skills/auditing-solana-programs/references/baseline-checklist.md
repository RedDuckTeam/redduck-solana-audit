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
- **Signal:** `wrapping_*`, `as u32` / `as u64` / `as i64` casts of larger or signed values, `saturating_*` on balances, division before multiplication, a workspace-root `Cargo.toml` without `overflow-checks = true` in `[profile.release]`, and fixed-point math with rounding in the user's favour.
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
This item also covers mint and freeze authorities, which apply to classic SPL mints as well.
- **Signal:**
  - `Interface<TokenInterface>` / `InterfaceAccount` accepting any mint;
  - `transfer` instead of `transfer_checked`;
  - accounting based on the amount sent rather than the amount received;
  - no extension allowlist;
  - an accepted asset mint (collateral, vault asset, reward) with `freeze_authority` set, USDC included;
  - a share, LP or receipt mint passed in by the caller, or not created with `mint::authority = <pda>`, with no check at pool creation that `mint_authority == Some(pda)`, `freeze_authority == None`, `supply == 0` and no Token-2022 extensions such as a permanent delegate, mint close authority or transfer hook;
  - a receipt token that represents a deposit, created without the `NonTransferable` extension, or not pinned with `address = config.receipt_mint`;
  - one instruction that pushes transfers to many user token accounts;
  - interest-bearing or scaled-UI mints whose raw `amount` is mixed with UI amounts or with prices quoted in UI units.
- **Question:**
  - With a mint that has a transfer fee, transfer hook, permanent delegate, non-transferable flag, mint close authority or default-frozen state, can the program's accounting break, can funds be pulled out by the delegate, or can a hook fail every transfer (freezing the token in the vault) or run logic the accounting does not expect? A hook cannot re-enter the calling program: that would be reentrancy through another program, which the runtime rejects.
  - Who holds each mint's mint and freeze authority? If the issuer freezes the vault or one user's token account, which instructions fail? A batch payout fails for everyone, and a user can also close an empty destination account before a push payment.
  - Can a pool creator keep the share mint's authority and mint shares without depositing?
  - Does each deposit have exactly one live claim on it? If a receipt token can be redeemed by whoever holds it while the stake record still lets the depositor withdraw, one deposit pays out twice.
- **Not a bug if:**
  - only the classic Token program is accepted, or mints are restricted to an allowlist, or the extensions are checked and accounting uses post-transfer balances;
  - share mints are created by the program with a PDA mint authority and no freeze authority;
  - payouts are pulled per user, so one frozen or closed account blocks only itself;
  - the receipt is non-transferable and pinned, or the stake record is consumed when the receipt is minted, so the bearer token is the only claim;
  - the issuer of an accepted asset can freeze it, which is a trusted-role fact: Info, or Low when one frozen account blocks other users' exits.

## 6. closing
- **Signal:**
  - manual closing that only moves lamports out without zeroing data and reassigning the owner;
  - closing an account that still holds tokens or obligations;
  - a close instruction callable by someone other than the owner;
  - a lamport withdrawal from a data-bearing program account checked only against `lamports()`, such as `**vault.try_borrow_mut_lamports()? -= amount`, with no `Rent::get()?.minimum_balance(data_len)` reserve.
- **Question:** Can a closed account be reused in the same transaction, can its rent go to the wrong party, and can closing strand other users' funds? Can a withdrawal take a data-bearing account to exactly zero lamports? The runtime deletes it at the end of the transaction, which is an implicit close that allows a fresh `init` and resets its state.
- **Not a bug if:** Anchor's `close = recipient` is used on an account with no remaining obligations, and `recipient` is constrained. Lamport withdrawals are capped at `lamports() - minimum_balance(data_len)`.

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
  - `system_program::create_account` on an address that may already hold lamports;
  - `init_if_needed` on a singleton or authority-bearing account (`seeds = [b"config"]`) whose handler writes `admin`, authority fields, flags or markers unconditionally;
  - a manual init path (`UncheckedAccount`, or native `try_from_slice` followed by a write) with no "already initialized" check;
  - plain `init` on an associated token account. Anyone can create any wallet's or PDA's ATA first.
- **Question:** Can an attacker initialize the global config first and become its admin, or block a user's `init` by squatting or pre-funding the address? On the reuse branch of `init_if_needed`, or a second call to a manual init, can a caller overwrite the admin or reset balances, flags or markers? Can a pre-created ATA make an `init` the protocol needs fail forever?
- **Not a bug if:** initialization is gated by the program's upgrade authority or a hardcoded key, or account creation tolerates pre-funded lamports (allocate and assign instead of `create_account`). The handler detects first use, for example `state.authority == Pubkey::default()`, and refuses to rewrite otherwise. ATAs are created idempotently (`init_if_needed` or `create_idempotent`).

## 9. realloc
This item also covers layout changes across program upgrades.
- **Signal:** `realloc` / `resize` on growing accounts, growth driven by user actions, shrinking then growing again, and per-call growth over 10 KiB. For a deployed program: an account struct changed relative to the deployed version (a field inserted, removed, reordered or retyped, or a zero-copy array resized), or a `migrate` instruction.
- **Question:** Can stale bytes reappear after a shrink and regrow, can a user grow an account until it is unusable, and is rent paid by the party that causes the growth? After an upgrade, do accounts already on chain still deserialize, with the same meaning? Can an unmigrated account still be used, or an account be migrated twice?
- **Not a bug if:** new space is zero-initialized (`realloc::zero = true`, or Anchor's resize behaviour in the version used), growth is bounded, and the payer is the actor. The program is not deployed yet, or new fields are only appended behind a version byte or a data-length check, and old accounts are rejected or migrated first.
- **Severity of a layout change:** an append to an exact-size Borsh account fails to deserialize, locking funds until migration: High with no migration path. Anchor does not reject trailing bytes, so a removed trailing field, a reorder or retype of equal-size fields, an append into spare or reserved bytes, and any zero-copy or manual layout change are read silently with the new meaning: Critical when the misread field is an amount, an authority or a status.

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
  - `remaining_accounts` used without validation;
  - Anchor `has_one = authority`, or `constraint = x.key() == state.admin`, where `authority` is `UncheckedAccount`, `AccountInfo` or `SystemAccount` rather than `Signer`;
  - native or Pinocchio parsing that checks `owner == program_id` but no discriminator or type tag, in a program with more than one account type;
  - `invoke_signed` into an external program (DEX, hook, adapter) with a protocol-wide PDA as signer, or with `remaining_accounts` passed straight through.
- **Question:** Can an authority be impersonated, can a non-canonical bump create a second valid address, or can the program be tricked into calling, and signing for, an attacker's program? Can one of this program's own account types be passed where another is expected? When the program signs into another program, what else can that program do with the PDA's signature during the call, such as moving every token account the PDA owns?
- **Not a bug if:** Anchor `Signer`, `#[account(signer)]` or a manual `is_signer` check, `seeds` with `bump` (canonical), and `Program<'info, T>` or an explicit program-id check are used, and `remaining_accounts` are validated like declared accounts. A `has_one` that binds a non-authority account, such as a destination or a vault, needs no `Signer`. Each account type starts with a distinct discriminator checked before parsing. External CPIs get a PDA scoped to the one pool or user involved, and only the accounts that call needs.
