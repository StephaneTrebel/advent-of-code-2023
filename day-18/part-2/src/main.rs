use regex::Regex;
use std::{fmt::Display, fs};

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<&str> for Direction {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "0" => Ok(Direction::Right),
            "1" => Ok(Direction::Down),
            "2" => Ok(Direction::Left),
            "3" => Ok(Direction::Up),
            _ => Err("Invalid character"),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "{}", "Up"),
            Direction::Down => write!(f, "{}", "Down"),
            Direction::Left => write!(f, "{}", "Left"),
            Direction::Right => write!(f, "{}", "Right"),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Coords {
    // i32 to account for inexistent, but requested nonetheless, negative values
    x: i64,
    y: i64,
}

impl Display for Coords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Map {
    perimeter: Vec<(i64, i64)>,
}

fn get_values_from_line(line: &str) -> (Direction, usize) {
    let capture = Regex::new(r"\(#([0-9a-f]+)([0-9])\)")
        .unwrap()
        .captures(&line)
        .unwrap();
    (
        Direction::try_from(capture.get(2).unwrap().as_str()).unwrap(),
        i64::from_str_radix(capture.get(1).unwrap().as_str(), 16).unwrap() as usize,
    )
}

#[cfg(test)]
mod tests_get_values_from_line {
    use super::*;

    #[test]
    fn get_values_from_line_tests() {
        assert_eq!(
            get_values_from_line(&"X 9 (#70c710)"),
            (Direction::Right, 461937)
        );
        assert_eq!(
            get_values_from_line(&"X 9 (#0dc571)"),
            (Direction::Down, 56407)
        );
        assert_eq!(
            get_values_from_line(&"X 9 (#5713f0)"),
            (Direction::Right, 356671)
        );
        assert_eq!(
            get_values_from_line(&"X 9 (#d2c081)"),
            (Direction::Down, 863240)
        );
        assert_eq!(
            get_values_from_line(&"X 9 (#59c680)"),
            (Direction::Right, 367720)
        );
        assert_eq!(
            get_values_from_line(&"X 9 (#411b91)"),
            (Direction::Down, 266681)
        );
        assert_eq!(
            get_values_from_line(&"X 9 (#8ceee2)"),
            (Direction::Left, 577262)
        );
        assert_eq!(
            get_values_from_line(&"X 9 (#caa173)"),
            (Direction::Up, 829975)
        );
        assert_eq!(
            get_values_from_line(&"X 9 (#1b58a2)"),
            (Direction::Left, 112010)
        );
        assert_eq!(
            get_values_from_line(&"X 9 (#caa171)"),
            (Direction::Down, 829975)
        );
        assert_eq!(
            get_values_from_line(&"X 9 (#7807d2)"),
            (Direction::Left, 491645)
        );
        assert_eq!(
            get_values_from_line(&"X 9 (#a77fa3)"),
            (Direction::Up, 686074)
        );
        assert_eq!(
            get_values_from_line(&"X 9 (#015232)"),
            (Direction::Left, 5411)
        );
        assert_eq!(
            get_values_from_line(&"X 9 (#7a21e3)"),
            (Direction::Up, 500254)
        );
    }
}

fn build_map(lines: &str) -> Map {
    let mut perimeter_vec: Vec<(i64, i64)> = vec![];
    let mut current_point = Coords { x: 5, y: 5 };

    perimeter_vec.push((current_point.x as i64, current_point.y as i64));
    for line in lines.lines() {
        let (direction, count) = get_values_from_line(&line);
        let mut tmp_point = current_point;
        match direction {
            Direction::Up => {
                tmp_point.y -= count as i64;
                perimeter_vec.push((tmp_point.x as i64, tmp_point.y as i64));
            }
            Direction::Down => {
                tmp_point.y += count as i64;
                perimeter_vec.push((tmp_point.x as i64, tmp_point.y as i64));
            }
            Direction::Left => {
                tmp_point.x -= count as i64;
                perimeter_vec.push((tmp_point.x as i64, tmp_point.y as i64));
            }
            Direction::Right => {
                tmp_point.x += count as i64;
                perimeter_vec.push((tmp_point.x as i64, tmp_point.y as i64));
            }
        };
        current_point = tmp_point;
    }

    Map {
        perimeter: perimeter_vec.clone(),
    }
}

trait Fill {
    fn fill(&mut self) -> u64;
}

impl Fill for Map {
    fn fill(&mut self) -> u64 {
        // dbg!(&self.perimeter);
        let tmp_perimeter = self.perimeter[1..].to_vec();

        let length = self
            .perimeter
            .iter()
            .zip(tmp_perimeter.iter())
            .fold(0. as u32, |acc, ((x1, y1), (x2, y2))| {
                acc + (x2 - x1).abs() as u32 + (y2 - y1).abs() as u32
            })
            / 2;

        dbg!(length);

        let area: i64 = self.perimeter.iter().zip(tmp_perimeter.iter()).fold(
            0. as i64,
            |acc, ((x1, y1), (x2, y2))| {
                acc + (*x1 as i64 * *y2 as i64) as i64 - (*x2 as i64 * *y1 as i64) as i64
            },
        ) / 2;

        dbg!(area);

        area.abs() as u64 + length as u64 + 1
    }
}

#[cfg(test)]
mod tests_fill {
    use super::*;

    #[test]
    fn fill_01() {
        let mut map = build_map(
            &"\
R 5 (#000020)
D 3 (#000021)
L 3 (#000022)
U 5 (#000023)",
        );
        assert_eq!(map.fill(), 9);
    }

    #[test]
    fn fill_02() {
        let mut map = build_map(
            &"\
D 2 (#000021)
R 2 (#000020)
U 2 (#000023)
R 2 (#000020)
U 2 (#000023)
L 2 (#000022)
D 2 (#000021)
L 2 (#000022)",
        );
        assert_eq!(map.fill(), 17);
    }

    #[test]
    fn fill_03() {
        let mut map = build_map(
            &"\
    R 6 (#70c710)
    D 5 (#0dc571)
    L 2 (#5713f0)
    D 2 (#d2c081)
    R 2 (#59c680)
    D 2 (#411b91)
    L 5 (#8ceee2)
    U 2 (#caa173)
    L 1 (#1b58a2)
    U 2 (#caa171)
    R 2 (#7807d2)
    U 3 (#a77fa3)
    L 2 (#015232)
    U 2 (#7a21e3)",
        );
        assert_eq!(map.fill(), 952408144115);
    }
}

fn main() {
    let content = get_file_content("assets/input");

    println!("Building map…");
    let mut map: Map = build_map(&content);
    println!("Done !");

    println!("Filling map…");
    let count = map.fill();
    println!("Done !");

    println!("Result: {:?}", count);
}
