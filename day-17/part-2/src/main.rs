use std::{collections::HashMap, fmt::Display, fs};

use colored::Colorize;
use pathfinding::prelude::dijkstra;

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

type Map = HashMap<Coords, usize>;

fn parse_line(map: &mut Map, line: &str, y: i32) {
    for (x, c) in line.chars().enumerate() {
        map.insert(
            Coords {
                x: x as i32,
                y: y as i32,
            },
            c.to_digit(10).unwrap() as usize,
        );
    }
}

fn parse_map(lines: &str) -> Map {
    let mut map: Map = HashMap::new();
    for (y, line) in lines.lines().enumerate() {
        parse_line(&mut map, &line.replace(" ", ""), y as i32);
    }
    map
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct BoundingRectangle {
    width: usize,
    height: usize,
}

fn get_bounding_rect(map: &Map) -> BoundingRectangle {
    BoundingRectangle {
        width: map.keys().map(|k| k.x).max().unwrap() as usize,
        height: map.keys().map(|k| k.y).max().unwrap() as usize,
    }
}

fn display_map_with_path(map: &Map, path: &Vec<Node>) {
    let br = get_bounding_rect(&map);
    for y in 0..=br.height {
        for x in 0..=br.width {
            print!("{}", {
                let coords = Coords {
                    x: x as i32,
                    y: y as i32,
                };
                let tmp = map.get(&coords).unwrap();
                if let Some(n) = path.iter().find(|n| n.current == coords) {
                    let straight = n.straight;
                    // Prevent offsetting the grid
                    if straight == 10 {
                        "A".to_string().bright_yellow().bold()
                    } else {
                        n.straight.to_string().bright_yellow().bold()
                    }
                } else {
                    tmp.to_string().blue()
                }
            })
        }
        println!("");
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct Node {
    current: Coords,
    previous: Coords,
    direction: Direction,
    straight: usize,
}

fn get_successors(map: &Map, current_node: &Node) -> Vec<(Node, usize)> {
    let mut out: Vec<(Node, usize)> = vec![];

    let current = current_node.current.clone();
    let direction = current_node.direction.clone();
    let straight = current_node.straight.clone();

    // Minimum blocks before turning
    let min = 4;
    // Maximum blocks before turning
    let max = 9;

    let right = Coords {
        x: current.x + 1,
        y: current.y,
    };
    if direction != Direction::Left
        && !(direction != Direction::Right && straight < min)
        && !(direction == Direction::Right && straight > max)
    {
        if let Some(heat_loss) = map.get(&right) {
            let new_direction = Direction::Right;
            out.push((
                Node {
                    current: right.clone(),
                    previous: current.clone(),
                    direction: new_direction.clone(),
                    straight: {
                        if direction == Direction::Right {
                            straight + 1
                        } else {
                            1
                        }
                    },
                },
                *heat_loss,
            ));
        }
    }

    let left = Coords {
        x: current.x - 1,
        y: current.y,
    };
    if direction != Direction::Right
        && !(direction != Direction::Left && straight < min)
        && !(direction == Direction::Left && straight > max)
    {
        if let Some(heat_loss) = map.get(&left) {
            let new_direction = Direction::Left;
            out.push((
                Node {
                    current: left.clone(),
                    previous: current.clone(),
                    direction: new_direction.clone(),
                    straight: {
                        if direction == Direction::Left {
                            straight + 1
                        } else {
                            1
                        }
                    },
                },
                *heat_loss,
            ));
        }
    }

    let up = Coords {
        x: current.x,
        y: current.y - 1,
    };
    if direction != Direction::Down
        && !(direction != Direction::Up && straight < min)
        && !(direction == Direction::Up && straight > max)
    {
        if let Some(heat_loss) = map.get(&up) {
            let new_direction = Direction::Up;
            out.push((
                Node {
                    current: up.clone(),
                    previous: current.clone(),
                    direction: new_direction.clone(),
                    straight: {
                        if direction == Direction::Up {
                            straight + 1
                        } else {
                            1
                        }
                    },
                },
                *heat_loss,
            ));
        }
    }

    let down = Coords {
        x: current.x,
        y: current.y + 1,
    };
    if direction != Direction::Up
        && !(direction != Direction::Down && straight < min)
        && !(direction == Direction::Down && straight > max)
    {
        if let Some(heat_loss) = map.get(&down) {
            let new_direction = Direction::Down;
            out.push((
                Node {
                    current: down.clone(),
                    previous: current.clone(),
                    direction: new_direction.clone(),
                    straight: {
                        if direction == Direction::Down {
                            straight + 1
                        } else {
                            1
                        }
                    },
                },
                *heat_loss,
            ));
        }
    }

    out
}

fn get_minimal_heat_loss(map: &Map) -> usize {
    let br = get_bounding_rect(&map);

    // Two possibilities, going from the top to the bottom
    // or from the left to the right
    let starting_nodes = vec![
        Node {
            current: Coords { x: 0, y: 0 },
            previous: Coords { x: 0, y: -1 },
            direction: Direction::Down,
            straight: 0,
        },
        Node {
            current: Coords { x: 0, y: 0 },
            previous: Coords { x: -1, y: 0 },
            direction: Direction::Right,
            straight: 0,
        },
    ];

    let result = starting_nodes
        .iter()
        .map(|starting_node| {
            dijkstra(
                starting_node,
                |node| get_successors(&map, &node),
                |p| {
                    p.current
                == Coords {
                    x: br.width as i32,
                    y: br.height as i32,
                }
                // We have to ensure that even the last node is conform to the
                // movement constraints
                && p.straight >= 4
                && p.straight <= 10
                },
            )
            .expect("no path found")
        })
        .min_by(|n1, n2| n1.1.cmp(&n2.1))
        .unwrap();

    display_map_with_path(&map, &result.0);

    result.1
}

#[cfg(test)]
mod tests_get_minimal_heat_loss {
    use super::*;

    #[test]
    fn get_minimal_heat_loss_01() {
        let map = parse_map(
            &"\
111111111111
999999999991
999999999991
999999999991
999999999991",
        );
        assert_eq!(get_minimal_heat_loss(&map), 71)
    }

    #[test]
    fn get_minimal_heat_loss_02() {
        let map = parse_map(
            &"\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533",
        );
        assert_eq!(get_minimal_heat_loss(&map), 94)
    }
}

fn main() {
    let content = get_file_content("assets/input");

    let map: Map = parse_map(&content);

    let least_heat_loss = get_minimal_heat_loss(&map);

    println!("Result: {:?}", least_heat_loss);
}
