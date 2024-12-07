use pinocchio::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction, Signer},
    program::invoke_signed,
    ProgramResult,
};
extern crate alloc;
use alloc::vec;

pub struct Create<'a> {
    pub metadata_account: &'a AccountInfo,
    pub mint: &'a AccountInfo,
    pub mint_authority: &'a AccountInfo,
    pub payer: &'a AccountInfo,
    pub update_authority: &'a AccountInfo,
    pub name: alloc::string::String,
    pub symbol: alloc::string::String,
    pub uri: alloc::string::String,
    pub seller_fee_basis_points: u16,
}

impl<'a> Create<'a> {
    pub fn invoke_signed(&self, signers: &[Signer]) -> ProgramResult {
        let accounts = [
            AccountMeta::writable(self.metadata_account.key()),
            AccountMeta::readonly(self.mint.key()),
            AccountMeta::readonly_signer(self.mint_authority.key()),
            AccountMeta::readonly_signer(self.payer.key()),
            AccountMeta::readonly(self.update_authority.key()),
        ];

        let mut data = vec![0u8];
        data.extend(self.name.as_bytes());
        data.extend(self.symbol.as_bytes());
        data.extend(self.uri.as_bytes());
        data.extend(&self.seller_fee_basis_points.to_le_bytes());

        let instruction = Instruction {
            program_id: &crate::ID,
            accounts: &accounts,
            data: &data,
        };

        invoke_signed(
            &instruction,
            &[
                self.metadata_account,
                self.mint,
                self.mint_authority,
                self.payer,
            ],
            signers,
        )
    }
}
