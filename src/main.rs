mod day1;
mod day2;

use clap::{Arg, Parser};
use std::io::Error;

#[derive(Parser, Debug)]
struct Args {
    #[arg(index(1))]
    day: u8,
}

fn main() -> color_eyre::Result<()> {
    let args = Args::parse();

    match &args.day {
        1 => day1::task::run(),
        2 => day2::task::run(),
        _ => panic!("Gimme a day"),
    }
}
