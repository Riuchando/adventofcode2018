use chrono::prelude::*;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind;

pub fn part1_wavefront() -> Result<u32, Error> {
    let f = File::open("resources/day6.txt")?;
    let f = BufReader::new(f);
    let mut coordinates: Vec<(u32, u32)> = Vec::new();
    for line in f.lines() {
        let line_split = line?
            .parse::<String>()
            .map_err(|_err| Error::new(ErrorKind::InvalidData, format!("couldn't parse")))
            .unwrap()
            .split(", ")
            .map(|x| {
                x.parse::<u32>()
                    .map_err(|_err| Error::new(ErrorKind::InvalidData, format!("couldn't parse")))
                    .unwrap()
            })
            .collect::<Vec<u32>>();
        coordinates.push((line_split[0], line_split[1]));
    }
    let x_max = match coordinates.clone().iter().map(|x| x.0).max() {
        Some(x) => x + 1,
        None => 0,
    };
    let y_max = match coordinates.clone().iter().map(|x| x.1).max() {
        Some(x) => x + 1,
        None => 0,
    };
    let mut grid = vec![vec![(0u32, 0u32); y_max as usize]; x_max as usize];
    let mut ring = 0;
    let mut index_sizes: HashMap<u32, u32> = HashMap::new();
    let mut hash_set: HashSet<u32> = HashSet::new();
    let mut d: VecDeque<(usize, usize, u32, u32)> = VecDeque::new();
    for (index, coordinate) in coordinates.clone().iter().enumerate() {
        // println!("{} {} {:?}", x_max, y_max, coordinate);
        grid[coordinate.0 as usize][coordinate.1 as usize] = (index as u32 + 1, ring);
        if coordinate.0 as usize + 1 < x_max as usize {
            d.push_back((
                coordinate.0 as usize + 1,
                coordinate.1 as usize,
                index as u32,
                0,
            ));
        }
        if coordinate.1 as usize + 1 < y_max as usize {
            d.push_back((
                coordinate.0 as usize,
                coordinate.1 as usize + 1,
                index as u32,
                0,
            ));
        }
        if coordinate.0 as usize >= 1 {
            d.push_back((
                coordinate.0 as usize - 1,
                coordinate.1 as usize,
                index as u32,
                0,
            ));
        }
        if coordinate.1 as usize >= 1 {
            d.push_back((
                coordinate.0 as usize,
                coordinate.1 as usize - 1,
                index as u32,
                0,
            ));
        }
    }

    // println!("{} {}", x_max, y_max);
    let mut visited_spots: HashSet<(u32, u32)> = HashSet::new();
    loop {
        if d.is_empty() {
            break;
        }
        let coordinate = d.pop_front().unwrap();
        visited_spots.insert((coordinate.0 as u32, coordinate.1 as u32));
        let index = coordinate.2;
        let ring = coordinate.3 + 1;
        // println!("{} {} {} {}", x_max, y_max, ring, d.len());
        if grid[coordinate.0 as usize][coordinate.1 as usize].1 > ring
            || grid[coordinate.0 as usize][coordinate.1 as usize].0 == 0
        {
            grid[coordinate.0 as usize][coordinate.1 as usize] = (index as u32 + 1, ring);
        } else if (grid[coordinate.0 as usize][coordinate.1 as usize].1 >= ring)
            && grid[coordinate.0 as usize][coordinate.1 as usize].0 != index + 1
        {
            // println!(
            //     "{} {:?} coordinate {:?}",
            //     index + 1,
            //     grid[coordinate.0 as usize][coordinate.1 as usize],
            //     coordinate
            // );
            grid[coordinate.0 as usize][coordinate.1 as usize] = (999, 0);
        }

        if coordinate.0 as usize + 1 < x_max as usize
            && (grid[coordinate.0 as usize + 1][coordinate.1 as usize].0 == 0
                || grid[coordinate.0 as usize + 1][coordinate.1 as usize].1 == ring)
        // && !visited_spots.contains(&(coordinate.0 as u32 + 1, coordinate.1 as u32))
        {
            d.push_back((
                coordinate.0 as usize + 1,
                coordinate.1 as usize,
                index as u32,
                ring,
            ));
            // visited_spots.insert((coordinate.0 as u32 + 1, coordinate.1 as u32));
        }
        if coordinate.1 as usize + 1 < y_max as usize
            && (grid[coordinate.0 as usize][coordinate.1 as usize + 1].0 == 0
                || grid[coordinate.0 as usize][coordinate.1 as usize + 1].1 == ring)
        // && !visited_spots.contains(&(coordinate.0 as u32, coordinate.1 as u32 + 1))
        {
            d.push_back((
                coordinate.0 as usize,
                coordinate.1 as usize + 1,
                index as u32,
                ring,
            ));
            // visited_spots.insert((coordinate.0 as u32, coordinate.1 as u32 + 1));
        }
        if coordinate.0 as u32 >= 1
            && (grid[coordinate.0 as usize - 1][coordinate.1 as usize].0 == 0
                || grid[coordinate.0 as usize - 1][coordinate.1 as usize].1 == ring)
        // && !visited_spots.contains(&(coordinate.0 as u32 - 1, coordinate.1 as u32))
        {
            d.push_back((
                coordinate.0 as usize - 1,
                coordinate.1 as usize,
                index as u32,
                ring,
            ));
            // visited_spots.insert((coordinate.0 as u32 - 1, coordinate.1 as u32));
        }
        if coordinate.1 as u32 >= 1
            && (grid[coordinate.0 as usize][coordinate.1 as usize - 1].0 == 0
                || grid[coordinate.0 as usize][coordinate.1 as usize - 1].1 == ring)
        {
            d.push_back((
                coordinate.0 as usize,
                coordinate.1 as usize - 1,
                index as u32,
                ring,
            ));
            // visited_spots.insert((coordinate.0 as u32, coordinate.1 as u32 - 1));
        }
    }
    for i in 0..y_max {
        hash_set.insert(grid[0][i as usize].0 as u32);
    }
    for i in 0..y_max {
        hash_set.insert(grid[(x_max - 1) as usize][i as usize].0 as u32);
    }
    for i in 0..x_max {
        hash_set.insert(grid[i as usize][0].0 as u32);
    }
    for i in 0..x_max {
        hash_set.insert(grid[i as usize][(y_max - 1) as usize].0 as u32);
    }
    // println!("{:?} {:?}", grid, hash_set);
    for i in 0..x_max {
        for j in 0..y_max {
            let data = grid[i as usize][j as usize].0;
            if hash_set.contains(&data) {
                continue;
            }
            let prev_value = match index_sizes.get(&data) {
                Some(x) => *x,
                None => 0,
            };
            index_sizes.insert(data, prev_value + 1);
        }
    }
    Ok(match index_sizes.iter().max_by(|(k, v), (a, b)| v.cmp(a)) {
        Some(x) => *x.1,
        None => 0,
    })
}
pub fn part1() -> Result<u32, Error> {
    let f = File::open("resources/day6.txt")?;
    let f = BufReader::new(f);
    let mut coordinates: Vec<(u32, u32)> = Vec::new();
    for line in f.lines() {
        let line_split = line?
            .parse::<String>()
            .map_err(|_err| Error::new(ErrorKind::InvalidData, format!("couldn't parse")))
            .unwrap()
            .split(", ")
            .map(|x| {
                x.parse::<u32>()
                    .map_err(|_err| Error::new(ErrorKind::InvalidData, format!("couldn't parse")))
                    .unwrap()
            })
            .collect::<Vec<u32>>();
        coordinates.push((line_split[0], line_split[1]));
    }
    let x_max = match coordinates.clone().iter().map(|x| x.0).max() {
        Some(x) => x + 1,
        None => 0,
    };
    let y_max = match coordinates.clone().iter().map(|x| x.1).max() {
        Some(x) => x + 1,
        None => 0,
    };
    let mut grid = vec![vec![0u32; y_max as usize]; x_max as usize];
    let mut sizes: HashMap<usize, usize> = HashMap::with_capacity(coordinates.len());
    (0..x_max).cartesian_product(0..y_max).foreach(|(x, y)| {
        let mut distances: Vec<(u32, usize)> = coordinates
            .iter()
            .enumerate()
            .map(|(i, (c_x, c_y))| (abs_diff(*c_x, x) + abs_diff(*c_y, y), i))
            .collect();
        distances.sort_unstable();
        if distances[0].0 != distances[1].0 {
            sizes
                .entry(distances[0].1)
                .and_modify(|count| *count += 1)
                .or_insert(1);
            grid[x as usize][y as usize] = distances[0].1 as u32;
        }
    });
    // Iterate along edges. Anything on an edge will go on infinitely
    for i in 0..y_max {
        sizes.remove(&(grid[0][i as usize] as usize));
        sizes.remove(&(grid[(x_max - 1) as usize][i as usize] as usize));
    }
    for i in 0..x_max {
        sizes.remove(&(grid[i as usize][0] as usize));
        sizes.remove(&(grid[i as usize][(y_max - 1) as usize] as usize));
    }

    let mut sizes: Vec<(&usize, &usize)> = sizes.iter().map(|(x, y)| (y, x)).collect();
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    Ok(*sizes[0].0 as u32)
}
pub fn part2() -> Result<u32, Error> {
    let f = File::open("resources/day6.txt")?;
    let f = BufReader::new(f);
    let mut coordinates: Vec<(u32, u32)> = Vec::new();
    for line in f.lines() {
        let line_split = line?
            .parse::<String>()
            .map_err(|_err| Error::new(ErrorKind::InvalidData, format!("couldn't parse")))
            .unwrap()
            .split(", ")
            .map(|x| {
                x.parse::<u32>()
                    .map_err(|_err| Error::new(ErrorKind::InvalidData, format!("couldn't parse")))
                    .unwrap()
            })
            .collect::<Vec<u32>>();
        coordinates.push((line_split[0], line_split[1]));
    }
    let x_max = match coordinates.clone().iter().map(|x| x.0).max() {
        Some(x) => x + 1,
        None => 0,
    };
    let y_max = match coordinates.clone().iter().map(|x| x.1).max() {
        Some(x) => x + 1,
        None => 0,
    };
    Ok((0..x_max)
        .cartesian_product(0..y_max)
        .map(|(x, y)| {
            coordinates
                .iter()
                .map(|(c_x, c_y)| abs_diff(*c_x, x) + abs_diff(*c_y, y))
                .sum()
        })
        .filter(|x: &u32| *x < 10000)
        .count() as u32)
}

fn abs_diff(a: u32, b: u32) -> u32 {
    if a > b {
        a - b
    } else {
        b - a
    }
}
