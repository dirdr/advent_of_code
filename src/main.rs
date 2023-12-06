mod args;
mod days;
mod helper_lib;

use clap::Parser;
use days::{day1, day2, day3};

#[macro_use]
extern crate anyhow;

macro_rules! dispatch_days {
    ($day:expr, $($day_num:expr => $day_mod:ident),*) => {{
        let result: anyhow::Result<()> = match $day {
            $(
                $day_num => $day_mod::run(),
            )*
            _ => Err(anyhow!("Day not implemented")),
        };
        result
    }};
}

fn main() -> anyhow::Result<()> {
    let args = args::Arguments::parse();
    let day = args.day;
    dispatch_days!(day, 1 => day1, 2 => day2, 3 => day3)?;
    Ok(())
}
