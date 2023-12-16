use std::{collections::VecDeque, str::FromStr};

use ahash::AHashSet;
use anyhow::Result;
use rayon::prelude::*;

use super::AocDay;

pub struct Day;

impl AocDay for Day {
	fn part1(&self, input: &str) -> Result<String> {
		let grid = Grid::from_str(input)?;
		let energized = grid.energized(Position { x: 0, y: 0 }, Direction::Right);
		Ok(energized.to_string())
	}

	fn part2(&self, input: &str) -> Result<String> {
		let grid = Grid::from_str(input)?;
		let energized = grid.max_energized();
		Ok(energized.to_string())
	}
}

impl Grid {
	pub fn max_energized(&self) -> usize {
		let mut starts = Vec::new();
		for x in 0..self.width {
			starts.push((Position { x, y: 0 }, Direction::Down));
			starts.push((Position { x, y: self.height() - 1 }, Direction::Up));
		}
		for y in 0..self.height() {
			starts.push((Position { x: 0, y }, Direction::Right));
			starts.push((Position { x: self.width - 1, y }, Direction::Left));
		}

		starts.into_par_iter().map(|(pos, dir)| self.energized(pos, dir)).max().unwrap()
	}

	pub fn energized(&self, start_pos: Position, start_direction: Direction) -> usize {
		let mut seen = AHashSet::new();
		let mut energized = AHashSet::new();
		let mut rays = VecDeque::new();

		energized.insert(start_pos);
		let (start_direction, second_direction) = start_direction.next(self.get(start_pos));
		rays.push_back((start_pos, start_direction));
		seen.insert((start_pos, start_direction));
		if let Some(start_direction) = second_direction {
			rays.push_back((start_pos, start_direction));
			seen.insert((start_pos, start_direction));
		}

		while let Some((pos, direction)) = rays.pop_front() {
			let new_pos = pos + direction.delta();
			if self.in_bounds(new_pos) {
				energized.insert(new_pos);

				let (new_direction, second_direction) = direction.next(self.get(new_pos));
				if seen.insert((new_pos, new_direction)) {
					rays.push_back((new_pos, new_direction));
				}
				if let Some(new_direction) = second_direction {
					if seen.insert((new_pos, new_direction)) {
						rays.push_back((new_pos, new_direction));
					}
				}
			}
		}

		energized.len()
	}

	fn get(&self, pos: Position) -> char {
		self.grid[pos.y * self.width + pos.x]
	}

	fn in_bounds(&self, pos: Position) -> bool {
		pos.x < self.width && pos.y < self.height()
	}

	fn height(&self) -> usize {
		self.grid.len() / self.width
	}
}

impl Direction {
	fn delta(&self) -> (isize, isize) {
		match self {
			Self::Up => (0, -1),
			Self::Down => (0, 1),
			Self::Left => (-1, 0),
			Self::Right => (1, 0),
		}
	}

	fn next(&self, tile: char) -> (Self, Option<Self>) {
		match self {
			Self::Up => match tile {
				'-' => (Self::Left, Some(Self::Right)),
				'/' => (Self::Right, None),
				'\\' => (Self::Left, None),
				_ => (*self, None),
			},
			Self::Down => match tile {
				'-' => (Self::Left, Some(Self::Right)),
				'/' => (Self::Left, None),
				'\\' => (Self::Right, None),
				_ => (*self, None),
			},
			Self::Left => match tile {
				'|' => (Self::Up, Some(Self::Down)),
				'/' => (Self::Down, None),
				'\\' => (Self::Up, None),
				_ => (*self, None),
			},
			Self::Right => match tile {
				'|' => (Self::Up, Some(Self::Down)),
				'/' => (Self::Up, None),
				'\\' => (Self::Down, None),
				_ => (*self, None),
			},
		}
	}
}

impl std::ops::Add<(isize, isize)> for Position {
	type Output = Position;

	fn add(mut self, rhs: (isize, isize)) -> Self::Output {
		self.x = self.x.wrapping_add(rhs.0 as usize);
		self.y = self.y.wrapping_add(rhs.1 as usize);
		self
	}
}

impl std::fmt::Display for Grid {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for y in 0..self.height() {
			let line: String =
				self.grid[y * self.width..y * self.width + self.width].iter().collect();
			f.write_str(&line)?;
			f.write_str("\n")?;
		}
		Ok(())
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
	Up,
	Down,
	Left,
	Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Position {
	x: usize,
	y: usize,
}

#[derive(Debug, Clone)]
struct Grid {
	grid: Vec<char>,
	width: usize,
}

impl FromStr for Grid {
	type Err = anyhow::Error;

	fn from_str(input: &str) -> Result<Self> {
		let mut width = 0;
		let grid = input
			.trim()
			.lines()
			.flat_map(|line| {
				width = line.trim().len();
				line.trim().chars()
			})
			.collect();
		Ok(Self { grid, width })
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"
		.|...\....
		|.-.\.....
		.....|-...
		........|.
		..........
		.........\
		..../.\\..
		.-.-/..|..
		.|....-|.\
		..//.|....
		"#;

	#[test]
	fn part1() -> Result<()> {
		let result = Day.part1(INPUT)?;
		assert_eq!(result.as_str(), "46");

		Ok(())
	}

	#[test]
	fn part2() -> Result<()> {
		let result = Day.part2(INPUT)?;
		assert_eq!(result.as_str(), "51");

		Ok(())
	}
}
