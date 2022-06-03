// #region core
use anchor_lang::prelude::*;
use crate::error::CrossCounterError;
use crate::instructions::initialize::Counter;
use mock_wormhole::program::MockWormhole;
use mock_wormhole::cpi::accounts::ReadMessage;
use mock_wormhole::{self, MessageData};
use std::convert::TryInto;
use crate::program_id;
use crate::instructions::initialize::SynchronizableAccount;
use mock_wormhole::cpi::Return;


/// This is the handler function used in lib.rs to execute the instruction to receive state,
/// update state, and activate the program.
pub fn handler(
    ctx: Context<Receive>,
) -> Result<()> {

    // Require program to be inactive
    require!(!ctx.accounts.counter.is_active, CrossCounterError::UpdatingActiveProgram);

    msg!("Receiving state and activating program");
    {
        // If and when solana allows for CPIs to return data, this is an alternative so as to not have to pass in message account
        // let cpi_program = ctx.accounts.mock_wormhole_program.to_account_info();
        // let mut cpi_accounts = ReadMessage {
        //     //counter_program_pda: ctx.accounts.counter.to_account_info(),
        //     message_data: ctx.accounts.message_data.to_account_info(),
        //     //system_program: ctx.accounts.system_program.to_account_info(),
        //     admin: ctx.accounts.admin.to_account_info(),
        // };
        // let counter_seeds = [
        //     "counter".as_bytes(),
        //     &[*ctx.bumps.get("counter").unwrap()],
        // ];

        // let signer_seeds: &[&[&[u8]]] = &[&counter_seeds];
        // let cpi_ctx = CpiContext::new(//_with_signer(
        //     cpi_program,
        //     cpi_accounts,
        //     //signer_seeds,
        // );

        // let (payload, time) = mock_wormhole::cpi::read_message(
        //     cpi_ctx,
        //     if crate::id() != program_id::mainnet::id() { program_id::mainnet::id() } else { program_id::shadow::id() },
        // ).unwrap().try_into().unwrap();

        msg!("Receiving state: Mocking read message from wormhole");
        let message_data = &ctx.accounts.message_data;
        let simulated_read_message: (Vec<u8>, i64) = (message_data.payload.clone(), message_data.submission_time.try_into().unwrap());
        let (payload, submission_time) = simulated_read_message;

        // Ensure state is not old.
        require_gte!(submission_time, ctx.accounts.counter.last_update, CrossCounterError::OldState);
        
        msg!("Updating state and activating program");
        ctx.accounts.counter.unpackage(payload)?;
        // This is here now because the program contains only one PDA but for multi-PDA programs (virtually all)
        // we will need to handle this differently
        ctx.accounts.counter.is_active = true; 
        ctx.accounts.counter.last_update = Clock::get()?.unix_timestamp;
    }
    
    Ok(())
}


#[derive(Accounts)]
/// This is the anchor context used in the `update_and_activate` instruction.
pub struct Receive<'info> {

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

    #[account()]
    /// Unchecked for now as it is PoC but will need to validate this.
    pub message_data: Account<'info, MessageData>,

    /// Unchecked for now as it is PoC but will need to validate this.
    pub mock_wormhole_program: Program<'info, MockWormhole>,

    pub system_program: Program<'info, System>,
}


// #endregion core