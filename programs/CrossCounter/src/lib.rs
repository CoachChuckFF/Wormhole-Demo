use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod instructions;
pub mod error;
use crate::instructions::{
    initialize::*,
    increment::*,
    send::*,
    receive::*
};

#[program]
pub mod cross_counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, mainnet: bool) -> Result<()> {
        instructions::initialize::handler(ctx, mainnet)
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        instructions::increment::handler(ctx)
    }

    pub fn freeze_and_send(ctx: Context<Send>) -> Result<()> {
        instructions::send::handler(ctx)
    }

    pub fn update_and_activate(ctx: Context<Receive>) -> Result<()> {
        instructions::receive::handler(ctx)
    }
}




pub mod admin {
    use anchor_lang::declare_id;
    #[cfg(feature = "mainnet")]
    declare_id!("FRANKC3ibsaBW1o2qRuu3kspyaV4gHBuUfZ5uq9SXsqa");
    #[cfg(not(feature = "mainnet"))]
    declare_id!("FRANKC3ibsaBW1o2qRuu3kspyaV4gHBuUfZ5uq9SXsqa");
}

pub mod program_id {
    pub mod mainnet{
        use anchor_lang::declare_id;
        declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
    }
    pub mod shadow{
        use anchor_lang::declare_id;
        declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnq");
    }
}