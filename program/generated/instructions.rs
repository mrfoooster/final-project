// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use crate::generated::errors::NftError;

#[derive(BorshSerialize, Debug)]
pub enum NftInstruction {
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
	Initialize,

/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
	Vote,

/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
	GetResults,

}

impl NftInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input.split_first().ok_or(NftError::InvalidInstruction)?;

        Ok(match variant {
			0 => Self::Initialize,
			1 => Self::Vote,
			2 => Self::GetResults,
			_ => return Err(NftError::InvalidInstruction.into())
        })
    }
}