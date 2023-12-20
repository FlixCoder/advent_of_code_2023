use std::{collections::VecDeque, str::FromStr};

use ahash::AHashMap;
use anyhow::{bail, Context, Result};
use once_cell::sync::Lazy;
use regex::Regex;

use super::AocDay;

pub struct Day;

impl AocDay for Day {
	fn part1(&self, input: &str) -> Result<String> {
		let mut network = Network::from_str(input)?;
		let mut low_pulses = 0;
		let mut high_pulses = 0;
		for _ in 0..1000 {
			let (low, high, _) = network.cycle()?;
			low_pulses += low;
			high_pulses += high;
		}
		let score = low_pulses * high_pulses;
		Ok(score.to_string())
	}

	fn part2(&self, input: &str) -> Result<String> {
		let mut network = Network::from_str(input)?;
		let mut times_pressed = 1;
		while !network.cycle()?.2 {
			times_pressed += 1;
		}
		Ok(times_pressed.to_string())
	}
}

impl Network {
	pub fn cycle(&mut self) -> Result<(usize, usize, bool)> {
		let mut low_pulses = 0;
		let mut high_pulses = 0;

		let mut queue = VecDeque::new();
		queue.push_back(("button".to_owned(), "broadcaster".to_owned(), false));
		while let Some((from, to, high)) = queue.pop_front() {
			if to == "rx" && !high {
				return Ok((0, 0, true));
			}
			if high {
				high_pulses += 1;
			} else {
				low_pulses += 1;
			}

			let Some(module) = self.modules.get_mut(&to) else {
				continue;
			};
			if let Some(pulse) = module.pulse(&from, high) {
				for next in module.next() {
					queue.push_back((to.clone(), next.to_owned(), pulse));
				}
			}
		}

		Ok((low_pulses, high_pulses, false))
	}

	/// Run to collect all inputs in the conjunctions.
	fn init_run(&mut self) -> Result<()> {
		let combinations = self
			.modules
			.iter()
			.map(|(name, module)| (name.clone(), module.next().clone()))
			.collect::<Vec<_>>();
		for (from, next) in combinations {
			for to in next {
				let Some(module) = self.modules.get_mut(&to) else {
					continue;
				};
				module.init_run(&from);
			}
		}
		Ok(())
	}
}

impl Module {
	/// Receive a pulse and determine whether what kind of pulse the next ones
	/// get (if at all).
	pub fn pulse(&mut self, from: &str, high: bool) -> Option<bool> {
		match &mut self.ty {
			ModuleType::Broadcast => Some(high),
			ModuleType::FlipFlop { on } => {
				if !high {
					*on = !*on;
					Some(*on)
				} else {
					None
				}
			}
			ModuleType::Conjunction { last_pulse_high } => {
				last_pulse_high.insert(from.to_owned(), high);
				if last_pulse_high.values().all(|last_high| *last_high) {
					Some(false)
				} else {
					Some(true)
				}
			}
		}
	}

	/// Run to collect all inputs for the conjunctions.
	pub fn init_run(&mut self, from: &str) {
		match &mut self.ty {
			ModuleType::Broadcast => {}
			ModuleType::FlipFlop { on } => {
				*on = false;
			}
			ModuleType::Conjunction { last_pulse_high } => {
				last_pulse_high.insert(from.to_owned(), false);
			}
		}
	}

	pub fn next(&self) -> &Vec<String> {
		&self.next
	}
}

#[derive(Debug)]
enum ModuleType {
	Broadcast,
	FlipFlop { on: bool },
	Conjunction { last_pulse_high: AHashMap<String, bool> },
}

#[derive(Debug)]
struct Module {
	ty: ModuleType,
	next: Vec<String>,
}

#[derive(Debug)]
struct Network {
	modules: AHashMap<String, Module>,
}

impl FromStr for Network {
	type Err = anyhow::Error;

	fn from_str(input: &str) -> Result<Self> {
		static REGEX: Lazy<Regex> =
			Lazy::new(|| Regex::new(r"(%|&)?([a-z]+) -> ((?:[a-z]+,? ?)+)").expect("create regex"));

		let mut modules = AHashMap::new();
		for line in input.trim().lines() {
			let captures = REGEX.captures(line).context("applying regex")?;
			let mut next = Vec::new();
			for n in captures[3].split(", ") {
				next.push(n.to_owned());
			}
			match captures.get(1).map(|m| m.as_str()) {
				None if &captures[2] == "broadcaster" => {
					modules.insert(
						"broadcaster".to_owned(),
						Module { ty: ModuleType::Broadcast, next },
					);
				}
				Some("%") => {
					modules.insert(
						captures[2].to_owned(),
						Module { ty: ModuleType::FlipFlop { on: false }, next },
					);
				}
				Some("&") => {
					modules.insert(
						captures[2].to_owned(),
						Module {
							ty: ModuleType::Conjunction { last_pulse_high: AHashMap::new() },
							next,
						},
					);
				}
				_ => bail!("Invalid module line: `{line}`"),
			}
		}

		let mut network = Network { modules };
		network.init_run()?;

		Ok(network)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT1: &str = r#"
		broadcaster -> a, b, c
		%a -> b
		%b -> c
		%c -> inv
		&inv -> a
		"#;
	const INPUT2: &str = r#"
		broadcaster -> a
		%a -> inv, con
		&inv -> b
		%b -> con
		&con -> output
		"#;

	#[test]
	fn part1() -> Result<()> {
		let result = Day.part1(INPUT1)?;
		assert_eq!(result.as_str(), "32000000");
		let result = Day.part1(INPUT2)?;
		assert_eq!(result.as_str(), "11687500");
		Ok(())
	}
}
