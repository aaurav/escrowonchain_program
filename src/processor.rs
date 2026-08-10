use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

use borsh::{BorshDeserialize, BorshSerialize};

use crate::instructions::EscrowInstructions;
use crate::state::EscrowState;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let ixs = EscrowInstructions::try_from_slice(instruction_data)?;

    match ixs {
        EscrowInstructions::Initialize { amount } => {
            process_initialize(accounts, program_id, amount)
        }

        EscrowInstructions::Claim => process_claim(accounts, program_id),
        EscrowInstructions::Cancel => process_cancel(accounts, program_id),
    }
}

pub fn process_initialize(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    amount: u64,
) -> ProgramResult {
    Ok(())
}
pub fn process_claim(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
    Ok(())
}
pub fn process_cancel(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
    Ok(())
}
