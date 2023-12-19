use colored::Colorize;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Tile {
    content: char,
    energized: usize,
}

type Coords = (usize, usize);
type Map = HashMap<Coords, Tile>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct Ray {
    starting_point: Coords,
    direction: Direction,
}

fn parse_line(map: &mut Map, line: &str, y: usize) {
    for (x, c) in line.chars().enumerate() {
        map.insert(
            (x, y),
            Tile {
                content: c,
                energized: 0,
            },
        );
    }
}

fn parse_map(lines: &str) -> Map {
    let mut map: Map = HashMap::new();
    for (y, line) in lines.lines().enumerate() {
        parse_line(&mut map, &line.replace(" ", ""), y);
    }
    map
}

fn get_bounding_rect(map: &Map) -> (usize, usize) {
    let width = map.keys().map(|k| k.0).max().unwrap();
    let height = map.keys().map(|k| k.1).max().unwrap();
    (width, height)
}

fn count_energized(map: &Map) -> usize {
    map.iter().map(|t| t.1.energized.clamp(0, 1)).sum()
}

fn display_map(map: &Map) {
    let br = get_bounding_rect(&map);
    dbg!(br);
    for y in 0..=br.1 {
        for x in 0..=br.0 {
            print!("{}", {
                let tmp = map.get(&(x, y)).unwrap();
                match tmp.energized {
                    0 => tmp.content.to_string().blue(),
                    1 => tmp.content.to_string().bright_yellow().bold(),
                    _ => tmp.content.to_string().to_string().bright_yellow().bold(),
                }
            })
        }
        println!("");
    }
}

fn update_tile(map: &mut Map, direction: &Direction, coords: &Coords) -> (Tile, Vec<Ray>) {
    let tile = map.get_mut(coords).unwrap();

    tile.energized = 1;

    let new_rays: Vec<Ray> = match tile.content {
        '.' => vec![],
        '-' => match direction {
            Direction::Up | Direction::Down => vec![
                Ray {
                    starting_point: *coords,
                    direction: Direction::Left,
                },
                Ray {
                    starting_point: *coords,
                    direction: Direction::Right,
                },
            ],
            _ => vec![],
        },
        '|' => match direction {
            Direction::Left | Direction::Right => vec![
                Ray {
                    starting_point: *coords,
                    direction: Direction::Up,
                },
                Ray {
                    starting_point: *coords,
                    direction: Direction::Down,
                },
            ],
            _ => vec![],
        },
        '/' => match direction {
            Direction::Up => vec![Ray {
                starting_point: *coords,
                direction: Direction::Right,
            }],
            Direction::Down => vec![Ray {
                starting_point: *coords,
                direction: Direction::Left,
            }],
            Direction::Left => vec![Ray {
                starting_point: *coords,
                direction: Direction::Down,
            }],
            Direction::Right => vec![Ray {
                starting_point: *coords,
                direction: Direction::Up,
            }],
        },
        '\\' => match direction {
            Direction::Up => vec![Ray {
                starting_point: *coords,
                direction: Direction::Left,
            }],
            Direction::Down => vec![Ray {
                starting_point: *coords,
                direction: Direction::Right,
            }],
            Direction::Left => vec![Ray {
                starting_point: *coords,
                direction: Direction::Up,
            }],
            Direction::Right => vec![Ray {
                starting_point: *coords,
                direction: Direction::Down,
            }],
        },
        _ => panic!("Invalid tile"),
    };
    (tile.clone(), new_rays)
}

fn cast_ray(map: &mut Map, starting_ray: Ray) {
    let br = get_bounding_rect(map);
    let mut orders: Vec<Ray> = vec![starting_ray];
    let mut done_rays: HashSet<Ray> = HashSet::new();

    while let Some(ray) = orders.pop() {
        if let Some(_) = done_rays.get(&ray) {
            continue;
        } else {
            done_rays.insert(ray.clone());
        }
        map.get_mut(&ray.starting_point).unwrap().energized += 1;
        match ray.direction {
            Direction::Up => {
                let mut y = ray.starting_point.1 as i32 - 1;
                while y >= 0 {
                    let (tile, new_orders) =
                        update_tile(map, &ray.direction, &(ray.starting_point.0, y as usize));
                    if new_orders.len() > 0 || "/\\-".contains(tile.content) {
                        for order in new_orders {
                            orders.push(order);
                        }
                        break;
                    }
                    y -= 1;
                }
            }
            Direction::Down => {
                let mut y = ray.starting_point.1 + 1;
                while y <= br.1 {
                    let (tile, new_orders) =
                        update_tile(map, &ray.direction, &(ray.starting_point.0, y as usize));
                    if new_orders.len() > 0 || "/\\-".contains(tile.content) {
                        for order in new_orders {
                            orders.push(order);
                        }
                        break;
                    }
                    y += 1;
                }
            }
            Direction::Left => {
                let mut x = (ray.starting_point.0 as i32) - 1;
                while x >= 0 {
                    let (tile, new_orders) =
                        update_tile(map, &ray.direction, &(x as usize, ray.starting_point.1));
                    if new_orders.len() > 0 || "/\\|".contains(tile.content) {
                        for order in new_orders {
                            orders.push(order);
                        }
                        break;
                    }
                    x -= 1;
                }
            }
            Direction::Right => {
                let mut x = ray.starting_point.0 + 1;
                while x <= br.0 {
                    let (tile, new_orders) =
                        update_tile(map, &ray.direction, &(x as usize, ray.starting_point.1));
                    if new_orders.len() > 0 || "/\\|".contains(tile.content) {
                        for order in new_orders {
                            orders.push(order);
                        }
                        break;
                    }
                    x += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests_cast_ray {
    use super::*;

    #[test]
    fn cast_ray_right() {
        let mut map: Map = parse_map(&".-..");
        cast_ray(
            &mut map,
            Ray {
                starting_point: (0, 0),
                direction: Direction::Right,
            },
        );
        assert_eq!(count_energized(&map), 4);
    }

    #[test]
    fn cast_ray_left() {
        let mut map: Map = parse_map(&"..-.");
        cast_ray(
            &mut map,
            Ray {
                starting_point: (3, 0),
                direction: Direction::Left,
            },
        );
        assert_eq!(count_energized(&map), 4);
    }

    #[test]
    fn cast_ray_up() {
        let mut map: Map = parse_map(
            &".
.
|
.",
        );
        cast_ray(
            &mut map,
            Ray {
                starting_point: (0, 3),
                direction: Direction::Up,
            },
        );
        assert_eq!(count_energized(&map), 4);
    }

    #[test]
    fn cast_ray_down() {
        let mut map: Map = parse_map(
            &".
|
.
.",
        );
        cast_ray(
            &mut map,
            Ray {
                starting_point: (0, 0),
                direction: Direction::Down,
            },
        );
        assert_eq!(count_energized(&map), 4);
    }

    #[test]
    fn cast_ray_integration_test_00() {
        let mut map: Map = parse_map(
            &"\\...\\.............
.............|/...
....\\......-.....|
|.....-....\\.|....
............../.|.
.-.-...|....-.-...
..........\\.....|.
...../............
......\\......\\....
.....|./..........
...../...../......
..\\...............
....|.........-.|.
.........-........
.............|....
................./",
        );
        cast_ray(
            &mut map,
            Ray {
                starting_point: (0, 0),
                direction: Direction::Down,
            },
        );
        display_map(&map);
        assert_eq!(count_energized(&map), 16);
    }

    #[test]
    fn cast_ray_integration_test_01() {
        let mut map: Map = parse_map(
            &"\\........-.........\\................................|...
......-/.............|-.../.....|...........././..\\.....
-.........................|.....\\...................|.\\.
.......-........../.......\\.........|..../........-.-|..",
        );
        cast_ray(
            &mut map,
            Ray {
                starting_point: (0, 0),
                direction: Direction::Down,
            },
        );
        display_map(&map);
        assert_eq!(count_energized(&map), 89);
    }
}

fn main() {
    let content = get_file_content("assets/input");

    let map: Map = parse_map(&content);

    // display_map(&map);

    println!("");
    println!("Energizing…");

    let mut energized_values: Vec<usize> = vec![];
    let br = get_bounding_rect(&map);

    println!("Direction::Right");
    for y in 0..=br.1 {
        let mut tmp_map = map.clone();
        cast_ray(
            &mut tmp_map,
            Ray {
                starting_point: (0, y),
                direction: Direction::Right,
            },
        );
        energized_values.push(count_energized(&tmp_map));
    }

    println!("Direction::Up");
    for x in 0..=br.0 {
        let mut tmp_map = map.clone();
        cast_ray(
            &mut tmp_map,
            Ray {
                starting_point: (x, br.1),
                direction: Direction::Up,
            },
        );
        energized_values.push(count_energized(&tmp_map));
    }

    println!("Direction::Down");
    for x in 0..=br.0 {
        let mut tmp_map = map.clone();
        cast_ray(
            &mut tmp_map,
            Ray {
                starting_point: (x, 0),
                direction: Direction::Down,
            },
        );
        energized_values.push(count_energized(&tmp_map));
    }

    println!("Direction::Left");
    for y in 0..=br.1 {
        let mut tmp_map = map.clone();
        cast_ray(
            &mut tmp_map,
            Ray {
                starting_point: (br.0, y),
                direction: Direction::Left,
            },
        );
        energized_values.push(count_energized(&tmp_map));
    }

    println!("Done casting rays !");
    println!("");

    display_map(&map);

    println!("Result: {:?}", energized_values.iter().max());
}
