use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

use crate::{Game, GameState, TictactoeError};

pub fn assert_signer(account: &AccountInfo) -> ProgramResult {
    if !account.is_signer {
        Err(TictactoeError::AccountShouldBeSigner.into())
    } else {
        Ok(())
    }
}

pub fn assert_game_key_matches_account(
    game_key: &Pubkey,
    game_account: &AccountInfo,
) -> ProgramResult {
    if game_key != game_account.key {
        Err(TictactoeError::InvalidGameAccount.into())
    } else {
        Ok(())
    }
}

pub fn assert_waiting_for_opponent(game: &Game) -> ProgramResult {
    if game.state != GameState::WaitingForOpponent {
        msg!(&format!(
            "Game state is {:?}, but should be waiting for opponent",
            game.state
        ));
        Err(TictactoeError::ShouldBeWaitingForOpponent.into())
    } else {
        Ok(())
    }
}
