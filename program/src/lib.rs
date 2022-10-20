use solana_program::declare_id;

mod accounts;
mod asserts;
mod entrypoint;
mod error;
mod instructions;
mod processor;
mod utils;

pub use accounts::*;
pub use asserts::*;
pub use error::*;
pub use instructions::*;
pub use utils::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub const TICTACTOE_PREFIX: &str = "tictactoe";
