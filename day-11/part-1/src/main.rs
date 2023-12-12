use itertools::Itertools;
use std::{collections::HashMap, fmt::Display, fs};

fn get_file_content(file_path: &String) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum TileType {
    Emptyness,
    Galaxy(u32),
}

fn parse_char(char: char, galaxy_count: u32) -> TileType {
    match char {
        '.' => TileType::Emptyness,
        '#' => TileType::Galaxy(galaxy_count + 1),
        _ => Err(format!("Invalid character: {}", char)).expect("Fatal Error"),
    }
}

impl Display for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TileType::Emptyness => write!(f, "."),
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
        assert_eq!(parse_char('.', 12), TileType::Emptyness);
    }

    #[test]
    fn parse_char_02() {
        assert_eq!(parse_char('#', 32), TileType::Galaxy(33));
    }

    #[test]
    #[should_panic]
    fn parse_char_03() {
        parse_char('X', 2);
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
            if let TileType::Galaxy(_) = new_tile {
                tmp_galaxy_count += 1;
            }
            new_tile
        })
        .enumerate()
        .for_each(|(i, p)| {
            hm.insert((i as u32, *height), p);
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
            HashMap::from_iter(vec![
                ((0, 13), TileType::Emptyness),
                ((1, 13), TileType::Emptyness),
                ((2, 13), TileType::Galaxy(4)),
                ((3, 13), TileType::Emptyness),
            ])
        );
        assert_eq!(result, 4);
    }
}

fn get_bounding_rect(map: &Map) -> (u32, u32) {
    let width = map.keys().map(|k| k.0).max().unwrap();
    let height = map.keys().map(|k| k.1).max().unwrap();
    (width, height)
}

/// Used for troubleshooting in expanding the universe
#[allow(dead_code)]
fn display_map(map: &Map) {
    let (width, height) = get_bounding_rect(map);
    for y in 0..=height {
        for x in 0..=width {
            print!("{}", map.get(&(x, y)).unwrap());
        }
        println!();
    }
}

fn expand_height(map: &mut Map, empty_lines: &Vec<u32>) {
    let (width, height) = get_bounding_rect(map);
    for (count, empty_height) in empty_lines.iter().enumerate() {
        for y in ((*empty_height + count as u32)..=(height + count as u32)).rev() {
            for x in 0..=width {
                map.insert((x, y + 1), map.get(&(x, y)).unwrap().clone());
            }
        }
    }
}

#[cfg(test)]
mod tests_expand_height {
    use super::*;

    #[test]
    fn expand_height_01() {
        let mut hm: Map = HashMap::new();
        parse_line(&"..#.", &0, &mut hm, 0);
        parse_line(&"....", &1, &mut hm, 1);
        parse_line(&".#..", &2, &mut hm, 2);
        parse_line(&"..#.", &3, &mut hm, 1);

        expand_height(&mut hm, &vec![1 as u32]);

        display_map(&hm);

        assert_eq!(hm.len(), 20);
    }

    #[test]
    fn expand_height_02() {
        let mut hm: Map = HashMap::new();
        parse_line(&"..#...", &0, &mut hm, 0);
        parse_line(&"......", &1, &mut hm, 1);
        parse_line(&".#....", &2, &mut hm, 1);
        parse_line(&"......", &3, &mut hm, 2);
        parse_line(&"....#.", &4, &mut hm, 2);
        parse_line(&"#.....", &5, &mut hm, 3);

        expand_height(&mut hm, &vec![1_u32, 3_u32]);

        display_map(&hm);

        assert_eq!(hm.len(), 48);
    }
}

fn expand_width(map: &mut Map, empty_columns: &Vec<u32>) {
    let (width, height) = get_bounding_rect(map);
    for (count, empty_width) in empty_columns.iter().enumerate() {
        for x in ((*empty_width + count as u32)..=(width + count as u32)).rev() {
            for y in 0..=height {
                map.insert((x + 1, y), map.get(&(x, y)).unwrap().clone());
            }
        }
    }
}

#[cfg(test)]
mod tests_expand_width {
    use super::*;

    #[test]
    fn expand_width_01() {
        let mut hm: Map = HashMap::new();
        parse_line(&"#...", &0, &mut hm, 0);
        parse_line(&"...#", &1, &mut hm, 1);
        parse_line(&".#..", &2, &mut hm, 2);
        parse_line(&"...#", &3, &mut hm, 3);

        display_map(&hm);

        expand_width(&mut hm, &vec![2_u32]);

        display_map(&hm);

        assert_eq!(hm.len(), 20);
    }

    #[test]
    fn expand_width_02() {
        let mut hm: Map = HashMap::new();

        parse_line(&"#....#", &0, &mut hm, 0);
        parse_line(&"......", &1, &mut hm, 2);
        parse_line(&"..#...", &2, &mut hm, 2);
        parse_line(&"......", &3, &mut hm, 3);
        parse_line(&"....#.", &4, &mut hm, 3);
        parse_line(&"....#.", &5, &mut hm, 4);

        display_map(&hm);

        expand_width(&mut hm, &vec![1_u32, 3_u32]);

        display_map(&hm);

        assert_eq!(hm.len(), 48);
    }
}

fn calc_path((x1, y1): &(u32, u32), (x2, y2): &(u32, u32)) -> u32 {
    return ((*y2 as i32 - *y1 as i32).abs() + (*x2 as i32 - *x1 as i32).abs()) as u32;
}

#[cfg(test)]
mod tests_calc_path {
    use super::*;

    #[test]
    fn calc_path_01() {
        assert_eq!(calc_path(&(0_u32, 0_u32), &(1_u32, 1_u32)), 2);
    }

    #[test]
    fn calc_path_02() {
        assert_eq!(calc_path(&(1_u32, 1_u32), &(0_u32, 0_u32)), 2);
    }

    #[test]
    fn calc_path_03() {
        assert_eq!(calc_path(&(1_u32, 6_u32), &(5_u32, 11_u32)), 9);
    }

    #[test]
    fn calc_path_04() {
        assert_eq!(calc_path(&(5_u32, 11_u32), &(1_u32, 6_u32)), 9);
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
                match map.get(&(x, y)).unwrap() {
                    TileType::Galaxy(_) => {
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
                match map.get(&(x, y)).unwrap() {
                    TileType::Galaxy(_) => {
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

    // Time to expand !
    // (it should be associative, though I'm not 100% confident ^^')
    expand_width(&mut map, &empty_columns);
    expand_height(&mut map, &empty_lines);

    // Now that we've expanded, we can register galaxies with their updated
    // coordinates for path computing
    let mut galaxies: Vec<(u32, u32)> = vec![];
    let br = get_bounding_rect(&map);
    for y in 0..=br.1 {
        for x in 0..=br.0 {
            match map.get(&(x, y)).unwrap() {
                TileType::Galaxy(_) => {
                    galaxies.push((x, y));
                }
                _ => {}
            }
        }
    }

    println!(
        "Path length after expansion: {:?}",
        galaxies.iter().combinations(2).fold(0, |acc, v| acc
            + calc_path(&(v[0].0, v[0].1), &(v[1].0, v[1].1)))
    );
}
