use std::{collections::BTreeMap, str::FromStr};

use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use regex::Regex;

use super::AocDay;

pub struct Day;

impl AocDay for Day {
	fn part1(&self, input: &str) -> Result<String> {
		let game = Game::from_str(input)?;

		let steps = game.steps_from_to("AAA", "ZZZ")?;

		Ok(steps.to_string())
	}

	fn part2(&self, input: &str) -> Result<String> {
		let game = Game::from_str(input)?;

		let steps = game.steps_part2()?;

		Ok(steps.to_string())
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
	Left,
	Right,
}

impl From<char> for Direction {
	fn from(c: char) -> Self {
		match c {
			'L' => Self::Left,
			'R' => Self::Right,
			_ => panic!("'{c}' is not a valid direction"),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Either<T> {
	left: T,
	right: T,
}

impl<T> Either<T> {
	fn get(&self, which: Direction) -> &T {
		match which {
			Direction::Left => &self.left,
			Direction::Right => &self.right,
		}
	}
}

struct Map {
	map: BTreeMap<String, Either<String>>,
}

impl FromStr for Map {
	type Err = anyhow::Error;

	fn from_str(map_str: &str) -> Result<Self> {
		static REGEX: Lazy<Regex> =
			Lazy::new(|| Regex::new(r"(.{3}) = \((.{3}), (.{3})\)").expect("create Regex"));

		let map = map_str
			.trim()
			.lines()
			.map(|line| {
				let captures = REGEX.captures(line.trim()).context("applying regex failed")?;

				let source = captures
					.get(1)
					.context("could not get source from regex capture")?
					.as_str()
					.to_owned();
				let left = captures
					.get(2)
					.context("could not get source from regex capture")?
					.as_str()
					.to_owned();
				let right = captures
					.get(3)
					.context("could not get source from regex capture")?
					.as_str()
					.to_owned();

				Ok::<_, anyhow::Error>((source, Either { left, right }))
			})
			.collect::<Result<_, _>>()?;

		Ok(Self { map })
	}
}

impl Map {
	fn next(&self, source: &str, direction: Direction) -> Option<&String> {
		let either = self.map.get(source)?;
		Some(either.get(direction))
	}
}

struct Game {
	sequence: Vec<Direction>,
	map: Map,
}

impl FromStr for Game {
	type Err = anyhow::Error;

	fn from_str(input: &str) -> Result<Self> {
		let (sequence, map) =
			input.split_once("\n\n").context("invalid input: no double new line")?;

		let sequence = sequence.trim().chars().map(Direction::from).collect();
		let map = map.parse()?;

		Ok(Self { sequence, map })
	}
}

impl Game {
	fn steps_from_to(&self, source: &str, target: &str) -> Result<usize> {
		let mut sequence = self.sequence.iter().copied().cycle();
		let mut current = source;
		let mut steps = 0;
		while current != target {
			let direction = sequence.next().context("get next direction")?;
			current = self.map.next(current, direction).context("could not find node")?;
			steps += 1;
		}
		Ok(steps)
	}

	fn steps_part2(&self) -> Result<usize> {
		let starts = self.map.map.keys().filter(|key| key.ends_with('A')).collect::<Vec<_>>();
		let zs_reached = starts
			.into_iter()
			.map(|mut current| {
				let mut sequence = self.sequence.iter().copied().cycle();
				let mut steps = 0;
				while !current.ends_with('Z') {
					let direction = sequence.next().expect("get next direction");
					current = self.map.next(current, direction).expect("could not find node");
					steps += 1;
				}
				steps
			})
			.collect::<Vec<usize>>();

		zs_reached.into_iter().reduce(lcm).context("find LCM")
	}
}

fn gcd(a: usize, b: usize) -> usize {
	if b == 0 {
		a
	} else {
		gcd(b, a % b)
	}
}

fn lcm(a: usize, b: usize) -> usize {
	a * b / gcd(a, b)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1() -> Result<()> {
		let input = r#"
		RL

		AAA = (BBB, CCC)
		BBB = (DDD, EEE)
		CCC = (ZZZ, GGG)
		DDD = (DDD, DDD)
		EEE = (EEE, EEE)
		GGG = (GGG, GGG)
		ZZZ = (ZZZ, ZZZ)
		"#;

		let result = Day.part1(input)?;
		assert_eq!(result.as_str(), "2");

		let input = r#"
		LLR

		AAA = (BBB, BBB)
		BBB = (AAA, ZZZ)
		ZZZ = (ZZZ, ZZZ)
		"#;

		let result = Day.part1(input)?;
		assert_eq!(result.as_str(), "6");

		Ok(())
	}

	#[test]
	fn part2() -> Result<()> {
		let input = r#"
		LR

		11A = (11B, XXX)
		11B = (XXX, 11Z)
		11Z = (11B, XXX)
		22A = (22B, XXX)
		22B = (22C, 22C)
		22C = (22Z, 22Z)
		22Z = (22B, 22B)
		XXX = (XXX, XXX)
		"#;

		let result = Day.part2(input)?;
		assert_eq!(result.as_str(), "6");

		Ok(())
	}
}
