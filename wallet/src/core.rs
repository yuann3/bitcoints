use anyhow::Result;
use btclib::crypto::{PrivateKey, PublicKey};
use btclib::network::Message;
use btclib::types::{Transaction, TransactionOutput};
use btclib::util::Saveable;
use crossbeam_skiplist::SkipMap;
use kanal::AsyncSender;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::TcpStream;

#[derive(Serialize, Deserialize, Clone)]
pub struct Key {
    public: PathBuf,
    private: PathBuf,
}

#[derive(Clone)]
struct LoadedKey {
    public: PublicKey,
    private: PrivateKey,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Recipient {
    pub name: String,
    pub key: PathBuf,
}

#[derive(Debug)]
pub struct LoadedRecipient {
    pub name: String,
    pub key: PublicKey,
}

impl Recipient {
    pub fn load(&self) -> Result<LoadedRecipient> {
        let key = PublicKey::load_from_file(&self.key)?;
        Ok(LoadedRecipient {
            name: self.name.clone(),
            key,
        })
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum FeeType {
    Fixed,
    Percent,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FeeConfig {
    pub fee_type: FeeType,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub my_keys: Vec<Key>,
    pub contacts: Vec<Recipient>,
    pub default_node: String,
    pub fee_config: FeeConfig,
}

#[derive(Clone)]
struct UtxoStore {
    my_keys: Vec<LoadedKey>,
    utxos: Arc<SkipMap<PublicKey, Vec<(bool, TransactionOutput)>>>,
}

impl UtxoStore {
    fn new() -> Self {
        UtxoStore {
            my_keys: Vec::new(),
            utxos: Arc::new(SkipMap::new()),
        }
    }

    fn add_key(&mut self, key: LoadedKey) {
        self.my_keys.push(key);
    }
}

#[derive(Clone)]
pub struct Core {
    pub config: Config,
    utxos: UtxoStore,
    pub tx_sender: AsyncSender<Transaction>,
}

impl Core {
    fn new(config: Config, utxos: UtxoStore) -> Self {
        let (tx_sender, _) = kanal::bounded(10);
        Core {
            config,
            utxos,
            tx_sender: tx_sender.clone_async(),
        }
    }

    pub fn load(config_path: PathBuf) -> Result<Self> {
        let config: Config = toml::from_str(&fs::read_to_string(&config_path)?)?;
        let mut utxos = UtxoStore::new();
        unimplemented!();
    }
}
