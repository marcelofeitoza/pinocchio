#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Key {
    Uninitialized,
    EditionV1,
    MasterEditionV1,
    ReservationListV1,
    MetadataV1,
    ReservationListV2,
    MasterEditionV2,
    EditionMarker,
    UseAuthorityRecord,
    CollectionAuthorityRecord,
    TokenOwnedEscrow,
    TokenRecord,
    MetadataDelegate,
    EditionMarkerV2,
    HolderDelegate,
}

impl TryFrom<u8> for Key {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Key::Uninitialized),
            1 => Ok(Key::EditionV1),
            2 => Ok(Key::MasterEditionV1),
            3 => Ok(Key::ReservationListV1),
            4 => Ok(Key::MetadataV1),
            5 => Ok(Key::ReservationListV2),
            6 => Ok(Key::MasterEditionV2),
            7 => Ok(Key::EditionMarker),
            8 => Ok(Key::UseAuthorityRecord),
            9 => Ok(Key::CollectionAuthorityRecord),
            10 => Ok(Key::TokenOwnedEscrow),
            11 => Ok(Key::TokenRecord),
            12 => Ok(Key::MetadataDelegate),
            13 => Ok(Key::EditionMarkerV2),
            14 => Ok(Key::HolderDelegate),
            _ => Err(()),
        }
    }
}
