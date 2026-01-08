use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, MintTo};

#[derive(Accounts)]
pub struct MintToken<'info> {
    // 1. O Token Mint: A "definição" da moeda (ex: wUSDC)
    // Precisamos escrever nele (aumentar o supply), por isso 'mut'
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    // 2. A Conta de Destino: Onde o token vai cair (Carteira do User)
    #[account(mut)]
    pub destination: Account<'info, TokenAccount>,

    // 3. A Autoridade: Quem tem a chave para ligar a impressora?
    // Neste exemplo simples, o próprio Relayer (signer) é o dono da Mint.
    #[account(mut)]
    pub signer: Signer<'info>,

    // 4. O Programa de Token: O "Software" oficial que sabe mexer com tokens
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<MintToken>, amount: u64, eth_sender: String) -> Result<()> {
    
    // Log informativo
    msg!("🌉 Bridge Event: Minting {} tokens from ETH sender {}", amount, eth_sender);

    // --- CPI: Chamada Cross-Program ---
    
    // A. Configurando o Contexto da CPI
    // Estamos preparando os "ingredientes" para passar para o Token Program
    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.destination.to_account_info(),
        authority: ctx.accounts.signer.to_account_info(),
    };

    // B. Definindo qual programa vamos chamar
    let cpi_program = ctx.accounts.token_program.to_account_info();

    // C. Criando o contexto da chamada
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    // D. Executando a ação (MintTo)
    token::mint_to(cpi_ctx, amount)?;

    msg!("✅ Minted successfully!");
    
    Ok(())
}