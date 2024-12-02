use clap::Parser;

/// Advent of Code 2024
#[derive(Parser)]
#[command(about)]
pub struct Args {
    /// The input file to use
    pub input: String
}
