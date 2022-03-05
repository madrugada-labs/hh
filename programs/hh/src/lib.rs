use std::io::{Write, self};

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Transfer, TokenAccount, Token};
use borsh::to_writer;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

const STAKING_STATS_SEED: &[u8] = b"staking_stats";
const USER_TOKEN_ACCOUNT_SEED: &[u8] = b"token_account";

type UnixTimestamp = i64;
#[program]
pub mod hh {
    use super::*;

    pub fn initialize_job_ad(_ctx: Context<JobAdInitialize>) -> Result<()> {
        Ok(())
    }

    pub fn initialize_application_id_pool(
        _ctx: Context<ApplicationStakingInitialize>,
        _bump: u8,
        _job_id: String,
        _application_id: String,
    ) -> Result<()> {
        Ok(())
    }

    pub fn stake(
        ctx: Context<ApplicationStake>,
        _bump: u8,
        _application_id: String,
        amount: u64,
    ) -> Result<()> {

        let cpi_accounts = Transfer{
            from: ctx.accounts.signer.to_account_info(),
            to: ctx.accounts.staker_authority_account.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };
        
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;
        
        // update the user staking account for this application id
        let reward_vouchers = 0; // TODO: implement the reward formula
        ctx.accounts.user_application_stake_account.amount_staked = amount;
        ctx.accounts.user_application_stake_account.reward_vouchers = reward_vouchers;

        ctx.accounts.staking_pool.actual_amount += amount;
        Ok(())
    }

    pub fn redeem(ctx: Context<ApplicationStakeRedemption>, application_id: String) -> Result<()> {
        // ctx.accounts.
        Ok(())
    }

    // close application staking pool... 
}

#[derive(Accounts)]
#[instruction(job_ad: String)]
pub struct JobAdInitialize<'info> {
    #[account(init, payer = signer, space = 8 + 8 + 8 + 8, seeds = [job_ad.as_bytes()], bump)]
    pub job_ad_account: Account<'info, JobAd>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
#[instruction(bump: u8, application_id: String)]
pub struct ApplicationStakingInitialize<'info> {
    #[account(init, payer = signer, space = 8 + 8 + 8 + 8 + 8, seeds = [application_id.as_bytes()], bump)]
    pub staking_pool: Account<'info, ApplicationStakingPool>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(bump: u8, application_id: String, amount: u64)]
pub struct ApplicationStake<'info> {
    #[account(mut, seeds = [application_id.as_bytes()], bump = bump)]
    pub staking_pool: Account<'info, ApplicationStakingPool>,

    #[account(mut, 
        seeds = [signer.key().as_ref(), application_id.as_bytes(), STAKING_STATS_SEED], 
        bump = bump,
    )]
    pub user_application_stake_account: Account<'info, UserApplicationStake>,


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
        seeds = [signer.key().as_ref(), application_id.as_bytes(), USER_TOKEN_ACCOUNT_SEED],
        bump
    )]
    pub user_staking_account: Account<'info, TokenAccount>,

    #[account(mut)]
    signer: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(job_ad_account_bump: u8, user_application_stake_bump: u8, user_staking_account_bump: u8, job_ad: String, application_id: String)]
pub struct ApplicationStakeRedemption<'info> {

    #[account(
        constraint = Clock::get()?.unix_timestamp <= job_ad_account.end_date,
        constraint = job_ad_account.hired_application.is_completed(), // there has to be a hired applicant
        seeds = [job_ad.as_bytes()],
        bump = job_ad_account_bump
    )]
    pub job_ad_account: Account<'info, JobAd>,

    #[account(
        mut, 
        seeds = [signer.key().as_ref(), application_id.as_bytes(), USER_TOKEN_ACCOUNT_SEED],
        bump = user_staking_account_bump
    )]
    pub user_staking_account: Account<'info, TokenAccount>,

    #[account(
        seeds = [signer.key().as_ref(), application_id.as_bytes(), STAKING_STATS_SEED],
        bump = user_application_stake_bump,
    )]
    pub user_stats: Account<'info, UserApplicationStake>,

    #[account(mut)]
    signer: Signer<'info>,
}

#[account]
pub struct JobAd {
    // when is this job being closed
    pub end_date: UnixTimestamp,
    // defines who's the application to be rewarded
    pub hired_application: JobState,
    // total reward pool
    pub reward_pool: u64,
    // authority of the job ad
    pub authority: Pubkey,
}
/// Keeps track of how much can be staked, and how much has there been staked
#[account]
pub struct ApplicationStakingPool {
    // total amount that can be staked on an application
    pub max_amount: u64,
    // how much has been staked on this particular application
    pub actual_amount: u64,
}

#[account]
pub struct UserApplicationStake {
    pub amount_staked: u64,
    pub reward_vouchers: u64,
}


#[derive(Clone, PartialEq)]
pub enum JobState {
    Hiring,
    Hired(String),
    Cancelled,
}

// impl AnchorDeserialize for JobState {
//     fn
// }

impl JobState {
    fn is_completed(&self) -> bool {
        match self {
            Self::Cancelled | Self::Hired(_) => true,
            Self::Hiring => false,
        }
    }
}
impl AnchorSerialize for JobState {
    fn serialize<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        match &self {
            Self::Hiring => to_writer(writer, b"0")?,
            Self::Hired(uuid) => to_writer(writer, &[b"1",  uuid.as_bytes()].concat())?,
            Self::Cancelled => to_writer(writer, b"2")?,
        };
        Ok(())
    }
}

impl AnchorDeserialize for JobState {
    fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
        Self::try_from_slice(buf)
    }
    fn try_from_slice(v: &[u8]) -> io::Result<Self> {
        match v {
            b"0" => Ok(JobState::Hiring),
            b"2" => Ok(JobState::Cancelled),
            hired if hired.len() > 0 && hired[0] == b'1' => Ok(JobState::Hired(String::from_utf8_lossy(&hired[1..]).to_string())),
            _ => Err(io::Error::new(io::ErrorKind::Other, "variant not recognized")),
        }
    }
}
#[error_code]
pub enum HHError {
    #[msg("The application staking pool does not exist for this application id")]
    ApplicationStakingPoolDoesNotExist,
    #[msg("Deserialization error")]
    DeserializationError
}
