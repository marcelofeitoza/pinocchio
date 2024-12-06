use pinocchio::program_error::ProgramError;

/// A simple array-based discriminator (8 bytes).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ArrayDiscriminator([u8; ArrayDiscriminator::LENGTH]);

impl ArrayDiscriminator {
    pub const LENGTH: usize = 8;
    pub const UNINITIALIZED: Self = Self([0; 8]);
    pub const fn new(value: [u8; Self::LENGTH]) -> Self {
        Self(value)
    }
    pub const fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

impl From<[u8; 8]> for ArrayDiscriminator {
    fn from(from: [u8; 8]) -> Self {
        Self(from)
    }
}

impl AsRef<[u8]> for ArrayDiscriminator {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl core::convert::TryFrom<&[u8]> for ArrayDiscriminator {
    type Error = ProgramError;
    fn try_from(a: &[u8]) -> Result<Self, Self::Error> {
        if a.len() == 8 {
            let mut arr = [0u8; 8];
            arr.copy_from_slice(a);
            Ok(Self(arr))
        } else {
            Err(ProgramError::InvalidAccountData)
        }
    }
}

/// A trait for types that have a SPL-compatible 8-byte discriminator.
pub trait SplDiscriminate {
    const SPL_DISCRIMINATOR: ArrayDiscriminator;
    const SPL_DISCRIMINATOR_SLICE: &'static [u8] = Self::SPL_DISCRIMINATOR.as_slice();
}
