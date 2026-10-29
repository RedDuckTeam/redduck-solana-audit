# Solana Audit Report: note_market

Scope: program/src (Anchor). Instructions: initialize_vault, open_position, deposit, accrue_yield, withdraw, redeem, close_position.

## Findings

### F1
Severity: High
Location: program/src/instructions/withdraw.rs:43
Mechanism: `notes_to_burn` is rounded down and never required to be > 0, so once yield makes a note worth more than one asset, any user (even with an empty, already-unlocked position) can withdraw `assets` below the note price while burning 0 notes, taking accrued yield from other holders; with a high ratio one call can take almost all yield.
Verification: confirmed by finding-verifier. Fix: round notes up and `require!(notes_to_burn > 0)`.

### F2
Severity: Medium
Location: program/src/instructions/accrue.rs:46
Mechanism: Yield is credited instantly to total_assets and the only lock is a 1-hour hold (state.rs:4), so an attacker can deposit just before a visible accrue_yield and withdraw an hour later, capturing a share of yield they did not earn.

### F3
Severity: Medium
Location: program/src/instructions/initialize.rs:17
Mechanism: The vault PDA is seeded only by the asset mint and the first caller becomes `authority`, so anyone can front-run and squat the vault for a mint (a single vault per mint), permanently owning the authority role and blocking the intended operator.

### F4
Severity: Low
Location: program/src/instructions/withdraw.rs:47
Mechanism: The lock check relies on `unlock_ts`, which `open_position` sets to 0 (open.rs:29). This is harmless on its own but is the reason F1 works with no deposit. A deposit also resets the lock for the whole position (deposit.rs:62).

### F5
Severity: Low
Location: program/src/instructions/accrue.rs:16
Mechanism: The authority is fixed at initialization with no rotation or recovery. If the key is lost, yield can never be added again. If it is compromised, the attacker controls yield timing (see F2).

### F6
Severity: Info
Location: program/src/instructions/deposit.rs:38
Mechanism: When total_notes hits 0, leftover total_assets (rounding dust) remains while the first new depositor is minted notes 1:1, so that dust is gifted to them. Accounting uses stored totals rather than the token balance, so direct donations do not inflate the share price.

### F7
Severity: Info
Location: program/src/instructions/redeem.rs:33
Mechanism: redeem emits no event, unlike deposit and withdraw, so off-chain indexers cannot track redemptions.

## Design findings

### D1
Severity: Low
Location: program/src/instructions/deposit.rs:13
Mechanism: Every user instruction writes the single global Vault account, so all activity is serialized on one hot account (scalability limit). The program is also limited to the legacy SPL Token program and has no pause or emergency exit. A freeze authority on the mint could freeze vault_token and lock all funds.
