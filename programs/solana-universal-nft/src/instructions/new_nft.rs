use anchor_lang::{
    prelude::*,
    solana_program::{hash, keccak},
    system_program,
};
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3,
        CreateMetadataAccountsV3, Metadata,
    },
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};
use mpl_token_metadata::types::DataV2;

use crate::{error::UniversalNftErrorCode, OriginNft, ProgramConfig};

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
    // #[account(init, payer = payer, space = 8 + OriginNft::INIT_SPACE)]
    // pub origin_nft: Account<'info, OriginNft>,
    /// CHECK: oooooookkkkk
    // #[account(mut)]
    pub origin_nft: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> NewNft<'info> {
    pub fn handler(&mut self, name: String, symbol: String, uri: String) -> Result<()> {
        self.create_a_new_nft(&name, &symbol, &uri)?;
        self.init_origin_nft_account(&name, &symbol, &uri)?;
        self.config_account.increment_token_nonce();
        Ok(())
    }

    fn init_origin_nft_account(
        &mut self,
        name: &String,
        symbol: &String,
        uri: &String,
    ) -> Result<()> {
        let slot = Clock::get()?.slot;
        msg!("slot: {}", slot);
        let next_token_nonce = self.config_account.next_token_nonce;
        msg!("next_token_nonce: {}", next_token_nonce);
        // make a next_token_id by combine [mint pubkey + slot + next_token_nonce]
        // let mut token_id_bytes = Vec::new();
        // token_id_bytes.extend_from_slice(self.mint.key().as_ref());
        // token_id_bytes.extend_from_slice(&slot.to_le_bytes());
        // token_id_bytes.extend_from_slice(&next_token_nonce.to_le_bytes());

        // // Hash the bytes for a fixed-length ID
        // let token_id = keccak::hash(&token_id_bytes).to_bytes();

        // let (xxx, bump) = self
        //     .origin_nft
        //     .validate_pda(self.origin_nft.key(), &token_id)?;

        let mut hasher = hash::Hasher::default();
        hasher.hash(self.mint.key().as_ref());
        hasher.hash(&slot.to_le_bytes());
        hasher.hash(&next_token_nonce.to_le_bytes());
        let token_id = hasher.result().to_bytes();

        let (pda, bump) = Pubkey::find_program_address(&[&token_id, OriginNft::SEED], &crate::ID);
        msg!("pda: {} and bump: {}", pda, bump);
        require!(
            pda.eq(&self.origin_nft.key()),
            UniversalNftErrorCode::InvalidOriginNft
        );

        require!(
            self.origin_nft.to_account_info().data_is_empty(),
            UniversalNftErrorCode::OriginNftAccountAlreadyExists
        );

        let space: usize = 8 + OriginNft::INIT_SPACE;

        let signer_seeds: &[&[u8]] = &[&token_id, OriginNft::SEED, &[bump]];

        // system_program::create_account(
        //     CpiContext::new_with_signer(
        //         self.system_program.to_account_info(),
        //         anchor_lang::system_program::CreateAccount {
        //             from: self.payer.to_account_info(),
        //             to: self.origin_nft.to_account_info(),
        //         },
        //         &[signer_seeds],
        //     ),
        //     Rent::get()?.minimum_balance(space),
        //     space as u64,
        //     &crate::ID,
        // )?;

        // self.origin_nft.set_inner(OriginNft {
        //     token_id: token_id,
        //     mint: self.mint.key(),
        //     name: name.to_string(),
        //     symbol: symbol.to_string(),
        //     uri: uri.to_string(),
        //     bump,
        // });
        Ok(())
    }

    fn create_a_new_nft(&mut self, name: &String, symbol: &String, uri: &String) -> Result<()> {
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
            name: name.to_string(),
            symbol: symbol.to_string(),
            uri: uri.to_string(),
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
