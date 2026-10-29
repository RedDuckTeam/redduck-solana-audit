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

## Anchor behaviour

6. **"An account closed with Anchor's `close` can be revived and reused."**
   - **Why it is wrong:** current Anchor zeroes the data, moves the lamports out and hands the account back to the System Program. Funding it again in the same transaction yields an empty system account, not the old program account.
   - **When it is real:** check the Anchor version, and look for manual closing code that only moves lamports.
7. **"`#[account(mut)]` without other constraints means anyone can pass any account."**
   - **Why it is wrong:** for `Account<'info, T>`, Anchor already checks the owner and discriminator.
   - **When it is real:** `UncheckedAccount` and `AccountInfo` have no such check, so they need an explicit address, owner or seeds check.
8. **"Anchor's `init` can be called twice."**
   - **Why it is wrong:** `init` fails when the account exists.
   - **When it is real:** `init_if_needed` can be called again. Report it only when the second call can reset or overwrite state that matters.
9. **Older Anchor issues fixed upstream.**
   - **Why it is wrong:** reports of stack overflow in large account structs, or of duplicate mutable accounts slipping past Anchor's checks, depend on the Anchor version in use.
   - **When it is real:** check `Cargo.toml` before reporting either.

## Arithmetic and tokens

10. **"`a + b` can overflow and wrap."**
    - **Why it is wrong:** Anchor workspaces set `overflow-checks = true` in the release profile, so plain arithmetic panics instead of wrapping.
    - **When it is real:** `wrapping_*`, `as` casts that truncate, unchecked arithmetic in a crate without that profile setting, and panics that block an action others depend on.
11. **"Transferring tokens to yourself breaks accounting."**
    - **Why it is wrong:** SPL Token allows source equal to destination, as a no-op.
    - **When it is real:** program logic that credits or debits based on two accounts it assumes are different. That is the duplicate-accounts checklist item.

## Design-level traps

12. **"The admin (or authority, oracle signer, chosen reporter) can do X."**
    - **Why it is wrong:** a role the protocol deliberately trusts is a design fact, not an exploit. Calling it High or Critical overstates it.
    - **When it is real:** report it as a design finding at Medium at most, naming the role and its powers. It is High or Critical only when the role can do more than its stated purpose, or when someone untrusted can take the role. If an ordinary user can profit while the role behaves honestly, it is not a trusted-role finding: rate it by its impact.
13. **"Anyone can call `settle` / `crank` / `claim_for`."**
    - **Why it is wrong:** permissionless execution is often the correct design (untrusted-executor).
    - **When it is real:** the caller can redirect value, choose a profitable moment the design did not intend, or repeat the action.
14. **"The program reads a price from its own pool."**
    - **Why it is wrong:** an internal oracle is a legitimate pattern.
    - **When it is real:** the price is a spot value an attacker can move inside the same transaction, through reserves, balances or donations. That is a real and severe finding.
