use std::str::FromStr;

use anyhow::{bail, Context, Result};

use super::AocDay;

pub struct Day;

impl AocDay for Day {
	fn part1(&self, input: &str) -> Result<String> {
		let games = Games::from_str(input)?;

		let sum_of_possible = games.sum_of_possible_ids(12, 13, 14);

		Ok(sum_of_possible.to_string())
	}

	fn part2(&self, input: &str) -> Result<String> {
		let games = Games::from_str(input)?;

		let sum_of_powers = games.sum_of_powers();

		Ok(sum_of_powers.to_string())
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Color {
	Red,
	Green,
	Blue,
}

impl FromStr for Color {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self> {
		match s {
			"red" => Ok(Self::Red),
			"green" => Ok(Self::Green),
			"blue" => Ok(Self::Blue),
			_ => bail!("Invalid color `{}`", s),
		}
	}
}

struct Shown {
	color: Color,
	number: usize,
}

impl FromStr for Shown {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self> {
		let (num, color) = s.trim().split_once(' ').context("Invalid 'shown' string")?;
		Ok(Self { color: color.parse()?, number: num.parse()? })
	}
}

struct Game {
	id: usize,
	shown: Vec<Vec<Shown>>,
}

impl FromStr for Game {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self> {
		let (game, showns) = s.trim().split_once(": ").context("Invalid game string")?;
		let id = game.split_once(' ').context("Invalig game ID")?.1;

		let shown = showns
			.split("; ")
			.map(|shown| shown.split(", ").map(Shown::from_str).collect::<Result<_, _>>())
			.collect::<Result<_, _>>()?;

		Ok(Self { id: id.parse()?, shown })
	}
}

impl Game {
	fn is_possible_with(&self, red: usize, green: usize, blue: usize) -> bool {
		self.shown.iter().flatten().all(|shown| match shown.color {
			Color::Red => shown.number <= red,
			Color::Green => shown.number <= green,
			Color::Blue => shown.number <= blue,
		})
	}

	fn power(&self) -> usize {
		let red = self
			.shown
			.iter()
			.flatten()
			.filter(|shown| shown.color == Color::Red)
			.fold(0, |min, shown| min.max(shown.number));
		let green = self
			.shown
			.iter()
			.flatten()
			.filter(|shown| shown.color == Color::Green)
			.fold(0, |min, shown| min.max(shown.number));
		let blue = self
			.shown
			.iter()
			.flatten()
			.filter(|shown| shown.color == Color::Blue)
			.fold(0, |min, shown| min.max(shown.number));
		red * green * blue
	}
}

struct Games(Vec<Game>);

impl FromStr for Games {
	type Err = anyhow::Error;

	fn from_str(input: &str) -> Result<Self> {
		input.trim().lines().map(Game::from_str).collect::<Result<_, _>>().map(Self)
	}
}

impl Games {
	fn sum_of_possible_ids(&self, red: usize, green: usize, blue: usize) -> usize {
		self.0
			.iter()
			.filter(|game| game.is_possible_with(red, green, blue))
			.map(|game| game.id)
			.sum()
	}

	fn sum_of_powers(&self) -> usize {
		self.0.iter().map(Game::power).sum()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"
		Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
		Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
		Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
		Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
		Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
		"#;

	#[test]
	fn part1() -> Result<()> {
		let result = Day.part1(INPUT)?;
		assert_eq!(result.as_str(), "8");

		Ok(())
	}

	#[test]
	fn part2() -> Result<()> {
		let result = Day.part2(INPUT)?;
		assert_eq!(result.as_str(), "2286");

		Ok(())
	}
}
