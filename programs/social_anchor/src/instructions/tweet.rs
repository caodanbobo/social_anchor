use crate::state::profile::*;
use crate::state::tweet::*;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::mint_to;
use anchor_spl::token::Mint;
use anchor_spl::token::MintTo;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
pub fn post_tweet(ctx: Context<PostTweet>, content: String) -> Result<()> {
    ctx.accounts.social_profile.tweet_count += 1;
    let tweet = SocialTweet::new(content, ctx.accounts.authority.key());
    ctx.accounts.social_tweet.set_inner(tweet);
    Ok(())
}

pub fn smash_like(ctx: Context<SmashLike>) -> Result<()> {
    ctx.accounts.social_tweet.like_count+=1;
    let tweet_like=TweetLike::new(ctx.accounts.social_profile.key(), ctx.accounts.social_tweet.key());
    ctx.accounts.tweet_like.set_inner(tweet_like);

    mint_to(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(), 
        MintTo{
            mint: ctx.accounts.mint_account.to_account_info(),
            to: ctx.accounts.author_ata.to_account_info(),
            authority: ctx.accounts.mint_account.to_account_info()
        },
        &[&[b"mint_v1", &[ctx.bumps.mint_account]]]), 
        100)?;
    Ok(())
}
#[derive(Accounts)]
pub struct PostTweet<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer=authority,
        space=8+SocialTweet::INIT_SPACE,
        seeds=[
            SocialTweet::SEED_PREFIX.as_bytes(),
         social_profile.key().as_ref(), 
         (social_profile.tweet_count+1).to_string().as_ref()],
        bump
    )]
    pub social_tweet: Account<'info, SocialTweet>,

    #[account(
        mut,
        seeds=[SocialProfile::SEED_PREFIX.as_bytes(), authority.key().as_ref()],
        bump
    )]
    pub social_profile: Account<'info, SocialProfile>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SmashLike<'info> {
    #[account(
        mut,
        seeds=[
            b"mint_v1",
        ],
        bump,
    )]
    pub mint_account: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer=authority,
        associated_token::mint=mint_account,
        associated_token::authority=author_wallet,
    )]
    pub author_ata:Account<'info, TokenAccount>,
    ///CHECK: this is the author
    pub author_wallet: AccountInfo<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init_if_needed,
        payer=authority,
        space=8+TweetLike::INIT_SPACE,
        seeds=[
            TweetLike::SEED_PREFIX.as_bytes(),
         social_profile.key().as_ref(), 
         social_tweet.key().as_ref()],
        bump
    )]
    pub tweet_like: Account<'info, TweetLike>,

    #[account(mut)]
    pub social_tweet: Account<'info, SocialTweet>,
    #[account(
        mut,
        seeds=[SocialProfile::SEED_PREFIX.as_bytes(), authority.key().as_ref()],
        bump
    )]
    pub social_profile: Account<'info, SocialProfile>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>
}
