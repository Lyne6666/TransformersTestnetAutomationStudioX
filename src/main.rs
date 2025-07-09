// src/main.rs
/*
 * Main executable for TransformersTestnetAutomationStudioX
 */

use clap::Parser;
use transformerstestnetautomationstudiox::{Result, run};

#[derive(Parser)]
#[command(version, about = "TransformersTestnetAutomationStudioX - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
