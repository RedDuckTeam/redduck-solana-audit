# Solana Audit Report: note_market

Program: `./program` (Anchor). Instructions: initialize_vault, open_position, deposit, accrue_yield, withdraw, redeem, close_position.

## Findings

### F1
Severity: Medium
Location: program/src/instructions/withdraw.rs:43
Mechanism: `notes_to_burn` is rounded down and never required to be > 0, so once `total_assets > total_notes` anyone (even an empty position with `unlock_ts = 0`, see open.rs:29) can call `withdraw` with small amounts that burn zero or too few notes and take yield from other depositors, repeatedly. Fix: round up and `require!(notes_to_burn > 0)`.
Verification: finding-verifier originally rated High and downgraded it to Medium. The loss is capped at the accrued yield above 1:1 and shrinks with each call, and principal is not at risk. No exploit test was run (no toolchain).

### F2
Severity: Low
Location: program/src/instructions/initialize.rs:17
Mechanism: Vault creation is permissionless and the PDA is derived only from the mint, so whoever calls first becomes `vault.authority` for that mint's canonical vault. This can be done by front-running the intended team. The authority can only fund yield, so impact is limited to a squatted or unexpected vault.

### F3
Severity: Low
Location: program/src/instructions/accrue.rs:29
Mechanism: Yield is added as a lump sum by a single authority key and is not distributed over time. A depositor who sees it coming can deposit beforehand, wait out the 1-hour `HOLD_SECS`, and capture a share of yield earned by earlier depositors. If the authority stops, no yield accrues.

### F4
Severity: Low
Location: program/src/instructions/deposit.rs:44
Mechanism: `deposit`, `withdraw` and `redeem` take no minimum-out or maximum-in bound (`withdraw.rs:43`, `redeem.rs:43`). The share price changes with `accrue_yield` and with other users' rounding, so users cannot bound the rate they sign for.

### F5
Severity: Info
Location: program/src/instructions/deposit.rs:62
Mechanism: Every deposit resets the whole position's `unlock_ts`. Anyone can lock their own funds again, but a lock timer that renews on each top-up can surprise the owner and makes the hold period unfriendly to dollar-cost-averaging users.

## Design findings

### D1
Severity: Low
Location: program/src/state.rs:10
Mechanism: Every user instruction writes the single global `Vault` account, so all deposits and withdrawals for a mint serialize on one hot account. Throughput for a popular vault is limited. Total assets are tracked internally and not read from the token balance, so direct donations are ignored (this is good, as it blocks classic inflation attacks).

### D2
Severity: Low
Location: program/src/instructions/accrue.rs:16
Mechanism: `vault.authority` is fixed at creation. There is no instruction to rotate or transfer it, so a lost or compromised authority key cannot be replaced, and yield stops permanently if the key is lost. The authority has no power over user funds.

## Coverage notes

- Checked: arithmetic (u128 intermediates, checked ops; `-=` in withdraw.rs:50-52 and redeem.rs:48-50 is guarded by earlier checks), account validation (PDA seeds, `has_one`, `address = vault.vault_token`, token mint/authority constraints), SPL Token only (no Token-2022), close_position requires empty notes, no `remaining_accounts`, no CPI beyond SPL Token, no reload-after-CPI issues.
- Redeem rounds down in the protocol's favour. Deposit rounds notes down in the protocol's favour and rejects zero notes.
- This report does not include the full per-pattern coverage table the audit skill asks for. Only the notes above summarize what was checked.
