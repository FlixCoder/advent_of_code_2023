use std::{cmp::Reverse, collections::BinaryHeap, str::FromStr};

use ahash::AHashSet;
use anyhow::{Context, Result};

use super::AocDay;

pub struct Day;

impl AocDay for Day {
	fn part1(&self, input: &str) -> Result<String> {
		let grid = Grid::from_str(input)?;
		let least_heat_loss = grid.least_heat_loss(0, 3).context("no path found")?;
		Ok(least_heat_loss.to_string())
	}

	fn part2(&self, input: &str) -> Result<String> {
		let grid = Grid::from_str(input)?;
		let least_heat_loss = grid.least_heat_loss(4, 10).context("no path found")?;
		Ok(least_heat_loss.to_string())
	}
}

impl Grid {
	pub fn least_heat_loss(&self, min_forward: usize, max_forward: usize) -> Option<u32> {
		let start = Position { x: 0, y: 0 };
		let target = Position { x: self.width - 1, y: self.height() - 1 };

		let mut visited = AHashSet::new();
		let mut queue = BinaryHeap::new();
		queue.push(Reverse(State {
			cost: 0,
			position: start,
			last_direction: Direction::Right,
			num_steps: 0,
		}));
		queue.push(Reverse(State {
			cost: 0,
			position: start,
			last_direction: Direction::Down,
			num_steps: 0,
		}));

		while let Some(Reverse(State { cost, position, last_direction, num_steps })) = queue.pop() {
			if position == target && num_steps >= min_forward {
				return Some(cost);
			}

			for direction in [Direction::Right, Direction::Down, Direction::Left, Direction::Up] {
				if direction == last_direction.opposite()
					|| (direction == last_direction && num_steps >= max_forward)
					|| (direction != last_direction && num_steps < min_forward)
				{
					continue;
				}

				let new_position = position.go(direction);
				if !self.in_bounds(new_position) {
					continue;
				}

				let new_cost = cost + self.get(new_position) as u32;
				let next_state = State {
					cost: new_cost,
					position: new_position,
					last_direction: direction,
					num_steps: if direction == last_direction { num_steps + 1 } else { 1 },
				};
				if visited.insert(VisitedKey::from(next_state)) {
					queue.push(Reverse(next_state));
				}
			}
		}
		None
	}

	fn get(&self, pos: Position) -> u8 {
		self.grid[pos.y * self.width + pos.x]
	}

	fn in_bounds(&self, pos: Position) -> bool {
		pos.x < self.width && pos.y < self.height()
	}

	fn height(&self) -> usize {
		self.grid.len() / self.width
	}
}

impl From<State> for VisitedKey {
	fn from(state: State) -> Self {
		Self {
			position: state.position,
			last_direction: state.last_direction,
			num_steps: state.num_steps,
		}
	}
}

impl std::cmp::Ord for State {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.cost.cmp(&other.cost).then_with(|| other.position.cmp(&self.position))
	}
}

impl std::cmp::PartialOrd for State {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Direction {
	pub fn opposite(self) -> Self {
		match self {
			Direction::Up => Direction::Down,
			Direction::Right => Direction::Left,
			Direction::Down => Direction::Up,
			Direction::Left => Direction::Right,
		}
	}
}

impl Position {
	pub fn go(self, direction: Direction) -> Self {
		match direction {
			Direction::Up => Self { x: self.x, y: self.y.wrapping_sub(1) },
			Direction::Right => Self { x: self.x + 1, y: self.y },
			Direction::Down => Self { x: self.x, y: self.y + 1 },
			Direction::Left => Self { x: self.x.wrapping_sub(1), y: self.y },
		}
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct VisitedKey {
	position: Position,
	last_direction: Direction,
	num_steps: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct State {
	cost: u32,
	position: Position,
	last_direction: Direction,
	num_steps: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
	Up,
	Right,
	Down,
	Left,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Position {
	x: usize,
	y: usize,
}

#[derive(Debug)]
struct Grid {
	grid: Vec<u8>,
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
				line.trim().chars().map(|c| c as u8 - b'0')
			})
			.collect();
		Ok(Self { grid, width })
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"
		2413432311323
		3215453535623
		3255245654254
		3446585845452
		4546657867536
		1438598798454
		4457876987766
		3637877979653
		4654967986887
		4564679986453
		1224686865563
		2546548887735
		4322674655533
		"#;

	#[test]
	fn part1() -> Result<()> {
		let result = Day.part1(INPUT)?;
		assert_eq!(result.as_str(), "102");

		Ok(())
	}

	#[test]
	fn part2() -> Result<()> {
		let result = Day.part2(INPUT)?;
		assert_eq!(result.as_str(), "94");

		let input = r#"
		111111111111
		999999999991
		999999999991
		999999999991
		999999999991
		"#;
		let result = Day.part2(input)?;
		assert_eq!(result.as_str(), "71");

		Ok(())
	}
}
