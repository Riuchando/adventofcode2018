// #[macro_use]
// #![feature(test)]
extern crate chrono;
extern crate itertools;
use std::env;
use std::io::Error;

extern crate petgraph;
// extern crate test;

use std::collections::HashSet;
use std::hash::Hash;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

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
            println!("{}", day3::part2_with_mathy_stuff()?);
        }
        "day4" => {
            println!("{}", day4::part1()?);
            println!("{}", day4::part2()?);
        }
        "day5" => {
            println!("{}", day5::part1()?);
            println!("{}", day5::part2()?);
        }
        "day6" => {
            println!("{}", day6::part1()?);
            println!("{}", day6::part2()?);
        }
        "day7" => {
            println!("{}", day7::part1()?);
            println!("{}", day7::part2()?);
        }
        _ => println!("{}", "invalid argument"),
    }

    Ok(())
}
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use test::Bencher;
//     #[bench]
//     fn bench_day5_part1(b: &mut Bencher) {
//         b.iter(|| day5::part1());
//     }
//     #[bench]
//     fn bench_day5_part2(b: &mut Bencher) {
//         b.iter(|| day5::part2());
//     }
// }
