use std::collections::{HashMap, VecDeque};

use crate::helper_lib::{answer::Answer, solution::Solution};

pub struct Day20;

impl Solution for Day20 {
    fn part_a(&self, input: &[String]) -> Answer {
        let modules = parse(input);
        const NUMBER_OF_PRESS: usize = 1000;
        let mut result = [0, NUMBER_OF_PRESS - 1];
        simulate_button_presses(&modules, |i, _curr, _dest, pulse| {
            match pulse {
                Pulse::High => result[0] += 1,
                Pulse::Low => result[1] += 1,
            }
            i < NUMBER_OF_PRESS
        });
        (result[0] * result[1]).into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let modules = parse(input);
        let ic = important_connections(&modules);
        let mut last_cycle = [0_usize; 4];
        let mut cycle_len = [0_usize; 4];
        simulate_button_presses(&modules, |i, curr, dest, pulse| {
            if dest == "nr" && pulse == Pulse::High {
                if let Some(j) = ic.iter().position(|s| s == &curr) {
                    let last = last_cycle[j];
                    last_cycle[j] = i;
                    cycle_len[j] = i - last + 1;
                    if cycle_len.iter().all(|&i| i != 0) {
                        return false;
                    }
                }
            }
            true
        });
        cycle_len.iter().product::<usize>().into()
    }
}

fn simulate_button_presses<'a>(
    modules: &HashMap<&'a str, Module<'a>>,
    mut hook: impl FnMut(usize, &str, &str, Pulse) -> bool,
) {
    let mut queue = VecDeque::new();

    let mut flip_flop_mem = HashMap::new();
    let mut conjuctions_mem = HashMap::new();

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

    for i in 0.. {
        queue.clear();
        let broadcaster = modules.get("broadcaster").unwrap();
        for t in broadcaster.destinations.iter() {
            queue.push_back(("broadcaster", *t, Pulse::Low));
        }

        while let Some((curr, dest, pulse)) = queue.pop_front() {
            if !hook(i, curr, dest, pulse) {
                return;
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
    }
}

fn important_connections<'a>(modules: &HashMap<&'a str, Module>) -> Vec<&'a str> {
    // the final module 'rx' in my input is targeted by only one conjuction module,
    // which gonna send a Low pulse only if all connected modules Last sent pulse was high
    // this function find all those 'important' modules;
    let connected_to_finals = modules
        .iter()
        .find(|(_, v)| v.destinations.contains(&"rx"))
        .map(|(k, _)| k)
        .unwrap();
    modules
        .iter()
        .filter(|(_, v)| v.destinations.contains(connected_to_finals))
        .map(|(&k, _)| k)
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
        assert_eq!(<i64 as Into<Answer>>::into(228282646835717), answer);
    }
}
