use std::{cmp::Ordering, collections::BTreeMap, str::FromStr};

use anyhow::{bail, Context, Result};

use super::AocDay;

pub struct Day;

impl AocDay for Day {
	fn part1(&self, input: &str) -> Result<String> {
		let game = Game::<Card1>::from_str(input)?;

		let total_winnings = game.total_winnings();

		Ok(total_winnings.to_string())
	}

	fn part2(&self, input: &str) -> Result<String> {
		let game = Game::<Card2>::from_str(input)?;

		let total_winnings = game.total_winnings();

		Ok(total_winnings.to_string())
	}
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card1 {
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten,
	Jack,
	Queen,
	King,
	Ace,
}

impl From<char> for Card1 {
	fn from(c: char) -> Self {
		match c {
			'2' => Self::Two,
			'3' => Self::Three,
			'4' => Self::Four,
			'5' => Self::Five,
			'6' => Self::Six,
			'7' => Self::Seven,
			'8' => Self::Eight,
			'9' => Self::Nine,
			'T' => Self::Ten,
			'J' => Self::Jack,
			'Q' => Self::Queen,
			'K' => Self::King,
			'A' => Self::Ace,
			_ => panic!("Invalid character for card: {c}"),
		}
	}
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card2 {
	Joker,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten,
	Queen,
	King,
	Ace,
}

impl From<char> for Card2 {
	fn from(c: char) -> Self {
		match c {
			'J' => Self::Joker,
			'2' => Self::Two,
			'3' => Self::Three,
			'4' => Self::Four,
			'5' => Self::Five,
			'6' => Self::Six,
			'7' => Self::Seven,
			'8' => Self::Eight,
			'9' => Self::Nine,
			'T' => Self::Ten,
			'Q' => Self::Queen,
			'K' => Self::King,
			'A' => Self::Ace,
			_ => panic!("Invalid character for card: {c}"),
		}
	}
}

impl From<Card2> for Card1 {
	fn from(card: Card2) -> Self {
		match card {
			Card2::Joker => Card1::Jack,
			Card2::Two => Card1::Two,
			Card2::Three => Card1::Three,
			Card2::Four => Card1::Four,
			Card2::Five => Card1::Five,
			Card2::Six => Card1::Six,
			Card2::Seven => Card1::Seven,
			Card2::Eight => Card1::Eight,
			Card2::Nine => Card1::Nine,
			Card2::Ten => Card1::Ten,
			Card2::Queen => Card1::Queen,
			Card2::King => Card1::King,
			Card2::Ace => Card1::Ace,
		}
	}
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
	HighCard,
	OnePair,
	TwoPair,
	ThreeOfAKind,
	FullHouse,
	FourOfAKind,
	FiveOfAKind,
}

impl From<[Card1; 5]> for HandType {
	fn from(mut cards: [Card1; 5]) -> Self {
		cards.sort();
		let mut deduplicated = cards.to_vec();
		deduplicated.dedup();

		if deduplicated.len() == 1 {
			Self::FiveOfAKind
		} else if deduplicated.len() == 2 {
			if cards[0] == cards[1] && cards[3] == cards[4] {
				Self::FullHouse
			} else {
				Self::FourOfAKind
			}
		} else if deduplicated.len() == 3 {
			#[allow(clippy::nonminimal_bool)]
			if (cards[0] == cards[1] && cards[1] == cards[2])
				|| (cards[1] == cards[2] && cards[2] == cards[3])
				|| (cards[2] == cards[3] && cards[3] == cards[4])
			{
				Self::ThreeOfAKind
			} else {
				Self::TwoPair
			}
		} else if deduplicated.len() == 4 {
			Self::OnePair
		} else {
			Self::HighCard
		}
	}
}

impl From<[Card2; 5]> for HandType {
	fn from(mut cards: [Card2; 5]) -> Self {
		let mut most = BTreeMap::new();
		for card in cards {
			if card != Card2::Joker {
				*most.entry(card).or_insert(0) += 1;
			}
		}
		let mut joker = Card2::Ace;
		let mut max_num = 0;
		for (card, num) in most {
			if num > max_num {
				max_num = num;
				joker = card;
			}
		}

		for card in &mut cards {
			if *card == Card2::Joker {
				*card = joker;
			}
		}

		let cards1 = cards.map(Card1::from);
		HandType::from(cards1)
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Hand<Card> {
	cards: [Card; 5],
	kind: HandType,
}

impl<Card> FromStr for Hand<Card>
where
	Card: From<char> + Copy,
	HandType: From<[Card; 5]>,
{
	type Err = anyhow::Error;

	fn from_str(hand: &str) -> Result<Self> {
		let cards = hand.chars().map(Card::from).collect::<Vec<_>>();
		if cards.len() != 5 {
			bail!("Hand is not exactly 5 cards");
		}

		let cards = [cards[0], cards[1], cards[2], cards[3], cards[4]];
		let kind = HandType::from(cards);

		Ok(Hand { cards, kind })
	}
}

impl<Card> PartialOrd for Hand<Card>
where
	Card: PartialEq + Eq + PartialOrd + Ord,
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl<Card> Ord for Hand<Card>
where
	Card: PartialEq + Eq + PartialOrd + Ord,
{
	fn cmp(&self, other: &Self) -> Ordering {
		self.kind.cmp(&other.kind).then(self.cards.cmp(&other.cards))
	}
}

struct Game<Card> {
	bids: BTreeMap<Hand<Card>, u64>,
}

impl<Card> FromStr for Game<Card>
where
	Card: From<char> + PartialEq + Eq + PartialOrd + Ord + Copy,
	HandType: From<[Card; 5]>,
{
	type Err = anyhow::Error;

	fn from_str(input: &str) -> Result<Self> {
		let bids = input
			.trim()
			.lines()
			.map(|line| {
				let (hand, bid) = line.trim().split_once(' ').context("Wrong line format")?;
				let hand = hand.parse::<Hand<Card>>()?;
				let bid = bid.parse::<u64>()?;
				Ok::<_, anyhow::Error>((hand, bid))
			})
			.collect::<Result<_, _>>()?;
		Ok(Self { bids })
	}
}

impl<Card> Game<Card> {
	fn total_winnings(&self) -> u64 {
		let mut total = 0;
		for (rank, (_hand, bid)) in self.bids.iter().enumerate() {
			total += (rank as u64 + 1) * *bid;
		}
		total
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"
		32T3K 765
		T55J5 684
		KK677 28
		KTJJT 220
		QQQJA 483
		"#;

	#[test]
	fn part1() -> Result<()> {
		let result = Day.part1(INPUT)?;
		assert_eq!(result.as_str(), "6440");

		Ok(())
	}

	#[test]
	fn part2() -> Result<()> {
		let result = Day.part2(INPUT)?;
		assert_eq!(result.as_str(), "5905");

		Ok(())
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum CorrectHand {
	HighCard { highest: Card1, high: Card1, middle: Card1, low: Card1, lowest: Card1 },
	OnePair { pair: Card1, high: Card1, middle: Card1, low: Card1 },
	TwoPair { high: Card1, low: Card1, other: Card1 },
	ThreeOfAKind { three: Card1, high: Card1, low: Card1 },
	FullHouse { three: Card1, two: Card1 },
	FourOfAKind { four: Card1, one: Card1 },
	FiveOfAKind { five: Card1 },
}

impl FromStr for CorrectHand {
	type Err = anyhow::Error;

	fn from_str(hand: &str) -> Result<Self> {
		let mut cards = hand.chars().map(Card1::from).collect::<Vec<_>>();
		if cards.len() != 5 {
			bail!("Hand is not exactly 5 cards");
		}

		cards.sort();
		let mut deduplicated = cards.clone();
		deduplicated.dedup();

		if deduplicated.len() == 1 {
			// All cards the same.
			Ok(Self::FiveOfAKind { five: cards[0] })
		} else if deduplicated.len() == 2 {
			// Four of a kind or FullHouse. Other can only be at start or end.
			if cards[0] == cards[1] && cards[3] == cards[4] {
				// Full House.
				if cards[1] == cards[2] {
					Ok(Self::FullHouse { three: cards[0], two: cards[4] })
				} else {
					Ok(Self::FullHouse { three: cards[4], two: cards[0] })
				}
			}
			// Four of a kind.
			else if cards[0] == cards[1] {
				Ok(Self::FourOfAKind { four: cards[0], one: cards[4] })
			} else {
				Ok(Self::FourOfAKind { four: cards[4], one: cards[0] })
			}
		} else if deduplicated.len() == 3 {
			// Three of a kind or two pairs.
			// Three of a kind.
			if cards[0] == cards[1] && cards[1] == cards[2] {
				Ok(Self::ThreeOfAKind { three: cards[1], high: cards[4], low: cards[3] })
			} else if cards[1] == cards[2] && cards[2] == cards[3] {
				Ok(Self::ThreeOfAKind { three: cards[2], high: cards[4], low: cards[0] })
			} else if cards[2] == cards[3] && cards[3] == cards[4] {
				Ok(Self::ThreeOfAKind { three: cards[3], high: cards[1], low: cards[0] })
			}
			// Two pairs.
			else if cards[0] != cards[1] {
				Ok(Self::TwoPair { high: cards[4], low: cards[2], other: cards[0] })
			} else if cards[3] != cards[4] {
				Ok(Self::TwoPair { high: cards[3], low: cards[1], other: cards[4] })
			} else {
				Ok(Self::TwoPair { high: cards[4], low: cards[0], other: cards[2] })
			}
		} else if deduplicated.len() == 4 {
			if cards[0] == cards[1] {
				Ok(Self::OnePair {
					pair: cards[0],
					high: cards[4],
					middle: cards[3],
					low: cards[2],
				})
			} else if cards[1] == cards[2] {
				Ok(Self::OnePair {
					pair: cards[1],
					high: cards[4],
					middle: cards[3],
					low: cards[0],
				})
			} else if cards[2] == cards[3] {
				Ok(Self::OnePair {
					pair: cards[2],
					high: cards[4],
					middle: cards[1],
					low: cards[0],
				})
			} else {
				Ok(Self::OnePair {
					pair: cards[3],
					high: cards[2],
					middle: cards[1],
					low: cards[0],
				})
			}
		} else {
			Ok(Self::HighCard {
				highest: cards[4],
				high: cards[3],
				middle: cards[2],
				low: cards[1],
				lowest: cards[0],
			})
		}
	}
}

impl PartialOrd for CorrectHand {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for CorrectHand {
	fn cmp(&self, other: &Self) -> Ordering {
		match (self, other) {
			(Self::FiveOfAKind { five: a }, Self::FiveOfAKind { five: b }) => a.cmp(b),
			(Self::FourOfAKind { four: a1, one: a2 }, Self::FourOfAKind { four: b1, one: b2 }) => {
				a1.cmp(b1).then(a2.cmp(b2))
			}
			(Self::FullHouse { three: a1, two: a2 }, Self::FullHouse { three: b1, two: b2 }) => {
				a1.cmp(b1).then(a2.cmp(b2))
			}
			(
				Self::ThreeOfAKind { three: a1, high: a2, low: a3 },
				Self::ThreeOfAKind { three: b1, high: b2, low: b3 },
			) => a1.cmp(b1).then(a2.cmp(b2)).then(a3.cmp(b3)),
			(
				Self::TwoPair { high: a1, low: a2, other: a3 },
				Self::TwoPair { high: b1, low: b2, other: b3 },
			) => a1.cmp(b1).then(a2.cmp(b2)).then(a3.cmp(b3)),
			(
				Self::OnePair { pair: a1, high: a2, middle: a3, low: a4 },
				Self::OnePair { pair: b1, high: b2, middle: b3, low: b4 },
			) => a1.cmp(b1).then(a2.cmp(b2)).then(a3.cmp(b3)).then(a4.cmp(b4)),
			(
				Self::HighCard { highest: a1, high: a2, middle: a3, low: a4, lowest: a5 },
				Self::HighCard { highest: b1, high: b2, middle: b3, low: b4, lowest: b5 },
			) => a1.cmp(b1).then(a2.cmp(b2)).then(a3.cmp(b3)).then(a4.cmp(b4)).then(a5.cmp(b5)),
			(a, b) => a.discriminant().cmp(&b.discriminant()),
		}
	}
}

impl CorrectHand {
	fn discriminant(&self) -> usize {
		match self {
			Self::HighCard { .. } => 0,
			Self::OnePair { .. } => 1,
			Self::TwoPair { .. } => 2,
			Self::ThreeOfAKind { .. } => 3,
			Self::FullHouse { .. } => 4,
			Self::FourOfAKind { .. } => 5,
			Self::FiveOfAKind { .. } => 6,
		}
	}
}
