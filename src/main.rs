// #[macro_use]
extern crate itertools;
use std::env;
use std::io::Error;

mod day1;
mod day2;
mod day3;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    match args[1].as_ref() {
        "day1" => {
            println!("{}", day1::part1()?);
            println!("{}", day1::part2()?);
        }
        "day2" => {
            println!("{}", day2::part1()?);
            println!("{}", day2::part2()?);
            println!("{}", day2::part2_functional()?);
        }
        "day3" => {
            println!("{}", day3::part1()?);
            println!("{}", day3::part2()?);
        }

        _ => println!("{}", "invalid argument"),
    }

    Ok(())
}
