use anchor_lang::prelude::*;

use crate::ProgramConfig;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + ProgramConfig::INIT_SPACE,
        seeds = [ProgramConfig::SEED],
        bump
    )]
    pub config_account: Account<'info, ProgramConfig>,

    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn handler(&mut self, gateway_program: Pubkey, bumps: &InitializeBumps) -> Result<()> {
        let config_account = &mut self.config_account;

        config_account.set_inner(ProgramConfig {
            authority: self.authority.key(),
            gateway_program,
            next_token_nonce: 1,
            bump: bumps.config_account,
        });

        Ok(())
    }
}
