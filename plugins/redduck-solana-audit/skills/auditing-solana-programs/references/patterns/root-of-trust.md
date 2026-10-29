# Root of trust

## Rule
Every chain of checks an instruction relies on must end at something the caller could not have created: a PDA of this program whose seeds are verified, a hardcoded address, or a stored authority that signed. If the chain ends at an account the caller supplied, the checks prove only that the caller's accounts agree with each other.

## Why Solana forces it
The transaction lists every account an instruction reads, and the sender picks them. Creating accounts is cheap, and so is filling them with data that has the right layout. A program that compares caller-supplied accounts with each other can be satisfied by a complete fake set. This applies to sysvars, oracle accounts, configs, pools and token accounts alike.

## Code signals
- **Anchor:**
  - `UncheckedAccount` / `AccountInfo` without `address =`, `seeds =` or an owner check;
  - `has_one = x` where `x` itself is not anchored;
  - `constraint = a.key() == b.field` where `b` is also caller-supplied;
  - `Account<T>` of a type that a permissionless instruction lets anyone create.
- **Native and Pinocchio:**
  - `try_from_slice` / `load` of account data without checking `owner == program_id`;
  - a PDA re-derivation that is missing or uses caller-supplied seeds;
  - `find_program_address` results compared with nothing.
- **Introspection:** reading the instructions sysvar (`load_instruction_at`, `get_instruction_relative`) from an account whose address is not compared with `sysvar::instructions::ID`, and Ed25519/Secp256k1 verification where the signer key or message inside the verified instruction is not compared with stored, trusted values.
- **Mints and token accounts:** the mint or vault is checked only against another passed account, instead of against a config PDA.
- **NFT gates and NFT positions:**
  - a collection gate that compares `metadata.collection` with the expected key but never reads its `verified` flag, or a creator address compared without `creator.verified`;
  - a metadata account not derived from the mint with `seeds::program` set to Token Metadata;
  - an "NFT" accepted with no check of `decimals == 0` and `supply == 1`, or of a master edition;
  - a position controlled through a stored `position.owner` while the position NFT can be sold, instead of through the holder's token account (`mint == position.nft_mint`, `amount == 1`, owner is the signer);
  - these signals are for Token Metadata. A Metaplex Core asset proves membership with `UpdateAuthority::Collection(key)` on an asset owned by the Core program, and has no `verified` flag. A compressed NFT proves ownership with a Bubblegum proof against the tree.

## Auditor questions
1. For each account the instruction trusts, what is it compared with, and what is that compared with? Follow the chain to its end.
2. Is the end something only this program or the deployer controls?
3. Could an attacker create a fresh config, pool, bank, oracle or vault through a public instruction, or as an unowned account with copied data, that satisfies every check?
4. Do all instructions that move value use the same root? Attackers look for the one instruction that skipped it.
5. For signature introspection: are the sysvar address, the verifying program id, the signer key and the signed message all checked against trusted values, in that order?
6. For NFT gates: could the attacker mint their own NFT, or a fungible token, with metadata that names the collection, and pass? Who controls a position after its NFT changes hands?

## How to confirm
Build the fake set. List the accounts the attacker creates, what data they put in them, and the call that passes every check. In a test (see `exploit-tests.md`), create those accounts in LiteSVM or bankrun and show the instruction succeeding and moving value. Without a toolchain, write the scenario with the exact constraint lines it passes.

## Not a bug if
- The chain ends at a PDA whose seeds are derived from trusted keys and checked: `seeds = [...]` with `bump` in Anchor, or `create_program_address` comparison in native code.
- The account is `Account<T>` and the only instruction that creates `T` is gated by a stored authority or uses fixed seeds.
- The instructions sysvar account is pinned with `#[account(address = sysvar::instructions::ID)]` on an `UncheckedAccount`, or read through `load_instruction_at_checked` / `load_current_index_checked`, which compare the address. The instructions sysvar does not implement Anchor's `Sysvar` trait, so it cannot be declared as `Sysvar<'info, Instructions>`.
- A token account is constrained by `token::mint = config.mint` where `config` is itself anchored.
- An NFT gate checks the verified collection on metadata derived from the mint, and an edition or `decimals == 0` with `supply == 1`.

## Real-world cases
- **Failure.** A stablecoin minted against collateral after checking that the collateral account, a "bank" record and some pool accounts were consistent with each other. Banks could be created by anyone, so an attacker created a consistent fake set and minted without real collateral.
- **Failure.** A bridge confirmed guardian signatures by reading the instruction list from an account the caller passed. It never compared that account's address with the real sysvar, so a fake list was enough.
- **Done right.** An on-demand oracle reads the same instruction list but first compares the account with the sysvar address. It then checks that the neighbouring instruction belongs to the Ed25519 program and that the verified key and message are the oracle's.

## Severity
Usually Critical, because forged accounts lead straight to theft or unbacked minting. High when the forged chain only lets the attacker change parameters or block others.
