use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;
const BOARD_SIZE: usize = 9;

#[derive(Debug, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
pub enum GameState {
    Uninitialized,
    WaitingForOpponent,
    Full,
    Finished,
}

#[derive(Debug, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
pub enum Player {
    PlayerX,
    PlayerO,
}

pub const BOARD_ITEM_FREE: u8 = 0;
pub const BOARD_ITEM_X: u8 = 1;
pub const BOARD_ITEM_O: u8 = 2;

#[rustfmt::skip]
pub const GAME_SIZE: usize = 
    /* player_x       */ 32 + 
    /* player_o       */ 32 + 
    /* board          */ BOARD_SIZE + 
    /* state          */ 1 + 
    /* player_to_move */ 1;

#[derive(Debug, BorshSerialize, BorshDeserialize, ShankAccount)]
pub struct Game {
    pub player_x: Pubkey,
    pub player_o: Pubkey,
    pub board: [u8; 9],
    pub state: GameState,
    pub player_to_move: Player,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            player_x: Default::default(),
            player_o: Default::default(),
            board: Default::default(),
            state: GameState::Uninitialized,
            player_to_move: Player::PlayerX,
        }
    }
}

impl Game {
    pub fn init(player_x: Pubkey) -> Game {
        Game {
            player_x,
            state: GameState::WaitingForOpponent,
            ..Default::default()
        }
    }

    pub fn toggle_player_to_move(&mut self) {
        use Player::*;
        match self.player_to_move {
            PlayerX => self.player_to_move = PlayerO,
            PlayerO => self.player_to_move = PlayerX,
        };
    }

    pub fn update_state(self: &mut Game) {
        // TODO: add an if branch here that determines if there is a winner and
        //       updates a property on the game account, i.e. `winner: Option<Player>`.
        if self.board.iter().all(|&p| p != BOARD_ITEM_FREE) {
            self.state = GameState::Finished;
        } else {
            self.toggle_player_to_move();
        }
    }
}
