use itertools::Itertools;
use std::{collections::HashMap, fmt::Display, fs};

fn get_file_content(file_path: &String) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum TileType {
    Galaxy(u32),
}

fn parse_char(char: char, galaxy_count: u32) -> Option<TileType> {
    match char {
        '#' => Some(TileType::Galaxy(galaxy_count + 1)),
        _ => None,
    }
}

impl Display for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Galaxy count goes wayyyy over 10 (even over 100…)
            TileType::Galaxy(_) => write!(f, "#"),
        }
    }
}

#[cfg(test)]
mod tests_parse_char {
    use super::*;

    #[test]
    fn parse_char_01() {
        assert_eq!(parse_char('#', 32), Some(TileType::Galaxy(33)));
    }

    #[test]
    fn parse_char_02() {
        assert_eq!(parse_char('X', 2), None);
    }
}

type Coordinates = (u32, u32);
type Map = HashMap<Coordinates, TileType>;

/// Parse a given line at given height into a given Map
///
/// Return the count of new galaxies added to update the general counter
fn parse_line(line: &str, height: &u32, hm: &mut Map, galaxy_count: u32) -> u32 {
    let mut tmp_galaxy_count = galaxy_count;
    line.chars()
        .map(|c| {
            let new_tile = parse_char(c, tmp_galaxy_count);
            if let Some(TileType::Galaxy(_)) = new_tile {
                tmp_galaxy_count += 1;
            }
            new_tile
        })
        .enumerate()
        .for_each(|(i, p)| match p {
            Some(thing) => {
                hm.insert((i as u32, *height), thing);
            }
            _ => {}
        });
    return tmp_galaxy_count;
}

#[cfg(test)]
mod tests_parse_line {
    use super::*;

    #[test]
    fn tests_parse_line_01() {
        let mut hm: Map = HashMap::new();
        let result = parse_line(&"..#.", &13, &mut hm, 3);

        assert_eq!(
            hm,
            HashMap::from_iter(vec![((2, 13), TileType::Galaxy(4)),])
        );
        assert_eq!(result, 4);
    }
}

fn get_bounding_rect(map: &Map) -> (u32, u32) {
    let width = map.keys().map(|k| k.0).max().unwrap();
    let height = map.keys().map(|k| k.1).max().unwrap();
    (width, height)
}

/// Used for troubleshooting
#[allow(dead_code)]
fn display_map(map: &Map) {
    let (width, height) = get_bounding_rect(map);
    for y in 0..=height {
        for x in 0..=width {
            print!(
                "{}",
                match map.get(&(x, y)) {
                    Some(g) => format!("{}", g),
                    None => format!("."),
                }
            );
        }
        println!();
    }
}

fn expand_height(map: &mut Map, empty_lines: &Vec<u32>, count_expanse: u32) {
    for (coords, value) in map.clone().iter() {
        match empty_lines
            .iter()
            .enumerate()
            .filter(|(_, e)| **e < coords.1)
            .last()
        {
            Some((count, _)) => {
                let (x, y) = coords;
                map.insert((*x, *y + count_expanse * (count as u32 + 1)), value.clone());
                map.remove(&(*x, *y));
            }
            None => {}
        };
    }
}

#[cfg(test)]
mod tests_expand_height {
    use super::*;

    #[test]
    fn expand_height_01() {
        let input = "#...\n....\n.#..\n...#\n";

        let mut hm: Map = {
            let mut tmp = HashMap::new();
            let mut galaxy_count: u32 = 0;
            for (index, line) in input.lines().enumerate() {
                galaxy_count = parse_line(&line, &(index as u32), &mut tmp, galaxy_count);
            }
            tmp
        };

        display_map(&hm);
        dbg!(&hm);

        expand_height(&mut hm, &vec![1 as u32], 2);

        display_map(&hm);

        dbg!(&hm);
        assert_eq!(hm.len(), 3);
    }

    #[test]
    fn expand_height_02() {
        let input = "..#...\n......\n.#....\n.#....\n......\n.....\n....#.\n.....#\n";

        let mut hm: Map = {
            let mut tmp = HashMap::new();
            let mut galaxy_count: u32 = 0;
            for (index, line) in input.lines().enumerate() {
                galaxy_count = parse_line(&line, &(index as u32), &mut tmp, galaxy_count);
            }
            tmp
        };

        dbg!(&hm);

        expand_height(&mut hm, &vec![1_u32, 4_u32, 5_u32], 100);

        dbg!(&hm);

        assert_eq!(hm.len(), 4);
    }
}

fn expand_width(map: &mut Map, empty_columns: &Vec<u32>, count_expanse: u32) {
    for (coords, value) in map.clone().iter() {
        match empty_columns
            .iter()
            .enumerate()
            .filter(|(_, e)| **e < coords.0)
            .last()
        {
            Some((count, _)) => {
                let (x, y) = coords;
                map.insert((*x + count_expanse * (count as u32 + 1), *y), value.clone());
                map.remove(&(*x, *y));
            }
            None => {}
        };
    }
}

fn calc_path((x1, y1): &(u64, u64), (x2, y2): &(u64, u64)) -> u64 {
    return ((*y2 as i64 - *y1 as i64).abs() + (*x2 as i64 - *x1 as i64).abs()) as u64;
}

#[cfg(test)]
mod tests_calc_path {
    use super::*;

    #[test]
    fn calc_path_01() {
        assert_eq!(calc_path(&(0_u64, 0_u64), &(1_u64, 1_u64)), 2);
    }

    #[test]
    fn calc_path_02() {
        assert_eq!(calc_path(&(1_u64, 1_u64), &(0_u64, 0_u64)), 2);
    }

    #[test]
    fn calc_path_03() {
        assert_eq!(calc_path(&(1_u64, 6_u64), &(5_u64, 11_u64)), 9);
    }

    #[test]
    fn calc_path_04() {
        assert_eq!(calc_path(&(5_u64, 11_u64), &(1_u64, 6_u64)), 9);
    }
}

fn main() {
    let content = get_file_content(&String::from("assets/input"));

    let mut map: Map = {
        let mut tmp = HashMap::new();
        let mut galaxy_count: u32 = 0;
        for (index, line) in content.lines().enumerate() {
            galaxy_count = parse_line(&line, &(index as u32), &mut tmp, galaxy_count);
        }
        tmp
    };

    let empty_lines: Vec<u32> = {
        let mut tmp = vec![];
        let br = get_bounding_rect(&map);
        // Horizontal scan, for drawing and empty lines registry
        for y in 0..=br.1 {
            let mut to_add = true;
            for x in 0..=br.0 {
                match map.get(&(x, y)) {
                    Some(TileType::Galaxy(_)) => {
                        to_add = false;
                        // No need to keep going, here
                        break;
                    }
                    _ => {}
                }
            }
            if to_add {
                tmp.push(y);
            }
        }
        tmp
    };

    let empty_columns: Vec<u32> = {
        let mut tmp = vec![];
        let br = get_bounding_rect(&map);
        // Vertical scan, this time for empty lines registry only
        for x in 0..=br.0 {
            let mut to_add = true;
            for y in 0..=br.1 {
                match map.get(&(x, y)) {
                    Some(TileType::Galaxy(_)) => {
                        to_add = false;
                        // No need to keep going, here
                        break;
                    }
                    _ => {}
                }
            }
            if to_add {
                tmp.push(x);
            }
        }
        tmp
    };

    println!("Empty columns: {} {:?}", empty_columns.len(), empty_columns);
    println!("Empty lines: {} {:?}", empty_lines.len(), empty_lines);
    println!("Galaxy count: {}", map.len());

    // Every empty line/column need to be replaced with
    // `count_expanse` + THE ORIGINAL LINE !!!
    // Hence no 1_000_000 :'D
    let count_expanse: u32 = 999_999;

    dbg!(map.iter().find(|(_, v)| **v == TileType::Galaxy(200)));

    // Time to expand !
    println!("Expanding universe width…");
    expand_width(&mut map, &empty_columns, count_expanse);

    println!("Expanding universe height…");
    expand_height(&mut map, &empty_lines, count_expanse);

    println!("Done expanding, pfiou !");

    println!("Galaxy count: {}", map.len());

    dbg!(map.iter().find(|(_, v)| **v == TileType::Galaxy(200)));

    println!(
        "Galaxies path length sum: {:?}",
        map.iter()
            .map(|(coords, _)| (coords.0 as u64, coords.1 as u64))
            .combinations(2)
            .fold(0 as u64, |acc, v| acc
                + calc_path(&(v[0].0, v[0].1), &(v[1].0, v[1].1)))
    );
}
