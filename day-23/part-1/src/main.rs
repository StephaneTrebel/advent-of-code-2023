use std::{
    collections::{HashMap, VecDeque},
    fs,
};
use uuid::Uuid;

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct Coords {
    x: usize,
    y: usize,
}

type Map = HashMap<Coords, char>;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Content {
    map: Map,
    width: usize,
    height: usize,
}

fn parse_content(lines: &str) -> Content {
    Content {
        map: lines
            .split("\n")
            .filter(|line| !line.is_empty())
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| (Coords { x, y }, c))
                    .collect::<Vec<(Coords, char)>>()
            })
            .collect::<Map>(),
        width: lines.split("\n").next().unwrap().len(),
        height: lines.split("\n").filter(|line| !line.is_empty()).count(),
    }
}

#[cfg(test)]
mod tests_parse_content {
    use super::*;

    #[test]
    fn parse_content_01() {
        let content = parse_content(
            &"\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
",
        );
        assert_eq!(content.map.len(), 529);
        assert_eq!(content.width, 23);
        assert_eq!(content.height, 23);
    }
}

#[allow(dead_code)]
fn display_map(content: &Content) {
    for y in 0..content.height {
        for x in 0..content.width {
            print!("{} ", content.map.get(&Coords { x, y }).unwrap());
        }
        println!();
    }
}

/// Returns the path length and the updated map with the path visible
fn find_longest_path(content: &Content) -> (usize, Content) {
    let mut new_content = content.clone();
    let width = content.width;
    let height = content.height;

    let starting_point = Coords { x: 1, y: 0 };
    let starting_uuid = Uuid::new_v4();
    let finish_point = Coords {
        x: width - 2,
        y: height - 1,
    };

    // Visited cannot be only Coords since we want the longest path
    // So we have to track both a tile coordinates and the path (usize) it's on
    // Parent is optional because the starting point does not have a parent.
    // Also, a "Parent" is a couple of Coords and usize because a parent will
    // not be on the same path (at each intersection we create new paths)
    let mut visited: HashMap<(Coords, Uuid), Option<(Coords, Uuid)>> = HashMap::new();

    // We are going to store every path, so we need a counter
    let mut path_list: Vec<Uuid> = vec![starting_uuid];

    visited.insert((starting_point.clone(), starting_uuid), None);
    new_content.map.insert(starting_point.clone(), 'S');

    // Path id is stored in the queue to match a successor with the according visited map
    let mut queue: VecDeque<(Coords, Uuid)> =
        VecDeque::from_iter(vec![(starting_point, starting_uuid)]);

    while let Some(element) = queue.pop_front() {
        let parent = element.0;
        let x = parent.x;
        let y = parent.y;
        let current_path = element.1;

        // Goal
        if parent == finish_point {
            continue;
        }

        // Light optimization to avoid checking unnecessary tiles
        let possible_successors = match content.map.get(&parent) {
            None => vec![],
            Some('.') => vec![(-1, 0), (1, 0), (0, -1), (0, 1)],
            Some('^') => vec![(0, -1)],
            Some('v') => vec![(0, 1)],
            Some('<') => vec![(-1, 0)],
            Some('>') => vec![(1, 0)],
            wat => {
                panic!("Invalid tile in map: {:?}", wat);
            }
        };

        // Collect valid successors from a list of possible ones
        let valid_successors = possible_successors
            .iter()
            .filter_map(|succ| {
                let new_x = x as isize + succ.0;
                let new_y = y as isize + succ.1;
                if new_x >= 0 && new_x < width as isize && new_y >= 0 && new_y < height as isize {
                    let new_coords = Coords {
                        x: new_x as usize,
                        y: new_y as usize,
                    };

                    if let None = visited.get(&(new_coords.clone(), current_path)) {
                        match content.map.get(&new_coords) {
                            None => {}
                            Some('#') => {}
                            Some(c) => {
                                return Some((new_coords, *c, parent.clone(), current_path));
                            }
                        }
                    }
                }
                return None;
            })
            .collect::<Vec<(Coords, char, Coords, Uuid)>>();

        // If we are at an intersection
        if valid_successors.len() > 1 {
            valid_successors
                .iter()
                .for_each(|(successor, successor_tile, parent, path_id)| {
                    // Only consider new paths when it's possible to go there
                    if *successor_tile == '.'
                        || *successor_tile == '<' && successor.x < parent.x
                        || *successor_tile == '>' && successor.x > parent.x
                        || *successor_tile == '^' && successor.y < parent.y
                        || *successor_tile == 'v' && successor.y > parent.y
                    {
                        // UUid to ensure path id uniqueness
                        let new_path_id = Uuid::new_v4();
                        path_list.push(new_path_id);

                        // Algorithm core: we mark the tile as visited for this path
                        // and link it to its parent (which may be on another path)
                        visited.insert(
                            (successor.clone(), new_path_id),
                            Some((parent.clone(), *path_id)),
                        );
                        // And we iterate on it !
                        queue.push_back((successor.clone(), new_path_id));
                    }
                })
        }
        // If not an intersection, we go further in the path
        else {
            valid_successors
                .iter()
                .for_each(|(successor, _, parent, path_id)| {
                    visited.insert(
                        (successor.clone(), *path_id),
                        Some((parent.clone(), *path_id)),
                    );
                    queue.push_back((successor.clone(), *path_id));
                })
        }
    }

    let mut tmp_element: Coords;

    println!("Browsing all paths to retrieve the longest one");

    // Loop to determine the longest path id and length
    let (longest_path_id, longest_path_length) = path_list
        .iter()
        .map(|path_id| {
            let mut length = 0;
            let mut tmp_path_id = *path_id;
            let mut tmp_element = finish_point.clone();
            while let Some(parent_tuple) = visited.get(&(tmp_element.clone(), tmp_path_id)) {
                match parent_tuple.clone() {
                    None => {
                        break;
                    }
                    Some(parent) => {
                        length += 1;
                        tmp_element = parent.0.clone();
                        tmp_path_id = parent.1;
                    }
                }
            }
            (*path_id, length)
        })
        .max_by(|(_, length_a), (_, length_b)| length_a.cmp(length_b))
        .unwrap();

    // Special loop to mutate the map in order to «see» the longest path
    tmp_element = Coords {
        x: width - 2,
        y: height - 1,
    };
    let mut tmp_path_id = longest_path_id;
    while let Some(parent_tuple) = visited.get(&(tmp_element.clone(), tmp_path_id)) {
        match parent_tuple.clone() {
            None => {
                break;
            }
            Some(parent) => {
                match new_content.map.get(&tmp_element.clone()) {
                    Some('.') => {
                        new_content.map.insert(tmp_element.clone(), 'O');
                    }
                    // We don't want to «hide» the icy slopes in the resulting path display
                    _ => {}
                }
                tmp_element = parent.0.clone();
                tmp_path_id = parent.1;
            }
        }
    }

    (longest_path_length, new_content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_longest_path_01() {
        let content = parse_content(
            &"\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
",
        );

        let result = find_longest_path(&content);

        display_map(&result.1);

        assert_eq!(result.0, 94);
    }
}

fn main() {
    let content = &parse_content(&get_file_content("assets/input"));

    println!("Part 1: {}", find_longest_path(&content).0);
}
