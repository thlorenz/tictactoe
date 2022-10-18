use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult,
    program_error::PrintProgramError, pubkey::Pubkey,
};

use crate::{processor, TictactoeError};

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) =
        processor::process(program_id, accounts, instruction_data)
    {
        error.print::<TictactoeError>();
        return Err(error);
    }
    Ok(())
}
