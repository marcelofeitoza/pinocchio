use pinocchio::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction, Signer},
    program::invoke_signed,
    ProgramResult,
};
extern crate alloc;
use alloc::vec;

pub struct TransferMetadata<'a> {
    pub metadata: &'a AccountInfo,
    pub owner: &'a AccountInfo,
    pub new_owner: &'a AccountInfo,
}

impl<'a> TransferMetadata<'a> {
    pub fn invoke_signed(&self, signers: &[Signer]) -> ProgramResult {
        let accounts = [
            AccountMeta::writable(self.metadata.key()),
            AccountMeta::readonly_signer(self.owner.key()),
            AccountMeta::readonly(self.new_owner.key()),
        ];

        let instruction_data = vec![1u8];

        let instruction = Instruction {
            program_id: &crate::ID,
            accounts: &accounts,
            data: &instruction_data,
        };

        invoke_signed(
            &instruction,
            &[self.metadata, self.owner, self.new_owner],
            signers,
        )
    }
}
