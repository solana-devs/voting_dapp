use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Start date cannot be greater than end date")]
    InvalidDates,
    #[msg("Poll does not exist or not found")]
    PollDoesNotExist,
    #[msg("Candidate cannot register twice")]
    CandidateAlreadyRegistered,
    
}