use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};

use crate::VaultError;

/// Reads instruction `index` from `account`, which holds the instructions sysvar's data.
///
/// Layout of the sysvar data: `u16` count, `u16` offset per instruction, then for each
/// instruction `u16` number of accounts, per account one flag byte (bit 0 signer, bit 1 writable)
/// and a 32-byte key, a 32-byte program id, a `u16` data length and the data.
pub fn load_instruction(index: usize, account: &AccountInfo) -> Result<Instruction> {
    let data = account.try_borrow_data()?;
    let read_u16 = |at: usize| -> Result<usize> {
        let bytes: [u8; 2] = data
            .get(at..at + 2)
            .and_then(|s| s.try_into().ok())
            .ok_or(VaultError::MalformedSysvar)?;
        Ok(u16::from_le_bytes(bytes) as usize)
    };
    let count = read_u16(0)?;
    require!(index < count, VaultError::MalformedSysvar);
    let mut at = read_u16(2 + index * 2)?;

    let accounts_len = read_u16(at)?;
    at += 2;
    let mut accounts = Vec::with_capacity(accounts_len);
    for _ in 0..accounts_len {
        let flags = *data.get(at).ok_or(VaultError::MalformedSysvar)?;
        let key: [u8; 32] = data
            .get(at + 1..at + 33)
            .and_then(|s| s.try_into().ok())
            .ok_or(VaultError::MalformedSysvar)?;
        accounts.push(AccountMeta {
            pubkey: Pubkey::from(key),
            is_signer: flags & 1 != 0,
            is_writable: flags & 2 != 0,
        });
        at += 33;
    }
    let program_id: [u8; 32] = data
        .get(at..at + 32)
        .and_then(|s| s.try_into().ok())
        .ok_or(VaultError::MalformedSysvar)?;
    at += 32;
    let data_len = read_u16(at)?;
    at += 2;
    let ix_data = data
        .get(at..at + data_len)
        .ok_or(VaultError::MalformedSysvar)?
        .to_vec();
    Ok(Instruction {
        program_id: Pubkey::from(program_id),
        accounts,
        data: ix_data,
    })
}
