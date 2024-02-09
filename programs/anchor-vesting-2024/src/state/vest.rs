use anchor_lang::prelude::*;

#[account]
pub struct Vest {
    pub vester_ta: Pubkey,
    pub amount: u64,
    pub timeout: i64,
    pub bump: u8
}

impl Space for Vest {
    const INIT_SPACE: usize = 8 + 32 + 8 + 8 + 1;
}