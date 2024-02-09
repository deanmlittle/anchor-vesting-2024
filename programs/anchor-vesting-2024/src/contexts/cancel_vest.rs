use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

use crate::{errors::VestingError, state::{Config, Vest}};

#[derive(Accounts)]
pub struct CancelVest<'info> {
    #[account(mut)]
    admin: Signer<'info>,
    mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        constraint = config.finalized == false @ VestingError::VestingFinalized, // Vesting cannot be cancelled after vest is finalized
        has_one = admin, // Arbitrary check as admin is baked into the PDA
        has_one = mint, // Arbitrary check as mint is baked into the PDA
        seeds = [b"config", admin.key().as_ref(), mint.key().as_ref(), config.seed.to_le_bytes().as_ref()],
        bump = config.bump
    )]
    config: Account<'info, Config>,
    #[account(
        mut,
        close = admin,
        seeds = [b"vest", vest.vester_ta.key().as_ref(), vest.timeout.to_le_bytes().as_ref()],
        bump = vest.bump
    )]
    vest: Account<'info, Vest>,    
    system_program: Program<'info, System>
}

impl<'info> CancelVest<'info> {
    pub fn cancel_vest(&mut self) -> Result<()> {
        self.config.vested = self.config.vested.checked_sub(self.vest.amount).ok_or(VestingError::Underflow)?;
        Ok(())
    }
}