use crate::lib::FileSystem;
use clap::Parser;
use std::collections::HashMap;
mod lib;
mod swap1;
mod swap2;
mod swap_spec;

#[derive(Parser)]
#[command(name = "verus_bash")]
#[command(about = "A file swapping utility")]
struct Args {
    file1: String,
    file2: String,
}

fn main() {
    let args = Args::parse();
    let mut fs = FileSystem {
        contents: HashMap::new(),
    };

    match swap1::swap(&args.file1, &args.file2, &mut fs) {
        Ok(()) => println!("Swap completed successfully"),
        Err(swap_spec::SwapError::BadArgs) => {
            eprintln!("Error: Invalid arguments. Files cannot be the same or named 'tmp_file'");
            std::process::exit(1);
        }
        Err(swap_spec::SwapError::OperationFailed) => {
            eprintln!("Error: Operation failed");
            std::process::exit(1);
        }
    }
}
