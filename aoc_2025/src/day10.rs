use aoc_lib::{answer::Answer, solution::Solution};
use good_lp::{Expression, Solution as LpSolution, coin_cbc};
use good_lp::{SolverModel, constraint, variable, variables};

pub struct Day10;

impl Solution for Day10 {
    fn part_a(&self, input: &[String]) -> Answer {
        let machines = parse(input);
        let mut total = 0;
        for machine in &machines {
            total += machine.fewest_lights_presses();
        }
        total.into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let machines = parse(input);
        let mut total = 0;
        for machine in &machines {
            total += machine.fewest_joltage_presses();
        }
        total.into()
    }
}

fn parse(input: &[String]) -> Vec<Machine> {
    let mut machines = vec![];
    for line in input {
        machines.push(Machine::from_input(line));
    }
    machines
}

struct Machine {
    lights: u16,
    buttons: Vec<[u8; 16]>,
    joltages: [u16; 16],
}

impl Machine {
    fn fewest_lights_presses(&self) -> usize {
        fn backtrack(buttons: &[u16], curr: u16, target: u16, idx: usize, press: usize) -> usize {
            if curr == target {
                return press;
            }

            if idx >= buttons.len() {
                return usize::MAX;
            }

            let take = backtrack(buttons, curr ^ buttons[idx], target, idx + 1, press + 1);
            let not_take = backtrack(buttons, curr, target, idx + 1, press);

            take.min(not_take)
        }

        let buttons = self
            .buttons
            .iter()
            .map(|&b| {
                u16::from_str_radix(
                    &b.iter()
                        .map(|&d| if d == 1 { '1' } else { '0' })
                        .collect::<String>(),
                    2,
                )
                .unwrap()
            })
            .collect::<Vec<u16>>();

        backtrack(&buttons, 0, self.lights, 0, 0)
    }

    fn fewest_joltage_presses(&self) -> usize {
        let mut vars = variables!();
        let xs: Vec<_> = self
            .buttons
            .iter()
            .map(|_| vars.add(variable().integer().min(0)))
            .collect();

        let objective: Expression = xs.iter().fold(0.into(), |sum: Expression, x| sum + *x);
        let mut model = vars.minimise(objective).using(coin_cbc);

        for i in 0..16 {
            let mut expr: Expression = 0.into();
            for (j, btn) in self.buttons.iter().enumerate() {
                expr += (btn[i] as f64) * xs[j];
            }
            model = model.with(constraint!(expr == self.joltages[i] as f64));
        }

        let solution = model.solve().unwrap();
        xs.iter().map(|&x| solution.value(x) as usize).sum()
    }

    fn from_input(input: &str) -> Self {
        let mut frags = input.split_whitespace().peekable();

        let lights = frags.next().unwrap();
        let mut lights = lights[1..lights.len() - 1]
            .chars()
            .map(|e| match e {
                '.' => '0',
                '#' => '1',
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();
        while lights.len() != 16 {
            lights.push('0');
        }
        let lights = lights.iter().collect::<String>();
        let lights = u16::from_str_radix(&lights, 2).unwrap();

        let mut buttons = vec![];
        while let Some(peek) = frags.peek() {
            if !peek.contains('(') {
                break;
            }
            let b = frags.next().unwrap();
            let mut mask = [0; 16];
            for idx in b[1..b.len() - 1].split(',') {
                let idx = idx.parse::<usize>().unwrap();
                mask[idx] = 1;
            }
            buttons.push(mask);
        }

        let joltages_str = frags.next().unwrap();
        let mut joltages = [0; 16];
        for (i, idx) in joltages_str[1..joltages_str.len() - 1]
            .split(',')
            .enumerate()
        {
            let joltage = idx.parse::<u16>().unwrap();
            joltages[i] = joltage;
        }

        Self {
            lights,
            buttons,
            joltages,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Day10;
    use aoc_lib::{answer::Answer, input, solution::Solution};

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_10_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        assert_eq!(<i64 as Into<Answer>>::into(7), Day10.part_a(&input));
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_10_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        assert_eq!(<i64 as Into<Answer>>::into(33), Day10.part_b(&input));
    }
}
