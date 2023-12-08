use anyhow::Result;
use once_cell::sync::Lazy;
use regex::Regex;

use super::AocDay;

pub struct Day;

impl AocDay for Day {
	fn part1(&self, input: &str) -> Result<String> {
		let lines = input.trim().lines().map(|line| line.trim()).collect::<Vec<_>>();

		let mut total: u64 = 0;
		for i in 0..lines.len() {
			for number_match in NUMBER_REGEX.find_iter(lines[i]) {
				if is_number_adjacent_to_symbol(&lines, i, number_match.start(), number_match.end())
				{
					let number: u64 = number_match.as_str().parse()?;
					total += number;
				}
			}
		}
		Ok(total.to_string())
	}

	fn part2(&self, _input: &str) -> Result<String> {
		Ok("unimplemented".to_string())
	}
}

static NUMBER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[0-9]+").expect("create Regex"));

fn is_number_adjacent_to_symbol(lines: &[&str], line_i: usize, start: usize, end: usize) -> bool {
	let start = start.saturating_sub(1);
	let end = end.saturating_add(1).min(lines[line_i].len());
	let len = end - start;

	if let Some(line) = lines.get(line_i.wrapping_sub(1)) {
		if line.chars().skip(start).take(len).any(is_symbol) {
			return true;
		}
	}
	if lines[line_i].chars().skip(start).take(len).any(is_symbol) {
		return true;
	}
	if let Some(line) = lines.get(line_i.saturating_add(1)) {
		if line.chars().skip(start).take(len).any(is_symbol) {
			return true;
		}
	}

	false
}

fn is_symbol(c: char) -> bool {
	!c.is_alphanumeric() && c != '.' && !c.is_whitespace()
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"
		467..114..
		...*......
		..35..633.
		......#...
		617*......
		.....+.58.
		..592.....
		......755.
		...$.*....
		.664.598..
		"#;

	#[test]
	fn part1() -> Result<()> {
		let result = Day.part1(INPUT)?;
		assert_eq!(result.as_str(), "4361");

		Ok(())
	}
}
