use pinocchio::pubkey::Pubkey;

pub const MAX_CREATOR_LIMIT: usize = 5;
pub const MAX_CREATOR_LEN: usize = 32 + 1 + 1;

#[repr(C)]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Creator {
    pub address: Pubkey,
    pub verified: bool,
    // In percentages, NOT basis points ;) Watch out!
    pub share: u8,
}
