use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

use crate::{PlayerMove, TictactoeInstruction};

pub fn process(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = TictactoeInstruction::try_from_slice(instruction_data)?;

    use TictactoeInstruction::*;
    match instruction {
        InitializeGame => initialize_game(accounts),
        PlayerJoin => player_join(accounts),
        PlayerMove(args) => player_move(accounts, args),
    }
}

pub fn initialize_game(accounts: &[AccountInfo]) -> ProgramResult {
    msg!("IX: initialize_game, passed {} accounts", accounts.len());
    Ok(())
}

pub fn player_join(accounts: &[AccountInfo]) -> ProgramResult {
    msg!("IX: player_join, passed {} accounts", accounts.len());
    Ok(())
}

pub fn player_move(
    accounts: &[AccountInfo],
    _player_move: PlayerMove,
) -> ProgramResult {
    msg!("IX: player_move, passed {} accounts", accounts.len());
    Ok(())
}
