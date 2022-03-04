use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod hh {
    use super::*;

    pub fn initialize(
        _ctx: Context<ApplicationStakingInitialize>,
        bump: u8,
        application_id: String,
    ) -> Result<()> {
        Ok(())
    }

    pub fn stake(
        _ctx: Context<ApplicationStake>,
        bump: u8,
        application_id: String,
        amount: u64,
    ) -> Result<()> {
        Ok(())
    }

    pub fn redeem(_ctx: Context<ApplicationStakeRedemption>, application_id: String) -> Result<()> {
        Ok(())
    }

    // close application staking pool... 
}

#[derive(Accounts)]
#[instruction(bump: u8, application_id: String)]
pub struct ApplicationStakingInitialize<'info> {
    #[account(init, payer = signer, space = 8 + 8 + 8 + 8, seeds = [application_id.as_bytes()], bump)]
    pub staking_pool: Account<'info, ApplicationStakingPool>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(bump: u8, application_id: String, amount: u64)]
pub struct ApplicationStake<'info> {
    #[account(mut, 
        constraint = staking_pool.actual_amount + amount <= staking_pool.max_amount,
        seeds = [application_id.as_bytes()], 
        bump
    )]
    pub staking_pool: Account<'info, ApplicationStakingPool>,

    #[account(mut,
        constraint = staker_authority_account.owner == signer.key(),
        constraint = staker_authority_account.mint == spl_token_mint.key())]
    pub staker_authority_account: Account<'info, TokenAccount>,

    #[account(constraint = spl_token_mint.key() == staker_authority_account.mint)]
    pub spl_token_mint: Account<'info, Mint>,

    #[account(
        init, 
        payer = signer, 
        space = TokenAccount::LEN,
        seeds = [signer.key().as_ref(), application_id.as_bytes()],
        bump
    )]
    pub user_staking_account: Account<'info, TokenAccount>,

    #[account(mut)]
    signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(application_id: String)]
pub struct ApplicationStakeRedemption<'info> {
    #[account(mut)]
    signer: Signer<'info>,
}

/// Keeps track of how much can be staked, and how much has there been staked
#[account]
pub struct ApplicationStakingPool {
    pub max_amount: u64,
    pub actual_amount: u64,
    pub current_reward_pool: u64,
}

#[error_code]
pub enum HHError {
    #[msg("The application staking pool does not exist for this application id")]
    ApplicationStakingPoolDoesNotExist,
}
