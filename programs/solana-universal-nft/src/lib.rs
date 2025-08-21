pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("A9FvZ2NMVPug73mYiKfkJEaB5NwxKTNsJvZeM6dqiufY");

#[program]
pub mod solana_universal_nft {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
