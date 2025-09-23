use anyhow::Result;
use btclib::types::Transaction;
use clap::{Parser, Subcommand};
use kanal::bounded;
use std::io::{self, Write};
use std::path::PathBuf;
use tokio::time::{self, Duration};
use core::{Config, Core, FeeConfig, FeeType, Recipient};

mod core;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
    #[arg(short, long, value_name = "ADDR")]
    node: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    GenerateConfig {
        #[arg(short, long, value_name = "FILE")]
        output: PathBuf,
    },
}

async fn update_utxos(core: Arc<Core>) {}
