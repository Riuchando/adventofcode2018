use chrono::prelude::*;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind;

use chrono::{Duration, Utc};

pub fn part1() -> Result<u32, Error> {
    let f = File::open("resources/day4.txt")?;
    let f = BufReader::new(f);
    let mut vec: Vec<(DateTime<Utc>, String)> = Vec::new();
    for line in f.lines() {
        let line_split = line?
            .parse::<String>()
            .map_err(|_err| Error::new(ErrorKind::InvalidData, format!("couldn't parse")))
            .unwrap();
        let line_split = line_split.split("] ").collect::<Vec<&str>>();
        let time = Utc
            .datetime_from_str(line_split[0], "[%Y-%m-%d %H:%M")
            .map_err(|_err| {
                Error::new(ErrorKind::InvalidData, format!("couldn't parse datetime"))
            })?;
        vec.push((time, line_split[1].to_owned()))
    }
    vec.sort_unstable_by(|left, right| left.0.timestamp().cmp(&right.0.timestamp()));
    let mut gaurd = 0;
    let mut sleep_start_time: DateTime<Utc> = Utc::now();
    let mut gaurd_sleep_time: HashMap<u32, u32> = HashMap::new();
    for instruction in vec.clone() {
        if instruction.1.contains("Guard") {
            gaurd = instruction
                .1
                .replace("#", "")
                .split(" ")
                .collect::<Vec<&str>>()[1]
                .parse::<u32>()
                .map_err(|_err| {
                    Error::new(
                        ErrorKind::InvalidData,
                        format!("couldn't parse gaurd number"),
                    )
                })?;
        } else if instruction.1.contains("wakes") {
            // println!(
            //     "Gaurd {} was asleep for these times {} {} for this amount of minutes {}",
            //     gaurd,
            //     instruction.0,
            //     sleep_start_time,
            //     (instruction.0 - sleep_start_time).num_minutes() as u32
            // );
            let prev_sleep_time = match gaurd_sleep_time.get(&gaurd) {
                Some(x) => *x,
                None => 0,
            };
            gaurd_sleep_time.insert(
                gaurd,
                (instruction.0 - sleep_start_time).num_minutes() as u32 + prev_sleep_time,
            );
        } else if instruction.1.contains("asleep") {
            sleep_start_time = instruction.0;
        }
    }

    let (mut target_gaurd, mut target_sleepy_time) = (0, 0);
    for (gaurd_id, sleepy_time) in gaurd_sleep_time {
        if target_sleepy_time < sleepy_time {
            target_gaurd = gaurd_id;
            target_sleepy_time = sleepy_time;
        }
        println!("{} {}", gaurd_id, sleepy_time)
    }
    println!("target gaurd {}", target_gaurd);
    let mut gaurd_sleep_schedule: HashMap<u32, u32> = HashMap::new();

    for instruction in vec.clone() {
        if instruction.1.contains("Guard") {
            gaurd = instruction
                .1
                .replace("#", "")
                .split(" ")
                .collect::<Vec<&str>>()[1]
                .parse::<u32>()
                .map_err(|_err| {
                    Error::new(
                        ErrorKind::InvalidData,
                        format!("couldn't parse gaurd number"),
                    )
                })?;
        } else if instruction.1.contains("wakes") && gaurd == target_gaurd {
            println!(
                "target gaurd {} wakes up from {} {}",
                target_gaurd,
                instruction.0.minute(),
                sleep_start_time.minute()
            );
            let mut minute_start = sleep_start_time;
            loop {
                if minute_start == instruction.0 {
                    break;
                }
                let time_slept = match gaurd_sleep_schedule.get(&minute_start.minute()) {
                    Some(x) => x + 1,
                    None => 0,
                };
                gaurd_sleep_schedule.insert(minute_start.minute(), time_slept);
                minute_start = minute_start + Duration::minutes(1);
            }
        } else if instruction.1.contains("asleep") && gaurd == target_gaurd {
            // println!("target gaurd {} goes to sleep", target_gaurd);
            sleep_start_time = instruction.0;
        }
    }
    // let (mut max_count, mut max_time) = (0, 0);
    // for (minute, count) in gaurd_sleep_schedule.clone() {
    //     if count > max_count {
    //         max_count = count;
    //         max_time = minute;
    //     }
    //     println!("{} {}", max_count, max_time);
    // }
    // Ok(target_gaurd * max_time)

    Ok(
        match gaurd_sleep_schedule.iter().max_by(|x, y| x.1.cmp(y.1)) {
            Some(x) => *x.0 as u32,
            None => 0,
        } * target_gaurd,
    )
}

pub fn part2() -> Result<u32, Error> {
    let f = File::open("resources/day4.txt")?;
    let f = BufReader::new(f);
    let mut vec: Vec<(DateTime<Utc>, String)> = Vec::new();
    for line in f.lines() {
        let line_split = line?
            .parse::<String>()
            .map_err(|_err| Error::new(ErrorKind::InvalidData, format!("couldn't parse")))
            .unwrap();
        let line_split = line_split.split("] ").collect::<Vec<&str>>();
        let time = Utc
            .datetime_from_str(line_split[0], "[%Y-%m-%d %H:%M")
            .map_err(|_err| {
                Error::new(ErrorKind::InvalidData, format!("couldn't parse datetime"))
            })?;
        vec.push((time, line_split[1].to_owned()))
    }
    vec.sort_unstable_by(|left, right| left.0.timestamp().cmp(&right.0.timestamp()));
    let mut gaurd = 0;
    let mut sleep_start_time: DateTime<Utc> = Utc::now();
    let mut gaurds_sleep_schedule: HashMap<u32, HashMap<u32, u32>> = HashMap::new();
    for instruction in vec.clone() {
        if instruction.1.contains("Guard") {
            gaurd = instruction
                .1
                .replace("#", "")
                .split(" ")
                .collect::<Vec<&str>>()[1]
                .parse::<u32>()
                .map_err(|_err| {
                    Error::new(
                        ErrorKind::InvalidData,
                        format!("couldn't parse gaurd number"),
                    )
                })?;
        } else if instruction.1.contains("wakes") {
            let mut minute_start = sleep_start_time;
            let mut gaurd_sleep_schedule = match gaurds_sleep_schedule.get(&gaurd) {
                Some(x) => x.clone(),
                None => HashMap::new(),
            };
            loop {
                if minute_start == instruction.0 {
                    break;
                }
                let time_slept = match gaurd_sleep_schedule.get(&minute_start.minute()) {
                    Some(x) => x + 1,
                    None => 0,
                };
                gaurd_sleep_schedule.insert(minute_start.minute(), time_slept);
                minute_start = minute_start + Duration::minutes(1);
            }
            gaurds_sleep_schedule.insert(gaurd, gaurd_sleep_schedule);
        } else if instruction.1.contains("asleep") {
            sleep_start_time = instruction.0;
        }
    }
    // let (mut max_count, mut max_time, mut target_gaurd) = (0, 0, 0);
    // for (gaurd, gaurd_sleep_schedule) in gaurds_sleep_schedule.clone() {
    //     for (minute, count) in gaurd_sleep_schedule {
    //         if count > max_count {
    //             max_count = count;
    //             max_time = minute;
    //             target_gaurd = gaurd;
    //             println!("{} {} {}", gaurd, max_count, max_time);
    //         }
    //     }
    // }
    // println!("iter version {} ", target_gaurd * max_time);

    // Ok(target_gaurd * max_time)
    Ok(match gaurds_sleep_schedule
        .iter()
        .map(|(target_gaurd, gaurd_sleep_schedule)| {
            match gaurd_sleep_schedule.iter().max_by(
                |(left_minute_count, _left_minute), (right_minute_count, _right_minute)| {
                    left_minute_count.cmp(right_minute_count)
                },
            ) {
                Some((minute_count, minute)) => (target_gaurd, *minute_count, *minute),
                None => (target_gaurd, 0, 0),
            }
        })
        .max_by(
            |(_left_gaurd, left_minute_count, _left_minute),
             (_right_gaurd, right_minute_count, _right_minute)| {
                left_minute_count.cmp(&right_minute_count)
            },
        ) {
        Some((gaurd, _minute_count, minute)) => gaurd * minute,
        None => 0,
    })
}
