use core::slice::from_raw_parts;

use pinocchio::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction, Signer},
    program::invoke_signed,
    pubkey::Pubkey,
    ProgramResult,
};

use crate::{write_bytes, UNINIT_BYTE};

/// Initialize a new mint.
///
/// ### Accounts:
///   0. `[WRITABLE]` Mint account
pub struct InitilizeMint2<'a> {
    /// Mint Account.
    pub mint: &'a AccountInfo,
    /// Decimals.
    pub decimals: u8,
    /// Mint Authority.
    pub mint_authority: &'a Pubkey,
    /// Freeze Authority.
    pub freeze_authority: Option<&'a Pubkey>,
}

impl<'a> InitilizeMint2<'a> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[Signer]) -> ProgramResult {
        // Account metadata
        let account_metas: [AccountMeta; 1] = [AccountMeta::writable(self.mint.key())];

        // Instruction data layout:
        // -  [0]: instruction discriminator
        // -  [1]: decimals
        // -  [2..34]: mint_authority
        // -  [34..35]: freeze_authority presence flag
        // -  [35..67]: freeze_authority
        let mut instruction_data = [UNINIT_BYTE; 67];

        // Set discriminator as u8 at offset [0]
        write_bytes(&mut instruction_data, &[20]);
        // Set decimals as u8 at offset [1]
        write_bytes(&mut instruction_data[1..2], &[self.decimals]);
        // Set mint_authority as Pubkey at offset [2..34]
        write_bytes(&mut instruction_data[2..34], self.mint_authority);
        // Set COption & freeze_authority at offset [34..67]
        if let Some(freeze_auth) = self.freeze_authority {
            write_bytes(&mut instruction_data[34..35], &[1]);
            write_bytes(&mut instruction_data[35..], freeze_auth);
        } else {
            write_bytes(&mut instruction_data[34..35], &[0]);
        }

        let instruction = Instruction {
            program_id: &crate::ID,
            accounts: &account_metas,
            data: unsafe { from_raw_parts(instruction_data.as_ptr() as _, 67) },
        };

        invoke_signed(&instruction, &[self.mint], signers)
    }
}