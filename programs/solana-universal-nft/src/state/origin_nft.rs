use anchor_lang::{prelude::*, solana_program::hash};
use mpl_token_metadata::{MAX_NAME_LENGTH, MAX_SYMBOL_LENGTH, MAX_URI_LENGTH};

use crate::error::UniversalNftErrorCode;

#[account]
#[derive(InitSpace)]
pub struct OriginNft {
    pub token_id: [u8; 32],
    pub slot: u64,
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

    pub fn validate_pda(target: Pubkey, token_id: &[u8]) -> Result<(Pubkey, u8)> {
        let (pda, bump) = Pubkey::find_program_address(&[token_id, Self::SEED], &crate::ID);
        require!(pda.eq(&target), UniversalNftErrorCode::InvalidOriginNft);
        Ok((pda, bump))
    }
}

pub fn mint_to_token_id(mint: &Pubkey, slot: u64, next_token_nonce: u64) -> [u8; 32] {
    let mut hasher = hash::Hasher::default();
    hasher.hash(mint.as_ref());
    hasher.hash(&slot.to_le_bytes());
    hasher.hash(&next_token_nonce.to_le_bytes());
    hasher.result().to_bytes()
}
