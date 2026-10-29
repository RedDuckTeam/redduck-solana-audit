# Solana audit report: note_market (Anchor)

Scope: `program/src` (7 instructions: initialize_vault, open_position, deposit, accrue_yield, withdraw, redeem, close_position). No Critical or High findings. Account validation (PDA seeds, has_one, token mint/authority, address on vault_token) is sound, and share math rounds in the protocol's favour. No Solana toolchain or verifier run was needed because nothing is rated High or above.

## Findings

Severity: Medium
Location: program/src/instructions/initialize.rs:17
Mechanism: The vault PDA is seeded only by the mint and `initialize_vault` is permissionless, so anyone can create the vault for a mint first and become its `authority` (the only account that can call `accrue_yield`). A squatter can front-run the intended team's vault and pass off a hostile vault as the canonical one; the PDA cannot be re-created by the real team.

Severity: Medium
Location: program/src/instructions/accrue.rs:45
Mechanism: Yield is added as a lump sum to `total_assets`, which raises the note price instantly for everyone currently holding notes. The authority's timing is predictable (or visible in the mempool), so an attacker can deposit an hour or more ahead (the 1-hour hold at state.rs:4 is the only protection) and then redeem. They capture a share of yield they did not earn, diluting long-term depositors. No streaming or time-weighting is used.

Severity: Low
Location: program/src/instructions/redeem.rs:43
Mechanism: `redeem` and `deposit` (deposit.rs:44) take no minimum-out or slippage bound. `redeem` can burn notes and pay out 0 assets when the rounded-down amount is 0 (nothing checks `assets_out > 0`), so users lose dust. The share price only changes upward through `accrue_yield`, which limits the impact.

Severity: Low
Location: program/src/instructions/deposit.rs:62
Mechanism: Every deposit, including a tiny one, resets `unlock_ts` for the owner's whole position to now + 1 hour. This means a user adding funds also re-locks their existing notes. Only the owner can trigger it, so it is a footgun and not a third-party attack.

Severity: Low
Location: program/src/instructions/deposit.rs:44
Mechanism: Notes are rounded down, so each deposit donates up to one note-value of rounding to existing holders. The authority can be the first depositor, hold 1 note, and then accrue yield. Later depositors lose up to one note-value on each deposit (deposits below the note price revert at line 46). This is bounded and needs the trusted authority, but the vault has no minimum-liquidity protection.

Severity: Low
Location: program/src/state.rs:10
Mechanism: `total_assets` is internal accounting. Tokens sent directly to `vault_token` are never credited to anyone and are stuck forever. Dust left in `total_assets` after `total_notes` reaches 0 is taken by the next first depositor, because `deposit` at line 38 mints `notes = assets` while the old dust stays in `total_assets`.

Severity: Info
Location: program/src/instructions/redeem.rs:33
Mechanism: `redeem` emits no event, unlike `deposit` and `withdraw`. Off-chain indexers will see an incomplete record of vault exits.

## Design findings

Severity: Medium
Location: program/src/instructions/accrue.rs:10
Mechanism: Centralization and liveness: the vault's yield exists only because one `authority` key pushes its own tokens in. The key is fixed at init (initialize.rs:39) and cannot be rotated or recovered. If the key is lost or compromised, yield stops permanently. Principal stays withdrawable, so this is a yield liveness failure and not a loss of principal.

Severity: Low
Location: program/src/instructions/withdraw.rs:57
Mechanism: Dependency on the mint: `asset_mint` is any SPL mint with no check for a freeze authority (initialize.rs:11). If the mint's freeze authority freezes `vault_token`, every withdrawal and redemption fails, and users have no exit path that doesn't depend on the issuer.

Severity: Low
Location: program/src/instructions/deposit.rs:13
Mechanism: Scalability: every deposit, withdraw, redeem and accrue writes the single `Vault` account, so all vault operations are serialized on one writable account. Per-user positions are separate PDAs, so this is acceptable, but the hot account limits throughput.

## Coverage

| Item | Verdict |
|---|---|
| root-of-trust | clear: vault PDA seeds (deposit.rs:13), `address = vault.vault_token` (deposit.rs:25) |
| identity-rejection | n/a: no allow/deny lists |
| untrusted-executor | found: yield is authority-only (accrue.rs:10) |
| role-inversion | n/a: no operator-pushed data |
| lazy-loading | clear: share-price model, no iteration |
| pay-as-you-act | clear: totals updated on every path |
| divide-and-conquer | found: single Vault hot account (design finding) |
| complexity-asymmetry | n/a: no loops or hints |
| commit-execution-split | n/a: no randomness or previewable outcome |
| action-as-data | n/a |
| virtual-execution | n/a |
| one-way-state | clear: position closed only at zero notes (close.rs:25) |
| boundary-of-consent | found: no slippage bounds (redeem.rs:43) |
| internal-oracle | clear: `total_assets` is internal accounting that donations cannot move |
| defensive-computation | clear: deposit and redeem round down, withdraw rounds up (withdraw.rs:45); dust issues noted above |
| fail-fast | n/a: no external oracle |
| execution-layer-shift | n/a |
| dependency-independency | found: mint freeze risk |
| checklist-arithmetic | clear: checked_* and u128 intermediates; the unchecked `-=` at redeem.rs:48-50 and withdraw.rs:51-53 are guarded by earlier requires |
| checklist-duplicate-accounts | clear: vault_token and owner_token have different constraints; a self-transfer is harmless |
| checklist-unsafe-and-panics | clear: no unsafe; no division by zero reachable (the denominator is nonzero when total_notes > 0) |
| checklist-reload-after-cpi | clear: no state read after CPIs |
| checklist-token-2022 | n/a: the legacy Token program is pinned via `Program<Token>` |
| checklist-closing | clear: Anchor `close` on position (close.rs:14) |
| checklist-seed-collisions | clear: distinct prefixes |
| checklist-init-frontrun | found: vault squatting (initialize.rs:17) |
| checklist-realloc | n/a |
| checklist-authority-transfer | found: no authority rotation (design finding) |
| checklist-signers-bumps-cpi | clear: stored bumps, signer seeds at state.rs:23 |
