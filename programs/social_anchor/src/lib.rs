use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;

use instructions::*;
declare_id!("9rDkiKxYckkXZb7XN5rZBbxJA927yBncUfUmmULb1QbF");

#[program]
pub mod social_anchor {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String) -> Result<()> {
        instructions::profile::initialize(ctx, name)
    }

    pub fn post_tweet(ctx: Context<PostTweet>, content: String) -> Result<()> {
        instructions::tweet::post_tweet(ctx, content)
    }

    pub fn smash_like(ctx: Context<SmashLike>) -> Result<()> {
        instructions::tweet::smash_like(ctx)
    }

    pub fn create_mint(ctx: Context<CreatMintToken>) -> Result<()> {
        instructions::mint::creat_mint_token(ctx)
    }
}
