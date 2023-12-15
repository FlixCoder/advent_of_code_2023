use std::str::FromStr;

use anyhow::{bail, Result};

use super::AocDay;

pub struct Day;

impl AocDay for Day {
	fn part1(&self, input: &str) -> Result<String> {
		let sum_of_hashes = input.trim().split(',').map(hash).map(|hash| hash as u64).sum::<u64>();
		Ok(sum_of_hashes.to_string())
	}

	fn part2(&self, input: &str) -> Result<String> {
		let operations =
			input.trim().split(',').map(Operation::from_str).collect::<Result<Vec<_>, _>>()?;

		let mut boxes = vec![Vec::new(); 256];
		operations.into_iter().for_each(|op| op.apply_to_boxes(&mut boxes));

		// Calculate focusing power.
		let mut focusing_power = 0;
		for (i, box_) in boxes.iter().enumerate() {
			for (j, lense) in box_.iter().enumerate() {
				let power = (i + 1) * (j + 1) * lense.1 as usize;
				focusing_power += power;
			}
		}

		Ok(focusing_power.to_string())
	}
}

fn hash(s: &str) -> u8 {
	s.chars().map(|c| c as u8).fold(0, |hash, c| hash.wrapping_add(c).wrapping_mul(17))
}

impl Operation {
	pub fn apply_to_boxes(self, boxes: &mut [Vec<(String, u8)>]) {
		let id = self.label_hash() as usize;
		match self {
			Operation::Add { label, lense } => {
				if let Some(pos) = boxes[id].iter().position(|(l, _)| *l == label) {
					boxes[id][pos] = (label, lense);
				} else {
					boxes[id].push((label, lense));
				}
			}
			Operation::Remove { label } => {
				if let Some(pos) = boxes[id].iter().position(|(l, _)| *l == label) {
					boxes[id].remove(pos);
				}
			}
		}
	}

	fn label_hash(&self) -> u8 {
		let label = match self {
			Operation::Remove { label } => label,
			Operation::Add { label, .. } => label,
		};
		hash(label)
	}
}

enum Operation {
	Remove { label: String },
	Add { label: String, lense: u8 },
}

impl FromStr for Operation {
	type Err = anyhow::Error;

	fn from_str(seq: &str) -> Result<Self> {
		if let Some((label, lense)) = seq.split_once('=') {
			Ok(Self::Add { label: label.to_owned(), lense: lense.parse()? })
		} else if let Some((label, _)) = seq.split_once('-') {
			Ok(Self::Remove { label: label.to_owned() })
		} else {
			bail!("Neither operation found");
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_hash() {
		let hash = hash("HASH");
		assert_eq!(hash, 52);
	}

	const INPUT: &str = r#"
		rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
		"#;

	#[test]
	fn part1() -> Result<()> {
		let result = Day.part1(INPUT)?;
		assert_eq!(result.as_str(), "1320");

		Ok(())
	}

	#[test]
	fn part2() -> Result<()> {
		let result = Day.part2(INPUT)?;
		assert_eq!(result.as_str(), "145");

		Ok(())
	}
}
