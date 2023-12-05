use std::str::FromStr;

use anyhow::{bail, Context, Result};
use rayon::prelude::*;

use super::AocDay;

pub struct Day;

impl AocDay for Day {
	fn part1(&self, input: &str) -> Result<String> {
		let data = Data::from_str(input)?;

		let min_mapped = data
			.seeds
			.into_iter()
			.map(|mut seed| {
				for map in &data.maps {
					seed = map.map_min(seed);
				}
				seed
			})
			.min()
			.context("Finding minimum mapped location")?;

		Ok(min_mapped.to_string())
	}

	fn part2(&self, input: &str) -> Result<String> {
		let data = Data::from_str(input)?;

		let mut seeds = Vec::new();
		for chunk in data.seeds.chunks_exact(2) {
			let &[start, length] = chunk else {
				unreachable!("chunks are always exact 2");
			};
			for i in 0..length {
				seeds.push(start + i);
			}
		}

		let min_mapped = seeds
			.into_par_iter()
			.map(|mut seed| {
				for map in &data.maps {
					seed = map.map_min(seed);
				}
				seed
			})
			.min()
			.context("Finding minimum mapped location")?;

		Ok(min_mapped.to_string())
	}
}

struct Data {
	seeds: Vec<u64>,
	maps: Vec<Map>,
}

impl FromStr for Data {
	type Err = anyhow::Error;

	fn from_str(input: &str) -> Result<Self> {
		let mut segments = input.split("\n\n");
		let seeds = segments.next().context("Seed segment must exist")?.trim();

		let seeds =
			seeds.split_whitespace().skip(1).map(|n| n.parse::<u64>()).collect::<Result<_, _>>()?;

		let maps = segments.map(Map::from_str).collect::<Result<_, _>>()?;

		Ok(Self { seeds, maps })
	}
}

struct Map {
	map: Vec<MapItem>,
}

impl Map {
	fn map_min(&self, source: u64) -> u64 {
		self.map.iter().filter_map(|map| map.map_min(source)).min().unwrap_or(source)
	}
}

impl FromStr for Map {
	type Err = anyhow::Error;

	fn from_str(map_str: &str) -> Result<Self> {
		let mappings =
			map_str.trim().lines().skip(1).map(MapItem::from_str).collect::<Result<_, _>>()?;
		Ok(Self { map: mappings })
	}
}

struct MapItem {
	destination: u64,
	source: u64,
	range_len: u64,
}

impl MapItem {
	fn map_min(&self, source: u64) -> Option<u64> {
		// panics with subtraction wrapping otherwise.
		#[allow(clippy::unnecessary_lazy_evaluations)]
		(self.source..(self.source + self.range_len))
			.contains(&source)
			.then(|| self.destination + source - self.source)
	}
}

impl FromStr for MapItem {
	type Err = anyhow::Error;

	fn from_str(line: &str) -> Result<Self> {
		let numbers =
			line.split_whitespace().map(|n| n.parse::<u64>()).collect::<Result<Vec<_>, _>>()?;

		if numbers.len() != 3 {
			bail!("Not exactly three numbers in map line");
		}

		Ok(Self { destination: numbers[0], source: numbers[1], range_len: numbers[2] })
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"
		seeds: 79 14 55 13

		seed-to-soil map:
		50 98 2
		52 50 48

		soil-to-fertilizer map:
		0 15 37
		37 52 2
		39 0 15

		fertilizer-to-water map:
		49 53 8
		0 11 42
		42 0 7
		57 7 4

		water-to-light map:
		88 18 7
		18 25 70

		light-to-temperature map:
		45 77 23
		81 45 19
		68 64 13

		temperature-to-humidity map:
		0 69 1
		1 0 69

		humidity-to-location map:
		60 56 37
		56 93 4
		"#;

	#[test]
	fn part1() -> Result<()> {
		let result = Day.part1(INPUT)?;
		assert_eq!(result.as_str(), "35");

		Ok(())
	}

	#[test]
	fn part2() -> Result<()> {
		let result = Day.part2(INPUT)?;
		assert_eq!(result.as_str(), "46");

		Ok(())
	}
}
