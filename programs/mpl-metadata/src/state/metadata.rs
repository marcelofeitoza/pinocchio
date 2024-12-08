use super::*;
use crate::state::asset_data::AssetData;
use crate::state::key::Key;
use crate::state::TokenStandard;
use crate::utils::bytes_to_pubkey;
use alloc::vec::Vec;
use pinocchio::account_info::AccountInfo;
use pinocchio::program_error::ProgramError;
use pinocchio::pubkey::Pubkey;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Metadata {
    pub key: Key,
    pub update_authority: Pubkey,
    pub mint: Pubkey,
    pub data: AssetData,
    pub primary_sale_happened: bool,
    pub is_mutable: bool,
    pub edition_nonce: Option<u8>,
    pub token_standard: Option<TokenStandard>,
    pub collection: Option<Collection>, // Define Collection similarly
    pub uses: Option<Uses>,             // Define Uses similarly
    pub collection_details: Option<CollectionDetails>, // Define CollectionDetails similarly
    pub programmable_config: Option<ProgrammableConfig>, // Define ProgrammableConfig similarly
}

impl Metadata {
    pub fn from_account_info(account_info: &AccountInfo) -> Result<Self, ProgramError> {
        let data = account_info.try_borrow_data()?;
        Self::deserialize(&data).map_err(|_| ProgramError::InvalidAccountData)
    }

    /// Serialize Metadata into bytes
    pub fn serialize(&self, buffer: &mut Vec<u8>) {
        // Serialize key
        buffer.push(self.key as u8);

        // Serialize update_authority
        buffer.extend_from_slice(self.update_authority.as_ref());

        // Serialize mint
        buffer.extend_from_slice(self.mint.as_ref());

        // Serialize data
        self.data.serialize();

        // Serialize primary_sale_happened
        buffer.push(self.primary_sale_happened as u8);

        // Serialize is_mutable
        buffer.push(self.is_mutable as u8);

        // Serialize edition_nonce
        match &self.edition_nonce {
            Some(nonce) => {
                buffer.push(1); // Some
                buffer.push(*nonce);
            }
            None => {
                buffer.push(0); // None
            }
        }

        // Serialize token_standard
        match &self.token_standard {
            Some(ts) => {
                buffer.push(1); // Some
                buffer.push(*ts as u8);
            }
            None => {
                buffer.push(0); // None
            }
        }

        // Serialize collection
        match &self.collection {
            Some(collection) => {
                buffer.push(1); // Some
                collection.serialize(buffer);
            }
            None => {
                buffer.push(0); // None
            }
        }

        // Serialize uses
        match &self.uses {
            Some(uses) => {
                buffer.push(1); // Some
                uses.serialize(buffer);
            }
            None => {
                buffer.push(0); // None
            }
        }

        // Serialize collection_details
        match &self.collection_details {
            Some(details) => {
                buffer.push(1); // Some
                details.serialize(buffer);
            }
            None => {
                buffer.push(0); // None
            }
        }

        // Serialize programmable_config
        match &self.programmable_config {
            Some(config) => {
                buffer.push(1); // Some
                config.serialize(buffer);
            }
            None => {
                buffer.push(0); // None
            }
        }
    }

    /// Deserialize Metadata from bytes
    pub fn deserialize(data: &[u8]) -> Result<Self, &'static str> {
        let mut cursor = 0;

        // Deserialize key
        if cursor + 1 > data.len() {
            return Err("Insufficient data for key");
        }
        let key = Key::try_from(data[cursor]).map_err(|_| "Invalid key")?;
        cursor += 1;

        // Deserialize update_authority
        if cursor + 32 > data.len() {
            return Err("Insufficient data for update_authority");
        }
        let update_authority =
            bytes_to_pubkey(&data[cursor..cursor + 32]).ok_or("Invalid update_authority Pubkey")?;
        cursor += 32;

        // Deserialize mint
        if cursor + 32 > data.len() {
            return Err("Insufficient data for mint");
        }
        let mint = bytes_to_pubkey(&data[cursor..cursor + 32]).ok_or("Invalid mint Pubkey")?;
        cursor += 32;

        // Deserialize data
        let data_length = AssetData::deserialize(&data[cursor..])
            .map_err(|_| "Failed to deserialize AssetData")?;
        // Determine how many bytes were consumed by AssetData
        // This requires knowing the exact byte consumption of AssetData, which can be tracked in its implementation
        // For simplicity, assume a fixed offset or adjust accordingly
        // Placeholder:
        let asset_data = AssetData::deserialize(&data[cursor..])
            .map_err(|_| "Failed to deserialize AssetData")?;
        cursor += asset_data.calculate_serialized_length(); // Implement calculate_serialized_length()

        // Deserialize primary_sale_happened
        if cursor + 1 > data.len() {
            return Err("Insufficient data for primary_sale_happened");
        }
        let primary_sale_happened = data[cursor] != 0;
        cursor += 1;

        // Deserialize is_mutable
        if cursor + 1 > data.len() {
            return Err("Insufficient data for is_mutable");
        }
        let is_mutable = data[cursor] != 0;
        cursor += 1;

        // Deserialize edition_nonce
        if cursor + 1 > data.len() {
            return Err("Insufficient data for edition_nonce option");
        }
        let has_edition_nonce = data[cursor] != 0;
        cursor += 1;
        let edition_nonce = if has_edition_nonce {
            if cursor + 1 > data.len() {
                return Err("Insufficient data for edition_nonce");
            }
            let nonce = data[cursor];
            cursor += 1;
            Some(nonce)
        } else {
            None
        };

        // Deserialize token_standard
        if cursor + 1 > data.len() {
            return Err("Insufficient data for token_standard option");
        }
        let has_token_standard = data[cursor] != 0;
        cursor += 1;
        let token_standard = if has_token_standard {
            if cursor + 1 > data.len() {
                return Err("Insufficient data for token_standard");
            }
            let ts = TokenStandard::try_from(data[cursor]).map_err(|_| "Invalid token_standard")?;
            cursor += 1;
            Some(ts)
        } else {
            None
        };

        // Deserialize collection
        if cursor + 1 > data.len() {
            return Err("Insufficient data for collection option");
        }
        let has_collection = data[cursor] != 0;
        cursor += 1;
        let collection = if has_collection {
            // Implement Collection deserialization similarly
            // Placeholder:
            None // Replace with actual deserialization
        } else {
            None
        };

        // Deserialize uses
        if cursor + 1 > data.len() {
            return Err("Insufficient data for uses option");
        }
        let has_uses = data[cursor] != 0;
        cursor += 1;
        let uses = if has_uses {
            // Implement Uses deserialization similarly
            // Placeholder:
            None // Replace with actual deserialization
        } else {
            None
        };

        // Deserialize collection_details
        if cursor + 1 > data.len() {
            return Err("Insufficient data for collection_details option");
        }
        let has_collection_details = data[cursor] != 0;
        cursor += 1;
        let collection_details = if has_collection_details {
            // Implement CollectionDetails deserialization similarly
            // Placeholder:
            None // Replace with actual deserialization
        } else {
            None
        };

        // Deserialize programmable_config
        if cursor + 1 > data.len() {
            return Err("Insufficient data for programmable_config option");
        }
        let has_programmable_config = data[cursor] != 0;
        cursor += 1;
        let programmable_config = if has_programmable_config {
            // Implement ProgrammableConfig deserialization similarly
            // Placeholder:
            None // Replace with actual deserialization
        } else {
            None
        };

        Ok(Metadata {
            key,
            update_authority,
            mint,
            data: asset_data,
            primary_sale_happened,
            is_mutable,
            edition_nonce,
            token_standard,
            collection,
            uses,
            collection_details,
            programmable_config,
        })
    }

    pub fn calculate_serialized_length(&self) -> usize {
        let mut len = 1 + 32 + 32; // key + update_authority + mint

        len += self.data.calculate_serialized_length(); // Implement calculate_serialized_length() in AssetData

        len += 1 + 1 + 1; // primary_sale_happened + is_mutable + edition_nonce option

        len += match &self.token_standard {
            Some(_) => 1 + 1,
            None => 1,
        };

        len += match &self.collection {
            Some(_) => 1 + 0, // Add actual length if collection is Some
            None => 1,
        };

        len += match &self.uses {
            Some(_) => 1 + 0, // Add actual length if uses is Some
            None => 1,
        };

        len += match &self.collection_details {
            Some(_) => 1 + 0, // Add actual length if collection_details is Some
            None => 1,
        };

        len += match &self.programmable_config {
            Some(_) => 1 + 0, // Add actual length if programmable_config is Some
            None => 1,
        };

        len
    }
}

impl Resizable for Metadata {
    fn save(&self, account_info: &AccountInfo) -> Result<(), ProgramError> {
        let mut buffer = Vec::new();
        self.serialize(&mut buffer);
        account_info.try_borrow_mut_data()?.copy_from_slice(&buffer);
        Ok(())
    }

    fn load(&self, _account_info: &AccountInfo) -> Result<Self, ProgramError>
    where
        Self: Sized,
    {
        // Implement loading logic based on your deserialization
        Err(ProgramError::InvalidAccountData)
    }
}

/// Configuration for programmable assets.
#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ProgrammableConfig {
    V1 {
        /// Programmable authorization rules.
        rule_set: Option<Pubkey>,
    },
}

impl ProgrammableConfig {
    pub fn serialize(&self, buffer: &mut Vec<u8>) {
        match self {
            ProgrammableConfig::V1 { rule_set } => {
                buffer.push(1); // Version identifier for V1
                match rule_set {
                    Some(pubkey) => {
                        buffer.push(1); // Some
                        buffer.extend_from_slice(pubkey.as_ref());
                    }
                    None => {
                        buffer.push(0); // None
                    }
                }
            }
        }
    }
}
