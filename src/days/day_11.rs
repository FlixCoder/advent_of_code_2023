use std::str::FromStr;

use anyhow::{bail, ensure, Result};

use super::AocDay;

pub struct Day;

impl AocDay for Day {
	fn part1(&self, input: &str) -> Result<String> {
		let mut grid = Grid::from_str(input)?;
		grid.expand(2)?;
		let distances = grid.sum_of_distances();
		Ok(distances.to_string())
	}

	fn part2(&self, input: &str) -> Result<String> {
		let mut grid = Grid::from_str(input)?;
		grid.expand(1_000_000)?;
		let distances = grid.sum_of_distances();
		Ok(distances.to_string())
	}
}

impl Grid {
	pub fn expand(&mut self, factor: usize) -> Result<()> {
		for x in (0..self.grid[0].len()).rev() {
			let mut is_empty = true;
			for y in 0..self.grid.len() {
				if self.grid[y][x] != Field::Empty {
					is_empty = false;
					break;
				}
			}
			if is_empty {
				self.cost_x[x] = factor;
			}
		}

		for y in (0..self.grid.len()).rev() {
			let is_empty = self.grid[y].iter().all(|field| *field == Field::Empty);
			if is_empty {
				self.cost_y[y] = factor;
			}
		}
		Ok(())
	}

	pub fn sum_of_distances(&self) -> usize {
		let mut galaxies = Vec::new();
		for y in 0..self.grid.len() {
			for x in 0..self.grid[y].len() {
				if self.grid[y][x] == Field::Galaxy {
					galaxies.push((x, y));
				}
			}
		}

		let mut distances = 0;
		for i in 0..galaxies.len() {
			for j in i + 1..galaxies.len() {
				let mut distance = 0;

				let small = galaxies[i].0.min(galaxies[j].0);
				let big = galaxies[i].0.max(galaxies[j].0);
				for x in small..big {
					distance += self.cost_x[x];
				}
				let small = galaxies[i].1.min(galaxies[j].1);
				let big = galaxies[i].1.max(galaxies[j].1);
				for y in small..big {
					distance += self.cost_y[y];
				}

				distances += distance;
			}
		}

		distances
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Field {
	Empty,
	Galaxy,
}

struct Grid {
	grid: Vec<Vec<Field>>,
	cost_x: Vec<usize>,
	cost_y: Vec<usize>,
}

impl TryFrom<char> for Field {
	type Error = anyhow::Error;

	fn try_from(c: char) -> Result<Self> {
		match c {
			'.' => Ok(Self::Empty),
			'#' => Ok(Self::Galaxy),
			_ => bail!("Invalid char for field: '{c}'"),
		}
	}
}

impl FromStr for Grid {
	type Err = anyhow::Error;

	fn from_str(input: &str) -> Result<Self> {
		let grid: Vec<Vec<Field>> = input
			.trim()
			.lines()
			.map(|line| line.trim().chars().map(Field::try_from).collect())
			.collect::<Result<_, _>>()?;

		ensure!(!grid.is_empty(), "Grid should not be empty");
		ensure!(!grid[0].is_empty(), "Grid should not be empty");

		Ok(Grid { cost_x: vec![1; grid[0].len()], cost_y: vec![1; grid.len()], grid })
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"
		...#......
		.......#..
		#.........
		..........
		......#...
		.#........
		.........#
		..........
		.......#..
		#...#.....
		"#;

	#[test]
	fn part1() -> Result<()> {
		let result = Day.part1(INPUT)?;
		assert_eq!(result.as_str(), "374");

		Ok(())
	}

	#[test]
	fn part2() -> Result<()> {
		let mut grid = Grid::from_str(INPUT)?;
		grid.expand(100)?;
		let distances = grid.sum_of_distances();
		assert_eq!(distances, 8410);

		let result = Day.part2(INPUT)?;
		assert_eq!(result.as_str(), "82000210");

		Ok(())
	}
}
