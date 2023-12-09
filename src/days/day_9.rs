use std::str::FromStr;

use anyhow::Result;

use super::AocDay;

pub struct Day;

impl AocDay for Day {
	fn part1(&self, input: &str) -> Result<String> {
		let mut histories = Histories::from_str(input)?;

		let predictions: i64 = histories.predictions().into_iter().sum();

		Ok(predictions.to_string())
	}

	fn part2(&self, input: &str) -> Result<String> {
		let histories = Histories::from_str(input)?;

		let result: i64 = histories.backward_extrapolations().into_iter().sum();

		Ok(result.to_string())
	}
}

#[derive(Debug)]
struct Histories(Vec<History>);

#[derive(Debug)]
struct History {
	history: Vec<i64>,
}

impl Histories {
	fn predictions(&mut self) -> Vec<i64> {
		self.0.iter_mut().map(|history| history.add_prediction()).collect()
	}

	fn backward_extrapolations(&self) -> Vec<i64> {
		self.0.iter().map(|history| history.backward_extrapolation()).collect()
	}
}

impl History {
	fn add_prediction(&mut self) -> i64 {
		let mut last_nums = vec![*self.history.last().unwrap()];
		let mut current_sequence = self.history.clone();
		loop {
			current_sequence =
				current_sequence.windows(2).map(|window| window[1] - window[0]).collect();
			last_nums.push(*current_sequence.last().unwrap());
			if current_sequence.iter().all(|i| *i == 0) {
				break;
			}
		}

		let mut add = 0;
		while let Some(last_num) = last_nums.pop() {
			let prediction = last_num + add;
			add = prediction;
		}

		self.history.push(add);
		add
	}

	fn backward_extrapolation(&self) -> i64 {
		let mut first_nums = vec![*self.history.first().unwrap()];
		let mut current_sequence = self.history.clone();
		loop {
			current_sequence =
				current_sequence.windows(2).map(|window| window[1] - window[0]).collect();
			first_nums.push(*current_sequence.first().unwrap());
			if current_sequence.iter().all(|i| *i == 0) {
				break;
			}
		}

		let mut sub = 0;
		while let Some(first_num) = first_nums.pop() {
			let prediction = first_num - sub;
			sub = prediction;
		}
		sub
	}
}

impl FromStr for Histories {
	type Err = anyhow::Error;

	fn from_str(input: &str) -> Result<Self> {
		let histories = input.trim().lines().map(History::from_str).collect::<Result<_, _>>()?;
		Ok(Self(histories))
	}
}

impl FromStr for History {
	type Err = anyhow::Error;

	fn from_str(line: &str) -> Result<Self> {
		let history = line.split_whitespace().map(i64::from_str).collect::<Result<_, _>>()?;
		Ok(Self { history })
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"
		0 3 6 9 12 15
		1 3 6 10 15 21
		10 13 16 21 30 45
		"#;

	#[test]
	fn part1() -> Result<()> {
		let result = Day.part1(INPUT)?;
		assert_eq!(result.as_str(), "114");

		Ok(())
	}

	#[test]
	fn part2() -> Result<()> {
		let result = Day.part2(INPUT)?;
		assert_eq!(result.as_str(), "2");

		Ok(())
	}
}
