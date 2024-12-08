use pinocchio::{account_info::AccountInfo, pubkey::Pubkey, ProgramResult};

use crate::error::MetadataError;

pub fn assert_owned_by(account: &AccountInfo, owner: &Pubkey) -> ProgramResult {
    if account.owner() != owner {
        Err(MetadataError::IncorrectOwner.into())
    } else {
        Ok(())
    }
}
