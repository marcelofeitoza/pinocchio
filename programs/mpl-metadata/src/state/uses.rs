use pinocchio::program_error::ProgramError;

use super::*;

pub const USE_AUTHORITY_RECORD_SIZE: usize = 18; // Including padding

#[repr(C)]
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum UseMethod {
    Burn,
    Multiple,
    Single,
}

#[repr(C)]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Uses {
    // 17 bytes + Option byte
    pub use_method: UseMethod, // 1 byte
    pub remaining: u64,        // 8 bytes
    pub total: u64,            // 8 bytes
}

#[repr(C)]
pub struct UseAuthorityRecord {
    pub key: Key,          // 1 byte
    pub allowed_uses: u64, // 8 bytes
    pub bump: u8,          // 1 byte
    // Padding or reserved bytes to make up USE_AUTHORITY_RECORD_SIZE
    pub padding: [u8; 8], // 8 bytes padding
}

impl Default for UseAuthorityRecord {
    fn default() -> Self {
        UseAuthorityRecord {
            key: Key::UseAuthorityRecord,
            allowed_uses: 0,
            bump: 255,
            padding: [0; 8],
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

    fn safe_deserialize(data: &[u8]) -> Result<Self, ProgramError> {
        UseAuthorityRecord::from_bytes(data)
    }
}

impl UseAuthorityRecord {
    pub fn from_bytes(b: &[u8]) -> Result<UseAuthorityRecord, ProgramError> {
        if b.len() != USE_AUTHORITY_RECORD_SIZE {
            return Err(ProgramError::InvalidAccountData);
        }

        // Deserialize key
        let key = Key::try_from(b[0]).map_err(|_| ProgramError::InvalidAccountData)?;

        if key != Key::UseAuthorityRecord {
            return Err(ProgramError::InvalidAccountData);
        }

        // Deserialize allowed_uses (bytes 1..9)
        let allowed_uses = u64::from_le_bytes(
            b[1..9]
                .try_into()
                .map_err(|_| ProgramError::InvalidAccountData)?,
        );

        // Deserialize bump (byte 9)
        let bump = b[9];

        // Deserialize padding (bytes 10..18)
        let padding = b[10..18]
            .try_into()
            .map_err(|_| ProgramError::InvalidAccountData)?;

        Ok(UseAuthorityRecord {
            key,
            allowed_uses,
            bump,
            padding,
        })
    }

    pub fn bump_empty(&self) -> bool {
        self.bump == 0 && self.key == Key::UseAuthorityRecord
    }
}
