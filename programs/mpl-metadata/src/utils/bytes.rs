use pinocchio::pubkey::{Pubkey, PUBKEY_BYTES};

pub fn bytes_to_pubkey(bytes: &[u8]) -> Option<Pubkey> {
    if bytes.len() != 32 {
        return None;
    }
    let mut array = [0u8; 32];
    array.copy_from_slice(bytes);
    Some(Pubkey::try_from(array).ok()?)
}

pub fn pubkey_to_bytes(pubkey: &Pubkey) -> [u8; PUBKEY_BYTES] {
    *pubkey
}
