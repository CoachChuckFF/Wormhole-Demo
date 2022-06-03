// #region core
use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnf");

#[program]
pub mod mock_wormhole {
    use super::*;

    pub fn post_message(
        ctx: Context<PostMessage>,
        _sender: Pubkey,
        vaa_version: u8,
        consistency_level: u8,
        vaa_time: u32,
        vaa_signature_account: Pubkey,
        submission_time: u32,
        nonce: u32,
        sequence: u64,
        emitter_chain: u16,
        emitter_address: [u8; 32],
        payload: Vec<u8>,
    ) -> Result<()> {

        msg!("sent from {}", _sender);
        {
            let message_data = &mut ctx.accounts.message_data;

            message_data.vaa_version = vaa_version;
            message_data.consistency_level = consistency_level;
            message_data.vaa_time = vaa_time;
            message_data.vaa_signature_account = vaa_signature_account;
            message_data.submission_time = submission_time;
            message_data.nonce = nonce;
            message_data.sequence = sequence;
            message_data.emitter_chain = emitter_chain;
            message_data.emitter_address = emitter_address;
            message_data.payload = payload;
        }

        Ok(())
    }

    pub fn read_message(
        ctx: Context<ReadMessage>,
        _sender: Pubkey,
    ) -> Result<(Vec<u8>, i64)> {

        let message_data = &ctx.accounts.message_data;

        // This is where we would do checks to ensure this message is good with the fields below

        //message_data.vaa_version = vaa_version;
        //message_data.consistency_level = consistency_level;
        //message_data.vaa_signature_account = vaa_signature_account
        //message_data.submission_time = submission_time
        //message_data.nonce = nonce
        //message_data.sequence = sequence
        //message_data.emitter_chain = emitter_chain
        //message_data.emitter_address = emitter_address

        Ok((message_data.payload.clone(), message_data.submission_time.try_into().unwrap()))
    }
}

#[derive(Accounts)]
#[instruction(sender: Pubkey)]
/// This is the anchor context used in the `post_message` instruction.
pub struct PostMessage<'info> {

    #[account(
        init,//_if_needed,
        payer = admin,
        space = 1000,
        seeds = [
            "fake-wormhole".as_bytes(),
            &sender.to_bytes(),
        ],
        bump
    )]
    pub message_data: Account<'info, MessageData>,

    // #[account(mut)]
    // /// CHECK: lol
    // pub counter_program_pda: AccountInfo<'info>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(sender: Pubkey)]
/// This is the anchor context used in the `read_message` instruction (unused in this PoC).
pub struct ReadMessage<'info> {

    #[account(
        seeds = [
            "fake-wormhole".as_bytes(),
            &sender.to_bytes(),
        ],
        bump
    )]
    pub message_data: Account<'info, MessageData>,

    #[account(mut)]
    pub admin: Signer<'info>,
}


#[account]
/// At the time of writing, the fields in this struct are identical to that of the wormhole rust sdk in dev.v2.
/// As such, it serves to mock wormhole messages.
pub struct MessageData {
    /// Header of the posted VAA
    pub vaa_version: u8,

    /// Level of consistency requested by the emitter
    pub consistency_level: u8,

    /// Time the vaa was submitted
    pub vaa_time: u32,

    /// Account where signatures are stored
    pub vaa_signature_account: Pubkey,

    /// Time the posted message was created
    pub submission_time: u32,

    /// Unique nonce for this message
    pub nonce: u32,

    /// Sequence number of this message
    pub sequence: u64,

    /// Emitter of the message
    pub emitter_chain: u16,

    /// Emitter of the message
    pub emitter_address: [u8; 32],

    /// Message payload
    pub payload: Vec<u8>,
}
// #endregion core