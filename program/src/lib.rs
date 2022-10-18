use solana_program::declare_id;

mod entrypoint;
mod error;
mod instructions;
mod processor;

pub use error::*;
pub use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
