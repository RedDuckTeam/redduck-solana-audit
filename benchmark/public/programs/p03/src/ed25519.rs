use anchor_lang::prelude::*;
use anchor_lang::solana_program::ed25519_program;
use anchor_lang::solana_program::instruction::Instruction;

use crate::ClaimError;

/// What an Ed25519 instruction verified: the key that signed and the message.
pub struct Ed25519Signed {
    pub pubkey: Pubkey,
    pub message: Vec<u8>,
}

/// Parses a single-signature instruction built for the Ed25519 program (the layout
/// `Ed25519Program.createInstructionWithPrivateKey` produces): a 16-byte header with
/// offsets, then the public key, the signature and the message, all inside this instruction.
pub fn parse_ed25519(ix: &Instruction) -> Result<Ed25519Signed> {
    require_keys_eq!(ix.program_id, ed25519_program::ID, ClaimError::NotEd25519);
    let d = &ix.data;
    require!(d.len() >= 16 && d[0] == 1, ClaimError::MalformedEd25519);
    let u16_at = |i: usize| u16::from_le_bytes([d[i], d[i + 1]]) as usize;
    let (sig_ix, pk_off, pk_ix) = (u16_at(4), u16_at(6), u16_at(8));
    let (msg_off, msg_size, msg_ix) = (u16_at(10), u16_at(12), u16_at(14));
    require!(
        sig_ix == 0xffff && pk_ix == 0xffff && msg_ix == 0xffff,
        ClaimError::MalformedEd25519
    );
    require!(
        d.len() >= pk_off + 32 && d.len() >= msg_off + msg_size,
        ClaimError::MalformedEd25519
    );
    let mut pubkey = [0u8; 32];
    pubkey.copy_from_slice(&d[pk_off..pk_off + 32]);
    Ok(Ed25519Signed {
        pubkey: Pubkey::from(pubkey),
        message: d[msg_off..msg_off + msg_size].to_vec(),
    })
}
