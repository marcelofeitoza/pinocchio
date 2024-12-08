use pinocchio::{account_info::AccountInfo, program_error::ProgramError};

pub trait Resizable {
    fn save(&self, account_info: &AccountInfo) -> Result<(), ProgramError>;
    fn load(&self, account_info: &AccountInfo) -> Result<Self, ProgramError>
    where
        Self: Sized;
}
