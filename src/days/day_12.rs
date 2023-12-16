use std::str::FromStr;

use ahash::AHashMap;
use anyhow::{bail, Context, Result};
use rayon::prelude::*;

use super::AocDay;

pub struct Day;

impl AocDay for Day {
	fn part1(&self, input: &str) -> Result<String> {
		let rows = Rows::from_str(input)?;
		let sum_of_possible_arrangements = rows.sum_possible_arrangements();
		Ok(sum_of_possible_arrangements.to_string())
	}

	fn part2(&self, input: &str) -> Result<String> {
		let mut rows = Rows::from_str(input)?;
		rows.expand();
		let sum_of_possible_arrangements = rows.sum_possible_arrangements();
		Ok(sum_of_possible_arrangements.to_string())
	}
}

impl Rows {
	pub fn sum_possible_arrangements(&self) -> usize {
		self.0.par_iter().map(|row| row.possible_arrangements()).sum()
	}

	pub fn expand(&mut self) {
		self.0.iter_mut().for_each(|row| row.expand())
	}
}

impl Row {
	pub fn possible_arrangements(&self) -> usize {
		Self::internal_possible_arrangements(&self.records, &self.continuous, &mut AHashMap::new())
	}

	fn internal_possible_arrangements<'a>(
		records: &'a [Option<Status>],
		continuous: &'a [usize],
		cache: &mut AHashMap<(&'a [Option<Status>], &'a [usize]), usize>,
	) -> usize {
		if continuous.is_empty() {
			if records.contains(&Some(Status::Damaged)) {
				return 0;
			} else {
				return 1;
			}
		} else if records.is_empty() {
			return 0;
		}

		if let Some(cached) = cache.get(&(records, continuous)) {
			return *cached;
		}

		let mut combinations = 0;
		if records[0].is_none() || records[0] == Some(Status::Operational) {
			let mut index = 1;
			while records.len() > index && records[index] == Some(Status::Operational) {
				index += 1;
			}
			combinations +=
				Self::internal_possible_arrangements(&records[index..], continuous, cache);
		}

		if records[0].is_none() || records[0] == Some(Status::Damaged) {
			let mut count = 1;
			let mut index = 1;
			while count < continuous[0] {
				if records.len() <= index {
					return combinations;
				} else if records[index].is_none() || records[index] == Some(Status::Damaged) {
					count += 1;
					index += 1;
				} else {
					return combinations;
				}
			}

			if !(records.len() > index && records[index] == Some(Status::Damaged)) {
				index = (index + 1).min(records.len());
				combinations += Self::internal_possible_arrangements(
					&records[index..],
					&continuous[1..],
					cache,
				);
			}
		}

		cache.insert((records, continuous), combinations);
		combinations
	}

	pub fn expand(&mut self) {
		let new_records = [
			self.records.as_slice(),
			&[None],
			self.records.as_slice(),
			&[None],
			self.records.as_slice(),
			&[None],
			self.records.as_slice(),
			&[None],
			self.records.as_slice(),
		]
		.into_iter()
		.flatten()
		.copied()
		.collect();
		self.records = new_records;

		let new_continuous = self.continuous.repeat(5);
		self.continuous = new_continuous;
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Status {
	Operational,
	Damaged,
}

#[derive(Debug, Clone)]
struct Row {
	records: Vec<Option<Status>>,
	continuous: Vec<usize>,
}

struct Rows(Vec<Row>);

impl FromStr for Row {
	type Err = anyhow::Error;

	fn from_str(line: &str) -> Result<Self> {
		let (records, continuous) =
			line.trim().split_once(' ').context("line has no separating space")?;

		let records = records
			.chars()
			.map(|c| match c {
				'.' => Ok(Some(Status::Operational)),
				'#' => Ok(Some(Status::Damaged)),
				'?' => Ok(None),
				_ => bail!("Invalid char '{c}'"),
			})
			.collect::<Result<_, _>>()?;
		let continuous = continuous.split(',').map(usize::from_str).collect::<Result<_, _>>()?;

		Ok(Self { records, continuous })
	}
}

impl FromStr for Rows {
	type Err = anyhow::Error;

	fn from_str(input: &str) -> Result<Self> {
		let rows = input.trim().lines().map(Row::from_str).collect::<Result<_, _>>()?;
		Ok(Self(rows))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"
		???.### 1,1,3
		.??..??...?##. 1,1,3
		?#?#?#?#?#?#?#? 1,3,1,6
		????.#...#... 4,1,1
		????.######..#####. 1,6,5
		?###???????? 3,2,1
		"#;

	#[test]
	fn part1() -> Result<()> {
		let result = Day.part1(INPUT)?;
		assert_eq!(result.as_str(), "21");

		Ok(())
	}

	#[test]
	fn part2() -> Result<()> {
		let result = Day.part2(INPUT)?;
		assert_eq!(result.as_str(), "525152");

		Ok(())
	}
}
