use memoize::memoize;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct Coords {
    x: isize,
    y: isize,
}

type Map = HashMap<Coords, char>;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Content {
    map: Map,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct BoundingRectangle {
    origin: Coords,
    width: usize,
    height: usize,
}

#[allow(dead_code)]
fn get_bounding_rect(map: &Map) -> BoundingRectangle {
    BoundingRectangle {
        origin: Coords {
            x: map.keys().map(|k| k.x).min().unwrap(),
            y: map.keys().map(|k| k.y).min().unwrap(),
        },
        width: map.keys().map(|k| k.x).max().unwrap() as usize,
        height: map.keys().map(|k| k.y).max().unwrap() as usize,
    }
}

#[allow(dead_code)]
fn display(map: &Map) {
    let br = get_bounding_rect(&map);
    for y in 0..=br.height {
        for x in 0..=br.width {
            print!(
                "{}",
                map.get(&Coords {
                    x: x as isize,
                    y: y as isize
                })
                .unwrap()
            );
        }
        println!();
    }
}

fn parse_content(lines: &str) -> Content {
    Content {
        map: HashMap::from_iter(
            lines
                .split("\n")
                .filter(|line| !line.is_empty())
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .map(|(x, c)| {
                            (
                                Coords {
                                    x: x as isize,
                                    y: y as isize,
                                },
                                c,
                            )
                        })
                        .collect::<Vec<(Coords, char)>>()
                })
                .collect::<Vec<(Coords, char)>>(),
        ),
    }
}

#[cfg(test)]
mod tests_parse_content {
    use super::*;

    #[test]
    fn parse_content_01() {
        let content = parse_content(
            &"\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
",
        );
        assert_eq!(content.map.len(), 121);
    }
}

#[memoize]
fn take_n_steps(input: String, from: Coords, count: usize) -> HashSet<Coords> {
    let content = parse_content(&input);
    dbg!(&count);
    if count == 0 {
        return HashSet::from_iter(vec![from]);
    } else {
        return HashSet::from_iter(
            vec![
                Coords {
                    x: (from.x - 1),
                    y: from.y,
                },
                Coords {
                    x: from.x,
                    y: (from.y - 1),
                },
                Coords {
                    x: (from.x + 1),
                    y: from.y,
                },
                Coords {
                    x: from.x,
                    y: (from.y + 1),
                },
            ]
            .iter()
            .map(|test| match content.map.get(&test) {
                Some(c) => {
                    if *c != '#' {
                        Some(test)
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .filter(|e| if let Some(_) = *e { true } else { false })
            .map(|e| {
                if let Some(x) = e {
                    x.clone()
                } else {
                    panic!("oula")
                }
            })
            .flat_map(|step| take_n_steps(input.clone(), step, count - 1))
            .collect::<Vec<Coords>>(),
        );
    }
}

#[cfg(test)]
mod tests_take_n_steps {
    use super::*;

    #[test]
    fn take_n_steps_01() {
        let input = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
"
        .to_string();
        assert_eq!(
            take_n_steps(input, Coords { x: 5, y: 5 }, 2),
            HashSet::from_iter(vec![
                Coords { x: 3, y: 5 },
                Coords { x: 5, y: 5 },
                Coords { x: 4, y: 6 },
                Coords { x: 5, y: 3 },
            ])
        );
    }

    #[test]
    fn take_n_steps_02() {
        let input = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
"
        .to_string();
        assert_eq!(
            take_n_steps(input, Coords { x: 5, y: 5 }, 3),
            HashSet::from_iter(vec![
                Coords { x: 3, y: 4 },
                Coords { x: 3, y: 6 },
                Coords { x: 4, y: 7 },
                Coords { x: 6, y: 3 },
                Coords { x: 5, y: 4 },
                Coords { x: 4, y: 5 },
            ])
        );
    }

    #[test]
    fn take_n_steps_03() {
        let input = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
"
        .to_string();
        let result = take_n_steps(input, Coords { x: 5, y: 5 }, 4);
        assert_eq!(
            result,
            HashSet::from_iter(vec![
                Coords { x: 2, y: 4 },
                Coords { x: 3, y: 3 },
                Coords { x: 3, y: 5 },
                Coords { x: 3, y: 7 },
                Coords { x: 5, y: 5 },
                Coords { x: 4, y: 6 },
                Coords { x: 5, y: 7 },
                Coords { x: 5, y: 3 },
                Coords { x: 7, y: 3 },
            ])
        );
    }

    #[test]
    fn take_n_steps_04() {
        let input = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
"
        .to_string();
        let result = take_n_steps(input, Coords { x: 5, y: 5 }, 6);
        assert_eq!(result.len(), 16);
        assert_eq!(
            result,
            HashSet::from_iter([
                Coords { x: 0, y: 4 },
                Coords { x: 1, y: 3 },
                Coords { x: 2, y: 4 },
                Coords { x: 3, y: 3 },
                Coords { x: 3, y: 5 },
                Coords { x: 1, y: 7 },
                Coords { x: 3, y: 7 },
                Coords { x: 5, y: 7 },
                Coords { x: 3, y: 9 },
                Coords { x: 5, y: 5 },
                Coords { x: 4, y: 6 },
                Coords { x: 5, y: 3 },
                Coords { x: 7, y: 3 },
                Coords { x: 8, y: 2 },
                Coords { x: 8, y: 4 },
                Coords { x: 6, y: 6 },
            ])
        );
    }
}

fn main() {
    let input = get_file_content("assets/input");

    println!(
        "Result: {:?}",
        take_n_steps(input, Coords { x: 65, y: 65 }, 64).len()
    );
}
