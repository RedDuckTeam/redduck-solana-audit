# Common misreports

Check every draft finding against this list before reporting it. Each entry says why the claim is usually wrong and when a real finding hides behind it. The list follows Asymmetric Research's write-up on Solana misreports (https://asymmetric.com/solana-vulnerabilities-that-arent-unpacking-common-misreports/) and runtime behaviour documented by Solana and Anchor.

## Contents
- Runtime guarantees
- Anchor behaviour
- Arithmetic and tokens
- Design-level traps

## Runtime guarantees

1. **Reentrancy through another program (A calls B, B calls back into A).**
   - **Why it is wrong:** the runtime rejects it with a reentrancy error. Only direct self-recursion is allowed.
   - **When it is real:** stale data after a CPI, where an account was changed by the callee but not reloaded. That belongs to the checklist item on reloading after CPI.
2. **"The CPI's return value is not checked."**
   - **Why it is wrong:** a failing CPI aborts the whole transaction, and the caller cannot catch the failure and continue.
   - **When it is real:** a callee that returns success while doing something unexpected, which is a trust problem in the callee.
3. **"A failed transaction leaves partial state."**
   - **Why it is wrong:** transactions are atomic, so any error reverts every account change in the transaction.
   - **When it is real:** state committed by an earlier, separate transaction that later steps assumed would be rolled back. That is a design issue: see one-way-state and commit-execution-split.
4. **"Floating point makes validators disagree."**
   - **Why it is wrong:** floats in SBF programs are computed in software and are deterministic.
   - **When it is real:** precision and rounding, which are still defensive-computation questions.
5. **"`load_instruction_at` lets an attacker fake the instruction list."**
   - **Why it is wrong:** the old unchecked helper is the problem only when nothing verifies that the account is the instructions sysvar. `load_instruction_at_checked`, `load_current_index_checked`, or `#[account(address = sysvar::instructions::ID)]` on the account close it.
   - **When it is real:** report it only when you find no address check.
6. **"The withdrawal leaves the account below rent-exempt, so the runtime deletes it."**
   - **Why it is wrong:** the runtime rejects any transaction that leaves an account with lamports between zero and its rent-exempt minimum.
   - **When it is real:** the rejection blocks the only exit, as in a withdraw-all that ignores the minimum, or a withdrawal to exactly zero deletes a data-bearing account at the end of the transaction. That case belongs to the closing checklist item.
7. **"The instruction needs more than 200,000 compute units, so it is a denial of service."**
   - **Why it is wrong:** 200,000 is the default per instruction. Any client can request up to 1,400,000 per transaction with `SetComputeUnitLimit`.
   - **When it is real:** the worst case exceeds 1,400,000, including what callers add when they must compose the instruction through CPI.
8. **"The called program can make a read-only account writable, or a non-signer a signer."**
   - **Why it is wrong:** the runtime rejects privilege escalation through CPI. A callee gets at most the privileges the caller passed.
   - **When it is real:** the callee misuses the authority it was actually given, such as a protocol-wide PDA signature. That is the signers-bumps-cpi checklist item.

## Anchor behaviour

9. **"An account closed with Anchor's `close` can be revived and reused."**
   - **Why it is wrong:** current Anchor zeroes the data, moves the lamports out and hands the account back to the System Program. Funding it again in the same transaction yields an empty system account, not the old program account.
   - **When it is real:** check the Anchor version, and look for manual closing code that only moves lamports.
10. **"`#[account(mut)]` without other constraints means anyone can pass any account."**
    - **Why it is wrong:** for `Account<'info, T>`, Anchor already checks the owner and discriminator.
    - **When it is real:** `UncheckedAccount` and `AccountInfo` have no such check, so they need an explicit address, owner or seeds check.
11. **"Anchor's `init` can be called twice."**
    - **Why it is wrong:** `init` fails when the account exists.
    - **When it is real:** `init_if_needed` can be called again. Report it only when the second call can reset or overwrite state that matters.
12. **Older Anchor issues fixed upstream.**
    - **Why it is wrong:** reports of stack overflow in large account structs, or of duplicate mutable accounts slipping past Anchor's checks, depend on the Anchor version in use.
    - **When it is real:** check `Cargo.toml` before reporting either.
13. **"With `init_if_needed`, an attacker can create the PDA first with fake data."**
    - **Why it is wrong:** only the deriving program can sign to create or assign its PDA, and `Account<T>` checks the owner and discriminator on the reuse branch. An ATA can only be created by the ATA program, always as a valid token account.
    - **When it is real:** the handler overwrites state, or fails to reset it, on the reuse branch (init-frontrun, one-way-state). Another instruction of the same program can create an account at that address (seed-collisions). The handler assumes a reused ATA is fresh, with a zero balance and no delegate.
14. **"An `UncheckedAccount` has no constraint."**
    - **Why it is wrong:** an account the program never reads, does not sign for, and only forwards to a CPI as a destination where any account is acceptable is validated by the callee.
    - **When it is real:** the program reads its data or lamports; the program's PDA signs for it, as with a source vault, where the Token program accepts any account that PDA owns; its balance feeds accounting; or its identity matters: a payout destination that must belong to the owner, an oracle, a config, a program id.

## Arithmetic and tokens

15. **"`a + b` can overflow and wrap."**
    - **Why it is wrong:** plain arithmetic panics instead of wrapping when the workspace root `Cargo.toml` sets `overflow-checks = true` under `[profile.release]`. Anchor's template does.
    - **When it is real:** the setting is missing from the workspace root. Cargo's release default does not set it, many native and Pinocchio repositories leave it off, and a `[profile]` section in a member crate is ignored. Also `wrapping_*`, `as` casts that truncate, and panics that block an action others depend on.
16. **"Transferring tokens to yourself breaks accounting."**
    - **Why it is wrong:** SPL Token allows source equal to destination, as a no-op.
    - **When it is real:** program logic that credits or debits based on two accounts it assumes are different. That is the duplicate-accounts checklist item.
17. **"The mint authority can move users' tokens."**
    - **Why it is wrong:** the mint authority can only mint. Transfer and burn need the account owner or a delegate, and closing needs the owner or the account's close authority.
    - **When it is real:** minting dilutes anything priced in the token or redeemable for vault assets, or the mint has a Token-2022 permanent delegate.

## Design-level traps

18. **"The admin (or authority, oracle signer, chosen reporter) can do X."**
    - **Why it is wrong:** a role the protocol deliberately trusts is a design fact, not an exploit. Calling it High or Critical overstates it.
    - **When it is real:** report it as a design finding at Medium at most, naming the role and its powers. It is High or Critical only when the role can do more than its stated purpose, or when someone untrusted can take the role. If an ordinary user can profit while the role behaves honestly, it is not a trusted-role finding: rate it by its impact.
19. **"Anyone can call `settle` / `crank` / `claim_for`."**
    - **Why it is wrong:** permissionless execution is often the correct design (untrusted-executor).
    - **When it is real:** the caller can redirect value, choose a profitable moment the design did not intend, or repeat the action.
20. **"The program reads a price from its own pool."**
    - **Why it is wrong:** an internal oracle is a legitimate pattern.
    - **When it is real:** the price is a spot value an attacker can move inside the same transaction, through reserves, balances or donations. That is a real and severe finding.
21. **"No event is emitted for X."**
    - **Why it is wrong:** on-chain code cannot read logs, so a missing event is Info at most.
    - **When it is real:** a documented off-chain component (a bridge relayer, a rewards indexer, a keeper) acts on events. Then check that event fields match what actually moved, such as post-fee amounts, and that `emit!` output cannot be lost to log truncation, which `emit_cpi!` avoids. A later instruction that expects to read an emitted value is a functional bug: the value belongs in state.
