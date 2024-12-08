use alloc::vec::Vec;
use pinocchio::{program_error::ProgramError, pubkey::Pubkey};

use crate::{error::MetadataError, utils::bytes_to_pubkey};

use super::*;

pub const COLLECTION_AUTHORITY_RECORD_SIZE: usize = 35;

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Collection {
    pub verified: bool,
    pub key: Pubkey,
}

impl Collection {
    pub const LEN: usize = 1 + 32;

    /// Serialize Creator into bytes
    pub fn serialize(&self, buffer: &mut alloc::vec::Vec<u8>) {
        buffer.push(self.verified as u8);
        buffer.extend_from_slice(self.key.as_ref());
    }

    /// Deserialize Creator from bytes
    pub fn deserialize(data: &[u8]) -> Result<Self, &'static str> {
        if data.len() < Self::LEN {
            return Err("Insufficient data for Creator");
        }

        let verified = data[0] != 0;
        let key = bytes_to_pubkey(&data[1..33]).ok_or("Invalid Pubkey")?;

        Ok(Collection { verified, key })
    }
}

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CollectionAuthorityRecord {
    pub key: Key,                         //1
    pub bump: u8,                         //1
    pub update_authority: Option<Pubkey>, //33 (1 + 32)
}

impl Default for CollectionAuthorityRecord {
    fn default() -> Self {
        CollectionAuthorityRecord {
            key: Key::CollectionAuthorityRecord,
            bump: 255,
            update_authority: None,
        }
    }
}

impl TokenMetadataAccount for CollectionAuthorityRecord {
    fn key() -> Key {
        Key::CollectionAuthorityRecord
    }

    fn size() -> usize {
        COLLECTION_AUTHORITY_RECORD_SIZE
    }

    fn safe_deserialize(data: &[u8]) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        if data.len() != COLLECTION_AUTHORITY_RECORD_SIZE {
            return Err(DeserializeError::InvalidData);
        }

        // Check the key
        let key = Key::try_from(data[0]).map_err(|_| DeserializeError::DataTypeMismatch)?;
        if key != Key::CollectionAuthorityRecord {
            return Err(DeserializeError::DataTypeMismatch);
        }

        // Extract bump
        let bump = data[1];

        // Extract update_authority
        let update_authority_flag = data[2];
        let update_authority = match update_authority_flag {
            0 => None,
            1 => {
                if data.len() < 3 + 32 {
                    return Err(DeserializeError::InvalidData);
                }
                let pubkey_bytes = &data[3..35];
                // let pubkey = Pubkey::new_from_array({
                //     let mut array = [0u8; 32];
                //     array.copy_from_slice(pubkey_bytes);
                //     array
                // });
                let mut array = [0u8; 32];
                array.copy_from_slice(pubkey_bytes);
                let pubkey = pinocchio::pubkey::Pubkey::try_from(&array[..]).ok();
                pubkey
            }
            _ => return Err(DeserializeError::InvalidData),
        };

        Ok(CollectionAuthorityRecord {
            key,
            bump,
            update_authority,
        })
    }
}

impl CollectionAuthorityRecord {
    pub fn from_bytes(b: &[u8]) -> Result<CollectionAuthorityRecord, ProgramError> {
        let ca: CollectionAuthorityRecord =
            Self::safe_deserialize(b).map_err(|_| MetadataError::DataTypeMismatch)?;
        Ok(ca)
    }
}

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum CollectionDetails {
    #[deprecated(
        since = "1.13.1",
        note = "The collection size tracking feature is deprecated and will soon be removed."
    )]
    V1 {
        size: u64,
    },
    V2 {
        padding: [u8; 8],
    },
}

impl CollectionDetails {
    pub const LEN: usize = 8;

    /// Serialize Creator into bytes
    pub fn serialize(&self, buffer: &mut Vec<u8>) {
        match self {
            CollectionDetails::V1 { size } => {
                buffer.extend_from_slice(&size.to_le_bytes());
            }
            CollectionDetails::V2 { padding } => {
                buffer.extend_from_slice(padding);
            }
        }
    }

    /// Deserialize Creator from bytes
    pub fn deserialize(data: &[u8]) -> Result<Self, &'static str> {
        if data.len() < Self::LEN {
            return Err("Insufficient data for CollectionDetails");
        }

        let size = u64::from_le_bytes(data[0..8].try_into().unwrap());

        if size == 0 {
            let padding: [u8; 8] = data[0..8].try_into().unwrap();
            Ok(CollectionDetails::V2 { padding })
        } else {
            Ok(CollectionDetails::V1 { size })
        }
    }
}

#[cfg(test)]
mod tests {}
