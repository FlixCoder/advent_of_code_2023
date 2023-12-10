use std::collections::BTreeMap;

use anyhow::Result;
use once_cell::sync::Lazy;

pub trait AocDay: Send + Sync {
	fn part1(&self, input: &str) -> Result<String>;
	fn part2(&self, input: &str) -> Result<String>;
}

macro_rules! implemented_days {
	($($days: literal),*) => {
		paste::paste! {
			$(
				mod [< day_ $days >];
			)*

			pub static DAYS: Lazy<BTreeMap<u8, Box<dyn AocDay>>> = Lazy::new(|| {
				let mut map = BTreeMap::<u8, Box<dyn AocDay>>::new();
				$(
					map.insert($days, Box::new(self::[< day_ $days >]::Day));
				)*
				map
			});
		}
	};
}

implemented_days!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
