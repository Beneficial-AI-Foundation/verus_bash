use std::collections::HashMap;
use clap::Parser;

mod lib;
mod swap1;

#[derive(Parser)]
#[command(name = "verus_bash")]
#[command(about = "A file swapping utility")]
struct Args {
    /// First filename
    file1: String,
    /// Second filename  
    file2: String,
}

fn main() {
    let args = Args::parse();
    let mut fs = HashMap::new();
    
    swap1::swap(&args.file1, &args.file2, &mut fs);
}
