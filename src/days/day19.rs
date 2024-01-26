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
                    sum += part.ratings.values().sum::<usize>();
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
        todo!()
    }
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
        let ratings = part.split(',').collect::<Vec<_>>();
        let mut map = HashMap::new();
        for token in ratings {
            let (rating, value) = token.split_once('=').unwrap();
            let rating = rating.chars().next().unwrap();
            map.insert(rating, value.parse::<usize>().unwrap());
        }
        parts.push(Part { ratings: map });
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
        let part_value = part.ratings.get(&self.category).unwrap();
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
    ratings: HashMap<char, usize>,
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
