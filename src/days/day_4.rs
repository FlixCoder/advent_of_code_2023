use std::{
	collections::{HashSet, VecDeque},
	str::FromStr,
};

use anyhow::{Context, Result};

use super::AocDay;

pub struct Day;

impl AocDay for Day {
	fn part1(&self, input: &str) -> Result<String> {
		let cards = Cards::from_str(input)?;
		Ok(cards.worth_points().to_string())
	}

	fn part2(&self, input: &str) -> Result<String> {
		let cards = Cards::from_str(input)?;
		Ok(cards.total_scratch_cards().to_string())
	}
}

#[derive(Debug, Clone)]
struct Card {
	#[allow(dead_code)]
	id: usize,
	winning: HashSet<usize>,
	you_have: HashSet<usize>,
}

impl FromStr for Card {
	type Err = anyhow::Error;

	fn from_str(line: &str) -> Result<Self> {
		let (card, numbers) = line.trim().split_once(": ").context("line split ': '")?;
		let (winning, you_have) = numbers.split_once(" | ").context("numbers split ' | '")?;

		let id = card.rsplit_once(' ').context("card split ' '")?.1;
		let winning = winning.split_whitespace().map(usize::from_str).collect::<Result<_, _>>()?;
		let you_have =
			you_have.split_whitespace().map(usize::from_str).collect::<Result<_, _>>()?;

		Ok(Self { id: id.parse()?, winning, you_have })
	}
}

impl Card {
	fn num_winning(&self) -> usize {
		self.winning.intersection(&self.you_have).count()
	}
}

struct Cards(Vec<Card>);

impl FromStr for Cards {
	type Err = anyhow::Error;

	fn from_str(input: &str) -> Result<Self> {
		input.trim().lines().map(Card::from_str).collect::<Result<_, _>>().map(Self)
	}
}

impl Cards {
	fn worth_points(&self) -> u64 {
		self.0
			.iter()
			.map(Card::num_winning)
			.map(|num| if num == 0 { 0 } else { 2_u64.pow(num.saturating_sub(1) as u32) })
			.sum()
	}

	fn total_scratch_cards(&self) -> usize {
		let mut current_cards = VecDeque::from_iter(0..self.0.len());
		let mut total_num = 0;
		while let Some(card_index) = current_cards.pop_front() {
			total_num += 1;
			for i in 1..=self.0[card_index].num_winning() {
				current_cards.push_back(card_index + i);
			}
		}
		total_num
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"
		Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
		Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
		Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
		Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
		Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
		Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
		"#;

	#[test]
	fn part1() -> Result<()> {
		let result = Day.part1(INPUT)?;
		assert_eq!(result.as_str(), "13");

		Ok(())
	}

	#[test]
	fn part2() -> Result<()> {
		let result = Day.part2(INPUT)?;
		assert_eq!(result.as_str(), "30");

		Ok(())
	}
}
