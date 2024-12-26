use std::{fs::File, path::PathBuf};

use clap::Parser;

#[derive(Parser)]
struct Args {
    file: PathBuf,
}

fn main() {
    let args = Args::parse();

    println!(
        "{:?}",
        geli_core::read_pipeline(File::open(args.file).unwrap())
    );
}
