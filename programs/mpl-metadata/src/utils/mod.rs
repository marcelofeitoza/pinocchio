pub(crate) mod bytes;
pub(crate) mod misc;
pub(crate) mod serialization;

pub use bytes::*;
pub use misc::*;
use pinocchio::pubkey::Pubkey;
use pinocchio_pubkey::pubkey;
pub use serialization::*;

pub const SPL_TOKEN_ID: Pubkey = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
pub const SYSVAR_ID: Pubkey = pubkey!("Sysvar1nstructions111111111111111111111111");
pub const SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID: Pubkey =
    pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
pub const SPL_TOKEN_2022_ID: Pubkey = pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");
