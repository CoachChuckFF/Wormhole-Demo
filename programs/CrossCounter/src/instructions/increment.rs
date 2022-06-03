use anchor_lang::prelude::*;
use crate::instructions::initialize::Counter;
use crate::error::CrossCounterError;

/// This is the handler function used in lib.rs to execute the instruction to increment
/// the counter
pub fn handler(
    ctx: Context<Increment>,
) -> Result<()> {

    require!(ctx.accounts.counter.is_active, CrossCounterError::InactiveProgram);

    msg!("Incrementing Counter");
    {
        let counter = &mut ctx.accounts.counter;

        // Increment counter
        counter.value = counter.value + 1;
    }
    
    Ok(())
}


#[derive(Accounts)]
/// This is the anchor context used in the `increment` instruction.
pub struct Increment<'info> {

    #[account(
        mut,
        seeds = [
            "counter".as_bytes()
        ],
        bump
    )]
    pub counter: Account<'info, Counter>,

    #[account(mut, address=crate::admin::id())]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}