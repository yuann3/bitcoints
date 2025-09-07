use std::process::exit;
use std::{env, usize};
use btclib::types::Block;
use btclib::util::Saveable;

fn main() {
    let (path, steps) = if let (Some(arg), Some(arg2)) = (env::args().nth(1), env::args().nth(2)) {
        (arg, arg2)
    } else {
        eprintln!("Usage: miner <block_file> <steps>");
        exit(1);
    };

    let steps: usize = if let Ok(s @ 1..=usize::MAX) = steps.parse() {
        s
    } else {
        eprintln!("<steps> should be a positive integer");
        exit(1);
    };

    // load block from file
    let og_block = Block::load_from_file(path).expect("Failed to load block");
    let mut block = og_block.clone();

    // mine
    while !block.header.mine(steps) {
        println!("mining.....");
    }

    // print original block and hash
    println!("original: {:#?}", og_block);
    println!("hash: {}", og_block.header.hash());

    // print mined block and hash
    println!("final: {:#?}", block);
    println!("hash: {}", block.header.hash());

}
