use serde::{Deserialize, Serialize};
use crate::crypto::PublicKey;
use crate::types::{Block, Transaction, TransactionOutput};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Message {
    /// Fetch all UTXOs belonging to a public key
    FetchUTXOs(PublicKey),
    /// UTXOs belonging to a public key. Bool determines if marked
    UTXOs(Vec<(TransactionOutput, bool)>),
}
