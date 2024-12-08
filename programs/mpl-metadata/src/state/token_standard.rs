#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenStandard {
    NonFungible = 0,
    FungibleAsset = 1,
    Fungible = 2,
    NonFungibleEdition = 3,
    ProgrammableNonFungible = 4,
    ProgrammableNonFungibleEdition = 5,
}

impl TryFrom<u8> for TokenStandard {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TokenStandard::NonFungible),
            1 => Ok(TokenStandard::FungibleAsset),
            2 => Ok(TokenStandard::Fungible),
            3 => Ok(TokenStandard::NonFungibleEdition),
            4 => Ok(TokenStandard::ProgrammableNonFungible),
            5 => Ok(TokenStandard::ProgrammableNonFungibleEdition),
            _ => Err(()),
        }
    }
}
