use anchor_lang::prelude::*;

// Módulos
pub mod instructions;
pub mod state;

// Importar tudo de instructions
use instructions::*;

declare_id!("DnxJHPRqJZhcGdjZunm4BtUNq6UnWXeFrjyY3GM5qjuz");

#[program]
pub mod solana_bridge {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        wlink_mint: Pubkey,
        usdc_mint: Pubkey,
    ) -> Result<()> {
        instructions::initialize::handler(ctx, wlink_mint, usdc_mint)
    }

    pub fn mint_wlink(ctx: Context<MintWLink>, amount: u64) -> Result<()> {
        instructions::mint_wlink::handler(ctx, amount)
    }

    pub fn receive_cctp_message(
        ctx: Context<ReceiveCctp>,
        amount: u64,
        mock_signature: String,
    ) -> Result<()> {
        instructions::receive_cctp::handler(ctx, amount, mock_signature)
    }
}
