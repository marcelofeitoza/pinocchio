use alloc::{string::String, vec::Vec};
use pinocchio::pubkey::Pubkey;

use super::*;

/// Data representation of an asset.
#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AssetData {
    /// The name of the asset.
    pub name: String,
    /// The symbol for the asset.
    pub symbol: String,
    /// URI pointing to JSON representing the asset.
    pub uri: String,
    /// Royalty basis points that goes to creators in secondary sales (0-10000).
    pub seller_fee_basis_points: u16,
    /// Array of creators.
    pub creators: Option<Vec<Creator>>,
    // Immutable, once flipped, all sales of this metadata are considered secondary.
    pub primary_sale_happened: bool,
    // Whether or not the data struct is mutable (default is not).
    pub is_mutable: bool,
    /// Type of the token.
    pub token_standard: TokenStandard,
    /// Collection information.
    pub collection: Option<Collection>,
    /// Uses information.
    pub uses: Option<Uses>,
    /// Collection item details.
    pub collection_details: Option<CollectionDetails>,
    /// Programmable rule set for the asset.
    pub rule_set: Option<Pubkey>,
}

impl AssetData {
    pub fn new(token_standard: TokenStandard, name: String, symbol: String, uri: String) -> Self {
        Self {
            name,
            symbol,
            uri,
            seller_fee_basis_points: 0,
            creators: None,
            primary_sale_happened: false,
            is_mutable: true,
            token_standard,
            collection: None,
            uses: None,
            collection_details: None,
            rule_set: None,
        }
    }

    pub fn as_data_v2(&self) -> DataV2 {
        DataV2 {
            collection: self.collection.clone(),
            creators: self.creators.clone(),
            name: self.name.clone(),
            seller_fee_basis_points: self.seller_fee_basis_points,
            symbol: self.symbol.clone(),
            uri: self.uri.clone(),
            uses: self.uses.clone(),
        }
    }

    pub fn as_data(&self) -> Data {
        Data {
            name: self.name.clone(),
            symbol: self.symbol.clone(),
            uri: self.uri.clone(),
            seller_fee_basis_points: self.seller_fee_basis_points,
            creators: self.creators.clone(),
        }
    }

    /// Calculate the length of the serialized data.
    pub fn calculate_serialized_length(&self) -> usize {
        let mut size = 0;
        size += 4 + self.name.len(); // 4 bytes for name length
        size += 4 + self.symbol.len(); // 4 bytes for symbol length
        size += 4 + self.uri.len(); // 4 bytes for uri length
        size += 2; // u16 for seller_fee_basis_points
        size += 1; // 1 byte for Option<creators> presence flag
        if let Some(creators) = &self.creators {
            size += 4; // 4 bytes for creators length
            size += creators.len() * Creator::LEN; // Assuming fixed length for each Creator
        }
        size += 1; // primary_sale_happened as u8
        size += 1; // is_mutable as u8
        size += 1; // token_standard as u8
        size += 1; // Option<collection> presence flag
        if self.collection.is_some() {
            size += Collection::LEN;
        }
        size += 1; // Option<uses> presence flag
        if self.uses.is_some() {
            size += Uses::LEN;
        }
        size += 1; // Option<collection_details> presence flag
        if self.collection_details.is_some() {
            size += CollectionDetails::LEN;
        }
        size += 1; // Option<rule_set> presence flag
        if self.rule_set.is_some() {
            size += 32; // Pubkey is 32 bytes
        }
        size
    }

    /// Serialize the AssetData into a byte vector using Pinocchio.
    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.calculate_serialized_length());

        // Serialize name
        buf.extend(&(self.name.len() as u32).to_le_bytes());
        buf.extend(self.name.as_bytes());

        // Serialize symbol
        buf.extend(&(self.symbol.len() as u32).to_le_bytes());
        buf.extend(self.symbol.as_bytes());

        // Serialize uri
        buf.extend(&(self.uri.len() as u32).to_le_bytes());
        buf.extend(self.uri.as_bytes());

        // Serialize seller_fee_basis_points
        buf.extend(&self.seller_fee_basis_points.to_le_bytes());

        // Serialize creators
        match &self.creators {
            Some(creators) => {
                buf.push(1); // Presence flag
                buf.extend(&(creators.len() as u32).to_le_bytes());
                for creator in creators {
                    creator.serialize(&mut buf);
                }
            }
            None => {
                buf.push(0); // Absence flag
            }
        }

        // Serialize primary_sale_happened
        buf.push(self.primary_sale_happened as u8);

        // Serialize is_mutable
        buf.push(self.is_mutable as u8);

        // Serialize token_standard
        buf.push(self.token_standard as u8);

        // Serialize collection
        match &self.collection {
            Some(collection) => {
                buf.push(1); // Presence flag
                collection.serialize(&mut buf);
            }
            None => {
                buf.push(0); // Absence flag
            }
        }

        // Serialize uses
        match &self.uses {
            Some(uses) => {
                buf.push(1); // Presence flag
                uses.serialize(&mut buf);
            }
            None => {
                buf.push(0); // Absence flag
            }
        }

        // Serialize collection_details
        match &self.collection_details {
            Some(details) => {
                buf.push(1); // Presence flag
                details.serialize(&mut buf);
            }
            None => {
                buf.push(0); // Absence flag
            }
        }

        // Serialize rule_set
        match &self.rule_set {
            Some(rule_set) => {
                buf.push(1); // Presence flag
                buf.extend(rule_set.as_ref());
            }
            None => {
                buf.push(0); // Absence flag
            }
        }

        buf
    }

    /// Deserialize AssetData from a byte slice using Pinocchio.
    pub fn deserialize(data: &[u8]) -> Result<Self, DeserializeError> {
        let mut cursor = 0;

        // Deserialize name
        if cursor + 4 > data.len() {
            return Err(DeserializeError::InvalidData);
        }
        let name_len = u32::from_le_bytes(
            data[cursor..cursor + 4]
                .try_into()
                .map_err(|_| DeserializeError::InvalidData)?,
        ) as usize;
        cursor += 4;
        if cursor + name_len > data.len() {
            return Err(DeserializeError::InvalidData);
        }
        let name = String::from_utf8(data[cursor..cursor + name_len].to_vec())
            .map_err(|_| DeserializeError::InvalidData)?;
        cursor += name_len;

        // Deserialize symbol
        if cursor + 4 > data.len() {
            return Err(DeserializeError::InvalidData);
        }
        let symbol_len = u32::from_le_bytes(
            data[cursor..cursor + 4]
                .try_into()
                .map_err(|_| DeserializeError::InvalidData)?,
        ) as usize;
        cursor += 4;
        if cursor + symbol_len > data.len() {
            return Err(DeserializeError::InvalidData);
        }
        let symbol = String::from_utf8(data[cursor..cursor + symbol_len].to_vec())
            .map_err(|_| DeserializeError::InvalidData)?;
        cursor += symbol_len;

        // Deserialize uri
        if cursor + 4 > data.len() {
            return Err(DeserializeError::InvalidData);
        }
        let uri_len = u32::from_le_bytes(
            data[cursor..cursor + 4]
                .try_into()
                .map_err(|_| DeserializeError::InvalidData)?,
        ) as usize;
        cursor += 4;
        if cursor + uri_len > data.len() {
            return Err(DeserializeError::InvalidData);
        }
        let uri = String::from_utf8(data[cursor..cursor + uri_len].to_vec())
            .map_err(|_| DeserializeError::InvalidData)?;
        cursor += uri_len;

        // Deserialize seller_fee_basis_points
        if cursor + 2 > data.len() {
            return Err(DeserializeError::InvalidData);
        }
        let seller_fee_basis_points = u16::from_le_bytes(
            data[cursor..cursor + 2]
                .try_into()
                .map_err(|_| DeserializeError::InvalidData)?,
        );
        cursor += 2;

        // Deserialize creators
        if cursor + 1 > data.len() {
            return Err(DeserializeError::InvalidData);
        }
        let creators_present = data[cursor];
        cursor += 1;
        let creators = if creators_present == 1 {
            if cursor + 4 > data.len() {
                return Err(DeserializeError::InvalidData);
            }
            let creators_len = u32::from_le_bytes(
                data[cursor..cursor + 4]
                    .try_into()
                    .map_err(|_| DeserializeError::InvalidData)?,
            ) as usize;
            cursor += 4;
            let mut creators_vec = Vec::with_capacity(creators_len);
            for _ in 0..creators_len {
                if cursor + Creator::LEN > data.len() {
                    return Err(DeserializeError::InvalidData);
                }
                let creator =
                    Creator::deserialize(&data[cursor..cursor + Creator::LEN]).map_err(|_| {
                        cursor += Creator::LEN;
                        DeserializeError::InvalidData
                    })?;
                creators_vec.push(creator);
                cursor += Creator::LEN;
            }
            Some(creators_vec)
        } else {
            None
        };

        // Deserialize primary_sale_happened
        if cursor + 1 > data.len() {
            return Err(DeserializeError::InvalidData);
        }
        let primary_sale_happened = match data[cursor] {
            0 => false,
            1 => true,
            _ => return Err(DeserializeError::InvalidData),
        };
        cursor += 1;

        // Deserialize is_mutable
        if cursor + 1 > data.len() {
            return Err(DeserializeError::InvalidData);
        }
        let is_mutable = match data[cursor] {
            0 => false,
            1 => true,
            _ => return Err(DeserializeError::InvalidData),
        };
        cursor += 1;

        // Deserialize token_standard
        if cursor + 1 > data.len() {
            return Err(DeserializeError::InvalidData);
        }
        let token_standard =
            TokenStandard::try_from(data[cursor]).map_err(|_| DeserializeError::InvalidData)?;
        cursor += 1;

        // Deserialize collection
        if cursor + 1 > data.len() {
            return Err(DeserializeError::InvalidData);
        }
        let collection_present = data[cursor];
        cursor += 1;
        let collection = if collection_present == 1 {
            if cursor + Collection::LEN > data.len() {
                return Err(DeserializeError::InvalidData);
            }
            let coll =
                Collection::deserialize(&data[cursor..cursor + Collection::LEN]).map_err(|_| {
                    cursor += Collection::LEN;
                    DeserializeError::InvalidData
                })?;
            cursor += Collection::LEN;
            Some(coll)
        } else {
            None
        };

        // Deserialize uses
        if cursor + 1 > data.len() {
            return Err(DeserializeError::InvalidData);
        }
        let uses_present = data[cursor];
        cursor += 1;
        let uses = if uses_present == 1 {
            if cursor + Uses::LEN > data.len() {
                return Err(DeserializeError::InvalidData);
            }
            let u = Uses::deserialize(&data[cursor..cursor + Uses::LEN]).map_err(|_| {
                cursor += Uses::LEN;
                DeserializeError::InvalidData
            })?;
            cursor += Uses::LEN;
            Some(u)
        } else {
            None
        };

        // Deserialize collection_details
        if cursor + 1 > data.len() {
            return Err(DeserializeError::InvalidData);
        }
        let details_present = data[cursor];
        cursor += 1;
        let collection_details = if details_present == 1 {
            if cursor + CollectionDetails::LEN > data.len() {
                return Err(DeserializeError::InvalidData);
            }
            let cd = CollectionDetails::deserialize(&data[cursor..cursor + CollectionDetails::LEN])
                .map_err(|_| {
                    cursor += CollectionDetails::LEN;
                    DeserializeError::InvalidData
                })?;
            cursor += CollectionDetails::LEN;
            Some(cd)
        } else {
            None
        };

        // Deserialize rule_set
        if cursor + 1 > data.len() {
            return Err(DeserializeError::InvalidData);
        }
        let rule_set_present = data[cursor];
        cursor += 1;
        let rule_set = if rule_set_present == 1 {
            if cursor + 32 > data.len() {
                return Err(DeserializeError::InvalidData);
            }
            // let pubkey = Pubkey::from(
            //     data[cursor..cursor + 32]
            //         .try_into()
            //         .map_err(|_| DeserializeError::InvalidData)?,
            // );
            let pubkey = Pubkey::try_from(&data[cursor..cursor + 32])
                .map_err(|_| DeserializeError::InvalidData)?;
            cursor += 32;
            Some(pubkey)
        } else {
            None
        };

        Ok(AssetData {
            name,
            symbol,
            uri,
            seller_fee_basis_points,
            creators,
            primary_sale_happened,
            is_mutable,
            token_standard,
            collection,
            uses,
            collection_details,
            rule_set,
        })
    }
}

/// Custom error type for deserialization.
#[derive(Debug)]
pub enum DeserializeError {
    InvalidData,
    DataTypeMismatch,
}
