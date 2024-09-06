use anchor_lang::prelude::*;

declare_id!("AdSqjLd5Zw8cSm7F1nGuLWJVbMZj79fBoPuX5hWCwaGa");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
