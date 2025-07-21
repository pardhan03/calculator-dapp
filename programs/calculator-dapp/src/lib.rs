use anchor_lang::prelude::*;

declare_id!("AohSsWK9gVxfgsFe8AuPcidZdwXbwjSmMgsXsmZB58SJ");

#[program]
pub mod calculator_dapp {
    use super::*;

    // Initializes a calculator account with a greeting message
    pub fn create(ctx: Context<Create>, init_message: String) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        Ok(())
    }

    // function to add two number
    pub fn add(ctx: Context<Addition>, num1:i64, num2:i64) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 + num2;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 8 + 4 + 64 + 8 + 8)]
    pub calculator: Account<'info, Calculator>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// in this context we don't need to pass init because we are not creating a new account. Instead we are just using an account.
#[derive(Accounts)]
pub struct Addition<'info> {
    #[account(mut)] // we are passing a mut here because we will modify a account.
    pub calculator: Account<'info, Calculator>,
}

#[account]
pub struct Calculator {
    pub greeting: String,
    pub result: i64,
    pub remainder: i64,
}
