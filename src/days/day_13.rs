use std::str::FromStr;

use anyhow::{bail, Result};

use super::AocDay;

pub struct Day;

impl AocDay for Day {
	fn part1(&self, input: &str) -> Result<String> {
		let grids = Grids::from_str(input)?;
		let reflection_points = grids.reflection_points(0);
		Ok(reflection_points.to_string())
	}

	fn part2(&self, input: &str) -> Result<String> {
		let grids = Grids::from_str(input)?;
		let reflection_points = grids.reflection_points(1);
		Ok(reflection_points.to_string())
	}
}

impl Grids {
	pub fn reflection_points(&self, defects: usize) -> usize {
		self.0
			.iter()
			.map(|grid| {
				if let Some(horizonal) = grid.horizontal_reflection(defects) {
					horizonal * 100
				} else if let Some(vertical) = grid.transpose().horizontal_reflection(defects) {
					vertical
				} else {
					panic!("One has no reflection");
				}
			})
			.sum()
	}
}

impl Grid {
	pub fn height(&self) -> usize {
		self.grid.len() / self.width
	}

	fn row(&self, y: usize) -> &[Item] {
		&self.grid[(y * self.width)..(y * self.width + self.width)]
	}

	pub fn horizontal_reflection(&self, expected_defects: usize) -> Option<usize> {
		for i in 1..self.height() {
			let mut defects = 0;
			for (a, b) in (0..i).rev().zip(i..self.height()) {
				for x in 0..self.width {
					if self.row(a)[x] != self.row(b)[x] {
						defects += 1;
					}
				}
			}
			if defects == expected_defects {
				return Some(i);
			}
		}
		None
	}

	pub fn transpose(&self) -> Self {
		let height = self.height();
		let mut new_grid = Vec::with_capacity(self.grid.len());
		for x in 0..self.width {
			for y in 0..height {
				new_grid.push(self.grid[y * self.width + x]);
			}
		}
		Self { grid: new_grid, width: height }
	}
}

#[derive(Debug)]
struct Grids(Vec<Grid>);

#[derive(Debug, Clone)]
struct Grid {
	grid: Vec<Item>,
	width: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Item {
	Ash,  // .
	Rock, // #
}

impl FromStr for Grid {
	type Err = anyhow::Error;

	fn from_str(grid: &str) -> Result<Self> {
		let mut width = 0;
		let grid = grid
			.trim()
			.lines()
			.flat_map(|line| {
				width = line.trim().len();
				line.trim().chars()
			})
			.map(|c| match c {
				'.' => Ok(Item::Ash),
				'#' => Ok(Item::Rock),
				_ => bail!("{c} is neither ash nor rock"),
			})
			.collect::<Result<_, _>>()?;
		Ok(Self { grid, width })
	}
}

impl FromStr for Grids {
	type Err = anyhow::Error;

	fn from_str(input: &str) -> Result<Self> {
		let grids = input.trim().split("\n\n").map(Grid::from_str).collect::<Result<_, _>>()?;
		Ok(Self(grids))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"
		#.##..##.
		..#.##.#.
		##......#
		##......#
		..#.##.#.
		..##..##.
		#.#.##.#.

		#...##..#
		#....#..#
		..##..###
		#####.##.
		#####.##.
		..##..###
		#....#..#
		"#;

	#[test]
	fn part1() -> Result<()> {
		let result = Day.part1(INPUT)?;
		assert_eq!(result.as_str(), "405");

		Ok(())
	}

	#[test]
	fn part2() -> Result<()> {
		let result = Day.part2(INPUT)?;
		assert_eq!(result.as_str(), "400");

		Ok(())
	}
}
