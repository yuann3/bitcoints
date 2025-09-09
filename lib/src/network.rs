use crate::crypto::PublicKey;
use crate::types::{Block, Transaction, TransactionOutput};
use serde::{Deserialize, Serialize};
use std::io::{Error as IoError, Read, Write};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Message {
    /// Fetch all UTXOs belonging to a public key
    FetchUTXOs(PublicKey),
    /// UTXOs belonging to a public key. Bool determines if marked
    UTXOs(Vec<(TransactionOutput, bool)>),
    /// Send a transaction to the network
    SubmitTransaction(Transaction),
    /// Boardcast a new transaction to other nodes
    NewTransaction(Transaction),
    /// Ask the node to prepare the optimal block template
    /// with the coinbase transaction paying the specified
    /// public key
    FetchTemplate(PublicKey),
    /// The template
    Template(Block),
    /// Ask the node to validate a block template.
    /// This is to prevent the node from mining an invalid
    /// block (e.g. if one has been found in the meantime,
    /// or if transactions have been removed from the mempool)
    ValidateTemplate(Block),
    /// If template is valid
    TemplateValidity(bool),
    /// Submit a mined block to a node
    SubmitTemplate(Block),
    /// Ask a node to report all the other nodes it knows
    /// about
    DiscoverNodes,
    /// This is the response to DiscoverNodes
    NodeList(Vec<String>),
    /// Ask a node whats the highest block it knows about
    /// in comparison to the local blockchain
    AskDifference(u32),
    /// This is the response to AskDifference
    Difference(i32),
    /// Ask a node to send a block with the specified height
    FetchBlock(usize),
    /// Broadcast a new block to other nodes
    NewBlock(Block),
}

impl Message {
    pub fn encode(&self) -> Result<Vec<u8>, ciborium::ser::Error<IoError>> {
        let mut bytes = Vec::new();
        ciborium::into_writer(self, &mut bytes)?;
        Ok(bytes)
    }

    pub fn decode(data: &[u8]) -> Result<Self, ciborium::de::Error<IoError>> {
        ciborium::from_reader(data)
    }
}
