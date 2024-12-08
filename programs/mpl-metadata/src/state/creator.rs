use crate::utils::bytes_to_pubkey;
use pinocchio::pubkey::Pubkey;
extern crate alloc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Creator {
    pub address: Pubkey,
    pub verified: bool,
    pub share: u8,
}

impl Creator {
    pub const LEN: usize = 32 + 1 + 1;

    /// Serialize Creator into bytes
    pub fn serialize(&self, buffer: &mut alloc::vec::Vec<u8>) {
        buffer.extend_from_slice(self.address.as_ref());
        buffer.push(self.verified as u8);
        buffer.push(self.share);
    }

    /// Deserialize Creator from bytes
    pub fn deserialize(data: &[u8]) -> Result<Self, &'static str> {
        if data.len() < Self::LEN {
            return Err("Insufficient data for Creator");
        }

        let address = bytes_to_pubkey(&data[0..32]).ok_or("Invalid Pubkey")?;
        let verified = data[32] != 0;
        let share = data[33];

        Ok(Creator {
            address,
            verified,
            share,
        })
    }
}
