use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod states;

use instructions::*;
use states::*;

declare_id!("FQSjkWA6Sb6PcEJm693XsX9GETY7zhArPcPkkuLwdGws");

#[program]
pub mod voting {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handle_initialize(ctx)
    }

}
