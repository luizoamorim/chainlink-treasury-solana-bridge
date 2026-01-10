use crate::state::config::Config;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};

#[derive(Accounts)]
pub struct ReceiveCctp<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub config: Account<'info, Config>,

    // SECURITY CHECK: Ensure we are minting the correct USDC token
    #[account(
        mut,
        constraint = usdc_mint.key() == config.usdc_mint
    )]
    pub usdc_mint: Account<'info, Mint>,

    #[account(mut)]
    pub user_destination: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

/// Handler to simulate receiving a CCTP message from Circle.
/// Logic: User burned USDC on Ethereum -> Circle attests -> Relayer submits proof here.
pub fn handler(ctx: Context<ReceiveCctp>, amount: u64, _mock_signature: String) -> Result<()> {
    // Note: In a production Mainnet environment, we would verify the
    // '_mock_signature' against Circle's MessageTransmitter program.
    // For this POC, we trust the Relayer's signature on the transaction itself.

    let cpi_accounts = MintTo {
        mint: ctx.accounts.usdc_mint.to_account_info(),
        to: ctx.accounts.user_destination.to_account_info(),
        authority: ctx.accounts.signer.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    token::mint_to(cpi_ctx, amount)?;

    msg!("💵 USDC (CCTP Simulator) Minted: Amount {}", amount);
    Ok(())
}
