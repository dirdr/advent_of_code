use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub day: u16,

    #[arg(short, long, default_value_t = 2024)]
    pub year: u16,
}
