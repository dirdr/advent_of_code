use std::{cmp::max, collections::HashMap};

use crate::helper_lib::{answer::Answer, solution::Solution};

pub struct Day2;

const STARTING_BAG_A: [(&str, usize); 3] = [("red", 12), ("green", 13), ("blue", 14)];
const STARTING_BAG_B: [(&str, usize); 3] = [("red", 0), ("green", 0), ("blue", 0)];

impl Solution for Day2 {
    fn part_a(&self, input: &[String]) -> Answer {
        let parsed = parse(input, STARTING_BAG_A);
        parsed
            .get_valid_games()
            .iter()
            .map(|game| game.id)
            .sum::<usize>()
            .into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let parsed = parse(input, STARTING_BAG_B);
        parsed
            .games
            .iter()
            .map(|game| game.get_minimum_set_power())
            .sum::<usize>()
            .into()
    }
}

#[derive(Debug)]
struct Parsed<'a> {
    games: Vec<Game<'a>>,
    bag: HashMap<&'a str, usize>,
}

fn parse<'a>(input: &'a [String], starting_bag: [(&'a str, usize); 3]) -> Parsed<'a> {
    let mut games = vec![];
    let bag_map: HashMap<_, _> = starting_bag
        .iter()
        .enumerate()
        .map(|(i, (color, _))| (*color, i))
        .collect();
    for line in input {
        if let Some(iter) = line.strip_prefix("Game ") {
            let mut sets = vec![];
            let (id, colors) = iter.split_once(':').unwrap();
            let id = id.parse::<usize>().unwrap();
            for sets_str in colors.split(';') {
                let mut set = starting_bag.map(|(color, _)| (color, 0));
                for token in sets_str.split(',') {
                    let (val, color) = token.trim().split_once(' ').unwrap();
                    if let Some(&entry) = bag_map.get(color) {
                        set[entry] = (color, val.parse::<usize>().unwrap());
                    }
                }
                sets.push(set);
            }
            games.push(Game { id, sets });
        }
    }
    Parsed {
        games,
        bag: HashMap::from(starting_bag),
    }
}

impl<'a> Parsed<'a> {
    fn get_valid_games(self) -> Vec<Game<'a>> {
        self.games
            .into_iter()
            .filter(|game| game.is_valid(&self.bag))
            .collect()
    }
}

#[derive(Debug)]
struct Game<'a> {
    id: usize,
    sets: Vec<[(&'a str, usize); 3]>,
}

impl<'a> Game<'a> {
    fn is_valid(&self, bag: &HashMap<&'a str, usize>) -> bool {
        self.sets
            .iter()
            .all(|set| set.iter().all(|entry| Some(&entry.1) <= bag.get(entry.0)))
    }

    fn get_minimum_set_power(&self) -> usize {
        let mut bag: HashMap<_, _> = HashMap::from(STARTING_BAG_B);
        for set in self.sets.iter() {
            for entry in set {
                bag.entry(entry.0).and_modify(|e| *e = max(entry.1, *e));
            }
        }
        bag.values().product::<usize>()
    }
}

#[cfg(test)]
mod test {
    use crate::helper_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day2;

    #[test]
    fn test_a() {
        let input = input::read_file(&format!(
            "{}day_02_test.txt",
            helper_lib::consts::FILES_PREFIX_TEST
        ))
        .unwrap();
        let answer = Day2.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(8), answer);
    }

    #[test]
    fn test_b() {
        let input = input::read_file(&format!(
            "{}day_02_test.txt",
            helper_lib::consts::FILES_PREFIX_TEST
        ))
        .unwrap();
        let answer = Day2.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(2286), answer);
    }
}
