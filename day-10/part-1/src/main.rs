use std::{collections::HashMap, fs};

fn get_file_content(file_path: &String) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum PipeType {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    StartingPosition,
}

fn parse_char(char: char) -> PipeType {
    match char {
        '|' => PipeType::NorthSouth,
        '-' => PipeType::EastWest,
        'L' => PipeType::NorthEast,
        'J' => PipeType::NorthWest,
        '7' => PipeType::SouthWest,
        'F' => PipeType::SouthEast,
        '.' => PipeType::Ground,
        'S' => PipeType::StartingPosition,
        _ => Err(format!("Invalid character: {}", char)).expect("Fatal Error"),
    }
}

#[cfg(test)]
mod tests_parse_char {
    use super::*;

    #[test]
    fn parse_char_01() {
        assert_eq!(parse_char('|'), PipeType::NorthSouth);
    }

    #[test]
    fn parse_char_02() {
        assert_eq!(parse_char('-'), PipeType::EastWest);
    }

    #[test]
    fn parse_char_03() {
        assert_eq!(parse_char('L'), PipeType::NorthEast);
    }

    #[test]
    fn parse_char_04() {
        assert_eq!(parse_char('J'), PipeType::NorthWest);
    }

    #[test]
    fn parse_char_05() {
        assert_eq!(parse_char('7'), PipeType::SouthWest);
    }

    #[test]
    fn parse_char_06() {
        assert_eq!(parse_char('F'), PipeType::SouthEast);
    }

    #[test]
    fn parse_char_07() {
        assert_eq!(parse_char('.'), PipeType::Ground);
    }

    #[test]
    fn parse_char_08() {
        assert_eq!(parse_char('S'), PipeType::StartingPosition);
    }

    #[test]
    #[should_panic]
    fn parse_char_09() {
        parse_char('X');
    }
}

type Coordinates = (u32, u32);
type Map = HashMap<Coordinates, PipeType>;

/// Parse a given line at given height into a given Map
///
/// Return the coordinates of the starting position, when found
fn parse_line(line: &str, height: &u32, hm: &mut Map) -> Option<Coordinates> {
    let mut maybe: Option<Coordinates> = None;
    line.chars().map(parse_char).enumerate().for_each(|(i, p)| {
        if p == PipeType::StartingPosition {
            maybe = Some((i as u32, *height));
        }
        hm.insert((i as u32, *height), p);
    });
    return maybe;
}

#[cfg(test)]
mod tests_parse_line {
    use super::*;

    #[test]
    fn tests_parse_line_01() {
        let mut hm: Map = HashMap::new();
        let result = parse_line(&"|-J.7FSL", &13, &mut hm);

        assert_eq!(
            hm,
            HashMap::from_iter(vec![
                ((0, 13), PipeType::NorthSouth),
                ((1, 13), PipeType::EastWest),
                ((2, 13), PipeType::NorthWest),
                ((3, 13), PipeType::Ground),
                ((4, 13), PipeType::SouthWest),
                ((5, 13), PipeType::SouthEast),
                ((6, 13), PipeType::StartingPosition),
                ((7, 13), PipeType::NorthEast),
            ])
        );
        assert_eq!(result, Some((6, 13)));
    }
}

fn get_next_step(current: &Coordinates, coming_from: &Coordinates, map: &Map) -> Coordinates {
    match map.get(current).unwrap() {
        PipeType::NorthSouth => {
            if (current.1 as i32) - (coming_from.1 as i32) > (0 as i32) {
                (current.0, current.1 + 1)
            } else {
                (current.0, current.1 - 1)
            }
        }
        PipeType::EastWest => {
            if (current.0 as i32) - (coming_from.0 as i32) > (0 as i32) {
                (current.0 + 1, current.1)
            } else {
                (current.0 - 1, current.1)
            }
        }
        PipeType::NorthEast => {
            if (current.1 as i32) - (coming_from.1 as i32) > (0 as i32) {
                (current.0 + 1, current.1)
            } else {
                (current.0, current.1 - 1)
            }
        }
        PipeType::NorthWest => {
            if (current.1 as i32) - (coming_from.1 as i32) > (0 as i32) {
                (current.0 - 1, current.1)
            } else {
                (current.0, current.1 - 1)
            }
        }
        PipeType::SouthWest => {
            if (current.1 as i32) - (coming_from.1 as i32) < (0 as i32) {
                (current.0 - 1, current.1)
            } else {
                (current.0, current.1 + 1)
            }
        }
        PipeType::SouthEast => {
            if (current.1 as i32) - (coming_from.1 as i32) < (0 as i32) {
                (current.0 + 1, current.1)
            } else {
                (current.0, current.1 + 1)
            }
        }
        PipeType::Ground => Err("Ground encountered").expect("Fatal Error"),
        PipeType::StartingPosition => *current,
    }
}

#[cfg(test)]
mod tests_get_next_step {
    use super::*;

    #[test]
    fn get_next_step_01() {
        assert_eq!(
            get_next_step(
                &(1, 13),
                &(0, 13),
                &HashMap::from_iter(vec![((1, 13), PipeType::EastWest)])
            ),
            (2, 13)
        );
    }
}

fn main() {
    let content = get_file_content(&String::from("assets/input"));

    let mut height = 0;
    let mut map: Map = HashMap::new();
    let mut maybe_starting_position: Option<Coordinates> = None;
    for line in content.lines() {
        let result = parse_line(&line, &height, &mut map);
        match result {
            Some(_) => {
                maybe_starting_position = result;
            }
            None => {}
        }
        height += 1;
    }

    let starting_position = maybe_starting_position.unwrap();
    dbg!(starting_position);

    println!(
        "\nResult: {}",
        [
            (
                {
                    let mut clone = map.clone();
                    clone.insert(starting_position, PipeType::NorthSouth);
                    clone
                },
                (starting_position.0, starting_position.1 + 1)
            ),
            (
                {
                    let mut clone = map.clone();
                    clone.insert(starting_position, PipeType::EastWest);
                    clone
                },
                (starting_position.0 + 1, starting_position.1)
            ),
            (
                {
                    let mut clone = map.clone();
                    clone.insert(starting_position, PipeType::NorthEast);
                    clone
                },
                (starting_position.0, starting_position.1 - 1)
            ),
            (
                {
                    let mut clone = map.clone();
                    clone.insert(starting_position, PipeType::NorthWest);
                    clone
                },
                (starting_position.0, starting_position.1 - 1)
            ),
            (
                {
                    let mut clone = map.clone();
                    clone.insert(starting_position, PipeType::SouthEast);
                    clone
                },
                (starting_position.0, starting_position.1 + 1)
            ),
            (
                {
                    let mut clone = map.clone();
                    clone.insert(starting_position, PipeType::SouthWest);
                    clone
                },
                (starting_position.0, starting_position.1 + 1)
            ),
        ]
        .iter()
        .map(|(updated_map, prev)| {
            let mut tmp_prev = *prev;
            let mut stash = starting_position;
            let mut tmp_current = get_next_step(&starting_position, &tmp_prev, &updated_map);
            tmp_prev = stash;
            // Accounting for the two fake steps we have already taken
            let mut steps = 2;
            loop {
                stash = tmp_current;
                tmp_current = get_next_step(&tmp_current, &tmp_prev, &updated_map);
                tmp_prev = stash;
                // dbg!(&tmp_current);
                if tmp_current == starting_position {
                    break;
                }
                steps += 1;
            }
            return steps;
        })
        .max()
        .unwrap()
        // We don't want the total path length, but only the steps required to go to
        // the furthermost tile, hence half the total path (since beginning = end
        // = starting_position)
            / 2
    );
}
