use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::gateway;

#[derive(Accounts)]
pub struct TransferNft<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(mint::decimals = 0)]
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> TransferNft<'info> {
    pub fn handler(&mut self) -> Result<()> {
        // Transfer the NFT

        Ok(())
    }
}
