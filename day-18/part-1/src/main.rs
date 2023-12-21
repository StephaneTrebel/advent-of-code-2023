use geo::{point, Contains, Polygon};
use std::{collections::HashMap, fmt::Display, fs};

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum TileType {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Inside,
}

impl Display for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TileType::NorthSouth => write!(f, "│"),
            TileType::EastWest => write!(f, "─"),
            TileType::NorthEast => write!(f, "└"),
            TileType::NorthWest => write!(f, "┘"),
            TileType::SouthWest => write!(f, "┐"),
            TileType::SouthEast => write!(f, "┌"),
            TileType::Ground => write!(f, "."),
            TileType::Inside => write!(f, "#"),
        }
    }
}

fn get_tile_type(previous: &Direction, next: &Direction) -> TileType {
    match (previous, next) {
        (Direction::Up, Direction::Up) | (Direction::Down, Direction::Down) => TileType::NorthSouth,
        (Direction::Up, Direction::Left) | (Direction::Right, Direction::Down) => {
            TileType::SouthWest
        }
        (Direction::Up, Direction::Right) | (Direction::Left, Direction::Down) => {
            TileType::SouthEast
        }
        (Direction::Down, Direction::Left) | (Direction::Right, Direction::Up) => {
            TileType::NorthWest
        }
        (Direction::Down, Direction::Right) | (Direction::Left, Direction::Up) => {
            TileType::NorthEast
        }
        (Direction::Left, Direction::Left) | (Direction::Right, Direction::Right) => {
            TileType::EastWest
        }
        _ => panic!("Invalid directions"),
    }
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
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct Coords {
    // i32 to account for inexistent, but requested nonetheless, negative values
    x: i32,
    y: i32,
}

impl Display for Coords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct BoundingRectangle {
    origin: Coords,
    width: usize,
    height: usize,
}

fn get_bounding_rect(map: &Map) -> BoundingRectangle {
    BoundingRectangle {
        origin: Coords {
            x: map.content.keys().map(|k| k.x).min().unwrap(),
            y: map.content.keys().map(|k| k.y).min().unwrap(),
        },
        width: map.content.keys().map(|k| k.x).max().unwrap() as usize,
        height: map.content.keys().map(|k| k.y).max().unwrap() as usize,
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Map {
    content: HashMap<Coords, TileType>,
    perimeter: Vec<(f32, f32)>,
    polygon: Polygon<f32>,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let br = get_bounding_rect(&self);
        for y in br.origin.y..=br.height as i32 {
            for x in br.origin.x..=br.width as i32 {
                write!(f, "{}", {
                    let coords = Coords { x, y };
                    let tmp = self.content.get(&coords);
                    match tmp {
                        None => " ".to_string(),
                        Some(c) => c.to_string(),
                    }
                })
                .expect("Oula");
            }
            write!(f, "\n").expect("Oula");
        }
        Ok(())
    }
}

fn build_map(lines: &str) -> Map {
    let mut content: HashMap<Coords, TileType> = HashMap::new();
    let mut perimeter_vec: Vec<(f32, f32)> = vec![];

    let starting_point = Coords { x: 0, y: 0 };
    let mut current_point = starting_point.clone();
    // Initialization does not matter, since the first tile will be
    // erased at the end to «close off» the shape
    let mut previous = Direction::Down;
    let mut first = true;
    let mut first_direction: Direction = Direction::Down;

    for line in lines.lines() {
        let mut split = line.split_whitespace();
        let direction = Direction::try_from(split.next().unwrap_or("")).unwrap();
        if first {
            first_direction = direction.clone();
            first = false;
        }
        let count = split.next().unwrap().parse::<usize>().unwrap();
        match direction {
            Direction::Up => {
                let mut tmp_point = current_point.clone();
                for y in (current_point.y - count as i32..=current_point.y).rev() {
                    tmp_point.y = y;
                    content.insert(tmp_point.clone(), get_tile_type(&previous, &direction));
                    perimeter_vec.push((tmp_point.x as f32, tmp_point.y as f32));
                    previous = direction.clone();
                }
                current_point = tmp_point;
            }
            Direction::Down => {
                let mut tmp_point = current_point.clone();
                for y in current_point.y..=current_point.y + count as i32 {
                    tmp_point.y = y;
                    content.insert(tmp_point.clone(), get_tile_type(&previous, &direction));
                    perimeter_vec.push((tmp_point.x as f32, tmp_point.y as f32));
                    previous = direction.clone();
                }
                current_point = tmp_point;
            }
            Direction::Left => {
                let mut tmp_point = current_point.clone();
                for x in (current_point.x - count as i32..=current_point.x).rev() {
                    tmp_point.x = x;
                    content.insert(tmp_point.clone(), get_tile_type(&previous, &direction));
                    perimeter_vec.push((tmp_point.x as f32, tmp_point.y as f32));
                    previous = direction.clone();
                }
                current_point = tmp_point;
            }
            Direction::Right => {
                let mut tmp_point = current_point.clone();
                for x in current_point.x..=current_point.x + count as i32 {
                    tmp_point.x = x;
                    content.insert(tmp_point.clone(), get_tile_type(&previous, &direction));
                    perimeter_vec.push((tmp_point.x as f32, tmp_point.y as f32));
                    previous = direction.clone();
                }
                current_point = tmp_point;
            }
        };
    }

    // Closing off the shape with the first and last direction
    content.insert(starting_point, get_tile_type(&previous, &first_direction));

    Map {
        content,
        perimeter: perimeter_vec.clone(),
        polygon: Polygon::new(perimeter_vec.clone().into(), vec![]),
    }
}

#[cfg(test)]
mod tests_build_map {
    use super::*;

    #[test]
    fn build_map_01() {
        let map = build_map(
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
        println!("{}", map);
        assert_eq!(
            map.to_string(),
            "\
┌─────┐
│     │
└─┐   │
  │   │
  │   │
┌─┘ ┌─┘
│   │  
└┐  └─┐
 │    │
 └────┘
"
        );
    }

    #[test]
    fn build_map_02() {
        let map = build_map(
            &"\
    D 5 (#AAAAAA)
    R 5 (#AAAAAA)
    R 5 (#AAAAAA)
    U 5 (#AAAAAA)
    L 4 (#AAAAAA)
    D 3 (#AAAAAA)
    L 3 (#AAAAAA)
    U 3 (#AAAAAA)
    L 3 (#AAAAAA)",
        );
        println!("{}", map);
        assert_eq!(
            map.to_string(),
            "\
┌──┐  ┌───┐
│  │  │   │
│  │  │   │
│  └──┘   │
│         │
└─────────┘
"
        );
    }
}

trait Fill {
    fn fill(&mut self);
}

impl Fill for Map {
    fn fill(&mut self) {
        let br = get_bounding_rect(&self);
        let mut is_inside = false;

        for y in br.origin.y..=br.height as i32 {
            for x in br.origin.x..=br.width as i32 {
                let coords = (x as f32, y as f32);
                let tile = self.content.get(&Coords { x, y });

                // Tests triggered when on the boundary
                if self.perimeter.contains(&coords) {
                    match tile {
                        Some(TileType::NorthSouth) => {
                            is_inside = !is_inside;
                        }

                        Some(TileType::NorthEast) => {
                            // In the case of NE we have to check
                            // all the following chars to ensure that we are not
                            // in a "pseudo NS". Indeed └-----┐ is
                            // the same as | for our search purposes
                            let mut inner_x = x + 1;
                            while self.content.get(&Coords { x: inner_x, y })
                                == Some(&TileType::EastWest)
                            {
                                inner_x += 1;
                            }
                            // Coming out of this we have to cases:
                            // └-----┐ -> We have a pseudo "|" and we should flip is_inside
                            // or
                            // └-----┘ -> We have a pseudo "-" and we should NOT flip is_inside
                            // Any other case we have a consistency error
                            match self.content.get(&Coords { x: inner_x, y }) {
                                Some(TileType::SouthWest) => {
                                    is_inside = !is_inside;
                                }
                                Some(TileType::NorthWest) => {}
                                other => panic!("NE Consistency error: {:?}", other),
                            }
                        }

                        Some(TileType::SouthEast) => {
                            // In the case of SE we have to check
                            // all the following chars to ensure that we are not
                            // in a "pseudo NS". Indeed ┌-----┘ is
                            // the same as | for our search purposes
                            let mut inner_x = x + 1;
                            while self.content.get(&Coords { x: inner_x, y })
                                == Some(&TileType::EastWest)
                            {
                                inner_x += 1;
                            }
                            // Coming out of this we have to cases:
                            // ┌-----┘ -> We have a pseudo "|" and we should flip is_inside
                            // or
                            // ┌-----┐ -> We have a pseudo "-" and we should NOT flip is_inside
                            // Any other case we have a consistency error
                            match self.content.get(&Coords { x: inner_x, y }) {
                                Some(TileType::SouthWest) => {}
                                Some(TileType::NorthWest) => {
                                    is_inside = !is_inside;
                                }
                                a => panic!("SE Consistency error: {:?}", a),
                            }
                        }
                        _ => {}
                    }
                }

                // Tests triggered when inside the boundary
                if self.polygon.contains(&point! {x: x as f32, y: y as f32}) {
                    if is_inside {
                        self.content.insert(Coords { x, y }, TileType::Inside);
                    }
                } else if let None = tile {
                    self.content.insert(Coords { x, y }, TileType::Ground);
                }
            }
            is_inside = false;
        }
    }
}

#[cfg(test)]
mod tests_fill {
    use super::*;

    #[test]
    fn fill_01() {
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
        println!("{}", map);
        map.fill();
        println!("{}", map);
        assert_eq!(
            map.to_string(),
            "\
┌─────┐
│#####│
└─┐###│
..│###│
..│###│
┌─┘#┌─┘
│###│..
└┐##└─┐
.│####│
.└────┘
"
        );
    }

    #[test]
    fn fill_02() {
        let mut map = build_map(
            &"\
D 4 (#AAAAAA)
R 1 (#AAAAAA)
D 1 (#AAAAAA)
R 4 (#AAAAAA)
R 5 (#AAAAAA)
U 5 (#AAAAAA)
L 4 (#AAAAAA)
D 3 (#AAAAAA)
L 3 (#AAAAAA)
U 3 (#AAAAAA)
L 3 (#AAAAAA)",
        );
        println!("{}", map);
        map.fill();
        println!("{}", map);
        assert_eq!(
            map.to_string(),
            "\
┌──┐..┌───┐
│##│..│###│
│##│..│###│
│##└──┘###│
└┐########│
.└────────┘
"
        );
    }
}

fn count_filled_tiles(map: &Map) -> usize {
    map.content
        .iter()
        .map(|t| match *t.1 {
            TileType::Ground => 0,
            _ => 1,
        })
        .sum()
}

#[cfg(test)]
mod tests_count_filled_tiles {
    use super::*;

    #[test]
    fn count_filled_tiles_01() {
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
        println!("{}", map);
        map.fill();
        println!("{}", map);
        assert_eq!(count_filled_tiles(&map), 62);
    }
}

fn main() {
    let content = get_file_content("assets/input");

    let mut map: Map = build_map(&content);
    map.fill();

    println!("{}", map);

    println!("Result: {:?}", count_filled_tiles(&map));
}
