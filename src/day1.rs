use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind;

pub fn part1() -> Result<i32, Error> {
    let f = File::open("resources/day1.txt")?;
    let f = BufReader::new(f);
    let mut number = 0;
    for line in f.lines() {
        number += line?
            .parse::<i32>()
            .map_err(|_err| Error::new(ErrorKind::InvalidData, format!("couldn't parse")))?;
    }
    Ok(number)
}

pub fn part2() -> Result<i32, Error> {
    let f = File::open("resources/day1.txt")?;
    let f = BufReader::new(f);
    let mut frequency = 0;

    let mut vec: Vec<i32> = Vec::new();
    let mut hash_set: HashSet<i32> = HashSet::new();
    // technically the first number is 0
    hash_set.insert(0);
    for line in f.lines() {
        let new_number = line?
            .parse::<i32>()
            .map_err(|_err| Error::new(ErrorKind::InvalidData, format!("couldn't parse")))?;
        frequency += new_number;
        // copy numbers we have read into a buffer and use that to cycle, due to bufferReader.cycle being silly
        vec.push(new_number);
        if hash_set.contains(&frequency) {
            return Ok(frequency);
        }
        hash_set.insert(frequency);
    }

    for repeat_number in vec.into_iter().cycle() {
        frequency += repeat_number;
        if hash_set.contains(&frequency) {
            return Ok(frequency);
        }
        hash_set.insert(frequency);
    }
    // unreachable code
    Ok(0)
}
