use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct VaultState {
    pub owner: Pubkey,
    pub token_mint: Pubkey,
    pub token_account: Pubkey,
    pub balance: u64,
    pub timelock: i64,
    pub last_withdrawal: i64,
}

impl VaultState {
    pub const LEN: usize = 32 + 32 + 32 + 8 + 8 + 8;
}
