use borsh::BorshDeserialize;
use shank::ShankInstruction;

// -----------------
// Instruction
// -----------------
#[derive(BorshDeserialize, ShankInstruction)]
#[rustfmt::skip]
pub enum TictactoeInstruction {
    #[account(name = "player_x", signer, desc = "The player initializing the game")]
    #[account(name = "game_pda", mut, desc="The game PDA")]
    #[account(name = "system_program", desc="The system program")]
    InitializeGame,
    PlayerJoin,
    PlayerMove(PlayerMove),
}

// -----------------
// Player Move
// -----------------
#[derive(BorshDeserialize)]
pub struct PlayerMove {
    pub x_or_o: u8,
    pub field: u8,
}
