mod days;

use std::time::Instant;

use anyhow::{bail, Context, Result};
use clap::{value_parser, Parser};

#[derive(Debug, Parser)]
pub struct Cli {
	#[arg(value_parser = value_parser!(u8).range(1..=25))]
	day: u8,
}

impl Cli {
	pub fn run(self) -> Result<()> {
		if let Some(day) = days::DAYS.get(&self.day) {
			let input = fetch_input(self.day)?;

			let time = Instant::now();
			let result = day.part1(&input)?;
			println!("Part 1 ({:?}): {result}", time.elapsed());

			let time = Instant::now();
			let result = day.part2(&input)?;
			println!("Part 2 ({:?}): {result}", time.elapsed());
		} else {
			bail!("This day is not implemented");
		}
		Ok(())
	}
}

fn fetch_input(day: u8) -> Result<String> {
	let file = format!("./inputs/day_{day}.txt");
	let input = std::fs::read_to_string(&file).context(format!("Could not read file `{file}`"))?;
	Ok(input)
}

#[cfg(test)]
mod tests {
	use clap::CommandFactory;

	use super::*;

	#[test]
	fn cli() {
		Cli::command().debug_assert();
	}
}
