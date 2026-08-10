use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct EscrowState {
    pub depositor: Pubkey,
    pub recipient: Pubkey,
    pub amount: u64,
    pub status: Status,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
enum Status {
    Pending,
    Claimed,
    Cancelled,
}
