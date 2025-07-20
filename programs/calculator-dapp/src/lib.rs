use anchor_lang::prelude::*;

declare_id!("AohSsWK9gVxfgsFe8AuPcidZdwXbwjSmMgsXsmZB58SJ");

#[program]
pub mod calculator_dapp {
    use super::*;

    // init_message is greeting message for our calculare
    // Solana program are stateless which means they can't store the data
    // we have to create a account to store the date (account is like a file)
    // then we put the account ont the Solana Blockchain
    pub fn crete(ctx: Context<Create>, init_message:String) -> ProgramResult {
        let calcultor = ctx.accounts.calcultor;
        calcultor.greeting = init_message;
        // And finally, we use OK. So let's so long to know that the function was run successfully.
        ok(())
    }
}

#[derive(Accounts)]
pub struct create<'info> {
    #[account(init, payer = user, space=264)]
    pub calcultor = <Account<'info, Calcultor>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info Sytem>
}

#[account]
pub struct Calcultor {
    pub greeting: String,
    pub result: i64,
    pub remainder: i64,
}
