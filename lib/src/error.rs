use thiserror::Error;

#[derive(Error, Debug)]
pub enum BtcError {
   #[error("Invalid transaction")]
    InvalidTransaction,
   #[error("Invalid block")]
    InvalidBlock,
   #[error("Invalid block header")]
    InvalidBlockHeader,
   #[error("Invalid transaction input")]
    InvalidTransactionInput,
   #[error("Invalid transaction output")]
    InvalidTransactionOutput,
}
