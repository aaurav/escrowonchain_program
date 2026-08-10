#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint {
    use crate::processor::process_instruction;
    use solana_program::entrypoint;

    entrypoint!(process_instruction);
}
