use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct SocialProfile {
    #[max_len(20)]
    pub profile_name: String,
}
impl SocialProfile {
    pub const SEED_PREFIX: &'static str = "profile";
}
