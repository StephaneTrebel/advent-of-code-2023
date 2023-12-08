use num::integer::lcm;
use std::{collections::HashMap, fmt::Display, fs};

fn get_file_content(file_path: &String) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
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
    let i = instructions.unwrap();
    let mut cycle = i.iter().cycle();

    let mut states: Vec<(&String, &Node)> = map
        .iter()
        .filter(|(k, _)| k.chars().nth(2).unwrap() == 'A')
        .collect();
    dbg!(&states);
    let count = states.len();
    dbg!(count);
    let mut instruction: &Direction;

    // Number of steps per loop
    let mut steps: Vec<u32> = states.iter().map(|(_, _)| 0 as u32).collect();

    loop {
        instruction = cycle.next().unwrap();

        if states
            .iter()
            .filter(|(k, _)| k.chars().nth(2).unwrap() == 'Z')
            .count()
            == count
        {
            break;
        }

        states = states
            .iter()
            .enumerate()
            .map(|(index, (k, v))| {
                if k.chars().nth(2).unwrap() == 'Z' {
                    (*k, *v)
                } else {
                    steps[index] += 1;
                    map.get_key_value({
                        match instruction {
                            Direction::Left => &v.left_side,
                            Direction::Right => &v.right_side,
                        }
                    })
                    .unwrap()
                }
            })
            .collect();
    }
    dbg!(&steps);

    let total: u128 = steps.iter().fold(1, |acc, s| lcm(acc, *s as u128));
    println!("\nResult: {}", total);
}
