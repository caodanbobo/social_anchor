use crate::state::profile::*;
use crate::state::tweet::*;
use anchor_lang::prelude::*;
pub fn post_tweet(ctx: Context<PostTweet>, content: String) -> Result<()> {
    ctx.accounts.social_profile.tweet_count += 1;
    let tweet = SocialTweet::new(content);
    ctx.accounts.social_tweet.set_inner(tweet);
    Ok(())
}

pub fn smash_like(ctx: Context<SmashLike>) -> Result<()> {
    ctx.accounts.social_tweet.like_count+=1;
    let tweet_like=TweetLike::new(ctx.accounts.social_profile.key(), ctx.accounts.social_tweet.key());
    ctx.accounts.tweet_like.set_inner(tweet_like);
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
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
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
}
