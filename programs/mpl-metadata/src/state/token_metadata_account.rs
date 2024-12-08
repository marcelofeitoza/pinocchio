extern crate alloc;
use super::*;
use crate::{error::MetadataError, state::key::Key, utils::assert_owned_by, ID};
use alloc::vec;
use alloc::vec::Vec;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError};

pub trait TokenMetadataAccount {
    fn key() -> Key;

    fn size() -> usize;

    fn is_correct_account_type(data: &[u8], data_type: Key, data_size: usize) -> bool {
        if data.is_empty() {
            return false;
        }

        let key: Option<Key> = Key::try_from(data[0])
            .map_err(|_| DeserializeError::DataTypeMismatch)
            .ok();
        match key {
            Some(key) => {
                (key == data_type || key == Key::Uninitialized)
                    && (data.len() == data_size || data_size == 0)
            }
            None => false,
        }
    }

    fn pad_length(buf: &mut Vec<u8>) -> Result<(), MetadataError> {
        if Self::size() != 0 {
            let padding_length = Self::size()
                .checked_sub(buf.len())
                .ok_or(MetadataError::NumericalOverflowError)?;
            buf.extend(vec![0u8; padding_length]);
        }
        Ok(())
    }

    fn safe_deserialize(data: &[u8]) -> Result<Self, DeserializeError>
    where
        Self: Sized;

    fn from_account_info(a: &AccountInfo) -> Result<Self, ProgramError>
    where
        Self: Sized,
    {
        let data = &a.try_borrow_data()?;

        let ua: Self = Self::safe_deserialize(data).map_err(|_| MetadataError::DataTypeMismatch)?;

        // Check that this is a `token-metadata` owned account.
        assert_owned_by(a, &ID)?;

        Ok(ua)
    }
}
