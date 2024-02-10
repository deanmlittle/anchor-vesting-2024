use anchor_lang::prelude::*;

#[error_code]
pub enum VestingError {
    #[msg("Locktime not matured yet")]
    NotMatured,
    #[msg("Vault is not in surplus")]
    NotInSurplus,
    #[msg("Vesting finalized")]
    VestingFinalized,
    #[msg("Vesting unfinalized")]
    VestingUnfinalized,
    #[msg("Integer overflow")]
    Overflow,
    #[msg("Integer underflow")]
    Underflow,
}