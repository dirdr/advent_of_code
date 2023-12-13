use crate::helper_lib;
use crate::helper_lib::input;

pub fn run() -> anyhow::Result<()> {
    let lines = input::read_file(&format!("{}day_4.txt", helper_lib::FILES_PREFIX))?;
    part_a(&lines)?;
    part_b(&lines)?;
    Ok(())
}

pub fn part_a(lines: &[String]) -> anyhow::Result<()> {
    let mut sum = 0;
    for line in lines {
        let line = line.split_once(":").unwrap().1.trim();
        let (winning, pick) = line.split_once("|").unwrap();
        let winning = convert_to_nums(winning);
        let pick = convert_to_nums(pick);
        sum += 2_i32.pow((pick.iter().filter(|num| winning.contains(num)).count() - 1) as u32);
    }
    println!("{}", sum);
    Ok(())
}

fn convert_to_nums(str_set: &str) -> Vec<usize> {
    let mut nums = vec![];
    for token in str_set.split_whitespace() {
        nums.push(token.parse::<usize>().unwrap())
    }
    nums
}

fn part_b(lines: &[String]) -> anyhow::Result<()> {
    // we start with one copy of each of the cards,
    // cards are 0 indexed : 1st card : copies[0], ...
    let mut copies: Vec<usize> = vec![1; lines.len()];
    for (i, &ref line) in lines.iter().enumerate() {
        for _ in 0..copies[i] {
            let line = line.split_once(":").unwrap().1.trim();
            let (winning, pick) = line.split_once("|").unwrap();
            let winning = convert_to_nums(winning);
            let pick = convert_to_nums(pick);
            let winning_numbers = pick.iter().filter(|num| winning.contains(num)).count();
            for k in i..(i + winning_numbers) {
                copies[k + 1] += 1;
            }
        }
    }
    let sum = copies.iter().sum::<usize>();
    println!("{}", sum);
    Ok(())
}
