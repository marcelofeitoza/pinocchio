extern crate alloc;
use super::*;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Data {
    /// The name of the asset
    pub name: String,
    /// The symbol for the asset
    pub symbol: String,
    /// URI pointing to JSON representing the asset
    pub uri: String,
    /// Royalty basis points that goes to creators in secondary sales (0-10000)
    pub seller_fee_basis_points: u16,
    /// Array of creators, optional
    pub creators: Option<Vec<Creator>>,
}

#[repr(C)]
pub struct DataV2 {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
    pub creators: Option<Vec<Creator>>,
    pub collection: Option<Collection>,
    pub uses: Option<Uses>,
}

impl DataV2 {
    pub fn to_v1(&self) -> Data {
        let ns = self;
        Data {
            name: ns.name.clone(),
            symbol: ns.symbol.clone(),
            uri: ns.uri.clone(),
            seller_fee_basis_points: ns.seller_fee_basis_points,
            creators: ns.creators.clone(),
        }
    }
}
