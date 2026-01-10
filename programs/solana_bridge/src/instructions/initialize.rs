use anchor_lang::prelude::*;
use crate::state::config::Config;

/// Validation struct for the Initialize instruction.
/// Defines which accounts are needed to setup the program.
#[derive(Accounts)]
pub struct Initialize<'info> {
    // 1. We are creating (init) a new account 'config'.
    // 2. The 'signer' will pay (payer) for the storage rent.
    // 3. We allocate exact space defined in Config::MAXIMUM_SIZE.
    #[account(
        init, 
        payer = signer, 
        space = Config::MAXIMUM_SIZE
    )]
    pub config: Account<'info, Config>,
    
    // The user calling this transaction (must sign to pay for rent).
    #[account(mut)]
    pub signer: Signer<'info>,
    
    // Required system program to create new accounts on Solana.
    pub system_program: Program<'info, System>,
}

/// Handler logic to initialize the Treasury configuration.
pub fn handler(ctx: Context<Initialize>, wlink_mint: Pubkey, usdc_mint: Pubkey) -> Result<()> {
    // Get a mutable reference to the newly created account
    let config = &mut ctx.accounts.config;
    
    // Save the data to the blockchain
    config.admin = ctx.accounts.signer.key(); // The deployer becomes the admin
    config.wlink_mint = wlink_mint;           // Set the official wLINK address
    config.usdc_mint = usdc_mint;             // Set the official USDC address
    
    msg!("✅ Treasury Config Initialized");
    Ok(())
}