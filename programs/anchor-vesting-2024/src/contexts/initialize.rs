use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::state::Config;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    admin: Signer<'info>,
    mint: InterfaceAccount<'info, Mint>,
    // Initialize a vault for us to store our money in escrow for vesting
    #[account(
        init,
        payer = admin,
        associated_token::mint = mint,
        associated_token::authority = config,
        associated_token::token_program = token_program
    )]
    vault: InterfaceAccount<'info, TokenAccount>,
    // Initialize a vesting config for a specific admin, mint and seed
    #[account(
        init,
        payer = admin,
        space = Config::INIT_SPACE,
        seeds = [b"config", admin.key().key().as_ref(), mint.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    config: Account<'info, Config>,
    associated_token_program: Program<'info, AssociatedToken>,
    token_program: Interface<'info, TokenInterface>,
    system_program: Program<'info, System>
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, seed: u64, bump: u8) -> Result<()> {
        self.config.set_inner(Config {
            mint: self.mint.key(),
            admin: self.admin.key(),
            vested: 0,
            finalized: false,
            seed,
            bump
        });
        Ok(())
    }
}