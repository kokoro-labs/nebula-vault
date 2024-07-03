use borsh::BorshDeserialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
    clock::Clock,
};
use spl_token::state::Account as TokenAccount;

use crate::{instruction::VaultInstruction, state::VaultState, error::VaultError};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = VaultInstruction::try_from_slice(instruction_data)?;

    match instruction {
        VaultInstruction::Initialize => process_initialize(accounts, program_id),
        VaultInstruction::Deposit { amount } => process_deposit(accounts, amount, program_id),
        VaultInstruction::Withdraw { amount } => process_withdraw(accounts, amount, program_id),
        VaultInstruction::SetTimelock { new_timelock } => process_set_timelock(accounts, new_timelock, program_id),
    }
}

fn process_initialize(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_info = next_account_info(account_info_iter)?;
    let owner_info = next_account_info(account_info_iter)?;
    let token_mint_info = next_account_info(account_info_iter)?;

    if !Rent::get()?.is_exempt(vault_info.lamports(), vault_info.data_len()) {
        return Err(VaultError::NotRentExempt.into());
    }

    let vault_state = VaultState {
        owner: *owner_info.key,
        token_mint: *token_mint_info.key,
        token_account: Pubkey::default(), // set later
        balance: 0,
        timelock: 0,
        last_withdrawal: 0,
    };

    vault_state.serialize(&mut &mut vault_info.data.borrow_mut()[..])?;

    msg!("Vault initialized");
    Ok(())
}

fn process_deposit(accounts: &[AccountInfo], amount: u64, program_id: &Pubkey) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_info = next_account_info(account_info_iter)?;
    let owner_info = next_account_info(account_info_iter)?;
    let token_account_info = next_account_info(account_info_iter)?;

    let mut vault_state = VaultState::try_from_slice(&vault_info.data.borrow())?;

    if vault_state.owner != *owner_info.key {
        return Err(VaultError::AuthorityMismatch.into());
    }

    let token_account = TokenAccount::unpack(&token_account_info.data.borrow())?;
    if token_account.mint != vault_state.token_mint {
        return Err(ProgramError::InvalidAccountData);
    }

    vault_state.balance = vault_state.balance.checked_add(amount).ok_or(VaultError::AmountOverflow)?;
    vault_state.serialize(&mut &mut vault_info.data.borrow_mut()[..])?;

    msg!("Deposited {} tokens", amount);
    Ok(())
}

fn process_withdraw(accounts: &[AccountInfo], amount: u64, program_id: &Pubkey) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_info = next_account_info(account_info_iter)?;
    let owner_info = next_account_info(account_info_iter)?;
    let token_account_info = next_account_info(account_info_iter)?;

    let mut vault_state = VaultState::try_from_slice(&vault_info.data.borrow())?;

    if vault_state.owner != *owner_info.key {
        return Err(VaultError::AuthorityMismatch.into());
    }

    let current_time = Clock::get()?.unix_timestamp;
    if current_time < vault_state.last_withdrawal + vault_state.timelock {
        return Err(VaultError::TimelockNotExpired.into());
    }

    if vault_state.balance < amount {
        return Err(VaultError::InsufficientFunds.into());
    }

    vault_state.balance = vault_state.balance.checked_sub(amount).ok_or(VaultError::AmountOverflow)?;
    vault_state.last_withdrawal = current_time;
    vault_state.serialize(&mut &mut vault_info.data.borrow_mut()[..])?;

    msg!("Withdrawn {} tokens", amount);
    Ok(())
}

fn process_set_timelock(accounts: &[AccountInfo], new_timelock: i64, program_id: &Pubkey) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_info = next_account_info(account_info_iter)?;
    let owner_info = next_account_info(account_info_iter)?;

    let mut vault_state = VaultState::try_from_slice(&vault_info.data.borrow())?;

    if vault_state.owner != *owner_info.key {
        return Err(VaultError::AuthorityMismatch.into());
    }

    vault_state.timelock = new_timelock;
    vault_state.serialize(&mut &mut vault_info.data.borrow_mut()[..])?;

    msg!("Timelock set to {} seconds", new_timelock);
    Ok(())
}
