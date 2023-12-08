use std::{collections::HashMap, fs, fmt::Display};

fn get_file_content(file_path: &String) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Direction {
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Left => f.write_str("L"),
            Direction::Right => f.write_str("R"),
        }
    }
}

fn parse_instructions(line: &str) -> Vec<Direction> {
    line.chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => Err("Invalid character for direction").expect("Fatal error"),
        })
        .collect()
}

#[cfg(test)]
mod tests_instructions {
    use super::*;

    #[test]
    fn parse_instructions_trivial() {
        assert_eq!(parse_instructions("L"), vec![Direction::Left]);
        assert_eq!(parse_instructions("R"), vec![Direction::Right]);
    }

    #[test]
    fn parse_instructions_real() {
        assert_eq!(
            parse_instructions("LLRLR"),
            vec![
                Direction::Left,
                Direction::Left,
                Direction::Right,
                Direction::Left,
                Direction::Right,
            ]
        );
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Node {
    left_side: String,
    right_side: String,
}

fn main() {
    let content = get_file_content(&String::from("assets/input"));

    let mut instructions: Option<Vec<Direction>> = None;
    let mut map: HashMap<String, Node> = HashMap::new();

    for (index, line) in content.lines().enumerate() {
        if index == 0 {
            instructions = Some(parse_instructions(&line));
        }
        if line.len() == 16 {
            map.insert(
                line[0..=2].to_string(),
                Node {
                    left_side: line[7..=9].to_string(),
                    right_side: line[12..=14].to_string(),
                },
            );
        }
    }

    let mut steps = 0;
    let mut state = map.get_key_value("AAA").unwrap();
    for instruction in instructions.unwrap().iter().cycle() {
        if state.0 == "ZZZ" {
            break;
        }
        println!(
            "Step:{:0>5} Instruction:{} State:{} = ({},{})",
            steps, instruction, state.0, state.1.left_side, state.1.right_side
        );
        steps += 1;
        state = map
            .get_key_value({
                match instruction {
                    Direction::Left => &state.1.left_side,
                    Direction::Right => &state.1.right_side,
                }
            })
            .unwrap();
    }

    println!("\nResult: {}", steps);
}
