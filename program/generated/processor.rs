// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

use std::str::FromStr;
use borsh::BorshSerialize;
use solana_program::account_info::{AccountInfo, next_account_info, next_account_infos};
use solana_program::borsh0_10::try_from_slice_unchecked;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::{invoke, invoke_signed};
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction::create_account;
use solana_program::{msg, system_program};
use solana_program::sysvar::Sysvar;
use solana_program::program_pack::Pack;
use crate::generated::errors::NftError;
use crate::generated::instructions::NftInstruction;

use crate::generated::state::{
	Account,
	AccountPDA,
	VotingData,
};
use crate::src::*;

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        data: &[u8],
    ) -> ProgramResult {
        let instruction = NftInstruction::unpack(data)?;

        match instruction {
			NftInstruction::Initialize => {
				msg!("Instruction: Initialize");
				Self::process_initialize(program_id, accounts)
			}
			NftInstruction::Vote => {
				msg!("Instruction: Vote");
				Self::process_vote(program_id, accounts)
			}
			NftInstruction::GetResults => {
				msg!("Instruction: GetResults");
				Self::process_get_results(program_id, accounts)
			}
        }
    }

/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
	pub fn process_initialize(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
	) -> ProgramResult {
		let account_info_iter = &mut accounts.iter();
		let fee_payer_info = next_account_info(account_info_iter)?;
        let initial_votes_option1: u32 = 10;
        let initial_votes_option2: u32 = 20;


		// Security Checks
		if fee_payer_info.is_signer != true {
			return Err(NftError::InvalidSignerPermission.into());
		}



		// Security Checks
		if *fee_payer_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(NftError::WrongAccountOwner.into());
		}



		// Calling STUB
        initialize::initialize(program_id, accounts, initial_votes_option1, initial_votes_option2)?;
		
		Ok(())
	}

/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
	pub fn process_vote(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
	) -> ProgramResult {
		let account_info_iter = &mut accounts.iter();
		let fee_payer_info = next_account_info(account_info_iter)?;
        let selected_option: u8 =1;


		// Security Checks
		if fee_payer_info.is_signer != true {
			return Err(NftError::InvalidSignerPermission.into());
		}



		// Security Checks
		if *fee_payer_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(NftError::WrongAccountOwner.into());
		}



		// Calling STUB
		vote::vote(program_id, accounts, selected_option)?;

		
		Ok(())
	}

/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
	pub fn process_get_results(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
	) -> ProgramResult {
		let account_info_iter = &mut accounts.iter();
		let fee_payer_info = next_account_info(account_info_iter)?;


		// Security Checks
		if fee_payer_info.is_signer != true {
			return Err(NftError::InvalidSignerPermission.into());
		}



		// Security Checks
		if *fee_payer_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(NftError::WrongAccountOwner.into());
		}



		// Calling STUB
		get_results::get_results(
			program_id,
            accounts

		)?;
		
		Ok(())
	}
}