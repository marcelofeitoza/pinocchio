use core::str;
use pinocchio::{
    account_info::{AccountInfo, Ref},
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::ID;

use super::{ArrayDiscriminator, SplDiscriminate};

/// Maximum lengths for string fields.
const NAME_LEN: usize = 32;
const SYMBOL_LEN: usize = 32;
const URI_LEN: usize = 200;

// Maximum number of key-value pairs.
const MAX_KV_PAIRS: usize = 32;
// Maximum length of keys and values.
const KV_KEY_LEN: usize = 32;
const KV_VALUE_LEN: usize = 32;

/// The `TokenMetadata` account data in a static layout.
/// - Fixed-size arrays for names, symbols, URIs, and key-value pairs.
/// - No runtime expansion without on-chain upgrades.
///
/// **String Encoding:**
/// We'll use null-terminated UTF-8 strings. This means when you set a field,
/// you write the bytes of the string and then a `\0` (null) character, and fill the rest
/// with zeros. When reading, stop at `\0`.
///
/// For key-value pairs, we’ll do the same and treat all entries as potentially used or unused.
/// An unused entry can have a leading `\0` to represent "empty".
#[repr(C)]
pub struct TokenMetadata {
    /// The authority that can update this metadata.
    /// If not used, consider a sentinel value like all-zero pubkey [0x00; 32].
    update_authority: Pubkey,

    /// The associated mint. Must be set to the actual mint this metadata belongs to.
    mint: Pubkey,

    /// The token's name, null-terminated UTF-8, zero-padded.
    name: [u8; NAME_LEN],

    /// The token's symbol, null-terminated UTF-8, zero-padded.
    symbol: [u8; SYMBOL_LEN],

    /// The token's URI, null-terminated UTF-8, zero-padded.
    /// Should point to a resource with extended metadata.
    uri: [u8; URI_LEN],

    /// Additional metadata as a static array of `(key, value)` pairs.
    /// Both key and value are null-terminated UTF-8.
    /// Unused entries can be marked by having a `\0` as the first byte of the key.
    additional_metadata: [([u8; KV_KEY_LEN], [u8; KV_VALUE_LEN]); MAX_KV_PAIRS],
}

impl SplDiscriminate for TokenMetadata {
    const SPL_DISCRIMINATOR: ArrayDiscriminator =
        ArrayDiscriminator::new([112, 132, 90, 90, 11, 88, 157, 87]);
}

/// Utility functions to handle string operations on fixed-size arrays.
impl TokenMetadata {
    pub const LEN: usize = core::mem::size_of::<TokenMetadata>();

    #[inline]
    pub fn from_account_info(
        account_info: &AccountInfo,
    ) -> Result<Ref<TokenMetadata>, ProgramError> {
        if account_info.data_len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        if account_info.owner() != &ID {
            return Err(ProgramError::InvalidAccountOwner);
        }
        Ok(Ref::map(account_info.try_borrow_data()?, |data| unsafe {
            Self::from_bytes(data)
        }))
    }

    /// Return a `TokenMetadata` from the given account info.
    ///
    /// This method performs owner and length validation on `AccountInfo`, but does not
    /// perform the borrow check.
    ///
    /// # Safety
    ///
    /// The caller must ensure that it is safe to borrow the account data – e.g., there are
    /// no mutable borrows of the account data.
    #[inline]
    pub unsafe fn from_account_info_unchecked(
        account_info: &AccountInfo,
    ) -> Result<&TokenMetadata, ProgramError> {
        if account_info.data_len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        if account_info.owner() != &ID {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(Self::from_bytes(account_info.borrow_data_unchecked()))
    }

    /// Return a `TokenMetadata` from the given bytes.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `bytes` contains a valid representation of `TokenMetadata`.
    #[inline(always)]
    pub unsafe fn from_bytes(bytes: &[u8]) -> &Self {
        &*(bytes.as_ptr() as *const TokenMetadata)
    }

    pub fn has_update_authority(&self) -> bool {
        self.update_authority != Pubkey::default()
    }

    pub fn update_authority(&self) -> &Pubkey {
        &self.update_authority
    }

    pub fn mint(&self) -> &Pubkey {
        &self.mint
    }

    pub fn name(&self) -> &str {
        // Find the null terminator or the end of the array.
        let end = self
            .name
            .iter()
            .position(|&c| c == b'\0')
            .unwrap_or(NAME_LEN);
        core::str::from_utf8(&self.name[..end]).unwrap_or_default()
    }

    pub fn symbol(&self) -> &str {
        // Find the null terminator or the end of the array.
        let end = self
            .symbol
            .iter()
            .position(|&c| c == b'\0')
            .unwrap_or(SYMBOL_LEN);
        core::str::from_utf8(&self.symbol[..end]).unwrap_or_default()
    }

    pub fn uri(&self) -> &str {
        // Find the null terminator or the end of the array.
        let end = self.uri.iter().position(|&c| c == b'\0').unwrap_or(URI_LEN);
        core::str::from_utf8(&self.uri[..end]).unwrap_or_default()
    }

    fn read_str_from_bytes(bytes: &[u8]) -> &str {
        let end = bytes
            .iter()
            .position(|&c| c == b'\0')
            .unwrap_or(bytes.len());
        // Safety: Assuming valid UTF-8. If uncertain, consider from_utf8() and handle errors.
        core::str::from_utf8(&bytes[..end]).unwrap_or("")
    }

    fn find_key_index(&self, key: &str) -> Option<usize> {
        self.additional_metadata
            .iter()
            .position(|(k, _)| Self::read_str_from_bytes(k) == key)
    }

    pub fn set_key_value(&mut self, key: &str, value: &str) -> Result<(), ProgramError> {
        if let Some(i) = self.find_key_index(key) {
            // Update existing
            Self::write_str_to_bytes(&mut self.additional_metadata[i].1, value);
            Ok(())
        } else {
            // Find empty slot
            if let Some(i) = self.additional_metadata.iter().position(|(k, _)| k[0] == 0) {
                Self::write_str_to_bytes(&mut self.additional_metadata[i].0, key);
                Self::write_str_to_bytes(&mut self.additional_metadata[i].1, value);
                Ok(())
            } else {
                Err(ProgramError::AccountDataTooSmall)
            }
        }
    }

    pub fn remove_key(&mut self, key: &str) -> bool {
        if let Some(i) = self.find_key_index(key) {
            self.additional_metadata[i].0.fill(0);
            self.additional_metadata[i].1.fill(0);
            true
        } else {
            false
        }
    }

    pub fn get_value(&self, key: &str) -> Option<&str> {
        self.find_key_index(key)
            .map(|i| Self::read_str_from_bytes(&self.additional_metadata[i].1))
    }

    fn write_str_to_bytes(dst: &mut [u8], src: &str) {
        dst.fill(0);
        let bytes = src.as_bytes();
        let len = bytes.len().min(dst.len() - 1);
        dst[..len].copy_from_slice(&bytes[..len]);
        // Null terminator is ensured by zero-filling above.
    }

    pub fn set_name(&mut self, name: &str) {
        Self::write_str_to_bytes(&mut self.name, name);
    }

    pub fn set_symbol(&mut self, symbol: &str) {
        Self::write_str_to_bytes(&mut self.symbol, symbol);
    }

    pub fn set_uri(&mut self, uri: &str) {
        Self::write_str_to_bytes(&mut self.uri, uri);
    }

    pub fn update(&mut self, field: Field) -> Result<(), ProgramError> {
        match field {
            Field::Name(name) => self.set_name(name),
            Field::Symbol(symbol) => self.set_symbol(symbol),
            Field::Uri(uri) => self.set_uri(uri),
            Field::Key(key, value) => self.set_key_value(key, value)?,
        }
        Ok(())
    }
}

#[repr(C)]
pub enum Field<'a> {
    Name(&'a str),
    Symbol(&'a str),
    Uri(&'a str),
    Key(&'a str, &'a str),
}
