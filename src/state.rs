use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct EscrowState {
    pub depositor: Pubkey,
    pub recipient: Pubkey,
    pub amount: u64,
    pub status: Status,
}

impl EscrowState {
    pub const LEN: usize = std::mem::size_of::<Pubkey>()
        + std::mem::size_of::<Pubkey>()
        + std::mem::size_of::<u64>()
        + std::mem::size_of::<Status>();
}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub enum Status {
    Pending,
    Claimed,
    Cancelled,
}
