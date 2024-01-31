use std::collections::{HashMap, VecDeque};

use crate::helper_lib::{answer::Answer, solution::Solution};

pub struct Day20;

impl Solution for Day20 {
    fn part_a(&self, input: &[String]) -> Answer {
        let modules = parse(input);
        let mut flip_flops_memory = HashMap::new();
        let mut conjuctions_memory = HashMap::new();
        let mut result = [0_usize; 2];
        for _ in 0..1000 {
            let simulation_count =
                simulate_button_presses(&modules, &mut flip_flops_memory, &mut conjuctions_memory);
            result[0] += simulation_count[0];
            result[1] += simulation_count[1];
        }
        (result[0] * result[1]).into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let modules = parse(input);
        let ic = important_connections(&modules);
        for i in 0..1000 {}
        println!("{:?}", ic);
        todo!()
    }
}

fn simulate_button_presses<'a>(
    modules: &HashMap<&'a str, Module<'a>>,
    flip_flop_mem: &mut HashMap<&'a str, FlipFlopState>,
    conjuctions_mem: &mut HashMap<&'a str, HashMap<&'a str, Pulse>>,
) -> [usize; 2] {
    // 0 is for low pulses, 1 if for high pulses
    let mut count = [0_usize; 2];
    let mut queue = VecDeque::new();

    for m in modules.values() {
        match m.module_type {
            ModuleType::FlipFlop => {
                flip_flop_mem.insert(m.source, FlipFlopState::Off);
            }
            ModuleType::Conjuction => {
                let mut map = HashMap::new();
                for om in modules.values() {
                    if om.destinations.contains(&m.source) {
                        map.insert(om.source, Pulse::Low);
                    }
                }
                conjuctions_mem.insert(m.source, map);
            }
            _ => continue,
        }
    }

    let broadcaster = modules.get("broadcaster").unwrap();
    for t in broadcaster.destinations.iter() {
        queue.push_back(("broadcaster", *t, Pulse::Low));
    }
    count[0] += 1;

    while let Some((curr, dest, pulse)) = queue.pop_front() {
        match pulse {
            Pulse::Low => count[0] += 1,
            Pulse::High => count[1] += 1,
        }

        let Some(module) = modules.get(dest) else {
            continue;
        };

        let pulse_type = match module.module_type {
            ModuleType::Normal => continue,
            ModuleType::FlipFlop => {
                let state = flip_flop_mem.get_mut(dest).unwrap();
                match pulse {
                    Pulse::High => continue,
                    Pulse::Low => match state {
                        FlipFlopState::On => {
                            *state = FlipFlopState::Off;
                            Pulse::Low
                        }
                        FlipFlopState::Off => {
                            *state = FlipFlopState::On;
                            Pulse::High
                        }
                    },
                }
            }
            ModuleType::Conjuction => {
                let module_mem = conjuctions_mem.get_mut(dest).unwrap();
                let previous_state = module_mem.get_mut(curr).unwrap();
                *previous_state = pulse;
                if module_mem.values().all(|&p| p == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                }
            }
            _ => continue,
        };

        for futur in module.destinations.iter() {
            queue.push_back((dest, futur, pulse_type));
        }
    }
    count
}

fn important_connections<'a>(modules: &HashMap<&'a str, Module>) -> Vec<&'a str> {
    // the final module 'rx' in my input is targeted by only one conjuction module,
    // which gonna send a Low pulse only if all connected modules Last sent pulse was high
    // this function find all those 'important' modules;
    let mut connected_to_finals = modules
        .iter()
        .find(|(k, v)| v.destinations.contains(&"rx"))
        .map(|(k, v)| k)
        .unwrap();
    modules
        .iter()
        .filter(|(k, v)| v.destinations.contains(connected_to_finals))
        .map(|(&k, v)| k)
        .collect::<Vec<_>>()
}

fn parse(input: &[String]) -> HashMap<&str, Module> {
    let mut modules = HashMap::new();
    for line in input {
        let (source, dest) = line.split_once("->").unwrap();
        let source = source.trim();
        let module_type = ModuleType::from(source);
        let source = match module_type {
            ModuleType::FlipFlop | ModuleType::Conjuction => &source[1..source.len()],
            _ => source,
        };
        let destinations = dest.trim().split(',').map(|d| d.trim()).collect::<Vec<_>>();

        modules.insert(
            source,
            Module {
                source,
                module_type,
                destinations,
            },
        );
    }
    modules
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Pulse {
    Low,
    High,
}

enum FlipFlopState {
    On,
    Off,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Module<'a> {
    source: &'a str,
    destinations: Vec<&'a str>,
    module_type: ModuleType,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum ModuleType {
    // low is true and high is false
    FlipFlop,
    Conjuction,
    Broadcast,
    Normal,
}

impl From<&str> for ModuleType {
    fn from(value: &str) -> Self {
        match value.chars().nth(0).unwrap() {
            'b' => ModuleType::Broadcast,
            '%' => ModuleType::FlipFlop,
            '&' => ModuleType::Conjuction,
            _ => ModuleType::Normal,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::helper_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day20;

    #[test]
    fn test_a_1() {
        let input = input::read_file(&format!(
            "{}day_20_test_1.txt",
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day20.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(32000000), answer);
    }

    #[test]
    fn test_a_2() {
        let input = input::read_file(&format!(
            "{}day_20_test_2.txt",
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day20.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(11687500), answer);
    }

    #[test]
    fn test_b() {
        let input = input::read_file(&format!(
            "{}day_20_b_test.txt",
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day20.part_b(&input);
        assert_eq!(<i64 as Into<Answer>>::into(167409079868000), answer);
    }
}
