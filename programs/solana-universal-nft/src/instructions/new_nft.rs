use anchor_lang::{prelude::*, solana_program::keccak};
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3,
        CreateMetadataAccountsV3, Metadata,
    },
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};
use mpl_token_metadata::types::DataV2;

use crate::ProgramConfig;

#[derive(Accounts)]
pub struct NewNft<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub recipient: SystemAccount<'info>,
    #[account(
        seeds = [ProgramConfig::SEED],
        bump = config_account.bump,
    )]
    pub config_account: Account<'info, ProgramConfig>,
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = payer,
        mint::freeze_authority = payer,
    )]
    pub mint: Account<'info, Mint>,
    /// CHECK: Validated by the Metaplex token metadata program
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: Validated by the Metaplex token metadata program
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
    #[account(
        init,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = recipient
    )]
    pub recipient_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> NewNft<'info> {
    pub fn handler(&mut self, name: String, symbol: String, uri: String) -> Result<()> {
        let slot = Clock::get()?.slot;
        let next_token_nonce = self.config_account.next_token_nonce;

        // make a next_token_id by combine [mint pubkey + slot + next_token_nonce]
        let mut token_id_bytes = Vec::new();
        token_id_bytes.extend_from_slice(self.mint.key().as_ref());
        token_id_bytes.extend_from_slice(&slot.to_le_bytes());
        token_id_bytes.extend_from_slice(&next_token_nonce.to_le_bytes());

        // Hash the bytes for a fixed-length ID
        let _next_token_id: [u8; 32] = keccak::hash(&token_id_bytes)
            .to_bytes()
            .try_into()
            .expect("Failed to convert hash to fixed-length array");

        self.create_a_new_nft(name, symbol, uri)?;
        self.config_account.increment_token_nonce();
        Ok(())
    }

    fn create_a_new_nft(&mut self, name: String, symbol: String, uri: String) -> Result<()> {
        // mint to
        mint_to(
            CpiContext::new(
                self.token_program.to_account_info(),
                MintTo {
                    authority: self.payer.to_account_info(),
                    mint: self.mint.to_account_info(),
                    to: self.recipient_ata.to_account_info(),
                },
            ),
            1,
        )?;

        // create metadata
        let data = DataV2 {
            collection: Option::None,
            creators: Option::None,
            name,
            symbol,
            uri,
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

        Ok(())
    }
}
