use std::{collections::HashMap, fmt::Display, fs};

use colored::Colorize;
use geo::{point, Contains, LineString, Polygon};

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

impl Display for PipeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PipeType::NorthSouth => write!(f, "│"),
            PipeType::EastWest => write!(f, "─"),
            PipeType::NorthEast => write!(f, "└"),
            PipeType::NorthWest => write!(f, "┘"),
            PipeType::SouthWest => write!(f, "┐"),
            PipeType::SouthEast => write!(f, "┌"),
            PipeType::Ground => write!(f, "."),
            PipeType::StartingPosition => write!(f, "S"),
        }
    }
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

    let mut x_max: u32 = 0;
    let mut y_max: u32 = 0;

    let mut height = 0;
    let mut map: Map = HashMap::new();
    let mut maybe_starting_position: Option<Coordinates> = None;
    for line in content.lines() {
        y_max += 1;
        x_max = line.len() as u32;
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

    let mut line_string_vec: Vec<(f32, f32)> = vec![];

    println!(
        "\nResult: {}",
        [
        // We know from part 1 that N-S gives the right number of steps
            (
                {
                    let mut clone = map.clone();
                    clone.insert(starting_position, PipeType::NorthWest);
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
            line_string_vec.push(( tmp_prev.0 as f32, tmp_prev.1 as f32 ));
            line_string_vec.push(( tmp_current.0 as f32, tmp_current.1 as f32 ));


            loop {
                stash = tmp_current;
                tmp_current = get_next_step(&tmp_current, &tmp_prev, &updated_map);
                line_string_vec.push(( tmp_current.0 as f32, tmp_current.1 as f32 ));
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

    let line_string: LineString<f32> = line_string_vec.clone().into();
    let polygon = Polygon::new(line_string, vec![]);

    dbg!(x_max);
    dbg!(y_max);

    // Get rid of StartingPosition for consistency
    map.insert(starting_position, PipeType::NorthWest);

    let mut inside_loop = 0;
    let mut is_inside = false;
    for y in 0..y_max {
        for x in 0..x_max {
            let mut printed = false;
            let pt = map.get(&(x, y)).unwrap();

            // Tests triggered when ON THE BOUNDARY
            if line_string_vec.contains(&(x as f32, y as f32)) {
                match pt {
                    &PipeType::NorthSouth => {
                        is_inside = !is_inside;
                        print!("{}", "|".bright_yellow().bold());
                        printed = true;
                    }

                    PipeType::NorthEast => {
                        print!("{}", "└".bright_yellow().bold());
                        printed = true;
                        // In the case of NE we have to check
                        // all the following chars to ensure that we are not
                        // in a "pseudo NS". Indeed └-----┐ is
                        // the same as | for our search purposes
                        let mut inner_x = x + 1;
                        while map.get(&(inner_x, y)) == Some(&PipeType::EastWest) {
                            inner_x += 1;
                        }
                        // Coming out of this we have to cases:
                        // └-----┐ -> We have a pseudo "|" and we should flip is_inside
                        // or
                        // └-----┘ -> We have a pseudo "-" and we should NOT flip is_inside
                        // Any other case we have a consistency error
                        match map.get(&(inner_x, y)) {
                            Some(PipeType::SouthWest) => {
                                is_inside = !is_inside;
                            }
                            Some(PipeType::NorthWest) => {}
                            a => {
                                Err(format!("NE Consistency error: {:?}", a)).expect("Fatal Error")
                            }
                        }
                    }

                    PipeType::SouthEast => {
                        print!("{}", "┌".bright_yellow().bold());
                        printed = true;
                        // In the case of SE we have to check
                        // all the following chars to ensure that we are not
                        // in a "pseudo NS". Indeed ┌-----┘ is
                        // the same as | for our search purposes
                        let mut inner_x = x + 1;
                        while map.get(&(inner_x, y)) == Some(&PipeType::EastWest) {
                            inner_x += 1;
                        }
                        // Coming out of this we have to cases:
                        // ┌-----┘ -> We have a pseudo "|" and we should flip is_inside
                        // or
                        // ┌-----┐ -> We have a pseudo "-" and we should NOT flip is_inside
                        // Any other case we have a consistency error
                        match map.get(&(inner_x, y)) {
                            Some(PipeType::SouthWest) => {}
                            Some(PipeType::NorthWest) => {
                                is_inside = !is_inside;
                            }
                            a => {
                                Err(format!("SE Consistency error: {:?}", a)).expect("Fatal Error")
                            }
                        }
                    }
                    _ => {
                        print!("{}", format!("{}", pt).bold().blue());
                        printed = true;
                    }
                }
            }

            // Tests triggered when INSIDE THE BOUNDARY
            if polygon.contains(&point! {x: x as f32, y: y as f32}) {
                if is_inside {
                    inside_loop += 1;
                    print!("{}", "I".red());
                    printed = true;
                } else {
                    print!("{}", "O".blue());
                    printed = true;
                }
            }
            if !printed {
                print!("{}", pt);
            }
        }
        println!();
    }
    println!("Inside loop: {}", inside_loop);
}
