use anchor_lang::prelude::*;

use crate::error::UniversalNftErrorCode;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum CrossChainFunction {
    TransferNft,
    ReceiveNft,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct CrossChainMessage {
    pub mint: Pubkey,
    pub token_id: [u8; 32],
    pub sender: Pubkey,
    pub recipient: [u8; 20],
    pub dest_chain_id: u64,
    pub fun: CrossChainFunction,
}

impl CrossChainMessage {
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let message: CrossChainMessage = CrossChainMessage::try_from_slice(data)
            .map_err(|_| Error::from(UniversalNftErrorCode::InvalidInstructionData))?;
        Ok(message)
    }
}
