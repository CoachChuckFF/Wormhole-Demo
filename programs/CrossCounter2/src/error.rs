use anchor_lang::prelude::*;


#[error_code]
pub enum CrossCounterError {
    #[msg("This program is currently inactive")]
    InactiveProgram,
    #[msg("Attempting to update active program")]
    UpdatingActiveProgram,
    #[msg("Attempting to update inactive program to old state")]
    OldState,
}