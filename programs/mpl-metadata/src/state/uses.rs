use alloc::{vec, vec::Vec};
use pinocchio::{account_info::AccountInfo, program_error::ProgramError};

use crate::{error::MetadataError, state::key::Key, utils::assert_owned_by, ID};

pub const USE_AUTHORITY_RECORD_SIZE: usize = 18; //8 byte padding

#[repr(u8)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum UseMethod {
    Burn = 0,
    Multiple = 1,
    Single = 2,
}

impl UseMethod {
    pub fn from_u8(value: u8) -> Result<Self, DeserializeError> {
        match value {
            0 => Ok(UseMethod::Burn),
            1 => Ok(UseMethod::Multiple),
            2 => Ok(UseMethod::Single),
            _ => Err(DeserializeError::InvalidData),
        }
    }
}

#[derive(Debug)]
pub enum DeserializeError {
    DataTypeMismatch,
    InvalidData,
    NumericalOverflowError,
}

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
            Some(k) => {
                (k == data_type || k == Key::Uninitialized)
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

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Uses {
    pub use_method: UseMethod, // 1
    pub remaining: u64,        // 8
    pub total: u64,            // 8
}

impl Uses {
    pub const LEN: usize = 1 + 8 + 8;

    /// Serialize Creator into bytes
    pub fn serialize(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.use_method as u8);
        buffer.extend_from_slice(&self.remaining.to_le_bytes());
        buffer.extend_from_slice(&self.total.to_le_bytes());
    }

    /// Deserialize Creator from bytes
    pub fn deserialize(data: &[u8]) -> Result<Self, &'static str> {
        if data.len() < Self::LEN {
            return Err("Insufficient data for Uses");
        }

        let use_method = UseMethod::from_u8(data[0]).map_err(|_| "Invalid UseMethod")?;
        let remaining = u64::from_le_bytes(data[1..9].try_into().unwrap());
        let total = u64::from_le_bytes(data[9..17].try_into().unwrap());

        Ok(Uses {
            use_method,
            remaining,
            total,
        })
    }
}

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UseAuthorityRecord {
    pub key: Key,          // 1
    pub allowed_uses: u64, // 8
    pub bump: u8,          // 1
}

impl Default for UseAuthorityRecord {
    fn default() -> Self {
        UseAuthorityRecord {
            key: Key::UseAuthorityRecord,
            allowed_uses: 0,
            bump: 255,
        }
    }
}

impl TokenMetadataAccount for UseAuthorityRecord {
    fn key() -> Key {
        Key::UseAuthorityRecord
    }

    fn size() -> usize {
        USE_AUTHORITY_RECORD_SIZE
    }

    fn safe_deserialize(data: &[u8]) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        if data.len() != USE_AUTHORITY_RECORD_SIZE {
            return Err(DeserializeError::InvalidData);
        }

        // Check the key
        let key = Key::try_from(data[0]).map_err(|_| DeserializeError::DataTypeMismatch)?;
        if key != Key::UseAuthorityRecord {
            return Err(DeserializeError::DataTypeMismatch);
        }

        // Extract allowed_uses
        let allowed_uses = u64::from_le_bytes(
            data[1..9]
                .try_into()
                .map_err(|_| DeserializeError::InvalidData)?,
        );

        // Extract bump
        let bump = data[9];

        Ok(UseAuthorityRecord {
            key,
            allowed_uses,
            bump,
        })
    }
}

impl UseAuthorityRecord {
    pub fn from_bytes(b: &[u8]) -> Result<UseAuthorityRecord, ProgramError> {
        let ua: UseAuthorityRecord =
            Self::safe_deserialize(b).map_err(|_| MetadataError::DataTypeMismatch)?;
        Ok(ua)
    }

    pub fn bump_empty(&self) -> bool {
        self.bump == 0 && self.key == Key::UseAuthorityRecord
    }
}
