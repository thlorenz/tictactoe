use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, PartialEq, FromPrimitive)]
pub enum TictactoeError {
    #[error("You are not authorized to perform this action.")]
    Unauthorized = 0x71c7ac,
}

// -----------------
// Trait Impls
// -----------------
impl PrintProgramError for TictactoeError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<TictactoeError> for ProgramError {
    fn from(e: TictactoeError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for TictactoeError {
    fn type_of() -> &'static str {
        "Tictactoe Error"
    }
}
