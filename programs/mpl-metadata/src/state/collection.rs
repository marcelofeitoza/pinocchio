use pinocchio::pubkey::Pubkey;

#[repr(C)]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Collection {
    pub verified: bool,
    pub key: Pubkey,
}

#[repr(C)]
#[derive(PartialEq, Eq, Clone, Debug)]
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