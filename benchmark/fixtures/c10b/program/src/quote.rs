//! Publisher quotes are checked through the Ed25519 native program: the transaction
//! carries an Ed25519 signature-verify instruction directly before the instruction that
//! consumes the quote, and the program inspects it through the instructions sysvar.

use anchor_lang::prelude::*;
use anchor_lang::solana_program::ed25519_program;
use anchor_lang::solana_program::sysvar::instructions::{
    load_current_index_checked, load_instruction_at_checked,
};

use crate::errors::FxError;

pub const QUOTE_DOMAIN: &[u8; 8] = b"FXQUOTE1";

/// Header of Ed25519 instruction data: signature count and one padding byte.
const HEADER_LEN: usize = 2;
/// Size of one `Ed25519SignatureOffsets` entry.
const OFFSETS_LEN: usize = 14;
/// Instruction index meaning "inside the Ed25519 instruction itself".
const THIS_INSTRUCTION: u16 = u16::MAX;

/// Bytes the publisher signs for one quote.
pub fn quote_message(base_mint: &Pubkey, quote_mint: &Pubkey, rate: u64, published_at: i64) -> Vec<u8> {
    let mut message = Vec::with_capacity(88);
    message.extend_from_slice(QUOTE_DOMAIN);
    message.extend_from_slice(base_mint.as_ref());
    message.extend_from_slice(quote_mint.as_ref());
    message.extend_from_slice(&rate.to_le_bytes());
    message.extend_from_slice(&published_at.to_le_bytes());
    message
}

/// Requires that the previous instruction verified `signer`'s signature over `message`.
pub fn verify_signed_message(instructions: &AccountInfo, signer: &Pubkey, message: &[u8]) -> Result<()> {
    let current = load_current_index_checked(instructions)? as usize;
    require!(current > 0, FxError::MissingSignature);
    let ix = load_instruction_at_checked(current - 1, instructions)?;
    require_keys_eq!(ix.program_id, ed25519_program::ID, FxError::MissingSignature);

    let data = ix.data.as_slice();
    require!(data.len() >= HEADER_LEN + OFFSETS_LEN, FxError::MalformedSignature);
    require!(data[0] == 1, FxError::MalformedSignature);

    let field = |n: usize| {
        let at = HEADER_LEN + 2 * n;
        u16::from_le_bytes([data[at], data[at + 1]])
    };
    let (sig_ix, key_offset, key_ix) = (field(1), field(2) as usize, field(3));
    let (msg_offset, msg_len, msg_ix) = (field(4) as usize, field(5) as usize, field(6));
    require!(
        sig_ix == THIS_INSTRUCTION && key_ix == THIS_INSTRUCTION && msg_ix == THIS_INSTRUCTION,
        FxError::MalformedSignature
    );

    let key = data.get(key_offset..key_offset + 32).ok_or(FxError::MalformedSignature)?;
    require!(key == signer.as_ref(), FxError::WrongPublisher);
    let signed = data.get(msg_offset..msg_offset + msg_len).ok_or(FxError::MalformedSignature)?;
    require!(signed == message, FxError::MessageMismatch);
    Ok(())
}
