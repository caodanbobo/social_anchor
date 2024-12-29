use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3,
        Metadata,
    },
    token::{Mint, Token},
};

pub fn creat_mint_token(ctx: Context<CreatMintToken>) -> Result<()> {
    // let signer_seeds: &[&[&[u8]]] = &[&[b"mint", &[ctx.bumps.mint_account]]];
    // create_metadata_accounts_v3(
    //     CpiContext::new_with_signer(
    //         ctx.accounts.token_metadata_program.to_account_info(),
    //         CreateMetadataAccountsV3 {
    //             metadata: ctx.accounts.metadata_account.to_account_info(),
    //             mint: ctx.accounts.mint_account.to_account_info(),
    //             mint_authority: ctx.accounts.mint_account.to_account_info(),
    //             payer: ctx.accounts.authority.to_account_info(),
    //             update_authority: ctx.accounts.mint_account.to_account_info(),
    //             system_program: ctx.accounts.system_program.to_account_info(),
    //             rent: ctx.accounts.rent.to_account_info(),
    //         },
    //         signer_seeds,
    //     ),
    //     DataV2 {
    //         name: "cdbb".to_string(),
    //         symbol: "cdbb".to_string(),
    //         uri: "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcTGvQHbmrL1LUyjF08sKQ5JKeX3EaUtdy7_1g&s".to_string(),
    //         seller_fee_basis_points: 0,
    //         creators: None,
    //         collection: None,
    //         uses: None,
    //     },
    //     false,
    //     true,
    //     None
    // )?;
    Ok(())
}

#[derive(Accounts)]
pub struct CreatMintToken<'info> {
    // /// CHECK:` doc comment explaining why no checks through types are necessary.
    // #[account(
    //     mut,
    //     seeds=[b"metadata", token_metadata_program.key().as_ref(), mint_account.key().as_ref()],
    //     bump,
    //     seeds::program= token_metadata_program.key()
    // )]
    // pub metadata_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer=authority,
        seeds=[
            b"mint",
        ],
        bump,
        mint::decimals=100,
        mint::authority=mint_account.key(),
    )]
    pub mint_account: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
    //pub token_metadata_program: Program<'info, Metadata>,
    pub rent: Sysvar<'info, Rent>,
}
