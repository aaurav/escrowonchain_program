use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    lamports,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};

use solana_system_interface::instruction;
use solana_system_interface::program;

use crate::instructions::EscrowInstructions;
use crate::state::{EscrowState, Status};
use borsh::{BorshDeserialize, BorshSerialize};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let ixs = EscrowInstructions::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

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
    let account_info_iter = &mut accounts.iter();
    let depositor = next_account_info(account_info_iter)?;
    let recipient = next_account_info(account_info_iter)?;
    let escrow_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    //1.Check account meta flags
    if !depositor.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    if !depositor.is_writable {
        return Err(ProgramError::InvalidArgument);
    }
    if !escrow_account.is_writable {
        return Err(ProgramError::InvalidArgument);
    }
    //2.Check Identity/authorization: N/A — nothing exists yet, no stored party to check against
    //3. Check Ownership: N/A — account doesn't exist yet, nothing to own
    //4. Derivation checks
    let seeds = &[b"escrow", depositor.key.as_ref(), recipient.key.as_ref()];

    let (expected_escrow_pda, bump) = Pubkey::find_program_address(seeds, program_id);

    if escrow_account.key != &expected_escrow_pda {
        return Err(ProgramError::InvalidSeeds);
    }
    if !escrow_account.data_is_empty() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }
    if system_program.key != &program::ID {
        return Err(ProgramError::IncorrectProgramId);
    }
    // 5. State/business logic : N/A — no prior state to compare against, nothing stored yet

    // 6. Untrusted input arithmetic checks
    if amount == 0 {
        return Err(ProgramError::InvalidArgument);
    }

    //Rent calculation
    let rent = Rent::get()?;
    let required_lamports = rent.minimum_balance(EscrowState::LEN);
    let total_required = amount
        .checked_add(required_lamports)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    if depositor.lamports() < total_required {
        return Err(ProgramError::InsufficientFunds);
    }
    //build instruction and create account using cpi
    let from_address = depositor.key;
    let to_address = escrow_account.key;
    let lamports = total_required;
    let space = EscrowState::LEN as u64;
    let owner = program_id;

    let ix = instruction::create_account(from_address, to_address, lamports, space, owner);
    let cpi_accounts = [
        depositor.clone(),
        escrow_account.clone(),
        system_program.clone(),
    ];
    let escrow_seeds: &[&[u8]] = &[
        b"escrow",
        depositor.key.as_ref(),
        recipient.key.as_ref(),
        &[bump],
    ];
    invoke_signed(&ix, &cpi_accounts, &[escrow_seeds])?;

    // Write the escrow state data into escrow account
    let escrow_state = EscrowState {
        depositor: *depositor.key,
        recipient: *recipient.key,
        amount,
        status: Status::Pending,
    };

    let mut escrow_data = escrow_account.data.borrow_mut();
    let data = &mut &mut escrow_data[..];
    escrow_state.serialize(data)?;

    Ok(())
}
pub fn process_claim(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
    Ok(())
}
pub fn process_cancel(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
    Ok(())
}
