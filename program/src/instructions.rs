use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;
use solana_program::pubkey::Pubkey;

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
    PlayerJoin,
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
#[derive(BorshDeserialize)]
pub struct PlayerMove {
    pub x_or_o: u8,
    pub field: u8,
}
