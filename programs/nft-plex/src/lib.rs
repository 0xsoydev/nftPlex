use anchor_lang::prelude::*;

declare_id!("D1RrEBPUfQkp7CyF47y1soSpn3RLZY7BrFDvnm8KqEF1");

#[program]
pub mod nft_plex {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
