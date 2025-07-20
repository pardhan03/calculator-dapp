use anchor_lang::prelude::*;

declare_id!("AohSsWK9gVxfgsFe8AuPcidZdwXbwjSmMgsXsmZB58SJ");

#[program]
pub mod calculator_dapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
