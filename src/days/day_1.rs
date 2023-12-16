use ahash::AHashMap;
use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use regex::Regex;

use super::AocDay;

pub struct Day;

impl AocDay for Day {
	fn part1(&self, input: &str) -> Result<String> {
		let mut result = 0;

		for line in input.lines().map(str::trim).filter(|s| !s.is_empty()) {
			let digit_start =
				line.chars().find(|c| c.is_numeric()).context("Could not find start digit")?;
			let digit_end =
				line.chars().rfind(|c| c.is_numeric()).context("Could not find end digit")?;
			let full_digit = format!("{digit_start}{digit_end}").parse::<u64>()?;
			result += full_digit;
		}

		Ok(result.to_string())
	}

	fn part2(&self, input: &str) -> Result<String> {
		static FIRST_NUMBER_REGEX: Lazy<Regex> = Lazy::new(|| {
			Regex::new(r"^.*?([1-9]|one|two|three|four|five|six|seven|eight|nine).*$")
				.expect("creating regex")
		});
		static LAST_NUMBER_REGEX: Lazy<Regex> = Lazy::new(|| {
			Regex::new(r"^.*([1-9]|one|two|three|four|five|six|seven|eight|nine).*?$")
				.expect("creating regex")
		});

		let mut map = AHashMap::new();
		map.insert("one", "1");
		map.insert("two", "2");
		map.insert("three", "3");
		map.insert("four", "4");
		map.insert("five", "5");
		map.insert("six", "6");
		map.insert("seven", "7");
		map.insert("eight", "8");
		map.insert("nine", "9");

		let mut result = 0;

		for line in input.lines().map(str::trim).filter(|s| !s.is_empty()) {
			let captures =
				FIRST_NUMBER_REGEX.captures(line).context("Applying first number regex")?;
			let digit_start = map.get(&captures[1]).copied().unwrap_or(&captures[1]);

			let captures =
				LAST_NUMBER_REGEX.captures(line).context("Applying first number regex")?;
			let digit_end = map.get(&captures[1]).copied().unwrap_or(&captures[1]);

			let full_digit = format!("{digit_start}{digit_end}").parse::<u64>()?;
			result += full_digit;
		}

		Ok(result.to_string())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1() -> Result<()> {
		let input = r#"
		1abc2
		pqr3stu8vwx
		a1b2c3d4e5f
		treb7uchet
		"#;

		let result = Day.part1(input)?;
		assert_eq!(result.as_str(), "142");

		Ok(())
	}

	#[test]
	fn part2() -> Result<()> {
		let input = r#"
		two1nine
		eightwothree
		abcone2threexyz
		xtwone3four
		4nineeightseven2
		zoneight234
		7pqrstsixteen
		"#;

		let result = Day.part2(input)?;
		assert_eq!(result.as_str(), "281");

		Ok(())
	}
}
