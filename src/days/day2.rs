use std::{cmp::max, collections::HashMap};

use crate::helper_lib::{self, input};

pub fn run() -> anyhow::Result<()> {
    let lines = input::read_file(&format!("{}day_2.txt", helper_lib::FILES_PREFIX))?;
    part_a(&lines)?;
    part_b(&lines)?;
    Ok(())
}

pub fn part_a(lines: &[String]) -> anyhow::Result<()> {
    let mut sum = 0;
    let colors = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    for line in lines {
        if let Some(iter) = line.strip_prefix("Game ") {
            let (game_id_str, colors_str) = iter
                .split_once(':')
                .ok_or_else(|| anyhow::anyhow!("Invalid line format"))?;
            let game_id = game_id_str.parse::<i32>()?;

            let valid_game = colors_str.trim().split(';').all(|set| {
                set.split(',').all(|token| {
                    let tokens = token.trim().split_whitespace().collect::<Vec<&str>>();
                    let val = tokens[0].parse::<i32>().unwrap_or(0);
                    *colors.get(tokens[1]).unwrap_or(&i32::MAX) >= val
                })
            });
            if valid_game {
                sum += game_id;
            }
        }
    }

    println!("{}", sum);
    Ok(())
}

pub fn part_b(lines: &[String]) -> anyhow::Result<()> {
    let mut sum = 0;
    for line in lines {
        let mut colors = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
        if let Some(color_data) = line.strip_prefix("Game ").and_then(|s| s.split_once(':')) {
            for set in color_data.1.trim().split(';') {
                for token in set.split(',') {
                    let tokens = token.trim().split_whitespace().collect::<Vec<&str>>();
                    let val = tokens[0].parse::<i32>()?;
                    colors.entry(tokens[1]).and_modify(|e| *e = max(val, *e));
                }
            }
        }
        sum += colors.values().product::<i32>();
    }
    println!("{}", sum);
    Ok(())
}
