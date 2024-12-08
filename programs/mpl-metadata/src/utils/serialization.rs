extern crate alloc;

use alloc::vec::Vec;
use pinocchio::pubkey::Pubkey;

pub fn serialize_pubkey(pubkey: &Pubkey, buffer: &mut Vec<u8>) {
    buffer.extend_from_slice(pubkey.as_ref());
}

pub fn deserialize_pubkey(data: &[u8]) -> Result<Pubkey, &'static str> {
    if data.len() != 32 {
        return Err("Invalid Pubkey length");
    }
    let mut array = [0u8; 32];
    array.copy_from_slice(data);
    Pubkey::try_from(array).map_err(|_| "Invalid Pubkey data")
}
