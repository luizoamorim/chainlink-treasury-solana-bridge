// Here we import everything from the anchor_lang crate (Result, Pubkey, Context, etc.)
use anchor_lang::prelude::*;

// Import modules
pub mod instructions;
pub mod state;

// Import all from instructions
use instructions::*;

// Define the program ID
// This is generated using `solana address -k <keypair-file>` when deploying
// To check the program ID run anchor keys list
declare_id!("CN9K44XGc8gRwHijdaKxPg86jzkReA5CSMs3CZbE6ZqP");

// #[program] macro defines the entry point for the Solana program
#[program]
pub mod solana_bridge {
    // Use everything from the outer scope - all that is inside of what we imported above
    use super::*;

    // Initialize the bridge with necessary configurations
    pub fn initialize(
        ctx: Context<Initialize>,
        wlink_mint: Pubkey,
        usdc_mint: Pubkey,
    ) -> Result<()> {
        instructions::initialize::handler(ctx, wlink_mint, usdc_mint)
    }

    // Mint wrapped LINK tokens
    pub fn mint_wlink(ctx: Context<MintWLink>, amount: u64) -> Result<()> {
        instructions::mint_wlink::handler(ctx, amount)
    }

    // Receive CCTP messages and process them
    pub fn receive_cctp_message(
        ctx: Context<ReceiveCctp>,
        amount: u64,
        mock_signature: String,
    ) -> Result<()> {
        instructions::receive_cctp::handler(ctx, amount, mock_signature)
    }
}
