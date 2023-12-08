use anyhow::Result;
use regex::Regex;

use super::AocDay;

pub struct Day;

impl AocDay for Day {
	fn part1(&self, input: &str) -> Result<String> {
		let number_regex = Regex::new(r"[0-9]+")?;
		let lines = input.trim().lines().map(|line| line.trim()).collect::<Vec<_>>();

		let mut total: u64 = 0;
		for i in 0..lines.len() {
			for number_match in number_regex.find_iter(lines[i]) {
				if is_number_adjacent_to_symbol(&lines, i, number_match.start(), number_match.end())
				{
					let number: u64 = number_match.as_str().parse()?;
					total += number;
				}
			}
		}
		Ok(total.to_string())
	}

	fn part2(&self, input: &str) -> Result<String> {
		let lines = input.trim().lines().map(|line| line.trim()).collect::<Vec<_>>();

		let mut total = 0;
		for i in 0..lines.len() {
			for (location, _) in lines[i].match_indices('*') {
				if let Some(numbers) = find_adjacent_numbers(&lines, i, location)? {
					total += numbers.0 * numbers.1;
				}
			}
		}
		Ok(total.to_string())
	}
}

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

fn find_adjacent_numbers(
	lines: &[&str],
	line_i: usize,
	location: usize,
) -> Result<Option<(u64, u64)>> {
	let start = location.saturating_sub(1);
	let end = location.saturating_add(2).min(lines[line_i].len());
	let len = end - start;

	let mut first = None;
	if line_i > 0 {
		let upper: Vec<char> = lines[line_i - 1].chars().skip(start).take(len).collect();
		if upper.len() == 3
			&& !upper[1].is_numeric()
			&& upper[0].is_numeric()
			&& upper[2].is_numeric()
		{
			let first = find_number(lines[line_i - 1], location - 1)?;
			let second = find_number(lines[line_i - 1], location + 1)?;
			return Ok(Some((first, second)));
		} else if let Some((found, _)) = upper.iter().enumerate().find(|(_, c)| c.is_numeric()) {
			let loc = if location > 0 { location - 1 + found } else { location + found };
			first = Some(find_number(lines[line_i - 1], loc)?);
		}
	}

	if lines[line_i].chars().nth(start).unwrap().is_numeric() {
		let second = find_number(lines[line_i], start)?;
		if let Some(first) = first {
			return Ok(Some((first, second)));
		} else {
			first = Some(second);
		}
	}

	if lines[line_i].chars().nth(end - 1).unwrap_or('.').is_numeric() {
		let second = find_number(lines[line_i], end - 1)?;
		if let Some(first) = first {
			return Ok(Some((first, second)));
		} else {
			first = Some(second);
		}
	}

	if line_i + 1 < lines.len() {
		let lower: Vec<char> = lines[line_i + 1].chars().skip(start).take(len).collect();
		if let Some(first) = first {
			if let Some((found, _)) = lower.iter().enumerate().find(|(_, c)| c.is_numeric()) {
				let loc = if location > 0 { location - 1 + found } else { location + found };
				let second = find_number(lines[line_i + 1], loc)?;
				return Ok(Some((first, second)));
			}
		} else if lower.len() == 3
			&& !lower[1].is_numeric()
			&& lower[0].is_numeric()
			&& lower[2].is_numeric()
		{
			let first = find_number(lines[line_i + 1], location - 1)?;
			let second = find_number(lines[line_i + 1], location + 1)?;
			return Ok(Some((first, second)));
		}
	}

	Ok(None)
}

fn find_number(line: &str, start: usize) -> Result<u64> {
	let mut start = start;
	let mut end = start + 1;
	while start > 0 && line.chars().nth(start - 1).unwrap().is_numeric() {
		start -= 1;
	}
	while line.chars().nth(end).unwrap_or('.').is_numeric() {
		end += 1;
	}
	Ok(line[start..end].parse()?)
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

	#[test]
	fn part2() -> Result<()> {
		let result = Day.part2(INPUT)?;
		assert_eq!(result.as_str(), "467835");

		Ok(())
	}
}
