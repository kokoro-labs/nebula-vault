use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum VaultInstruction {
    Initialize,
    Deposit { amount: u64 },
    Withdraw { amount: u64 },
    SetTimelock { new_timelock: i64 },
}

pub fn initialize(program_id: &Pubkey, vault: &Pubkey, owner: &Pubkey, token_mint: &Pubkey) -> Instruction {
    Instruction::new_with_borsh(
        *program_id,
        &VaultInstruction::Initialize,
        vec![
            (*vault, true, false),
            (*owner, true, false),
            (*token_mint, false, false),
        ],
    )
}

pub fn deposit(program_id: &Pubkey, vault: &Pubkey, owner: &Pubkey, token_account: &Pubkey, amount: u64) -> Instruction {
    Instruction::new_with_borsh(
        *program_id,
        &VaultInstruction::Deposit { amount },
        vec![
            (*vault, false, false),
            (*owner, true, false),
            (*token_account, false, false),
        ],
    )
}

pub fn withdraw(program_id: &Pubkey, vault: &Pubkey, owner: &Pubkey, token_account: &Pubkey, amount: u64) -> Instruction {
    Instruction::new_with_borsh(
        *program_id,
        &VaultInstruction::Withdraw { amount },
        vec![
            (*vault, false, false),
            (*owner, true, false),
            (*token_account, false, false),
        ],
    )
}

pub fn set_timelock(program_id: &Pubkey, vault: &Pubkey, owner: &Pubkey, new_timelock: i64) -> Instruction {
    Instruction::new_with_borsh(
        *program_id,
        &VaultInstruction::SetTimelock { new_timelock },
        vec![
            (*vault, false, false),
            (*owner, true, false),
        ],
    )
}
