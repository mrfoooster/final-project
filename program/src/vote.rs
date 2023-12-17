use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use std::io::{Cursor, Result};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

/// Accounts:
/// 0. `[writable, signer]` voter: [AccountInfo] Account of the voter casting the vote
/// 1. `[writable]` voting_data: [AccountInfo] Voting data account
/// 2. `[]` token: [AccountInfo] Placeholder for the token account
pub fn vote(program_id: &Pubkey, accounts: &[AccountInfo], selected_option: u8) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let voter = next_account_info(accounts_iter)?;
    let voting_data = next_account_info(accounts_iter)?;
    let token = next_account_info(accounts_iter)?;

    // Ensure that the program owns the voting data account
    if voting_data.owner != program_id {
        msg!("Voting data account does not have the correct owner");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Validate the selected option
    if selected_option != 1 && selected_option != 2 {
        msg!("Invalid voting option selected");
        return Err(ProgramError::InvalidArgument);
    }

    // Read and update the voting data
    let mut voting_data_data = VotingData::unpack_unchecked(&voting_data.data.borrow())?;
    match selected_option {
        1 => voting_data_data.option1_votes += 1,
        2 => voting_data_data.option2_votes += 1,
        _ => unreachable!(), // Already validated, should not reach here
    }
    VotingData::pack(&voting_data_data, &mut voting_data.data.borrow_mut())?;

    // Additional logic for handling the vote and updating the token account can be added here

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

    fn pack(data: &Self, output: &mut [u8]) -> Result<()> {
        let mut cursor = Cursor::new(output);
        cursor.write_u32::<LittleEndian>(data.option1_votes)?;
        cursor.write_u32::<LittleEndian>(data.option2_votes)?;
        Ok(())
    }
}
