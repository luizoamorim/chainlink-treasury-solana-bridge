use anchor_lang::prelude::*;
pub mod instructions;
use instructions::*;

// IMPORTANTE: O Anchor vai reclamar se esse ID não bater com o do arquivo.
// Por enquanto mantenha este gerado ou substitua pelo que está no seu Anchor.toml
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solana_bridge {
    use super::*;

    pub fn mint_wrapped_token(ctx: Context<MintToken>, amount: u64, eth_sender: String) -> Result<()> {
        instructions::mint_wrapped_token::handler(ctx, amount, eth_sender)
    }
}