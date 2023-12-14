use std::{
	collections::{HashMap, VecDeque},
	str::FromStr,
};

use anyhow::{Context, Result};

use super::AocDay;

pub struct Day;

impl AocDay for Day {
	fn part1(&self, input: &str) -> Result<String> {
		let mut grid = Grid::from_str(input)?;
		grid.shift_north_far();
		let total_load = grid.total_load();
		Ok(total_load.to_string())
	}

	fn part2(&self, input: &str) -> Result<String> {
		let mut grid = Grid::from_str(input)?;
		let total_load = grid.spin_cycles()?;
		Ok(total_load.to_string())
	}
}

impl Grid {
	pub fn total_load(&self) -> usize {
		let mut total_load = 0;
		let mut load_factor = 1;
		for y in (0..self.grid.len()).rev() {
			for item in &self.grid[y] {
				if *item == 'O' {
					total_load += load_factor;
				}
			}
			load_factor += 1;
		}
		total_load
	}

	fn shift_north_once(&mut self) -> bool {
		let mut moved = false;
		for y in 1..self.grid.len() {
			for x in 0..self.grid[y].len() {
				if self.grid[y][x] == 'O' && self.grid[y - 1][x] == '.' {
					self.grid[y - 1][x] = 'O';
					self.grid[y][x] = '.';
					moved = true;
				}
			}
		}
		moved
	}

	fn shift_south_once(&mut self) -> bool {
		let mut moved = false;
		for y in 0..(self.grid.len() - 1) {
			for x in 0..self.grid[y].len() {
				if self.grid[y][x] == 'O' && self.grid[y + 1][x] == '.' {
					self.grid[y + 1][x] = 'O';
					self.grid[y][x] = '.';
					moved = true;
				}
			}
		}
		moved
	}

	fn shift_east_once(&mut self) -> bool {
		let mut moved = false;
		for x in 0..(self.grid[0].len() - 1) {
			for y in 0..self.grid.len() {
				if self.grid[y][x] == 'O' && self.grid[y][x + 1] == '.' {
					self.grid[y][x + 1] = 'O';
					self.grid[y][x] = '.';
					moved = true;
				}
			}
		}
		moved
	}

	fn shift_west_once(&mut self) -> bool {
		let mut moved = false;
		for x in 1..self.grid[0].len() {
			for y in 0..self.grid.len() {
				if self.grid[y][x] == 'O' && self.grid[y][x - 1] == '.' {
					self.grid[y][x - 1] = 'O';
					self.grid[y][x] = '.';
					moved = true;
				}
			}
		}
		moved
	}

	pub fn shift_north_far(&mut self) {
		while self.shift_north_once() {}
	}

	pub fn shift_south_far(&mut self) {
		while self.shift_south_once() {}
	}

	pub fn shift_east_far(&mut self) {
		while self.shift_east_once() {}
	}

	pub fn shift_west_far(&mut self) {
		while self.shift_west_once() {}
	}

	pub fn spin_cycles(&mut self) -> Result<usize> {
		let mut seen = HashMap::new();
		let mut last_ones = VecDeque::new();
		let mut cycles_left = 1_000_000_000;
		while cycles_left > 0 {
			self.shift_north_far();
			self.shift_west_far();
			self.shift_south_far();
			self.shift_east_far();
			cycles_left -= 1;

			if seen.contains_key(&self.grid) {
				break;
			} else {
				let load = self.total_load();
				seen.insert(self.grid.clone(), load);
				last_ones.push_back(self.grid.clone());
				if last_ones.len() > 60 {
					last_ones.pop_front();
				}
			}
		}

		let previous =
			last_ones.iter().position(|grid| *grid == self.grid).context("cut-off too short")?;
		let cycle_length = last_ones.len() - previous;
		let grid = &last_ones[(cycles_left % cycle_length) + previous];
		Ok(seen[grid])
	}
}

struct Grid {
	grid: Vec<Vec<char>>,
}

impl FromStr for Grid {
	type Err = anyhow::Error;

	fn from_str(input: &str) -> Result<Self> {
		let grid = input.trim().lines().map(|line| line.trim().chars().collect()).collect();
		Ok(Self { grid })
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"
		O....#....
		O.OO#....#
		.....##...
		OO.#O....O
		.O.....O#.
		O.#..O.#.#
		..O..#O..O
		.......O..
		#....###..
		#OO..#....
		"#;

	#[test]
	fn part1() -> Result<()> {
		let result = Day.part1(INPUT)?;
		assert_eq!(result.as_str(), "136");

		Ok(())
	}

	#[test]
	fn part2() -> Result<()> {
		let result = Day.part2(INPUT)?;
		assert_eq!(result.as_str(), "64");

		Ok(())
	}
}
