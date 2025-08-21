#![allow(unexpected_cfgs)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("A9FvZ2NMVPug73mYiKfkJEaB5NwxKTNsJvZeM6dqiufY");

declare_program!(gateway);

#[program]
pub mod solana_universal_nft {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, gateway_program: Pubkey) -> Result<()> {
        ctx.accounts.handler(gateway_program, &ctx.bumps)
    }

    pub fn new_nft(ctx: Context<NewNft>, name: String, symbol: String, uri: String) -> Result<()> {
        ctx.accounts.handler(name, symbol, uri)
    }
}
