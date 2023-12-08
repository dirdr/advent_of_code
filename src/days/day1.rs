use crate::helper_lib;
use crate::helper_lib::input;

pub fn run() -> anyhow::Result<()> {
    let lines = input::read_file(&format!("{}day_1.txt", helper_lib::FILES_PREFIX))?;
    part_a(&lines)?;
    part_b(&lines)?;
    Ok(())
}

pub fn part_a(lines: &[String]) -> anyhow::Result<()> {
    let result: u32 = lines
        .iter()
        .map(|l| {
            let mut digits = l.chars().filter_map(|c| c.to_digit(10));
            let first = digits.next().unwrap();
            let last = digits.last().unwrap_or(first);
            first * 10 + last
        })
        .sum();
    println!("{}", result);
    Ok(())
}

fn part_b(lines: &[String]) -> anyhow::Result<()> {
    let result: u32 = lines
        .iter()
        .map(|l| {
            let digits = get_digits(&l);
            digits[0] * 10 + digits[1]
        })
        .sum();
    println!("{}", result);
    Ok(())
}

fn get_digits(i: &str) -> [u32; 2] {
    let helper = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut first = None;
    let mut last = 0;

    let mut digit = |c| {
        first = first.or(Some(c));
        last = c;
    };

    let chars = i.as_bytes();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        if c.is_ascii_digit() {
            digit((c - b'0') as u32);
        } else {
            for (j, d) in helper.iter().enumerate() {
                if chars[i..].starts_with(d.as_bytes()) {
                    digit(j as u32 + 1);
                }
            }
        }
        i += 1;
    }

    [first.unwrap(), last]
}
