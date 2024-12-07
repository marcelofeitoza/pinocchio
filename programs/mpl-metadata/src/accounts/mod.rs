use pinocchio::{
    account_info::{AccountInfo, Ref},
    program_error::ProgramError,
    pubkey::Pubkey,
};
extern crate alloc;
use crate::ID;
use alloc::vec::Vec;

#[repr(C)]
pub struct MetadataAccount {
    pub update_authority: Pubkey,
    pub mint: Pubkey,
    pub data: MetadataData,
    pub primary_sale_happened: u8,
    pub is_mutable: u8,
}

impl MetadataAccount {
    pub const LEN: usize = 679;

    pub fn from_account_info(
        account_info: &AccountInfo,
    ) -> Result<Ref<MetadataAccount>, ProgramError> {
        if account_info.data_len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        if account_info.owner() != &ID {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(Ref::map(account_info.try_borrow_data()?, |data| unsafe {
            &*(data.as_ptr() as *const MetadataAccount)
        }))
    }
}

#[repr(C)]
pub struct MetadataData {
    pub name: [u8; 32],
    pub symbol: [u8; 10],
    pub uri: [u8; 200],
    pub seller_fee_basis_points: u16,
    pub creators: Option<Vec<Creator>>,
}

#[repr(C)]
pub struct Creator {
    pub address: Pubkey,
    pub verified: u8,
    pub share: u8,
}
