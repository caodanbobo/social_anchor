use crate::state::profile::*;
use anchor_lang::prelude::*;
pub fn initialize(ctx: Context<Initialize>, name: String) -> Result<()> {
    ctx.accounts.social_profile.profile_name = name;
    Ok(())
}
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer=authority,
        space=8+SocialProfile::INIT_SPACE,
        seeds=[SocialProfile::SEED_PREFIX.as_bytes(), authority.key().as_ref()],
        bump
    )]
    pub social_profile: Account<'info, SocialProfile>,
    pub system_program: Program<'info, System>,
}
