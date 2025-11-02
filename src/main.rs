use std::path::PathBuf;

use clap::Parser;

mod args;
use args::Args;

mod verilog;
use verilog::{Result, parse_verilog};

fn main() -> Result<()> {
    let args = Args::parse();

    if args.verbose {
        println!("verbose");
    }

    for name in args.names {
        let path = PathBuf::from(name);
        parse_verilog(path)?;
    }

    Ok(())
}
