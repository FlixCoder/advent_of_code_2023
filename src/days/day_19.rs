use std::{collections::VecDeque, ops::RangeInclusive, str::FromStr};

use ahash::AHashMap;
use anyhow::{bail, Context, Result};

use super::AocDay;

pub struct Day;

impl AocDay for Day {
	fn part1(&self, input: &str) -> Result<String> {
		let workflows = Workflows::from_str(input)?;
		let mut score = 0;
		for item in workflows.items() {
			if workflows.run_workflow(item)? {
				let sum = item.values().sum::<u32>();
				score += sum;
			}
		}
		Ok(score.to_string())
	}

	fn part2(&self, input: &str) -> Result<String> {
		let workflows = Workflows::from_str(input)?;
		let num_accepted_items = workflows.num_any_accepted()?;
		Ok(num_accepted_items.to_string())
	}
}

impl Workflows {
	pub fn num_any_accepted(&self) -> Result<usize> {
		let start_ranges = ItemRanges { x: 1..=4000, m: 1..=4000, a: 1..=4000, s: 1..=4000 };
		let mut queue = VecDeque::new();
		queue.push_back(("in", start_ranges));
		let mut accepted_ranges = Vec::new();
		while let Some((workflow, mut ranges)) = queue.pop_front() {
			if workflow == "A" {
				accepted_ranges.push(ranges);
				continue;
			} else if workflow == "R" {
				continue;
			}

			let workflow = self.workflows.get(workflow).context("Could not find workflow")?;
			for rule in &workflow.rules {
				let mut branch_ranges = ranges.clone();
				match rule {
					Rule::Always { target } => {
						queue.push_back((target, ranges));
						break;
					}
					Rule::LessThan { var, number, target } => {
						branch_ranges[*var] = (*branch_ranges[*var].start())
							..=(*branch_ranges[*var].end().min(number) - 1);
						ranges[*var] = (*ranges[*var].start().max(number))..=(*ranges[*var].end());
						queue.push_back((target, branch_ranges));
					}
					Rule::GreaterThan { var, number, target } => {
						branch_ranges[*var] = (*branch_ranges[*var].start().max(number) + 1)
							..=(*branch_ranges[*var].end());
						ranges[*var] = (*ranges[*var].start())..=(*ranges[*var].end().min(number));
						queue.push_back((target, branch_ranges));
					}
				}
			}
		}

		let total = accepted_ranges.into_iter().map(ItemRanges::possibilities).sum();
		Ok(total)
	}

	pub fn items(&self) -> &Vec<AHashMap<char, u32>> {
		&self.items
	}

	/// Run a workflow for one item and return whether it was accepted.
	pub fn run_workflow(&self, item: &AHashMap<char, u32>) -> Result<bool> {
		let mut current_workflow = "in";
		while current_workflow != "A" && current_workflow != "R" {
			let workflow =
				self.workflows.get(current_workflow).context("Could not find workflow")?;
			current_workflow = workflow.run_workflow(item)?;
		}
		Ok(current_workflow == "A")
	}
}

impl Workflow {
	pub fn run_workflow(&self, item: &AHashMap<char, u32>) -> Result<&String> {
		for rule in &self.rules {
			match rule {
				Rule::Always { target } => return Ok(target),
				Rule::LessThan { var, number, target } => {
					if item[var] < *number {
						return Ok(target);
					}
				}
				Rule::GreaterThan { var, number, target } => {
					if item[var] > *number {
						return Ok(target);
					}
				}
			}
		}
		bail!("No rule matched!");
	}
}

impl ItemRanges {
	pub fn possibilities(self) -> usize {
		let len_x = self.x.count();
		let len_m = self.m.count();
		let len_a = self.a.count();
		let len_s = self.s.count();
		len_x * len_m * len_a * len_s
	}
}

impl std::ops::Index<char> for ItemRanges {
	type Output = RangeInclusive<u32>;

	fn index(&self, index: char) -> &Self::Output {
		match index {
			'x' => &self.x,
			'm' => &self.m,
			'a' => &self.a,
			's' => &self.s,
			_ => panic!("Invalid variable {index}"),
		}
	}
}

impl std::ops::IndexMut<char> for ItemRanges {
	fn index_mut(&mut self, index: char) -> &mut Self::Output {
		match index {
			'x' => &mut self.x,
			'm' => &mut self.m,
			'a' => &mut self.a,
			's' => &mut self.s,
			_ => panic!("Invalid variable {index}"),
		}
	}
}

#[derive(Debug, Clone)]
struct ItemRanges {
	x: RangeInclusive<u32>,
	m: RangeInclusive<u32>,
	a: RangeInclusive<u32>,
	s: RangeInclusive<u32>,
}

#[derive(Debug)]
enum Rule {
	/// Always applies (the last rule).
	Always { target: String },
	/// If variable is less than number.
	LessThan { var: char, number: u32, target: String },
	/// If variable is greater than number.
	GreaterThan { var: char, number: u32, target: String },
}

#[derive(Debug)]
struct Workflow {
	/// List of rules.
	rules: Vec<Rule>,
}

#[derive(Debug)]
struct Workflows {
	/// Map from workflow name to workflow info.
	workflows: AHashMap<String, Workflow>,
	/// Map from variable name to value.
	items: Vec<AHashMap<char, u32>>,
}

impl FromStr for Workflow {
	type Err = anyhow::Error;

	fn from_str(line: &str) -> Result<Self> {
		let mut rules = Vec::new();
		for rule in line.split(',') {
			if let Some((condition, target)) = rule.split_once(':') {
				if let Some((var, number)) = condition.split_once('<') {
					let var = var.chars().nth(0).context("Empty var")?;
					let number = number.parse()?;
					rules.push(Rule::LessThan { var, number, target: target.to_owned() });
				} else if let Some((var, number)) = condition.split_once('>') {
					let var = var.chars().nth(0).context("Empty var")?;
					let number = number.parse()?;
					rules.push(Rule::GreaterThan { var, number, target: target.to_owned() });
				} else {
					bail!("Invalid condition `{condition}`");
				}
			} else {
				rules.push(Rule::Always { target: rule.to_owned() });
			}
		}
		Ok(Self { rules })
	}
}

impl FromStr for Workflows {
	type Err = anyhow::Error;

	fn from_str(input: &str) -> Result<Self> {
		let (workflows, items) =
			input.trim().split_once("\n\n").context("Could not find double newline")?;
		let workflows = workflows
			.lines()
			.map(|line| {
				let (name, workflow) = line.trim().split_once('{').context("Parsing workflow")?;
				let workflow = workflow.trim_end_matches('}').parse::<Workflow>()?;
				Ok::<_, anyhow::Error>((name.to_owned(), workflow))
			})
			.collect::<Result<_, _>>()?;
		let items = items
			.lines()
			.map(|line| {
				let mut item = AHashMap::new();
				for var in line.trim().trim_start_matches('{').trim_end_matches('}').split(',') {
					let (var, number) =
						var.split_once('=').context("Invalid variable assignment")?;
					let var = var.chars().nth(0).context("Empty variable name")?;
					let number = number.parse()?;
					item.insert(var, number);
				}
				Ok::<_, anyhow::Error>(item)
			})
			.collect::<Result<_, _>>()?;
		Ok(Self { workflows, items })
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"
		px{a<2006:qkq,m>2090:A,rfg}
		pv{a>1716:R,A}
		lnx{m>1548:A,A}
		rfg{s<537:gd,x>2440:R,A}
		qs{s>3448:A,lnx}
		qkq{x<1416:A,crn}
		crn{x>2662:A,R}
		in{s<1351:px,qqz}
		qqz{s>2770:qs,m<1801:hdj,R}
		gd{a>3333:R,R}
		hdj{m>838:A,pv}

		{x=787,m=2655,a=1222,s=2876}
		{x=1679,m=44,a=2067,s=496}
		{x=2036,m=264,a=79,s=2244}
		{x=2461,m=1339,a=466,s=291}
		{x=2127,m=1623,a=2188,s=1013}
		"#;

	#[test]
	fn part1() -> Result<()> {
		let result = Day.part1(INPUT)?;
		assert_eq!(result.as_str(), "19114");

		Ok(())
	}

	#[test]
	fn part2() -> Result<()> {
		let input = r#"
		in{x<2000:a,b}
		a{A}
		b{x>2000:R,A}

		{x=1,m=1,a=1,s=1}
		"#;
		let result = Day.part2(input)?;
		assert_eq!(result.as_str(), "128000000000000");

		let input = r#"
		in{s<1351:px,in2}
		  px{a<2006:qkq,px2}
		    qkq{x<1416:A,qkq2}
		      qkq2{x>2662:A,R}
		    px2{m>2090:A,px3}
		      px3{s<537:R,px4}
		        px4{x>2440:R,A}
		  in2{s>2770:A,in3}
		    in3{m<1801:hdj,R}
		      hdj{m>838:A,hdj2}
		        hdj2{a>1716:R,A}

		{x=1,m=1,a=1,s=1}
		"#;
		let result = Day.part2(input)?;
		assert_eq!(result.as_str(), "167409079868000");

		let result = Day.part2(INPUT)?;
		assert_eq!(result.as_str(), "167409079868000");

		Ok(())
	}
}
