use chrono::prelude::*;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind;

pub fn part1() -> Result<u32, Error> {
    let f = File::open("resources/day5.txt")?;
    let f = BufReader::new(f);
    let mut left_vec: Vec<char> = Vec::new();
    for line in f.lines() {
        let line_split = line?
            .parse::<String>()
            .map_err(|_err| Error::new(ErrorKind::InvalidData, format!("couldn't parse")))
            .unwrap();
        for character in line_split.chars() {
            if left_vec.is_empty() {
                left_vec.push(character);
            } else if (left_vec.last().unwrap().is_ascii_uppercase()
                && character.is_ascii_lowercase()
                && character.to_ascii_uppercase() == *left_vec.last().unwrap())
                || (left_vec.last().unwrap().is_ascii_lowercase() && character.is_ascii_uppercase()
                    && character.to_ascii_lowercase() == *left_vec.last().unwrap())
            {
                left_vec.pop();
            } else {
                left_vec.push(character);
            }
        }
    }
    Ok(left_vec.len() as u32)
}

pub fn part2() -> Result<u32, Error> {
    let f = File::open("resources/day5.txt")?;
    let f = BufReader::new(f);
    let mut char_count: HashMap<char, u32> = HashMap::new();
    for line in f.lines() {
        let line_split = line?
            .parse::<String>()
            .map_err(|_err| Error::new(ErrorKind::InvalidData, format!("couldn't parse")))
            .unwrap();
        for ascii_int in 97..122 {
            let mut left_vec: Vec<char> = Vec::new();
            for character in line_split.chars() {
                if left_vec.is_empty()
                    && !(ascii_int as u8 as char == character.to_ascii_uppercase()
                        || ascii_int as u8 as char == character.to_ascii_lowercase())
                {
                    left_vec.push(character);
                } else if (ascii_int as u8 as char == character.to_ascii_uppercase()
                    || ascii_int as u8 as char == character.to_ascii_lowercase())
                {
                    continue;
                } else if (match left_vec.last() {
                    Some(x) => {
                        (x.is_ascii_uppercase() && character.is_ascii_lowercase()
                            && character.to_ascii_uppercase() == *x
                            || (left_vec.last().unwrap().is_ascii_lowercase()
                                && character.is_ascii_uppercase()
                                && character.to_ascii_lowercase() == *left_vec.last().unwrap()))
                    }
                    None => false,
                }) {
                    left_vec.pop();
                } else {
                    left_vec.push(character);
                }
            }
            // println!("{} {:?}", ascii_int as u8 as char, left_vec);
            char_count.insert(ascii_int as u8 as char, left_vec.len() as u32);
        }
    }
    let mut min_value = 999999999;
    for (character, value) in char_count {
        if value < min_value {
            min_value = value
        }
        // println!("{} {}", character, value)
    }
    Ok(min_value)
}
