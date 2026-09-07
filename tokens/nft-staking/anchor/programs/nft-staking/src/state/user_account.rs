use anchor_lang::prelude::*;

/// Per-user totals, at seeds `["user", user]`. Created once and reused across
/// every NFT that user stakes.
#[account]
#[derive(InitSpace)]
pub struct UserAccount {
    /// Lifetime points paid out to this user, across all their stake positions.
    /// A running total for display; the authoritative per-NFT accrual state
    /// lives on each `StakeAccount`.
    pub points_earned: u64,
    /// How many NFTs this user currently has staked, checked against
    /// `StakeConfig::max_stake`.
    pub amount_staked: u8,
    pub bump: u8,
}
