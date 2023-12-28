use std::{
    cmp::{max, min},
    collections::HashSet,
};

use crate::helper_lib::{answer::Answer, solution::Solution};

pub struct Day11;

impl Solution for Day11 {
    fn part_a(&self, input: &[String]) -> Answer {
        solve(input, 2)
    }

    fn part_b(&self, input: &[String]) -> Answer {
        solve(input, 1000000)
    }
}

pub fn solve(input: &[String], expansion_factor: usize) -> Answer {
    let sky = parse(input);
    sky.make_pairs()
        .iter()
        .map(|&(a, b)| sky.expansion_taxicab(a, b, expansion_factor))
        .sum::<usize>()
        .into()
}

struct Sky {
    elements: Vec<Vec<Element>>,
}

impl Sky {
    fn retrieve_galaxies_positions(&self) -> Vec<(usize, usize)> {
        let mut galaxies_pos = vec![];
        for (i, row) in self.elements.iter().enumerate() {
            for (j, el) in row.iter().enumerate() {
                if &Element::Galaxy == el {
                    galaxies_pos.push((j, i));
                }
            }
        }
        galaxies_pos
    }

    pub fn make_pairs(&self) -> HashSet<((usize, usize), (usize, usize))> {
        let mut pairs = HashSet::new();
        let galaxies = self.retrieve_galaxies_positions();
        for (i, &el) in galaxies.iter().enumerate() {
            for &other in &galaxies[i + 1..] {
                let pair = if el <= other {
                    (el, other)
                } else {
                    (other, el)
                };
                pairs.insert(pair);
            }
        }
        pairs
    }

    fn count_empty_row_between(&self, a: (usize, usize), b: (usize, usize)) -> usize {
        let min = min(a.1, b.1);
        let max = max(a.1, b.1);
        let mut count = 0;
        for i in (min + 1)..max {
            if self.row_empty(i) {
                count += 1;
            }
        }
        count
    }

    fn count_empty_col_between(&self, a: (usize, usize), b: (usize, usize)) -> usize {
        let min = min(a.0, b.0);
        let max = max(a.0, b.0);
        let mut count = 0;
        for i in (min + 1)..max {
            if self.col_empty(i) {
                count += 1;
            }
        }
        count
    }

    pub fn expansion_taxicab(
        &self,
        a: (usize, usize),
        b: (usize, usize),
        expansion_factor: usize,
    ) -> usize {
        let dx = (a.0 as isize - b.0 as isize).abs() as usize;
        let dy = (a.1 as isize - b.1 as isize).abs() as usize;

        let expanded_dx = dx + self.count_empty_col_between(a, b) * (expansion_factor - 1);
        let expanded_dy = dy + self.count_empty_row_between(a, b) * (expansion_factor - 1);

        expanded_dx + expanded_dy
    }

    fn row_empty(&self, position: usize) -> bool {
        self.elements[position].iter().all(|e| *e == Element::Empty)
    }

    fn col_empty(&self, position: usize) -> bool {
        self.elements
            .iter()
            .all(|row| row[position] == Element::Empty)
    }
}

fn parse(input: &[String]) -> Sky {
    let mut elements = vec![];
    for row in input.iter() {
        let mut temp = vec![];
        for el in row.chars() {
            match el {
                '.' => temp.push(Element::Empty),
                '#' => temp.push(Element::Galaxy),
                _ => unreachable!(),
            }
        }
        elements.push(temp);
    }
    Sky { elements }
}

#[derive(Debug, Clone, PartialEq)]
enum Element {
    Galaxy,
    Empty,
}

#[cfg(test)]
mod test {
    use crate::helper_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day11;

    #[test]
    pub fn test_a() {
        let input =
            input::read_file(&format!("{}day_11_test.txt", helper_lib::FILES_PREFIX)).unwrap();
        let answer = Day11.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(374), answer);
    }

    #[test]
    pub fn test_b() {
        let input =
            input::read_file(&format!("{}day_11_test.txt", helper_lib::FILES_PREFIX)).unwrap();
        let answer = Day11.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(82000210), answer);
    }
}
