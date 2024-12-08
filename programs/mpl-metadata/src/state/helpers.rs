use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

/// Extracts a fixed-size array from a byte slice.
pub fn extract_fixed_array<const N: usize>(
    data: &[u8],
    start: usize,
) -> Result<[u8; N], ProgramError> {
    if data.len() < start + N {
        return Err(ProgramError::InvalidAccountData);
    }
    data[start..start + N]
        .try_into()
        .map_err(|_| ProgramError::InvalidAccountData)
}

/// Extracts a u64 from a byte slice.
pub fn extract_u64(data: &[u8], start: usize) -> Result<u64, ProgramError> {
    if data.len() < start + 8 {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(u64::from_le_bytes(
        data[start..start + 8]
            .try_into()
            .map_err(|_| ProgramError::InvalidAccountData)?,
    ))
}

/// Extracts a u8 from a byte slice.
pub fn extract_u8(data: &[u8], start: usize) -> Result<u8, ProgramError> {
    if data.len() < start + 1 {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(data[start])
}

pub fn assert_owned_by(account: &AccountInfo, owner: &Pubkey) -> Result<(), ProgramError> {
    if account.owner() != owner {
        return Err(ProgramError::IncorrectProgramId);
    }
    Ok(())
}
