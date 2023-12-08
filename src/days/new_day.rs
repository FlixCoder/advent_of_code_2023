use std::str::FromStr;

use anyhow::{Context, Result};

use super::AocDay;

pub struct Day;

impl AocDay for Day {
	fn part1(&self, input: &str) -> Result<String> {
		Ok("unimplemented".to_string())
	}

	fn part2(&self, input: &str) -> Result<String> {
		Ok("unimplemented".to_string())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"
		Time:      7  15   30
		Distance:  9  40  200
		"#;

	#[test]
	fn part1() -> Result<()> {
		let result = Day.part1(INPUT)?;
		assert_eq!(result.as_str(), "288");

		Ok(())
	}

	#[test]
	fn part2() -> Result<()> {
		let result = Day.part2(INPUT)?;
		assert_eq!(result.as_str(), "71503");

		Ok(())
	}
}
