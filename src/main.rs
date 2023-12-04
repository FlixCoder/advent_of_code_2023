use advent_of_code_2023::Cli;
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
	Cli::parse().run()
}
