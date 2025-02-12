use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
#[account]
#[derive(InitSpace)]
pub struct SocialTweet {
    #[max_len(50)]
    pub tweet_content: String,
    pub author_key: Pubkey,
    pub like_count: u32,
}
#[account]
#[derive(InitSpace)]
pub struct TweetLike {
    pub social_profile: Pubkey,
    pub social_tweet: Pubkey,
}
impl SocialTweet {
    pub const SEED_PREFIX: &'static str = "tweet_v1";

    pub fn new(content: String, author_key: Pubkey) -> Self {
        Self {
            like_count: 0,
            author_key,
            tweet_content: content,
        }
    }
}

impl TweetLike {
    pub const SEED_PREFIX: &'static str = "like_v1";

    pub fn new(social_profile: Pubkey, social_tweet: Pubkey) -> Self {
        Self {
            social_profile,
            social_tweet,
        }
    }
}
