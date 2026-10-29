# Example: audit of a tip jar program

This example shows the report format on a small program written for this purpose. The program is not part of the benchmark.

## The program

```rust
use anchor_lang::prelude::*;

declare_id!("TipJar1111111111111111111111111111111111111");

#[program]
pub mod tip_jar {
    use super::*;

    pub fn open(ctx: Context<Open>) -> Result<()> {
        ctx.accounts.jar.owner = ctx.accounts.owner.key();
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let jar = &mut ctx.accounts.jar;
        **jar.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.receiver.try_borrow_mut_lamports()? += amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Open<'info> {
    #[account(init, payer = owner, space = 8 + 32)]
    pub jar: Account<'info, Jar>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub jar: Account<'info, Jar>,
    /// CHECK: receives lamports
    #[account(mut)]
    pub receiver: UncheckedAccount<'info>,
    pub owner: UncheckedAccount<'info>,
}

#[account]
pub struct Jar {
    pub owner: Pubkey,
}
```

## The report

```markdown
# Solana audit report: tip_jar

## Scope
src/lib.rs, Anchor 0.31. Instructions: open, withdraw.
Surfaces: value moved by direct lamport arithmetic in withdraw; an authority account passed as UncheckedAccount
Pattern files read: references/patterns/root-of-trust.md

## Summary
1 Critical, 1 Low. Anyone can drain any jar.

## Findings

### F-1: withdraw never checks that the jar's owner signed

Severity: Critical
Location: src/lib.rs:38
Mechanism: `owner` is an unchecked, non-signer account and is never compared with `jar.owner`, so any wallet can call `withdraw` on any jar and send its lamports to itself.

Pattern: root-of-trust
Scenario:
1. Attacker finds a jar holding lamports.
2. Calls `withdraw(jar.lamports - rent)` with `receiver = attacker` and any `owner` account.
3. No constraint ties `owner` to the jar or requires a signature, so the lamports move to the attacker.
Test: unconfirmed (no toolchain)
Verification: confirmed by verifier
Fix: Make `owner: Signer<'info>` and add `has_one = owner` on `jar`.

### F-2: withdraw can drain the jar to zero without closing it

Severity: Low
Location: src/lib.rs:15
Mechanism: `withdraw` can move every lamport out of the jar but never zeroes its data or hands it back to the System Program. A zero-balance jar is not closed properly, and partial withdrawals that would leave it below the rent-exempt minimum simply fail.

Pattern: checklist-closing
Scenario:
1. The owner (after the F-1 fix) withdraws the full balance.
2. Within the same transaction the jar's data is still intact and still owned by the program. If anything refunds it before the transaction ends, the old `Jar` record survives, a closing-account revival.
3. A withdrawal that would leave a non-zero balance below rent exemption is rejected by the runtime (`InsufficientFundsForRent`), which confuses users about how much they can take out.
Test: not attempted
Verification: self-checked (Low, no verifier needed)
Fix: Add a `close_jar` instruction using Anchor's `close = owner`, and cap `withdraw` at `lamports - Rent::get()?.minimum_balance(8 + 32)`.

## Design findings
None.

## Coverage
| Item | Verdict | Evidence |
|---|---|---|
| root-of-trust | found | F-1 |
| identity-rejection | n/a | no allow lists or caller-based rules |
| untrusted-executor | n/a | no actions that need an operator |
| one-way-state | clear | no counters or messages; balance moves are direct |
| checklist-closing | found | F-2 |
| checklist-signers-bumps-cpi | found | F-1 (missing signer) |
| ... | ... | ... |
```
