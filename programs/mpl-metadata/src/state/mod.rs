extern crate alloc;
use alloc::vec::Vec;
use core::result::Result;
use pinocchio::account_info::AccountInfo;
use pinocchio::program_error::ProgramError;

pub(crate) mod collection;
pub(crate) mod creator;
pub(crate) mod data;
pub(crate) mod helpers;
pub(crate) mod metadata;
pub(crate) mod uses;

pub use collection::*;
pub use creator::*;
pub use data::*;
pub use helpers::*;
pub use metadata::*;
pub use uses::*;

#[repr(C)]
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Key {
    Uninitialized,
    EditionV1,
    MasterEditionV1,
    ReservationListV1,
    MetadataV1,
    ReservationListV2,
    MasterEditionV2,
    EditionMarker,
    UseAuthorityRecord,
    CollectionAuthorityRecord,
    TokenOwnedEscrow,
    TokenRecord,
    MetadataDelegate,
    EditionMarkerV2,
    HolderDelegate,
}

impl TryFrom<u8> for Key {
    type Error = MetadataError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Key::Uninitialized),
            1 => Ok(Key::EditionV1),
            2 => Ok(Key::MasterEditionV1),
            3 => Ok(Key::ReservationListV1),
            4 => Ok(Key::MetadataV1),
            5 => Ok(Key::ReservationListV2),
            6 => Ok(Key::MasterEditionV2),
            7 => Ok(Key::EditionMarker),
            8 => Ok(Key::UseAuthorityRecord),
            9 => Ok(Key::CollectionAuthorityRecord),
            10 => Ok(Key::TokenOwnedEscrow),
            11 => Ok(Key::TokenRecord),
            12 => Ok(Key::MetadataDelegate),
            13 => Ok(Key::EditionMarkerV2),
            14 => Ok(Key::HolderDelegate),
            _ => Err(MetadataError::InvalidKey),
        }
    }
}


pub trait TokenMetadataAccount {
    /// Returns the `Key` associated with the account type.
    fn key() -> Key;

    /// Returns the size of the account data.
    fn size() -> usize;

    /// Validates if the data corresponds to the expected account type and size.
    fn is_correct_account_type(data: &[u8], data_type: Key, data_size: usize) -> bool {
        if data.is_empty() {
            return false;
        }

        let key: Option<Key> = Key::try_from(data[0]).ok();
        match key {
            Some(k) => {
                (k == data_type || k == Key::Uninitialized)
                    && (data.len() == data_size || data_size == 0)
            }
            None => false,
        }
    }

    /// Pads the buffer to match the required size.
    fn pad_length(buf: &mut Vec<u8>) -> Result<(), MetadataError> {
        if Self::size() != 0 {
            let padding_length = Self::size()
                .checked_sub(buf.len())
                .ok_or(MetadataError::NumericalOverflowError)?;
            buf.extend(alloc::vec![0; padding_length]);
        }
        Ok(())
    }

    /// Safely deserializes the data into the account type.
    fn safe_deserialize(data: &[u8]) -> Result<Self, ProgramError>
    where
        Self: Sized;

    /// Initializes the account from `AccountInfo`.
    fn from_account_info(a: &AccountInfo) -> Result<Self, ProgramError>
    where
        Self: Sized,
    {
        let data = &a.try_borrow_data()?;

        let account = Self::safe_deserialize(data)?;

        // Ensure the account is owned by the Token Metadata program.
        assert_owned_by(a, &crate::ID)?;

        Ok(account)
    }
}

// Metadata error definitions
#[derive(Debug, Clone, PartialEq)]
pub enum MetadataError {
    NumericalOverflowError,
    InvalidAccountData,
    InvalidKey
}
