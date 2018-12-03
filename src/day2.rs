use itertools::Itertools;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind;

pub fn part1() -> Result<u32, Error> {
    let f = File::open("resources/day2.txt")?;
    let f = BufReader::new(f);

    let mut twos_counter = 0;
    let mut threes_counter = 0;
    for line in f.lines() {
        let char_vec: Vec<char> = line?.chars().collect();
        let mut chars_map: HashMap<char, u32> = HashMap::new();

        for character in char_vec {
            let value = chars_map.get(&character).cloned();
            chars_map.insert(
                character,
                match value {
                    Some(x) => x + 1,
                    None => 1,
                },
            );
        }
        for (_key, value) in &chars_map {
            if *value == 2 {
                twos_counter += 1;
                break;
            }
        }
        for (_key, value) in &chars_map {
            if *value == 3 {
                threes_counter += 1;
                break;
            }
        }
    }
    Ok(twos_counter * threes_counter)
}

pub fn part2() -> Result<String, Error> {
    let f = File::open("resources/day2.txt")?;
    let f = BufReader::new(f);
    let mut vec = Vec::new();
    for line in f.lines() {
        vec.push(line?
            .parse::<String>()
            .map_err(|_err| Error::new(ErrorKind::InvalidData, format!("couldn't parse")))?);
    }
    for (index, item) in vec.iter().enumerate() {
        for second_item in vec.iter().skip(index + 1) {
            let string_similarity = item.chars()
                .zip(second_item.chars())
                .map(|(a, b)| if a == b { 0 } else { 1 })
                .fold(0, |acc, curr| acc + curr);
            // This is our stringy BOIII
            if string_similarity == 1 {
                let string = item.chars()
                    .zip(second_item.chars())
                    .filter(|(a, b)| a == b)
                    .map(|(a, _b)| a)
                    .collect();
                return Ok(string);
            }
        }
    }
    Ok(String::new())
}

pub fn part2_functional() -> Result<String, Error> {
    let f = File::open("resources/day2.txt")?;
    let f = BufReader::new(f);
    let mut vec = Vec::new();
    for line in f.lines() {
        vec.push(line?
            .parse::<String>()
            .map_err(|_err| Error::new(ErrorKind::InvalidData, format!("couldn't parse")))?);
    }

    Ok(vec.iter()
        .cartesian_product(vec.iter())
        .fold(String::new(), |acc, (left, right)| {
            if !acc.is_empty() {
                acc
            } else if left != right {
                let string_similarity = left.chars()
                    .zip(right.chars())
                    .map(|(a, b)| if a == b { 0 } else { 1 })
                    .fold(0, |acc, curr| acc + curr);
                if string_similarity == 1 {
                    let string = left.chars()
                        .zip(right.chars())
                        .filter(|(a, b)| a == b)
                        .map(|(a, _b)| a)
                        .collect();
                    string
                } else {
                    acc
                }
            } else {
                acc
            }
        }))
}
