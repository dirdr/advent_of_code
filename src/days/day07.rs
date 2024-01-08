use std::cmp::Ordering;

use crate::helper_lib::{answer::Answer, solution::Solution};
use itertools::Itertools;

pub struct Day7;

const CARDS_ORDER_A: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

const CARDS_ORDER_B: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

impl Solution for Day7 {
    fn part_a(&self, input: &[String]) -> Answer {
        let mut parsed = parse(input, CardsOrder::OrderA);
        parsed.hands.sort_by(|a, b| {
            a.to_hand_type_a()
                .cmp(&b.to_hand_type_a())
                .then_with(|| b.compare(a))
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
        let mut parsed = parse(input, CardsOrder::OrderB);
        parsed.hands.sort_by(|a, b| {
            a.to_hand_type_b()
                .cmp(&b.to_hand_type_b())
                .then_with(|| b.compare(a))
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
}

struct Parsed {
    hands: Vec<Hand>,
}

fn parse(input: &[String], card_order: CardsOrder) -> Parsed {
    let mut hands = vec![];
    for line in input {
        let (cards, bid) = line.split_once(' ').unwrap();
        let cards = cards
            .chars()
            .map(|card| {
                let order = match card_order {
                    CardsOrder::OrderA => &CARDS_ORDER_A,
                    CardsOrder::OrderB => &CARDS_ORDER_B,
                };
                order.iter().position(|&ch| ch == card).unwrap() as u8
            })
            .collect();
        let bid = bid.parse::<u32>().unwrap();
        hands.push(Hand { cards, bid });
    }
    Parsed { hands }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<u8>,
    bid: u32,
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

    fn to_hand_type_a(&self) -> HandType {
        let mut count = [0; 13];
        for card in self.cards.iter() {
            count[*card as usize] += 1;
        }
        if count.iter().any(|&card| card == 5) {
            HandType::FiveOfAKind
        } else if count.iter().any(|&card| card == 4) {
            HandType::FourOfAKind
        } else if count.iter().any(|&card| card == 3) && count.iter().any(|&card| card == 2) {
            HandType::FullHouse
        } else if count.iter().any(|&card| card == 3) {
            HandType::ThreeOfAKind
        } else if count.iter().filter(|&&card| card == 2).count() == 2 {
            HandType::TwoPair
        } else if count.iter().any(|&card| card == 2) {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }

    fn to_hand_type_b(&self) -> HandType {
        let mut count = [0; 13];
        for card in self.cards.iter() {
            count[*card as usize] += 1;
        }
        let joker_count = count[0];

        let sorted = count[1..13]
            .iter()
            .copied()
            .filter(|&x| x != 0)
            .sorted()
            .rev()
            .collect::<Vec<_>>();

        // if full joker, then sorted len is 0
        if sorted.is_empty() || joker_count + sorted[0] == 5 {
            HandType::FiveOfAKind
        } else if joker_count + sorted[0] == 4 {
            HandType::FourOfAKind
        } else if ((sorted[0] + joker_count == 3) && (sorted[1] == 2))
            || ((sorted[0] == 3) && (sorted[1] + joker_count == 2))
        {
            HandType::FullHouse
        } else if sorted[0] + joker_count == 3 {
            HandType::ThreeOfAKind
        } else if ((sorted[0] + joker_count) == 2 && (sorted[1] == 2))
            || (sorted[0] == 2 && sorted[1] + joker_count == 2)
        {
            HandType::TwoPair
        } else if sorted[0] + joker_count == 2 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
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

enum CardsOrder {
    OrderA,
    OrderB,
}

#[cfg(test)]
mod test {
    use crate::helper_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day7;

    #[test]
    pub fn test_a() {
        let input = input::read_file(&format!(
            "{}day_07_test.txt",
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day7.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(6440), answer);
    }

    #[test]
    pub fn test_b() {
        let input = input::read_file(&format!(
            "{}day_07_test.txt",
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day7.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(5905), answer);
    }
}
