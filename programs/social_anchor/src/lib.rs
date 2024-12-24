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
}
