use borsh::BorshDeserialize;

// -----------------
// Instruction
// -----------------
#[derive(BorshDeserialize)]
pub enum TictactoeInstruction {
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
