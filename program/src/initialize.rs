use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use std::io::{Cursor, Result};
use byteorder::{LittleEndian, WriteBytesExt};
use std::cell::RefMut;

/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[writable]` voting_data: [AccountInfo] Voting data account
pub fn initialize(program_id: &Pubkey, accounts: &[AccountInfo], initial_votes_option1: u32, initial_votes_option2: u32) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let fee_payer = next_account_info(accounts_iter)?;
    let voting_data = next_account_info(accounts_iter)?;

    // Ensure that the program owns the voting data account
    if voting_data.owner != program_id {
        msg!("Voting data account does not have the correct owner");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Initialize the voting data
    let mut voting_data_data = VotingData {
        option1_votes: initial_votes_option1,
        option2_votes: initial_votes_option2,
    };

    // Pack the data directly from the RefMut
    let mut voting_data_ref_mut = voting_data.data.borrow_mut();
    VotingData::pack(&voting_data_data, &mut voting_data_ref_mut)?;

    Ok(())
}

struct VotingData {
    option1_votes: u32,
    option2_votes: u32,
}

impl VotingData {
    fn pack(data: &Self, output: &mut [u8]) -> Result<()> {
        let mut cursor = Cursor::new(output);
        cursor.write_u32::<LittleEndian>(data.option1_votes)?;
        cursor.write_u32::<LittleEndian>(data.option2_votes)?;
        Ok(())
    }
}
