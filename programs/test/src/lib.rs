use anchor_lang::prelude::*;

declare_id!("ARDviG2T3ZAmMdQcv5zBqFjodvRzPbSXw7nNffZTvnpT");

#[program]
pub mod hello_solana {
    use super::*;

    pub fn hello(ctx: Context<Hello>) -> Result<()> {
        let payer = &mut ctx.accounts.payer.to_account_info();
        let system_program = ctx.accounts.system_program.to_account_info();
        msg!("Hello World!!!");
        msg!("Our program ID is {}", &id());

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Hello<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
