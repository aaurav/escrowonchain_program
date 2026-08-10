use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum EscrowInstructions {
    Initialize { amount: u64 },
    Claim,
    Cancel,
}
