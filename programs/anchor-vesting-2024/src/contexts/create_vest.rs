use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{errors::VestingError, state::{Config, Vest}};

#[derive(Accounts)]
#[instruction(timeout: i64)]
pub struct CreateVest<'info> {
    #[account(mut)]
    admin: Signer<'info>,
    mint: InterfaceAccount<'info, Mint>,
    #[account(
        token::mint = mint,
        token::token_program = token_program
    )]
    vester_ta: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        constraint = config.finalized == false @ VestingError::VestingFinalized, // Vesting cannot be cancelled after vest is finalized
        has_one = admin, // This check is arbitrary, as mint is baked into the PDA
        has_one = mint, // This check is arbitrary, as mint is baked into the PDA
        seeds = [b"config", admin.key().as_ref(), mint.key().as_ref(), config.seed.to_le_bytes().as_ref()],
        bump = config.bump
    )]
    config: Account<'info, Config>,
    #[account(
        init,
        payer = admin,
        constraint = println!("{:?}", timeout.to_le_bytes()) == (),
        space = Vest::INIT_SPACE,
        seeds = [b"vest", vester_ta.key().as_ref(), timeout.to_le_bytes().as_ref()],
        bump
    )]
    vest: Account<'info, Vest>,
    token_program: Interface<'info, TokenInterface>,
    system_program: Program<'info, System>
}

impl<'info> CreateVest<'info> {
    pub fn create_vest(&mut self, timeout: i64, amount: u64, bump: u8) -> Result<()> {
        // Add to total vested amount
        self.config.vested = self.config.vested.checked_add(amount).ok_or(VestingError::Overflow)?;

        self.vest.set_inner(Vest {
            vester_ta: self.vester_ta.key(),
            amount,
            timeout,
            bump
        });

        Ok(())
    }
}