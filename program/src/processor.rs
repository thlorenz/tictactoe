use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::{
    assert_game_key_matches_account, assert_signer,
    create_account_owned_by_program, get_game_pda, Game, InitializeGameArgs,
    PlayerMove, TictactoeInstruction, GAME_SIZE, TICTACTOE_PREFIX,
};

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = TictactoeInstruction::try_from_slice(instruction_data)?;

    use TictactoeInstruction::*;
    match instruction {
        InitializeGame(args) => initialize_game(accounts, args, program_id),
        PlayerJoin => player_join(accounts),
        PlayerMove(args) => player_move(accounts, args),
    }
}

pub fn initialize_game(
    accounts: &[AccountInfo],
    args: InitializeGameArgs,
    program_id: &Pubkey,
) -> ProgramResult {
    msg!("IX: initialize_game, passed {} accounts", accounts.len());
    // 1. Extract accounts
    let account_info_iter = &mut accounts.iter();
    let player_info = next_account_info(account_info_iter)?;
    let game_pda_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    // 2. Check that the accounts conform to the requirements
    assert_signer(player_info)?;

    let (game_pda, bump) = get_game_pda(&args.game);
    assert_game_key_matches_account(&game_pda, game_pda_info)?;

    // 3. Create the game account
    let seeds = &[TICTACTOE_PREFIX.as_bytes(), args.game.as_ref(), &[bump]];
    create_account_owned_by_program(
        player_info,
        game_pda_info,
        system_info,
        program_id,
        seeds,
        GAME_SIZE,
    )?;

    // 4. Initialize the game account making the player account player x
    let game = Game::init(*player_info.key);
    game.serialize(&mut &mut game_pda_info.try_borrow_mut_data()?.as_mut())?;

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
