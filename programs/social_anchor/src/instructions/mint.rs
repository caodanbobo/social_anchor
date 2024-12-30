use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

pub fn creat_mint_token(_ctx: Context<CreatMintToken>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct CreatMintToken<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init_if_needed,
        payer=authority,
        seeds=[
            b"mint_v1",
        ],
        bump,
        mint::decimals=2,
        mint::authority=mint_account.key(),
    )]
    pub mint_account: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
