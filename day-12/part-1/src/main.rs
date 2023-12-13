use itertools::{repeat_n, Itertools};
use std::{fmt::Display, fs};

fn get_file_content(file_path: &String) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum SpringType {
    Functional,
    Broken,
    Unknown,
}

fn parse_char(char: char) -> SpringType {
    match char {
        '.' => SpringType::Functional,
        '#' => SpringType::Broken,
        '?' => SpringType::Unknown,
        _ => Err(format!("Invalid character: {}", char)).expect("Fatal Error"),
    }
}

impl Display for SpringType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpringType::Functional => write!(f, "."),
            SpringType::Broken => write!(f, "#"),
            SpringType::Unknown => write!(f, "?"),
        }
    }
}

#[cfg(test)]
mod tests_parse_char {
    use super::*;

    #[test]
    fn parse_char_01() {
        assert_eq!(parse_char('.'), SpringType::Functional);
    }

    #[test]
    #[should_panic]
    fn parse_char_03() {
        parse_char('X');
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Row {
    spring_list: Vec<SpringType>,
    broken_group_list: Vec<u32>,
}

fn parse_line(line: &str) -> Row {
    let mut split = line.split_whitespace();

    Row {
        spring_list: split.next().unwrap().chars().map(parse_char).collect(),
        broken_group_list: split
            .next()
            .unwrap()
            .split(',')
            .map(|c| c.parse().unwrap())
            .collect_vec(),
    }
}

#[cfg(test)]
mod tests_parse_line {
    use super::*;

    #[test]
    fn tests_parse_line_01() {
        assert_eq!(
            parse_line(&"#.#.### 1,1,3"),
            Row {
                spring_list: vec![
                    SpringType::Broken,
                    SpringType::Functional,
                    SpringType::Broken,
                    SpringType::Functional,
                    SpringType::Broken,
                    SpringType::Broken,
                    SpringType::Broken
                ],
                broken_group_list: vec![1, 1, 3]
            }
        );
    }
}

trait Cohesive {
    fn is_cohesive(&self) -> bool;
}

impl Cohesive for Row {
    fn is_cohesive(&self) -> bool {
        self.spring_list
            .iter()
            .group_by(|e| **e == SpringType::Broken)
            .into_iter()
            .filter(|(b, _)| *b == true)
            .map(|(_, g)| g.count() as u32)
            .into_iter()
            .collect::<Vec<u32>>()
            == self.broken_group_list
    }
}

#[cfg(test)]
mod tests_is_cohesive {
    use super::*;

    #[test]
    fn is_cohesive_01() {
        assert_eq!(
            (Row {
                spring_list: vec![
                    SpringType::Broken,
                    SpringType::Functional,
                    SpringType::Broken,
                    SpringType::Functional,
                    SpringType::Broken,
                    SpringType::Broken,
                    SpringType::Broken
                ],
                broken_group_list: vec![1, 1, 3]
            })
            .is_cohesive(),
            true
        );
    }
}

fn calc_permutations(row: Row) -> u32 {
    // Creating all possible permutations of Functional/Broken to cover
    // every «Unknown» state
    repeat_n(
        vec![SpringType::Functional, SpringType::Broken].iter(),
        row.spring_list
            .iter()
            .filter(|s| **s == SpringType::Unknown)
            .count(),
    )
    .multi_cartesian_product()
    // Checking every permutation to check whether it produces a cohesive Row
    .map(|permutation| {
        let mut tmp_perm = permutation.clone();
        tmp_perm.reverse();
        let tmp_row = Row {
            spring_list: row
                .spring_list
                .iter()
                .map(|spring| match spring {
                    SpringType::Unknown => tmp_perm.pop().unwrap().clone(),
                    x => x.clone(),
                })
                .collect_vec(),
            broken_group_list: row.broken_group_list.clone(),
        };
        // dbg!(&tmp_row.spring_list);
        tmp_row.is_cohesive()
    })
    // Count how many cohesive rows result of all that
    .filter(|e| *e == true)
    .count() as u32
}

#[cfg(test)]
mod tests_calc_permutations {
    use super::*;

    #[test]
    fn calc_permutations_01() {
        assert_eq!(calc_permutations(parse_line("???.### 1,1,3")), 1);
    }

    #[test]
    fn calc_permutations_02() {
        assert_eq!(calc_permutations(parse_line(" .??..??...?##. 1,1,3")), 4);
    }

    #[test]
    fn calc_permutations_03() {
        assert_eq!(calc_permutations(parse_line("?###???????? 3,2,1")), 10);
    }

}

fn main() {
    let content = get_file_content(&String::from("assets/input"));

    println!(
        "Permutation count: {:?}",
        content
            .lines()
            .map(parse_line)
            .map(calc_permutations)
            .fold(0, |acc, a| acc + a)
    );
}
