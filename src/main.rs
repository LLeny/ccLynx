/*
    ccLynx - a subset of C compiler for the Atari Lynx
*/

mod cli;
mod builder;

use cc6502::compile::compile;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use clap::Parser;
use builder::build_cartridge;
use crate::{builder::header::{create_cart_header, prepend_lnx_header}, cli::CliArgs};

fn main() -> Result<(), std::io::Error> {
    env_logger::init();

    let cli_args = CliArgs::parse();
    if cli_args.version {
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        println!("ccLynx v{VERSION} - a subset of C compiler targetting the Atari Lynx console");
        std::process::exit(0);
    }

    let args = cli_args.to_old_args();
    let reader = BufReader::new(File::open(&args.input)?);
    
    let dest = Path::new(&args.output);
    let temp_source = dest.with_extension("s");
    let mut writer = File::create(&temp_source)?;

    if let Err(e) = compile(reader, &mut writer, &args, build_cartridge) {
        eprintln!("{e}");
        std::process::exit(1)
    }

    if !cli_args.keep_asm {
        let _ = std::fs::remove_file(&temp_source);
    }

    let mut header = create_cart_header(&cli_args);
    header.bank0_block_size = cli_args.block_size as u16; 
    prepend_lnx_header(dest, &header)?;

    Ok(())
}
