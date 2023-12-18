use std::str::FromStr;

use anyhow::{bail, Context, Result};
use once_cell::sync::Lazy;
use regex::Regex;

use super::AocDay;

pub struct Day;

impl AocDay for Day {
	fn part1(&self, input: &str) -> Result<String> {
		let instructions = parse_instructions_1(input)?;
		let mut world = World::default();
		world.set_corner_points(instructions);

		let total_space = world.total_space_filled();
		Ok(total_space.to_string())
	}

	fn part2(&self, input: &str) -> Result<String> {
		let instructions = parse_instructions_2(input)?;
		let mut world = World::default();
		world.set_corner_points(instructions);

		let total_space = world.total_space_filled();
		Ok(total_space.to_string())
	}
}

impl World {
	pub fn set_corner_points(&mut self, instructions: Vec<Instruction>) {
		let mut pos = Position { x: 0, y: 0 };
		self.corners.push(pos);
		let mut total_length = 0;
		for instruction in instructions {
			total_length += instruction.steps;
			pos = pos.go(instruction.direction, instruction.steps);
			self.corners.push(pos);
		}
		self.perimeter_length = total_length;
	}

	pub fn total_space_filled(&self) -> usize {
		// Shoelace formula.
		let two_area: isize =
			self.corners.windows(2).map(|w| w[0].x * w[1].y - w[0].y * w[1].x).sum::<isize>();
		let area = (two_area / 2).unsigned_abs();

		// Pick's theorem.
		area + self.perimeter_length / 2 + 1
	}
}

impl Position {
	pub fn go(self, direction: Direction, steps: usize) -> Self {
		let steps = steps as isize;
		match direction {
			Direction::Up => Self { x: self.x, y: self.y - steps },
			Direction::Right => Self { x: self.x + steps, y: self.y },
			Direction::Down => Self { x: self.x, y: self.y + steps },
			Direction::Left => Self { x: self.x - steps, y: self.y },
		}
	}
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Position {
	x: isize,
	y: isize,
}

#[derive(Debug, Default)]
struct World {
	corners: Vec<Position>,
	perimeter_length: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
	Up,
	Right,
	Down,
	Left,
}

struct Instruction {
	direction: Direction,
	steps: usize,
}

impl FromStr for Direction {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self> {
		match s {
			"U" | "3" => Ok(Self::Up),
			"R" | "0" => Ok(Self::Right),
			"D" | "1" => Ok(Self::Down),
			"L" | "2" => Ok(Self::Left),
			_ => bail!("Invalid direction `{s}`"),
		}
	}
}

static LINE_REGEX: Lazy<Regex> = Lazy::new(|| {
	Regex::new(r"(U|R|D|L) (\d+) \(#([[:xdigit:]]{5})(\d)\)").expect("creating regex")
});

fn parse_instructions_1(input: &str) -> Result<Vec<Instruction>> {
	let mut instructions = Vec::new();
	for line in input.trim().lines() {
		let captures = LINE_REGEX.captures(line).context("apply line regex")?;
		let instruction =
			Instruction { direction: captures[1].parse()?, steps: captures[2].parse()? };
		instructions.push(instruction);
	}
	Ok(instructions)
}

fn parse_instructions_2(input: &str) -> Result<Vec<Instruction>> {
	let mut instructions = Vec::new();
	for line in input.trim().lines() {
		let captures = LINE_REGEX.captures(line).context("apply line regex")?;
		let instruction = Instruction {
			direction: captures[4].parse()?,
			steps: usize::from_str_radix(&captures[3], 16)?,
		};
		instructions.push(instruction);
	}
	Ok(instructions)
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"
		R 6 (#70c710)
		D 5 (#0dc571)
		L 2 (#5713f0)
		D 2 (#d2c081)
		R 2 (#59c680)
		D 2 (#411b91)
		L 5 (#8ceee2)
		U 2 (#caa173)
		L 1 (#1b58a2)
		U 2 (#caa171)
		R 2 (#7807d2)
		U 3 (#a77fa3)
		L 2 (#015232)
		U 2 (#7a21e3)
		"#;

	#[test]
	fn part1() -> Result<()> {
		let result = Day.part1(INPUT)?;
		assert_eq!(result.as_str(), "62");

		Ok(())
	}

	#[test]
	fn part2() -> Result<()> {
		let result = Day.part2(INPUT)?;
		assert_eq!(result.as_str(), "952408144115");

		Ok(())
	}
}
