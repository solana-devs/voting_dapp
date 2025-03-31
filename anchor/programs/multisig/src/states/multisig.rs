use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Multisig {
    pub admin: Pubkey,
    #[max_len(10)]
    pub approvals: Vec<Pubkey>,
    pub threshold: u8,
    pub nonce: u64,
    pub bump: u8,

}