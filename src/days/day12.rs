use std::{
    collections::HashMap,
    fmt::{Display, Write},
};

use crate::helper_lib::{answer::Answer, solution::Solution};

pub struct Day12;

impl Solution for Day12 {
    fn part_a(&self, input: &[String]) -> Answer {
        let parsed = parse(input);
        parsed
            .rows
            .iter()
            .map(|r| r.count_arrangements())
            .sum::<usize>()
            .into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let parsed = parse(input);
        parsed
            .rows
            .iter()
            .map(|r| r.expand().count_arrangements())
            .sum::<usize>()
            .into()
    }
}

struct Parsed {
    rows: Vec<Row>,
}

fn parse(input: &[String]) -> Parsed {
    let mut rows = vec![];
    for line in input {
        let (conditions, group_sized) = line.split_once(' ').unwrap();
        let group_sizes = group_sized
            .split(',')
            .map(|d| d.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let mut conditions = conditions
            .chars()
            .map(|c| SpringCondition::from(c))
            .collect::<Vec<_>>();
        // to ensure block separation at the end
        conditions.push(SpringCondition::Working);
        rows.push(Row {
            conditions,
            group_sizes,
        })
    }
    Parsed { rows }
}

#[derive(Clone)]
pub struct Row {
    conditions: Vec<SpringCondition>,
    group_sizes: Vec<usize>,
}

impl Row {
    pub fn count_arrangements(&self) -> usize {
        fn dfs(
            memo: &mut HashMap<(usize, usize, usize), usize>,
            row: &Row,
            pos: usize,
            group_index: usize,
            group_len: usize,
        ) -> usize {
            if let Some(&key) = memo.get(&(pos, group_index, group_len)) {
                return key;
            }
            let mut arrangements = 0;
            if pos == row.conditions.len() {
                arrangements = (group_index == row.group_sizes.len()) as usize;
            } else if row.conditions[pos] == SpringCondition::Broken {
                arrangements = dfs(memo, row, pos + 1, group_index, group_len + 1)
            } else if row.conditions[pos] == SpringCondition::Working
                || group_index == row.group_sizes.len()
            {
                if group_index < row.group_sizes.len() && group_len == row.group_sizes[group_index]
                {
                    // closing the block
                    arrangements = dfs(memo, row, pos + 1, group_index + 1, 0);
                } else if group_len == 0 {
                    // multiple working..
                    arrangements = dfs(memo, row, pos + 1, group_index, 0);
                }
            } else {
                // continue with broken spring
                arrangements += dfs(memo, row, pos + 1, group_index, group_len + 1);

                // finished the block, closing
                if group_len == row.group_sizes[group_index] {
                    arrangements += dfs(memo, row, pos + 1, group_index + 1, 0);
                } else if group_len == 0 {
                    arrangements += dfs(memo, row, pos + 1, group_index, 0);
                }
            }
            memo.insert((pos, group_index, group_len), arrangements);
            arrangements
        }
        dfs(&mut HashMap::new(), self, 0, 0, 0)
    }

    pub fn expand(&self) -> Self {
        let mut new_condition = self.conditions.clone();
        *new_condition.last_mut().unwrap() = SpringCondition::Unknown;
        Self {
            conditions: new_condition.repeat(5),
            group_sizes: self.group_sizes.repeat(5),
        }
    }
}

#[derive(Clone, PartialEq, Copy)]
pub enum SpringCondition {
    Broken,
    Working,
    Unknown,
}

impl Display for SpringCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpringCondition::Broken => f.write_char('#'),
            SpringCondition::Working => f.write_char('.'),
            SpringCondition::Unknown => f.write_char('?'),
        }
    }
}

impl From<SpringCondition> for char {
    fn from(value: SpringCondition) -> Self {
        match value {
            SpringCondition::Broken => '#',
            SpringCondition::Working => '.',
            SpringCondition::Unknown => '?',
        }
    }
}

impl From<char> for SpringCondition {
    fn from(value: char) -> Self {
        match value {
            '.' => SpringCondition::Working,
            '#' => SpringCondition::Broken,
            '?' => SpringCondition::Unknown,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::helper_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day12;

    #[test]
    pub fn test_a() {
        let input =
            input::read_file(&format!("{}day_12_test.txt", helper_lib::FILES_PREFIX)).unwrap();
        let answer = Day12.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(21), answer);
    }

    #[test]
    pub fn test_b() {
        let input =
            input::read_file(&format!("{}day_12_test.txt", helper_lib::FILES_PREFIX)).unwrap();
        let answer = Day12.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(525152), answer);
    }
}
