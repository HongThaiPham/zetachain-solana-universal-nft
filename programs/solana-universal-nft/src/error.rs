use anchor_lang::prelude::*;

#[error_code]
pub enum UniversalNftErrorCode {
    InvalidOriginNft,
    OriginNftAccountAlreadyExists,
    InvalidSlotProvided,
    InvalidGateway,
    InvalidInstructionData,
    InvalidCrossChainMessage,
}
