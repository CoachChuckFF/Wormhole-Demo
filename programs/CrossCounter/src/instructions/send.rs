// #region core
use anchor_lang::prelude::*;
use crate::error::CrossCounterError;
use crate::instructions::initialize::Counter;
use mock_wormhole::program::MockWormhole;
use mock_wormhole::cpi::accounts::PostMessage;
use mock_wormhole::{self, MessageData};
use std::convert::TryInto;
use crate::instructions::initialize::SynchronizableAccount;


/// This is the handler function used in lib.rs to execute the instruction to receive state,
/// update state, and activate the program
pub fn handler(
    ctx: Context<Send>,
) -> Result<()> {

    // Require program to be active
    require!(ctx.accounts.counter.is_active, CrossCounterError::InactiveProgram);

    msg!("Mocking post message to Wormhole");
    {
        msg!("message_Data {:?}", ctx.accounts.message_data.to_account_info());
        msg!("counter {:?}", ctx.accounts.counter.to_account_info());
        let cpi_program = ctx.accounts.mock_wormhole_program.to_account_info();
        let cpi_accounts = PostMessage {
            //counter_program_pda: ctx.accounts.counter.to_account_info(),
            message_data: ctx.accounts.message_data.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            admin: ctx.accounts.admin.to_account_info(),
        };
        let counter_seeds = [
            "counter".as_bytes(),
            &[*ctx.bumps.get("counter").unwrap()],
        ];

        let signer_seeds: &[&[&[u8]]] = &[&counter_seeds];
        let cpi_ctx = CpiContext::new(//_with_signer(
            cpi_program,
            cpi_accounts,
            //signer_seeds,
        );

        
        let time = Clock::get()?.unix_timestamp;

        // This is all bs for now, real stuff we care about below
        let vaa_version: u8 = 0;
        let consistency_level: u8 = 0;
        let vaa_time: u32 = time.try_into().unwrap();
        let vaa_signature_account: Pubkey = crate::id();
        let submission_time: u32 = time.try_into().unwrap();
        let nonce: u32 = 1;
        let sequence: u64 = 123;
        let emitter_chain: u16 = 1; // chain id = 1 is solana
        let emitter_address: [u8; 32] = crate::id().to_bytes();


        // This is the real thing we care about.
        // This is the state of the program.
        // This will need to be run for every program account.
        // loop through one gPA --> post messages --> sync messages.
        //
        // For multiple pda programs (virtually all), we can isolate the freeze ix,
        // and build some concensus mechansism to come to consensus about state of program
        // after freezing and before posting messages to wormhole.
        //
        // let payload: Vec<u8> = (*ctx.accounts.counter.to_account_info().data.borrow()).to_vec();
        let payload: Vec<u8> = ctx.accounts.counter.package(); // my own hacky serialization

        mock_wormhole::cpi::post_message(
            cpi_ctx,
            crate::id(),
            vaa_version,
            consistency_level,
            vaa_time,
            vaa_signature_account,
            submission_time,
            nonce,
            sequence,
            emitter_chain,
            emitter_address,
            payload,
        )?;
    }

    msg!("Deactivating program");
    {
        ctx.accounts.counter.is_active = false;
    }
    
    Ok(())
}


#[derive(Accounts)]
/// This is the anchor context used in the `freeze_and_send` instruction.
pub struct Send<'info> {

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

    #[account(mut)]
    /// CHECK: Unchecked for now as it is PoC but will need to validate this.
    pub message_data: AccountInfo<'info>,

    /// Unchecked for now as it is PoC but will need to validate this.
    pub mock_wormhole_program: Program<'info, MockWormhole>,

    pub system_program: Program<'info, System>,
}
