use anchor_lang::prelude::*;

pub mod contexts;
pub mod state;
pub mod errors;
use contexts::*;

declare_id!("3ENQYzPDHyjM81LxAxwEeQ7oEEhKrdLQ1mRPNxvLjzPS");

#[program]
pub mod anchor_vesting_2024 {
    use super::*;

    // Initialize a new Config, setting up a mint, vault and admin
    pub fn initialize(ctx: Context<Initialize>, seed: u64) -> Result<()> {
        ctx.accounts.initialize(seed, ctx.bumps.config)
    }

    // Finalize a Config, disabling any further creation or cancellation of Vesting accounts
    pub fn finalize(ctx: Context<Finalize>) -> Result<()> {
        ctx.accounts.finalize()
    }
    
    // Open a new Vesting account and deposit equivalent vested tokens to vault
    pub fn create_vesting(ctx: Context<CreateVesting>, maturation: i64, amount: u64) -> Result<()> {
        ctx.accounts.create_vesting(maturation, amount, ctx.bumps.vest)
    }

    // Claim from and close a Vesting account
    pub fn claim_vesting(ctx: Context<ClaimVesting>) -> Result<()> {
        ctx.accounts.close_vesting()
    }

    // Cancel and close a Vesting account for a non-finalized Config
    pub fn cancel_vesting(ctx: Context<CancelVesting>) -> Result<()> {
        ctx.accounts.cancel_vesting()
    }

    // Allow admin to withdraw surplus tokens in excess of total vested amount
    pub fn withdraw_surplus(ctx: Context<WithdrawSurplus>) -> Result<()> {
        ctx.accounts.withdraw_surplus()
    }

}