use anchor_lang::prelude::*;

/// This is the handler function used in lib.rs to execute the instruction to initialize
///  a data-containing PDA for the counter.
pub fn handler(
    ctx: Context<Initialize>,
    mainnet: bool,
) -> Result<()> {

    msg!("Initializing Counter");
    {
        let counter = &mut ctx.accounts.counter;

        // Initialize counter value, boolean, time
        counter.value = 0;
        counter.is_active = mainnet;
        counter.last_update = Clock::get()?.unix_timestamp;
    }
    
    Ok(())
}


#[derive(Accounts)]
/// This is the context used for the `initialize` instruction.
pub struct Initialize<'info> {

    #[account(
        init,
        payer = admin,
        space = 8 + 11,
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


#[account]
pub struct Counter {
    pub value: u16,
    pub is_active: bool,
    pub last_update: i64,
}

// #[account]
// /// To be used for the migration of multi-PDA programs (virtually all of them). Not used in this PoC.
// pub struct Freezer {
//     pub accounts_initialized: u64,
//     pub accounts_closed: u64,
//     pub new_state_updates: u64,
//     pub ready: bool,
// }


/// This is just a quick hacky serialization and deserialization trait
pub trait SynchronizableAccount {
    fn package(&self) -> Vec<u8>;
    fn unpackage(&mut self, package: Vec<u8>) -> Result<()>;
}

impl<'info> SynchronizableAccount for Account<'info, Counter> {

    fn package(&self) -> Vec<u8> {

        self.value.to_le_bytes().to_vec()

    }

    fn unpackage(&mut self, payload: Vec<u8>) -> Result<()> {

        self.value = u16::from_le_bytes(payload.try_into().unwrap());
        Ok(())
    }
}