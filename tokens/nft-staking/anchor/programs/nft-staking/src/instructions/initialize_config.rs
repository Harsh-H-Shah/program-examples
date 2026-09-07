use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::{StakeConfig, ANCHOR_DISCRIMINATOR};

#[derive(Accounts)]
#[instruction(points_per_day: u64, max_stake: u8, freeze_period_days: u32, reward_decimals: u8)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    /// The collection every staked NFT must belong to.
    pub collection_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = admin,
        space = ANCHOR_DISCRIMINATOR + StakeConfig::INIT_SPACE,
        seeds = [b"config"],
        bump,
    )]
    pub config: Account<'info, StakeConfig>,

    /// The reward token. Its mint authority is the config PDA, so rewards can
    /// only ever be minted by this program, from `claim` and `unstake`.
    #[account(
        init,
        payer = admin,
        seeds = [b"rewards", config.key().as_ref()],
        bump,
        mint::decimals = reward_decimals,
        mint::authority = config,
    )]
    pub rewards_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> InitializeConfig<'info> {
    pub fn initialize_config(
        &mut self,
        points_per_day: u64,
        max_stake: u8,
        freeze_period_days: u32,
        bumps: &InitializeConfigBumps,
    ) -> Result<()> {
        self.config.set_inner(StakeConfig {
            admin: self.admin.key(),
            collection: self.collection_mint.key(),
            points_per_day,
            max_stake,
            freeze_period_days,
            rewards_bump: bumps.rewards_mint,
            bump: bumps.config,
        });

        Ok(())
    }
}
