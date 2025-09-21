use anyhow::Result;
use clap::{Parser, Subcommand};
use kanal::bounded;
use tokio::time::{self, Duration};
use std::io::{self, Write};
use std::path::PathBuf;
use btclib::types::Transaction;

mod core;

fn main() {
    println!("ho");
}
