use std::collections::HashMap;

use crate::helper_lib::{answer::Answer, solution::Solution};

pub struct Day19;

impl Solution for Day19 {
    fn part_a(&self, input: &[String]) -> Answer {
        let parsed = parse(input);
        let mut sum = 0;
        for part in parsed.parts {
            let mut current_workflow = "in".to_owned();
            loop {
                if current_workflow == *"R" {
                    break;
                }
                if current_workflow == *"A" {
                    sum += part.ratings.iter().sum::<usize>();
                    break;
                }
                for rule in parsed.workflows.get(&current_workflow).unwrap() {
                    if let Some(condition) = &rule.condition {
                        if condition.valid(&part) {
                            current_workflow = rule.next_workflow.clone();
                            break;
                        }
                    } else {
                        current_workflow = rule.next_workflow.clone();
                    }
                }
            }
        }
        sum.into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let parsed = parse(input);
        solve_b([(1, 4000); 4], &parsed.workflows, "in".to_owned()).into()
    }
}

fn solve_b(
    mut ranges: [(usize, usize); 4],
    workflows: &HashMap<String, Vec<Rule>>,
    current_workflow: String,
) -> usize {
    let mut sum = 0;
    // base case
    if *current_workflow == *"R" {
        return sum;
    }
    if *current_workflow == *"A" {
        return sum + count_ranges_combinations(&ranges);
    }
    for rule in workflows.get(&current_workflow).unwrap() {
        if let Some(condition) = &rule.condition {
            let index = Part::to_rating_index(condition.category);
            let mut valid_ranges = ranges;
            let valid_rating = &mut valid_ranges[index];
            let invalid_rating = &mut ranges[index];

            match condition.condition_type {
                ConditionType::LessThan if valid_rating.0 < condition.value => {
                    valid_rating.1 = valid_rating.1.min(condition.value - 1);
                    invalid_rating.0 = invalid_rating.0.max(condition.value);
                }
                ConditionType::GreaterThan if valid_rating.1 > condition.value => {
                    valid_rating.0 = valid_rating.0.max(condition.value + 1);
                    invalid_rating.1 = invalid_rating.1.min(condition.value);
                }
                _ => continue,
            };

            // we explore with new valid range
            sum += solve_b(valid_ranges, workflows, rule.next_workflow.clone());
        } else {
            sum += solve_b(ranges, workflows, rule.next_workflow.clone())
        }
    }
    sum
}

fn parse(input: &[String]) -> Parsed {
    let mut split = input.split(|line| line.trim().is_empty());
    let workflows_str = split.next().unwrap_or(&[]).to_vec();
    let parts_str = split.next().unwrap_or(&[]).to_vec();
    let mut parts = vec![];
    let mut workflows = HashMap::new();
    for workflow in workflows_str {
        let (name, rule) = workflow.split_once('{').unwrap();
        let mut rules = vec![];
        for r in rule[..rule.len() - 1].split(',') {
            let Some((condition, next_workflow)) = r.split_once(':') else {
                rules.push(Rule {
                    condition: None,
                    next_workflow: r.to_string(),
                });
                continue;
            };
            let condition_type = ConditionType::from(&condition[1..2]);
            let value = condition[2..].parse::<usize>().unwrap();
            rules.push(Rule {
                condition: Some(Condition {
                    category: condition[0..1].chars().next().unwrap(),
                    condition_type,
                    value,
                }),
                next_workflow: next_workflow.to_string(),
            })
        }
        workflows.insert(name.to_string(), rules);
    }
    for part in parts_str {
        let part = part[1..part.len() - 1].to_owned();
        let ratings_str = part.split(',').collect::<Vec<_>>();
        let mut ratings = [0; 4];
        for token in ratings_str {
            let (rating, value) = token.split_once('=').unwrap();
            let rating = rating.chars().next().unwrap();
            ratings[Part::to_rating_index(rating)] = value.parse::<usize>().unwrap();
        }
        parts.push(Part { ratings });
    }
    Parsed { workflows, parts }
}

struct Parsed {
    workflows: HashMap<String, Vec<Rule>>,
    parts: Vec<Part>,
}

#[derive(Debug)]
struct Condition {
    category: char,
    condition_type: ConditionType,
    value: usize,
}

impl Condition {
    fn valid(&self, part: &Part) -> bool {
        let part_value = part
            .ratings
            .get(Part::to_rating_index(self.category))
            .unwrap();
        match self.condition_type {
            ConditionType::GreaterThan => part_value > &self.value,
            ConditionType::LessThan => part_value < &self.value,
        }
    }
}

#[derive(Debug)]
struct Rule {
    condition: Option<Condition>,
    next_workflow: String,
}

#[derive(Debug)]
enum ConditionType {
    GreaterThan,
    LessThan,
}

impl From<&str> for ConditionType {
    fn from(value: &str) -> Self {
        match value {
            ">" => ConditionType::GreaterThan,
            "<" => ConditionType::LessThan,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Part {
    ratings: [usize; 4],
}

impl Part {
    fn to_rating_index(ch: char) -> usize {
        match ch {
            'x' => 0,
            'm' => 1,
            'a' => 2,
            's' => 3,
            _ => unreachable!(),
        }
    }
}

fn count_ranges_combinations(ranges: &[(usize, usize); 4]) -> usize {
    ranges.iter().map(|r| r.1 - r.0 + 1).product::<usize>()
}

#[cfg(test)]
mod test {
    use crate::helper_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day19;

    #[test]
    fn test_a() {
        let input = input::read_file(&format!(
            "{}day_19_test.txt",
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day19.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(19114), answer);
    }

    #[test]
    fn test_b() {
        let input = input::read_file(&format!(
            "{}day_19_test.txt",
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day19.part_b(&input);
        assert_eq!(<i64 as Into<Answer>>::into(167409079868000), answer);
    }
}
