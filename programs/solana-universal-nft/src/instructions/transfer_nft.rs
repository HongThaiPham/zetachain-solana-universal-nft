use anchor_lang::{prelude::*, solana_program::stake::config::Config};
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{
    error::UniversalNftErrorCode,
    gateway::{self, cpi::accounts::DepositSplToken, program::Gateway, types::RevertOptions},
    mint_to_token_id, OriginNft, ProgramConfig,
};

#[derive(Accounts)]
pub struct TransferNft<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(mint::decimals = 0)]
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = sender
    )]
    pub sender_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        has_one = mint @UniversalNftErrorCode::InvalidOriginNft
    )]
    pub origin_nft: Account<'info, OriginNft>,
    #[account(
        seeds = [ProgramConfig::SEED],
        bump = config.bump
    )]
    pub config: Account<'info, ProgramConfig>,
    #[account(
        constraint = gateway_program.key() == config.gateway_program @ UniversalNftErrorCode::InvalidGateway
    )]
    pub gateway_program: Program<'info, Gateway>,
    /// CHECK: validate by gateway
    #[account(mut)]
    pub gateway_pda: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> TransferNft<'info> {
    pub fn handler(&mut self, dest_chain_id: u64, recipient: [u8; 20]) -> Result<()> {
        // Transfer the NFT
        let nft_origin = &self.origin_nft;

        let mut message = Vec::new();
        // message include sender, receiver, mint
        message.extend_from_slice(&dest_chain_id.to_le_bytes());
        message.extend_from_slice(self.mint.key().as_ref());
        message.extend_from_slice(nft_origin.token_id.as_ref());
        message.extend_from_slice(self.sender.key().as_ref());
        message.extend_from_slice(recipient.as_ref());

        gateway::cpi::deposit_and_call(
            CpiContext::new(
                self.gateway_program.to_account_info(),
                gateway::cpi::accounts::DepositAndCall {
                    signer: self.sender.to_account_info(),
                    system_program: self.system_program.to_account_info(),
                    pda: self.gateway_pda.to_account_info(),
                },
            ),
            1,
            recipient,
            message,
            None,
        )?;

        Ok(())
    }
}
