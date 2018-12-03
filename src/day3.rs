use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind;

pub fn part1() -> Result<u32, Error> {
    let f = File::open("resources/day3.txt")?;
    let f = BufReader::new(f);
    let mut vec: Vec<(u32, u32, u32, u32)> = Vec::new();
    for line in f.lines() {
        let line_split = line?
            .parse::<String>()
            .map_err(|_err| Error::new(ErrorKind::InvalidData, format!("couldn't parse")))
            .unwrap();

        let line_split = line_split.split(" ").collect::<Vec<&str>>();
        let coordinates = line_split[2][0..line_split[2].len() - 1]
            .split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| {
                x.parse::<u32>()
                    .map_err(|_err| Error::new(ErrorKind::InvalidData, format!("couldn't parse")))
                    .unwrap()
            })
            .collect::<Vec<u32>>();
        let size_values = line_split[3]
            .split("x")
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| {
                x.parse::<u32>()
                    .map_err(|_err| Error::new(ErrorKind::InvalidData, format!("couldn't parse")))
                    .unwrap()
            })
            .collect::<Vec<u32>>();
        vec.push((
            coordinates[0],
            coordinates[1],
            size_values[0],
            size_values[1],
        ))
    }
    // first let's find the max size
    let (mut max_height, mut max_width) = (0, 0);

    for instruction in vec.clone() {
        if instruction.0 + instruction.2 > max_width {
            max_width = instruction.0 + instruction.2;
        }
        if instruction.1 + instruction.3 > max_height {
            max_height = instruction.1 + instruction.3;
        }
    }
    let mut grid = vec![vec![0u32; max_height as usize]; max_width as usize];
    for instruction in vec {
        for x_coord in instruction.0..instruction.0 + instruction.2 {
            for y_coord in instruction.1..instruction.1 + instruction.3 {
                grid[x_coord as usize][y_coord as usize] += 1;
            }
        }
    }
    Ok(grid.iter()
        .flat_map(|row| row.iter().filter(|cell| **cell > 1).map(|_x| 1))
        .sum())
    // Ok(0)
}

pub fn part2() -> Result<u32, Error> {
    let f = File::open("resources/day3.txt")?;
    let f = BufReader::new(f);
    let mut vec: Vec<(u32, u32, u32, u32)> = Vec::new();
    for line in f.lines() {
        let line_split = line?
            .parse::<String>()
            .map_err(|_err| Error::new(ErrorKind::InvalidData, format!("couldn't parse")))
            .unwrap();

        let line_split = line_split.split(" ").collect::<Vec<&str>>();
        let coordinates = line_split[2][0..line_split[2].len() - 1]
            .split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| {
                x.parse::<u32>()
                    .map_err(|_err| Error::new(ErrorKind::InvalidData, format!("couldn't parse")))
                    .unwrap()
            })
            .collect::<Vec<u32>>();
        let size_values = line_split[3]
            .split("x")
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| {
                x.parse::<u32>()
                    .map_err(|_err| Error::new(ErrorKind::InvalidData, format!("couldn't parse")))
                    .unwrap()
            })
            .collect::<Vec<u32>>();
        vec.push((
            coordinates[0],
            coordinates[1],
            size_values[0],
            size_values[1],
        ))
    }
    // first let's find the max size
    let (mut max_height, mut max_width) = (0, 0);
    let mut coord_map: HashMap<(u32, u32), u32> = HashMap::new();
    for (index, instruction) in vec.clone().iter().enumerate() {
        if instruction.0 + instruction.2 > max_width {
            max_width = instruction.0 + instruction.2;
        }
        if instruction.1 + instruction.3 > max_height {
            max_height = instruction.1 + instruction.3;
        }
        coord_map.insert((instruction.0, instruction.1), index as u32 + 1);
    }
    let endpoint = vec.len() as u32 + 1;
    let mut grid = vec![vec![0u32; max_height as usize]; max_width as usize];
    for (index, instruction) in vec.clone().iter().enumerate() {
        for x_coord in instruction.0..instruction.0 + instruction.2 {
            for y_coord in instruction.1..instruction.1 + instruction.3 {
                if grid[x_coord as usize][y_coord as usize] == 0 {
                    grid[x_coord as usize][y_coord as usize] = index as u32 + 1;
                } else {
                    grid[x_coord as usize][y_coord as usize] = endpoint;
                }
            }
        }
    }

    // fuck it replay instructions
    for (index, instruction) in vec.clone().iter().enumerate() {
        let mut invalid = false;
        for x_coord in instruction.0..instruction.0 + instruction.2 {
            for y_coord in instruction.1..instruction.1 + instruction.3 {
                if grid[x_coord as usize][y_coord as usize] == vec.len() as u32 + 1 {
                    invalid = true;
                    break;
                }
            }
            if invalid {
                break;
            }
        }
        if !invalid {
            return Ok(index as u32 + 1);
        }
    }
    Ok(0)
}
