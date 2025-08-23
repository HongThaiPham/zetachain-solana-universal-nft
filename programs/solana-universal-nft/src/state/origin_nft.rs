use anchor_lang::prelude::*;
use mpl_token_metadata::{MAX_NAME_LENGTH, MAX_SYMBOL_LENGTH, MAX_URI_LENGTH};

use crate::error::UniversalNftErrorCode;

#[account]
#[derive(InitSpace)]
pub struct OriginNft {
    pub token_id: [u8; 32],
    pub mint: Pubkey,
    #[max_len(MAX_NAME_LENGTH)]
    pub name: String,
    #[max_len(MAX_SYMBOL_LENGTH)]
    pub symbol: String,
    #[max_len(MAX_URI_LENGTH)]
    pub uri: String,
    pub bump: u8,
}

impl OriginNft {
    pub const SEED: &'static [u8] = b"nft_origin";

    pub fn validate_pda(&self, target: Pubkey, token_id: &[u8]) -> Result<(Pubkey, u8)> {
        let (pda, bump) = Pubkey::find_program_address(&[token_id, Self::SEED], &crate::ID);
        require!(pda.eq(&target), UniversalNftErrorCode::InvalidOriginNft);
        Ok((pda, bump))
    }
}
