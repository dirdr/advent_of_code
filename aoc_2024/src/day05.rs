use std::collections::{HashMap, HashSet};

use aoc_lib::{answer::Answer, solution::Solution};

pub struct Day5;

impl Solution for Day5 {
    fn part_a(&self, input: &[String]) -> Answer {
        let pq = parse(input);
        let valids = pq.get_valid_updates();
        valids
            .into_iter()
            .map(|u| u[u.len() / 2])
            .sum::<i32>()
            .into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let pq = parse(input);
        let mut invalids = pq.get_invalid_updates();
        for invalid in invalids.iter_mut() {
            pq.sort_update(invalid);
        }
        invalids
            .into_iter()
            .map(|u| u[u.len() / 2])
            .sum::<i32>()
            .into()
    }
}

struct PrintQueue {
    order: HashMap<i32, HashSet<i32>>,
    updates: Vec<Vec<i32>>,
}

impl PrintQueue {
    fn get_valid_updates(&self) -> Vec<Vec<i32>> {
        self.updates
            .iter()
            .filter(|u| self.is_update_valid(u))
            .cloned()
            .collect::<Vec<_>>()
    }

    fn get_invalid_updates(&self) -> Vec<Vec<i32>> {
        self.updates
            .iter()
            .filter(|u| !self.is_update_valid(u))
            .cloned()
            .collect::<Vec<_>>()
    }

    fn is_update_valid(&self, update: &[i32]) -> bool {
        let mut seen: HashSet<i32> = HashSet::new();
        let mut invalids: HashSet<i32> = HashSet::new();

        for &el in update {
            if invalids.contains(&el) {
                return false;
            }
            let empty_set = HashSet::<i32>::new();
            let must_seen = self.order.get(&el).unwrap_or(&empty_set);
            let diff = must_seen.difference(&seen).collect::<HashSet<_>>();
            invalids.extend(diff.into_iter());
            seen.insert(el);
        }
        true
    }

    fn sort_update(&self, update: &mut [i32]) {
        update.sort_unstable_by(|a, b| {
            (self.order.contains_key(b) && self.order[b].contains(a)).cmp(&true)
        })
    }
}

fn parse(input: &[String]) -> PrintQueue {
    let sections: Vec<Vec<String>> = input
        .split(|line| line.is_empty())
        .map(|section| section.to_vec())
        .collect();

    let rules = &sections[0];
    let updates = &sections[1];

    let order = parse_rules(rules);

    let updates = parse_updates(updates);

    PrintQueue { order, updates }
}

fn parse_rules(rules: &[String]) -> HashMap<i32, HashSet<i32>> {
    let mut order: HashMap<i32, HashSet<i32>> = HashMap::new();
    for rule in rules {
        let (first, second) = rule.split_once('|').unwrap();

        let first = first.parse::<i32>().unwrap();
        let second = second.parse::<i32>().unwrap();

        order.entry(second).or_default().insert(first);
    }

    order
}

fn parse_updates(updates: &[String]) -> Vec<Vec<i32>> {
    updates
        .iter()
        .map(|update| {
            update
                .split(',')
                .map(|d| d.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use aoc_lib::{answer::Answer, input, solution::Solution};

    use super::Day5;

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_05_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day5.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(143), answer);
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_05_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day5.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(123), answer);
    }
}
