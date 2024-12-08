use crate::ID;

use super::*;

use core::option::Option;
use pinocchio::{
    account_info::{AccountInfo, Ref},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use core::convert::TryFrom;

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Metadata {
    /// Account discriminator.
    pub key: Key,
    /// Address of the update authority.
    pub update_authority: Pubkey,
    /// Address of the mint.
    pub mint: Pubkey,
    /// Asset data.
    pub data: Data,
    // Immutable, once flipped, all sales of this metadata are considered secondary.
    pub primary_sale_happened: bool,
    // Whether or not the data struct is mutable, default is not
    pub is_mutable: bool,
    /// nonce for easy calculation of editions, if present
    pub edition_nonce: Option<u8>,
    /// Since we cannot easily change Metadata, we add the new DataV2 fields here at the end.
    pub token_standard: Option<TokenStandard>,
    /// Collection
    pub collection: Option<Collection>,
    /// Uses
    pub uses: Option<Uses>,
    /// Collection Details
    pub collection_details: Option<CollectionDetails>,
    /// Programmable Config
    pub programmable_config: Option<ProgrammableConfig>,
}

impl Metadata {
    pub const LEN: usize = 288;

    pub fn from_account_info(account_info: &AccountInfo) -> Result<Ref<Self>, ProgramError> {
        if account_info.data_len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        if account_info.owner() != &ID {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(Ref::map(account_info.try_borrow_data()?, |data| {
            Self::from_bytes(data)
        }))
    }

    pub fn from_bytes(data: &[u8]) -> &Self {
        assert_eq!(data.len(), Self::LEN);
        unsafe { &*(data.as_ptr() as *const Self) }
    }

    /// # Safety
    ///
    /// This function is unsafe because it performs a raw pointer cast.
    /// The caller must ensure that the provided byte slice is correctly aligned
    /// and of the correct length for the `Metadata` type.
    pub unsafe fn from_bytes_unsafe(bytes: &[u8]) -> &Self {
        &*(bytes.as_ptr() as *const Metadata)
    }

    // Getter methods
    pub fn key(&self) -> &Key {
            &self.key
        }
}

#[repr(C)]
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum TokenStandard {
    NonFungible,                    // This is a master edition
    FungibleAsset,                  // A token with metadata that can also have attributes
    Fungible,                       // A token with simple metadata
    NonFungibleEdition,             // This is a limited edition
    ProgrammableNonFungible,        // NonFungible with programmable configuration
    ProgrammableNonFungibleEdition, // NonFungible with programmable configuration
}

impl TryFrom<u8> for TokenStandard {
    type Error = MetadataError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TokenStandard::NonFungible),
            1 => Ok(TokenStandard::FungibleAsset),
            2 => Ok(TokenStandard::Fungible),
            3 => Ok(TokenStandard::NonFungibleEdition),
            4 => Ok(TokenStandard::ProgrammableNonFungible),
            5 => Ok(TokenStandard::ProgrammableNonFungibleEdition),
            _ => Err(MetadataError::InvalidTokenStandard),
        }
    }
}


pub trait TokenMetadataAccount {
    fn key() -> Key;

    fn size() -> usize;

    fn is_correct_account_type(data: &[u8], data_type: Key, data_size: usize) -> bool {
        if data.is_empty() {
            return false;
        }

        match Key::try_from(data[0]) {
            Ok(k) => {
                (k == data_type || k == Key::Uninitialized)
                    && (data.len() == data_size || data_size == 0)
            }
            Err(_) => false,
        }
    }

    fn pad_length(buf: &mut Vec<u8>) -> Result<(), MetadataError> {
        if Self::size() != 0 {
            let padding_length = Self::size()
                .checked_sub(buf.len())
                .ok_or(MetadataError::NumericalOverflowError)?;
            buf.extend(alloc::vec![0; padding_length]);
        }
        Ok(())
    }

    fn safe_deserialize(data: &[u8]) -> Result<Self, ProgramError>
    where
        Self: Sized;

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

#[derive(Debug, Clone, PartialEq)]
pub enum MetadataError {
    NumericalOverflowError,
    InvalidAccountData,
    InvalidTokenStandard,
    InvalidKey,
}

#[repr(C)]
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ProgrammableConfig {
    V1 {
        /// Programmable authorization rules.
        rule_set: Option<Pubkey>,
    },
}
