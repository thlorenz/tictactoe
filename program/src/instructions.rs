use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;
use solana_program::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey,
};

use crate::{Game, Player, TictactoeError, BOARD_ITEM_FREE};

// -----------------
// Instruction
// -----------------
#[derive(BorshDeserialize, ShankInstruction)]
#[rustfmt::skip]
pub enum TictactoeInstruction {
    #[account(name = "player_x", signer, desc = "The player initializing the game")]
    #[account(name = "game_pda", mut, desc="The game PDA")]
    #[account(name = "system_program", desc="The system program")]
    InitializeGame(InitializeGameArgs),

    #[account(name = "player_o", signer, desc = "The player joining the game")]
    #[account(name = "game_pda", mut, desc="The game PDA")]
    PlayerJoin,

    #[account(name = "player", signer, desc = "The player making the move")]
    #[account(name = "game_pda", mut, desc="The game PDA")]
    PlayerMove(PlayerMove),
}

// -----------------
// Initialize Game
// -----------------
#[derive(BorshSerialize, BorshDeserialize)]
pub struct InitializeGameArgs {
    pub game: Pubkey,
}

// -----------------
// Player Move
// -----------------
#[derive(BorshSerialize, BorshDeserialize, Clone, Copy)]
pub enum MoveKind {
    X = 1, /* BOARD_ITEM_X */
    O = 2, /* BOARD_ITEM_O */
}
#[derive(BorshDeserialize)]
pub struct PlayerMove {
    pub x_or_o: MoveKind,
    pub field: u8,
}

impl PlayerMove {
    pub fn process(
        &self,
        player: &AccountInfo,
        game: &mut Game,
    ) -> Result<(), ProgramError> {
        // 1. Check that the move is valid for the player
        self.check(player, game)?;

        // 2. Update the game board
        let PlayerMove { x_or_o, field } = self;
        game.board[*field as usize] = *x_or_o as u8;

        // 3. Update the game state
        game.update_state();

        Ok(())
    }

    fn check(
        &self,
        player: &AccountInfo,
        game: &Game,
    ) -> Result<(), ProgramError> {
        let PlayerMove { x_or_o, field } = self;
        // Is the slot already taken?
        if Some(&BOARD_ITEM_FREE) != game.board.get(*field as usize) {
            return Err(TictactoeError::IllegalMove.into());
        }

        // Is the player account the correct one and is it his/her turn?
        match x_or_o {
            MoveKind::X => Self::check_move_player_x(player, game),
            MoveKind::O => Self::check_move_player_o(player, game),
        }
    }

    pub fn check_move_player_x(
        player: &AccountInfo,
        game: &Game,
    ) -> Result<(), ProgramError> {
        if player.key != &game.player_x {
            return Err(TictactoeError::Unauthorized.into());
        }
        if game.player_to_move != Player::PlayerX {
            return Err(TictactoeError::OutOfTurnMove.into());
        }
        Ok(())
    }

    pub fn check_move_player_o(
        player: &AccountInfo,
        game: &Game,
    ) -> Result<(), ProgramError> {
        if player.key != &game.player_o {
            return Err(TictactoeError::Unauthorized.into());
        }
        if game.player_to_move != Player::PlayerO {
            return Err(TictactoeError::OutOfTurnMove.into());
        }
        Ok(())
    }
}
