use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, metadata::{create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3, CreateMetadataAccountsV3, Metadata}, token::{burn, mint_to, Burn, Mint, MintTo, Token, TokenAccount}};
use mpl_token_metadata::types::DataV2;

use crate::{error::UniversalNftErrorCode, gateway::program::Gateway, CrossChainFunction, CrossChainMessage, OriginNft, ProgramConfig};

#[derive(Accounts)]
pub struct OnCall<'info> {
   #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
      mut, 
      seeds = [ProgramConfig::SEED], 
      bump = config.bump
    )]
    pub config: Account<'info, ProgramConfig>,
    #[account(
        init_if_needed,
        payer = payer,
        mint::decimals = 0,
        mint::authority = payer,
        mint::freeze_authority = payer,
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer
    )]
    pub token_account: Account<'info, TokenAccount>,
    /// CHECK: Validated by the Metaplex token metadata program
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: Validated by the Metaplex token metadata program
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
    #[account(
        mut,
        has_one = mint @UniversalNftErrorCode::InvalidOriginNft
    )]
    pub origin_nft: Account<'info, OriginNft>,
    #[account(
        constraint = gateway_program.key() == config.gateway_program @ UniversalNftErrorCode::InvalidGateway
    )]
    pub gateway_program: Program<'info, Gateway>,
    /// CHECK: validate by gateway
    #[account(mut)]
    pub gateway_pda: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub rent: Sysvar<'info, Rent>,


}

impl<'info> OnCall<'info> {
    pub fn handler(&mut self, message: &[u8]) -> Result<()> {
        let cross_chain_message = CrossChainMessage::from_bytes(message)?;

        require_eq!(
          cross_chain_message.mint,
          self.mint.key(),
          UniversalNftErrorCode::InvalidCrossChainMessage
        );

        match cross_chain_message.fun {
          CrossChainFunction::TransferNft => {
             require_eq!(
              cross_chain_message.sender,
              self.payer.key(),
              UniversalNftErrorCode::InvalidCrossChainMessage
            );

            burn(CpiContext::new(
              self.token_program.to_account_info(),
              Burn {
                  authority: self.payer.to_account_info(),
                  from: self.token_account.to_account_info(),
                  mint: self.mint.to_account_info(),
                }
              ), 
              1
            )?;
          },
          CrossChainFunction::ReceiveNft => {
            // Logic for receiving NFT can be implemented here
            // mint to
            mint_to(
                CpiContext::new(
                    self.token_program.to_account_info(),
                    MintTo {
                        authority: self.payer.to_account_info(),
                        mint: self.mint.to_account_info(),
                        to: self.token_account.to_account_info(),
                    },
                ),
                1,
            )?;

            // create metadata
            let data = DataV2 {
                collection: Option::None,
                creators: Option::None,
                name: self.origin_nft.name.to_string(),
                symbol: self.origin_nft.symbol.to_string(),
                uri: self.origin_nft.uri.to_string(),
                seller_fee_basis_points: 0,
                uses: Option::None,
            };

            create_metadata_accounts_v3(
                CpiContext::new(
                    self.token_metadata_program.to_account_info(),
                    CreateMetadataAccountsV3 {
                        metadata: self.metadata.to_account_info(),
                        mint: self.mint.to_account_info(),
                        payer: self.payer.to_account_info(),
                        update_authority: self.payer.to_account_info(),
                        mint_authority: self.payer.to_account_info(),
                        system_program: self.system_program.to_account_info(),
                        rent: self.rent.to_account_info(),
                    },
                ),
                data,
                false,
                true,
                Option::None,
            )?;

            create_master_edition_v3(
                CpiContext::new(
                    self.token_metadata_program.to_account_info(),
                    CreateMasterEditionV3 {
                        edition: self.master_edition.to_account_info(),
                        metadata: self.metadata.to_account_info(),
                        mint: self.mint.to_account_info(),
                        mint_authority: self.payer.to_account_info(),
                        payer: self.payer.to_account_info(),
                        update_authority: self.payer.to_account_info(),
                        system_program: self.system_program.to_account_info(),
                        token_program: self.token_program.to_account_info(),
                        rent: self.rent.to_account_info(),
                    },
                ),
                Some(0),
            )?;
          },            
        };

       
        Ok(())
    }
}