use std::{
    collections::{HashMap, VecDeque},
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

impl From<&str> for Content {
    fn from(value: &str) -> Self {
        let mut map: Map = HashMap::new();
        value
            .split("\n")
            .filter(|line| !line.is_empty())
            .enumerate()
            .for_each(|(y, line)| {
                line.chars().enumerate().for_each(|(x, c)| {
                    map.insert(
                        Coords {
                            x: x as isize,
                            y: y as isize,
                        },
                        c,
                    );
                })
            });
        Content { map }
    }
}

#[cfg(test)]
mod tests_parse_content {
    use super::*;

    #[test]
    fn parse_content_01() {
        let content = Content::try_from(
            "\
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
        )
        .unwrap();
        assert_eq!(content.map.len(), 121);
    }
}

type Visited = HashMap<Coords, usize>;

fn breadth_first_search(content: &Content, starting_point: &Coords) -> Visited {
    let mut queue: VecDeque<(Coords, usize)> = VecDeque::new();
    let mut visited: Visited = HashMap::new();
    visited.insert(starting_point.clone(), 0);
    queue.push_back((starting_point.clone(), 0));

    while let Some((coords, steps)) = queue.pop_front() {
        let (x, y) = (coords.x, coords.y);

        vec![
            Coords { x, y: y - 1 },
            Coords { x, y: y + 1 },
            Coords { x: x - 1, y },
            Coords { x: x + 1, y },
        ]
        .iter()
        .filter_map(|tmp| {
            if let None = visited.get(&tmp) {
                match content.map.get(&tmp) {
                    Some('#') => None,
                    Some(_) => {
                        visited.insert(tmp.clone(), steps + 1);
                        Some((tmp.clone(), steps + 1))
                    }
                    None => None,
                }
            } else {
                None
            }
        })
        .for_each(|c| queue.push_back(c));
    }

    visited
}

#[cfg(test)]
mod tests_bfs {
    use super::*;

    #[test]
    fn breadth_first_search_01() {
        let bfs = breadth_first_search(
            &Content::try_from(
                "\
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
            )
            .unwrap(),
            &Coords { x: 5, y: 5 },
        );
        assert_eq!(bfs.get(&Coords { x: 5, y: 5 }), Some(&0));
        assert_eq!(bfs.get(&Coords { x: 6, y: 5 }), None);
        assert_eq!(bfs.get(&Coords { x: 5, y: 6 }), None);
        assert_eq!(bfs.get(&Coords { x: 4, y: 5 }), Some(&1));
        assert_eq!(bfs.get(&Coords { x: 10, y: 0 }), Some(&10));
    }
}

fn main() {
    let input = get_file_content("assets/input");

    let content = Content::try_from(input.as_str()).unwrap();

    // Ok so I could not find this myself, and I had to use this wonderful
    // explanation to understand what was needed to solve this puzzle:
    // https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21

    // The magic constant `n` will indicate how many times the Elf will traverse the map
    // in either East-West or North-South, or even in diagonal in a diamond fashion.
    // The map is actually DESIGNED to not have rocks in these paths.
    let n = 202300;
    assert_eq!(n * 131 + 65, 26501365);

    // Identify, for all non-rock tiles, how many steps (at the minimum) are
    // required to reach the starting point
    let bfs = breadth_first_search(&content, &Coords { x: 65, y: 65 });

    // Part 1 is calculating how many tiles are reachable in less than 64 steps,
    // BUT with even parity (because if odd parity you cannot arrive on the tile from
    // the starting point)
    let part_1 = bfs.values().filter(|v| **v % 2 == 0 && **v <= 64).count();
    println!("Part 1: {:?}", part_1);

    // "Even" tiles are tiles that are reachable, from the starting point, in an
    // even number of steps
    // "Corner" tiles are tiles that are beyond the «diamond» of steps reached by
    // the Elf (see `n` description above). These tiles will have to be deducted
    // in the final formula
    let even_full = bfs.values().filter(|v| **v % 2 == 0).count();
    let even_corners = bfs.values().filter(|v| **v % 2 == 0 && **v > 65).count();

    // Likewise, "Odd" tiles are tiles reached in an odd number of steps from the
    // starting position, and "Corner" tiles are tiles beyond the diamond shape
    let odd_full = bfs.values().filter(|v| **v % 2 == 1).count();
    let odd_corners = bfs.values().filter(|v| **v % 2 == 1 && **v > 65).count();


    // Sanitiy check of computed values to ascertain that indeed Part 1 is equivalent
    // to taking a full Map and removing all tiles beyond the step count (> 64 steps
    // from the starting point)
    assert_eq!(even_full - even_corners, 3639);

    // The final formula, that calculates the tile count reached in `n` times the
    // map size (131 tiles) plus the initial 65 steps required to reach the map edge
    // from the starting point
    let part_2 =
        // There are (n+1)^2 odd tiles
        ((n + 1) * (n + 1)) * odd_full +
        // and n^2 even tiles
        (n * n) * even_full
        // but we need to remove n+1 odd "corners"
        - (n + 1) * odd_corners
        // and n even "corners"
        + n * even_corners
        // and (finally !) n steps because there is a drift of one tile everytime
        - n;
    println!("Part 2: {:?}", part_2);
}
