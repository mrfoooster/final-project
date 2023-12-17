use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use std::io::{Cursor, Result};
use byteorder::{LittleEndian, ReadBytesExt};

/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[writable]` voting_data: [AccountInfo] Voting data account
pub fn get_results(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let fee_payer = next_account_info(accounts_iter)?;
    let voting_data = next_account_info(accounts_iter)?;

    // Ensure that the program owns the voting data account
    if voting_data.owner != program_id {
        msg!("Voting data account does not have the correct owner");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Read the voting data
    let voting_data_data = VotingData::unpack_unchecked(&voting_data.data.borrow())?;
    msg!("Option 1 Votes: {}", voting_data_data.option1_votes);
    msg!("Option 2 Votes: {}", voting_data_data.option2_votes);

    Ok(())
}

struct VotingData {
    option1_votes: u32,
    option2_votes: u32,
}

impl VotingData {
    fn unpack_unchecked(input: &[u8]) -> Result<Self> {
        let mut cursor = Cursor::new(input);
        let option1_votes = cursor.read_u32::<LittleEndian>()?;
        let option2_votes = cursor.read_u32::<LittleEndian>()?;
        Ok(Self { option1_votes, option2_votes })
    }
}
