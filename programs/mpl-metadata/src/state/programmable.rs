pub const TOKEN_RECORD_SEED: &str = "token_record";

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TokenState {
    /// Token account is unlocked; operations are allowed on this account.
    Unlocked,
    /// Token account has been locked; no operations are allowed on this account.
    Locked,
    /// Token account has a `Sale` delegate set; operations are restricted.
    Listed,
}

pub const TOKEN_STATE_INDEX: usize = 2;
