use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey,
};

use crate::TictactoeError;

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
