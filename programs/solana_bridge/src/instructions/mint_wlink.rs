use crate::state::config::Config;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};

/// Validation context for Minting wLINK.
#[derive(Accounts)]
pub struct MintWLink<'info> {
    // The Relayer (backend) must sign this transaction.
    #[account(mut)]
    pub signer: Signer<'info>,

    // Read the config to ensure we are using the correct token addresses.
    pub config: Account<'info, Config>,

    // The Mint Account (the token factory).
    // SECURITY CHECK: We verify if this 'wlink_mint' matches the one saved in 'config'.
    #[account(
        mut,
        constraint = wlink_mint.key() == config.wlink_mint
    )]
    pub wlink_mint: Account<'info, Mint>,

    // The user wallet that will receive the tokens.
    #[account(mut)]
    pub user_destination: Account<'info, TokenAccount>,

    // Reference to the official SPL Token Program (needed for CPI).
    pub token_program: Program<'info, Token>,
}

/// Handler to mint wLINK tokens.
/// Logic: User Locked LINK on Ethereum -> Relayer calls this -> User gets wLINK on Solana.
pub fn handler(ctx: Context<MintWLink>, amount: u64) -> Result<()> {
    // SECURITY CHECK: Ensure only the admin/relayer can mint tokens.
    if ctx.accounts.signer.key() != ctx.accounts.config.admin {
        return Err(ProgramError::MissingRequiredSignature.into());
    }

    // 1. Prepare the Cross-Program Invocation (CPI) accounts.
    // We are telling the Token Program: "Please mint tokens using my authority".
    let cpi_accounts = MintTo {
        mint: ctx.accounts.wlink_mint.to_account_info(),
        to: ctx.accounts.user_destination.to_account_info(),
        authority: ctx.accounts.signer.to_account_info(), // Relayer signs the minting
    };

    // 2. Create the context for the CPI
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    // 3. Execute the mint command via SPL Token Program
    token::mint_to(cpi_ctx, amount)?;

    msg!("🔗 wLINK Minted: Amount {}", amount);
    Ok(())
}
