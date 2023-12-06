use std::str::FromStr;

use anyhow::{Context, Result};

use super::AocDay;

pub struct Day;

impl AocDay for Day {
	fn part1(&self, input: &str) -> Result<String> {
		let races = Races::from_str(input)?;

		let mut factor = 1;
		for race in races.races {
			factor *= race.ways_to_beat_distance();
		}

		Ok(factor.to_string())
	}

	fn part2(&self, input: &str) -> Result<String> {
		let race = Race::from_str(input)?;

		let ways_to_beat = race.ways_to_beat_distance();

		Ok(ways_to_beat.to_string())
	}
}

struct Races {
	races: Vec<Race>,
}

struct Race {
	time: u64,
	record: u64,
}

impl FromStr for Races {
	type Err = anyhow::Error;

	fn from_str(input: &str) -> Result<Self> {
		let mut lines = input.trim().lines();
		let times = lines.next().context("Times line")?;
		let distances = lines.next().context("Distances line")?;

		let times = times.split_whitespace().skip(1);
		let distances = distances.split_whitespace().skip(1);

		let mut races = Vec::new();
		for (time, distance) in times.zip(distances) {
			races.push(Race { time: time.parse()?, record: distance.parse()? });
		}

		Ok(Self { races })
	}
}

impl FromStr for Race {
	type Err = anyhow::Error;

	fn from_str(input: &str) -> Result<Self> {
		let mut lines = input.trim().lines();
		let time = lines
			.next()
			.context("Times line")?
			.split_once(':')
			.context("remove front part")?
			.1
			.replace(|c: char| c.is_whitespace(), "");
		let distance = lines
			.next()
			.context("Distances line")?
			.split_once(':')
			.context("remove front part")?
			.1
			.replace(|c: char| c.is_whitespace(), "");

		Ok(Self { time: time.parse()?, record: distance.parse()? })
	}
}

impl Race {
	fn ways_to_beat_distance(&self) -> usize {
		// The formula to whether it is beaten is - x^2 + time * k - record > 0,
		// so the pq-formula tells us that the roots are at:
		// x_1,2 = time/2 +- sqrt((time/2)^2 - record).
		// We add a small epsilon to make sure we are not at exactly 0, but higher.
		let p1 = self.time as f64 / 2.0;
		let p2 = ((self.time * self.time) as f64 / 4.0 - self.record as f64 - 0.1).sqrt();
		let start = (p1 - p2).ceil() as usize;
		let end = (p1 + p2).floor() as usize;
		end - start + 1
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"
		Time:      7  15   30
		Distance:  9  40  200
		"#;

	#[test]
	fn part1() -> Result<()> {
		let result = Day.part1(INPUT)?;
		assert_eq!(result.as_str(), "288");

		Ok(())
	}

	#[test]
	fn part2() -> Result<()> {
		let result = Day.part2(INPUT)?;
		assert_eq!(result.as_str(), "71503");

		Ok(())
	}
}
