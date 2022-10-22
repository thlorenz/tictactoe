use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh::try_from_slice_unchecked,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::{
    assert_game_key_matches_account, assert_signer,
    assert_waiting_for_opponent, create_account_owned_by_program, get_game_pda,
    Game, GameState, InitializeGameArgs, PlayerMove, TictactoeInstruction,
    GAME_SIZE, TICTACTOE_PREFIX,
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
    // 1. Extract accounts
    let account_info_iter = &mut accounts.iter();
    let player_info = next_account_info(account_info_iter)?;
    let game_pda_info = next_account_info(account_info_iter)?;

    // 2. Check that the accounts conform to the requirements
    assert_signer(player_info)?;

    let mut game: Game =
        try_from_slice_unchecked(&game_pda_info.try_borrow_data()?)?;
    assert_waiting_for_opponent(&game)?;

    // 3. Update game state
    game.player_o = *player_info.key;
    game.state = GameState::Full;

    // 4. Save game state
    game.serialize(&mut &mut game_pda_info.try_borrow_mut_data()?.as_mut())?;

    Ok(())
}

pub fn player_move(
    accounts: &[AccountInfo],
    player_move: PlayerMove,
) -> ProgramResult {
    msg!("IX: player_move, passed {} accounts", accounts.len());

    // 1. Extract accounts
    let account_info_iter = &mut accounts.iter();
    let player_info = next_account_info(account_info_iter)?;
    let game_info = &mut next_account_info(account_info_iter)?;
    let mut game: Game =
        try_from_slice_unchecked(&game_info.try_borrow_data()?)?;

    // 2. Use PlayerMove methods to verify and process the desired move
    player_move.process(player_info, &mut game)?;

    // 4. Save game state
    game.serialize(&mut &mut game_info.try_borrow_mut_data()?.as_mut())?;

    Ok(())
}
