// Import the Anchor prelude for Solana programs
use anchor_lang::prelude::*;

/// The main state structure for our Bridge/Treasury.
/// This acts like a "database row" stored on the blockchain.
#[account]
pub struct Config {
    /// The public key of the Relayer/Admin who has permission to mint.
    pub admin: Pubkey, // 32 bytes

    /// The specific address of the Wrapped LINK token we control.
    pub wlink_mint: Pubkey, // 32 bytes

    /// The specific address of the USDC token (for CCTP simulation).
    pub usdc_mint: Pubkey, // 32 bytes
}

impl Config {
    // Calculation of the space required to store this account on-chain.
    // 8 bytes (Anchor Discriminator - unique ID) + 32 bytes (Admin) + 32 bytes (wLink) + 32 bytes (USDC)
    pub const MAXIMUM_SIZE: usize = 8 + 32 + 32 + 32;
}
