use crate::helper_lib::answer::Answer;
use crate::helper_lib::solution::Solution;

pub struct Day4;

struct Parsed {
    cards: Vec<Card>,
}

struct Card {
    winning: Vec<usize>,
    picks: Vec<usize>,
}

impl Card {
    pub fn count_winning(&self) -> usize {
        self.picks
            .iter()
            .filter(|num| self.winning.contains(num))
            .count()
    }
}

fn parse(input: &[String]) -> Parsed {
    let mut cards = vec![];
    for line in input {
        let line = line.split_once(':').unwrap().1.trim();
        let (winning, pick) = line.split_once('|').unwrap();
        let winning = convert_to_nums(winning);
        let picks = convert_to_nums(pick);
        cards.push(Card { winning, picks });
    }
    Parsed { cards }
}

fn convert_to_nums(str_set: &str) -> Vec<usize> {
    let mut nums = vec![];
    for token in str_set.split_whitespace() {
        nums.push(token.parse::<usize>().unwrap())
    }
    nums
}

impl Solution for Day4 {
    fn part_a(&self, input: &[String]) -> Answer {
        let parsed = parse(input);
        parsed
            .cards
            .iter()
            .map(|card| {
                let count = card.count_winning();
                if count == 0 {
                    return 0;
                }
                2_i32.pow(count as u32 - 1) as u32
            })
            .sum::<u32>()
            .into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        // we start with one copy of each of the cards,
        // cards are 0 indexed : 1st card : copies[0], ...
        let parsed = parse(input);
        let mut copies: Vec<usize> = vec![1; parsed.cards.len()];
        parsed.cards.iter().enumerate().for_each(|(i, card)| {
            let winning_count = card.count_winning();
            for _ in 0..copies[i] {
                copies
                    .iter_mut()
                    .skip(i + 1)
                    .take(winning_count)
                    .for_each(|copy| *copy += 1);
            }
        });
        copies.iter().sum::<usize>().into()
    }
}

#[cfg(test)]
mod test {
    use crate::helper_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day4;

    #[test]
    pub fn test_a() {
        let input =
            input::read_file(&format!("{}day_4_test.txt", helper_lib::FILES_PREFIX)).unwrap();
        let answer = Day4.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(13), answer);
    }

    #[test]
    pub fn test_b() {
        let input =
            input::read_file(&format!("{}day_4_test.txt", helper_lib::FILES_PREFIX)).unwrap();
        let answer = Day4.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(30), answer);
    }
}
