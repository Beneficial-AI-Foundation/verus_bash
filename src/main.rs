use clap::Parser;

mod lib;
mod swap1;
mod swap2;
mod swap_spec;
use lib::FileSystem;

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
    let mut fs = FileSystem;

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
