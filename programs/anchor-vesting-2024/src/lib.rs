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

    // Finalize a Config, disabling any further creation or cancellation of Vest accounts
    pub fn finalize(ctx: Context<Finalize>) -> Result<()> {
        ctx.accounts.finalize()
    }
    
    // Open a new Vest account and deposit equivalent vested tokens to vault
    pub fn create_vest(ctx: Context<CreateVest>, timeout: i64, amount: u64) -> Result<()> {
        ctx.accounts.create_vest(timeout, amount, ctx.bumps.vest)
    }

    // Claim from and close a Vest account
    pub fn claim_vest(ctx: Context<ClaimVest>) -> Result<()> {
        ctx.accounts.close_vest()
    }

    // Cancel and close a Vest account for a non-finalized Config
    pub fn cancel_vest(ctx: Context<CancelVest>) -> Result<()> {
        ctx.accounts.cancel_vest()
    }

    // Allow admin to withdraw surplus tokens in excess of total vested amount
    pub fn withdraw_surplus(ctx: Context<WithdrawSurplus>) -> Result<()> {
        ctx.accounts.withdraw_surplus()
    }

}

// #[derive(Accounts)]
// pub struct OpenVault<'info> {
//     #[account(mut)]
//     admin: Signer<'info>,
//     mint: InterfaceAccount<'info, Mint>,
//     vault: InterfaceAccount<'info, TokenAccount>,
//     config: Account<'info, VestingConfig>,
//     token_program: Interface<'info, TokenInterface>,
//     system_program: Program<'info, System>
// }