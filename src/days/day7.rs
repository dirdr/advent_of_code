use std::cmp::Ordering;

use crate::helper_lib::{answer::Answer, solution::Solution};
use itertools::Itertools;

pub struct Day7;

struct Parsed {
    hands: Vec<Hand>,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<u8>,
    bid: u32,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

// so we can map our card (char) to u8 value (found index + 2)
const POSSIBLE_CARDS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

fn parse(input: &[String]) -> Parsed {
    let mut hands = vec![];
    for line in input {
        let (cards, bid) = line.split_once(" ").unwrap();
        let cards = cards
            .chars()
            .map(|card| POSSIBLE_CARDS.iter().position(|&ch| ch == card).unwrap() as u8 + 2)
            .collect();
        let bid = bid.parse::<u32>().unwrap();
        hands.push(Hand { cards, bid });
    }
    Parsed { hands }
}

impl Hand {
    fn compare(&self, other: &Hand) -> Ordering {
        for (&a, &b) in self.cards.iter().zip(&other.cards) {
            if a != b {
                return a.cmp(&b);
            }
        }
        Ordering::Equal
    }

    fn to_hand_type(&self) -> HandType {
        let mut count = [0; 13];
        for card in self.cards.iter() {
            count[*card as usize - 2] += 1;
        }
        if count.iter().any(|&card| card == 5) {
            return HandType::FiveOfAKind;
        } else if count.iter().any(|&card| card == 4) {
            return HandType::FourOfAKind;
        } else if count.iter().any(|&card| card == 3) && count.iter().any(|&card| card == 2) {
            return HandType::FullHouse;
        } else if count.iter().any(|&card| card == 3) {
            return HandType::ThreeOfAKind;
        } else if count.iter().filter(|&&card| card == 2).count() == 2 {
            return HandType::TwoPair;
        } else if count.iter().any(|&card| card == 2) {
            return HandType::OnePair;
        } else {
            HandType::HighCard
        }
    }
}

impl Solution for Day7 {
    fn part_a(&self, input: &[String]) -> Answer {
        let mut parsed = parse(input);
        parsed.hands.sort_by(|a, b| {
            a.to_hand_type()
                .cmp(&b.to_hand_type())
                .then_with(|| b.compare(&a))
        });
        parsed
            .hands
            .iter()
            .rev()
            .enumerate()
            .map(|(i, hand)| (i + 1) as u32 * hand.bid)
            .sum::<u32>()
            .into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::helper_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day7;

    #[test]
    pub fn test_a() {
        let input =
            input::read_file(&format!("{}day_7_test.txt", helper_lib::FILES_PREFIX)).unwrap();
        let answer = Day7.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(6440i32), answer);
    }

    pub fn test_b() {
        let input =
            input::read_file(&format!("{}day_7_test.txt", helper_lib::FILES_PREFIX)).unwrap();
        let answer = Day7.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(71503i32), answer);
    }
}
