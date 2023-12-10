use std::{collections::HashSet, str::FromStr};

use anyhow::{bail, Context, Result};

use super::AocDay;

pub struct Day;

impl AocDay for Day {
	fn part1(&self, input: &str) -> Result<String> {
		let grid = Grid::from_str(input)?;
		let start = grid.find_start()?;
		let l = grid.get_loop(start)?;
		let farthest = l.len() / 2;
		Ok(farthest.to_string())
	}

	fn part2(&self, input: &str) -> Result<String> {
		let grid = Grid::from_str(input)?;
		let start = grid.find_start()?;
		let l = grid.get_loop(start)?;

		let (width, height) = grid.dimensions();
		let l: HashSet<Position> = l.into_iter().collect();
		let mut enclosed = 0;
		for y in 0..height {
			for x in 0..width {
				let position = Position { x, y };
				if position.is_enclosed(&l, &grid) {
					enclosed += 1;
				}
			}
		}

		Ok(enclosed.to_string())
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Position {
	x: usize,
	y: usize,
}

impl Position {
	pub fn is_enclosed(&self, l: &HashSet<Self>, grid: &Grid) -> bool {
		if l.contains(self) {
			return false;
		}

		let mut directions_x = Directions::default();
		for ex in 0..self.x {
			if l.contains(&Position { x: ex, y: self.y }) {
				let pipe = grid.get(ex, self.y).unwrap();
				directions_x += Directions::from(pipe);
			}
		}
		let mut directions_y = Directions::default();
		for ey in 0..self.y {
			if l.contains(&Position { x: self.x, y: ey }) {
				let pipe = grid.get(self.x, ey).unwrap();
				directions_y += Directions::from(pipe);
			}
		}

		directions_x.is_vertical() && directions_y.is_horizontal()
	}
}

#[derive(Debug, Default)]
struct Directions {
	up: usize,
	down: usize,
	left: usize,
	right: usize,
}

impl Directions {
	pub fn is_vertical(&self) -> bool {
		self.up % 2 == 1 && self.down % 2 == 1
	}

	pub fn is_horizontal(&self) -> bool {
		self.left % 2 == 1 && self.right % 2 == 1
	}
}

impl From<char> for Directions {
	fn from(c: char) -> Self {
		match c {
			'|' => Directions { up: 1, down: 1, left: 0, right: 0 },
			'-' => Directions { up: 0, down: 0, left: 1, right: 1 },
			'L' => Directions { up: 1, down: 0, left: 0, right: 1 },
			'J' => Directions { up: 1, down: 0, left: 1, right: 0 },
			'7' => Directions { up: 0, down: 1, left: 1, right: 0 },
			'F' => Directions { up: 0, down: 1, left: 0, right: 1 },
			'S' => Directions { up: 1, down: 1, left: 1, right: 1 }, //TODO
			_ => Directions { up: 0, down: 0, left: 0, right: 0 },
		}
	}
}

impl std::ops::AddAssign for Directions {
	fn add_assign(&mut self, rhs: Self) {
		self.up += rhs.up;
		self.down += rhs.down;
		self.left += rhs.left;
		self.right += rhs.right;
	}
}

#[derive(Debug)]
struct Grid {
	grid: Vec<char>,
	width: usize,
}

impl Grid {
	pub fn get(&self, x: usize, y: usize) -> Option<char> {
		self.grid.get(y * self.width + x).copied()
	}

	pub fn dimensions(&self) -> (usize, usize) {
		(self.width, self.grid.len() / self.width)
	}

	pub fn find_start(&self) -> Result<Position> {
		let index = self
			.grid
			.iter()
			.enumerate()
			.find_map(|(i, c)| (*c == 'S').then_some(i))
			.context("Could not find start 'S'")?;
		let x = index % self.width;
		let y = index / self.width;
		Ok(Position { x, y })
	}

	fn next(&self, current: Position, previous: Position) -> Result<Position> {
		let pipe = self.get(current.x, current.y).context("Current position is not on the grid")?;
		let [p1, p2] = match pipe {
			'S' => {
				if let Some('-' | 'J' | '7') = self.get(current.x + 1, current.y) {
					return Ok(Position { x: current.x + 1, y: current.y });
				} else if let Some('|' | 'L' | 'J') = self.get(current.x, current.y + 1) {
					return Ok(Position { x: current.x, y: current.y + 1 });
				} else if let Some('-' | 'L' | 'F') = self.get(current.x.wrapping_sub(1), current.y)
				{
					return Ok(Position { x: current.x - 1, y: current.y });
				} else if let Some('|' | '7' | 'F') = self.get(current.x, current.y.wrapping_sub(1))
				{
					return Ok(Position { x: current.x, y: current.y - 1 });
				} else {
					bail!("No next position from 'S'");
				}
			}
			'|' => [
				Position { x: current.x, y: current.y.wrapping_sub(1) },
				Position { x: current.x, y: current.y + 1 },
			],
			'-' => [
				Position { x: current.x.wrapping_sub(1), y: current.y },
				Position { x: current.x + 1, y: current.y },
			],
			'L' => [
				Position { x: current.x, y: current.y.wrapping_sub(1) },
				Position { x: current.x + 1, y: current.y },
			],
			'J' => [
				Position { x: current.x, y: current.y.wrapping_sub(1) },
				Position { x: current.x.wrapping_sub(1), y: current.y },
			],
			'7' => [
				Position { x: current.x.wrapping_sub(1), y: current.y },
				Position { x: current.x, y: current.y + 1 },
			],
			'F' => [
				Position { x: current.x + 1, y: current.y },
				Position { x: current.x, y: current.y + 1 },
			],
			_ => bail!("Invalid pipe at current position: {pipe}"),
		};

		if p1 == previous {
			Ok(p2)
		} else {
			Ok(p1)
		}
	}

	pub fn get_loop(&self, start: Position) -> Result<Vec<Position>> {
		let mut current = start;
		let mut previous = start;
		let mut positions = Vec::new();
		loop {
			let next = self.next(current, previous)?;
			previous = current;
			current = next;
			positions.push(current);

			if current == start {
				break;
			}
		}
		Ok(positions)
	}
}

impl FromStr for Grid {
	type Err = anyhow::Error;

	fn from_str(input: &str) -> Result<Self> {
		let mut width = 0;
		let grid = input
			.trim()
			.lines()
			.flat_map(|line| {
				width = line.trim().len(); // oof xD
				line.trim().chars()
			})
			.collect::<Vec<_>>();

		if grid.len() % width != 0 {
			bail!("Bad grid size");
		}

		Ok(Self { grid, width })
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1() -> Result<()> {
		let input = r#"
		-L|F7
		7S-7|
		L|7||
		-L-J|
		L|-JF
		"#;
		let result = Day.part1(input)?;
		assert_eq!(result.as_str(), "4");

		let input = r#"
		7-F7-
		.FJ|7
		SJLL7
		|F--J
		LJ.LJ
		"#;
		let result = Day.part1(input)?;
		assert_eq!(result.as_str(), "8");

		Ok(())
	}

	#[test]
	fn part2() -> Result<()> {
		let input = r#"
		...........
		.S-------7.
		.|F-----7|.
		.||.....||.
		.||.....||.
		.|L-7.F-J|.
		.|..|.|..|.
		.L--J.L--J.
		...........
		"#;
		let result = Day.part2(input)?;
		assert_eq!(result.as_str(), "4");

		let input = r#"
		.F----7F7F7F7F-7....
		.|F--7||||||||FJ....
		.||.FJ||||||||L7....
		FJL7L7LJLJ||LJ.L-7..
		L--J.L7...LJS7F-7L7.
		....F-J..F7FJ|L7L7L7
		....L7.F7||L7|.L7L7|
		.....|FJLJ|FJ|F7|.LJ
		....FJL-7.||.||||...
		....L---J.LJ.LJLJ...
		"#;
		let result = Day.part2(input)?;
		assert_eq!(result.as_str(), "8");

		let input = r#"
		FF7FSF7F7F7F7F7F---7
		L|LJ||||||||||||F--J
		FL-7LJLJ||||||LJL-77
		F--JF--7||LJLJ7F7FJ-
		L---JF-JLJ.||-FJLJJ7
		|F|F-JF---7F7-L7L|7|
		|FFJF7L7F-JF7|JL---7
		7-L-JL7||F7|L7F-7F7|
		L.L7LFJ|||||FJL7||LJ
		L7JLJL-JLJLJL--JLJ.L
		"#;
		let result = Day.part2(input)?;
		assert_eq!(result.as_str(), "10");

		Ok(())
	}
}
