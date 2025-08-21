use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ProgramConfig {
    /// Authority that can update the config
    pub authority: Pubkey,
    /// Address of the ZetaChain protocol-contracts-solana gateway program
    pub gateway_program: Pubkey,
    /// Current next token ID for generating unique token IDs
    pub next_token_nonce: u64,
    /// Bump seed for the config PDA
    pub bump: u8,
}

impl ProgramConfig {
    pub const SEED: &'static [u8] = b"config";

    pub fn increment_token_nonce(&mut self) {
        self.next_token_nonce += 1;
    }
}
