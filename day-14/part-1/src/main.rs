use std::fs;

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

fn roll_left(line: &str) -> String {
    let chars = line.clone().chars();
    let mut new_line: Vec<char> = line.clone().chars().collect();
    for (x, c) in chars.enumerate() {
        match c {
            'O' => {
                if x > 0 {
                    let mut tmp = x;
                    for xi in (0..=x - 1).rev() {
                        match new_line[xi] {
                            'O' | '#' => {
                                break;
                            }
                            _ => {}
                        }
                        tmp = xi;
                    }
                    if tmp != x {
                        new_line[tmp] = 'O';
                        new_line[x] = '.'
                    }
                }
            }
            _ => {}
        }
    }
    new_line.iter().collect::<String>()
}

#[cfg(test)]
mod tests_roll_left {
    use super::*;

    #[test]
    fn roll_left_01() {
        assert_eq!(roll_left(&".....O..##"), "O.......##");
    }

    #[test]
    fn roll_left_02() {
        assert_eq!(roll_left(&"..O..O..##"), "OO......##");
    }

    #[test]
    fn roll_left_03() {
        assert_eq!(roll_left(&"..O#.O..##"), "O..#O...##");
    }

    #[test]
    fn roll_left_04() {
        assert_eq!(roll_left(&"..O#.O..#O"), "O..#O...#O");
    }

    #[test]
    fn roll_left_05() {
        assert_eq!(roll_left(&"#........O"), "#O........");
    }
}

fn rotate(block: &str) -> String {
    let lines = block.split_whitespace();
    let first_line = lines.clone().nth(0).unwrap();
    let mut columns: Vec<String> = vec![];
    for i in (0..first_line.len()).rev() {
        let column = lines
            .clone()
            .map(|l| l.to_string().chars().nth(i).unwrap().to_string())
            // .rev()
            .collect();
        columns.push(column);
    }
    columns.join("\n")
}

#[cfg(test)]
mod tests_rotate {
    use super::*;

    #[test]
    fn rotate_01() {
        let result = rotate(
            &"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        );
        let expected = String::from(
            ".#.O.#O...
....#.....
....O#.O#.
..#...O.#.
#.#..O#.##
.#.O......
.O.#......
.O...#O..O
...OO....O
OO.O.O..##",
        );
        assert_eq!(result, expected)
    }
}

fn roll_north(block: &str) -> String {
    rotate(&block)
        .lines()
        .map(|l| roll_left(l) + "\n")
        .collect()
}

#[cfg(test)]
mod tests_roll_north {
    use super::*;

    #[test]
    fn roll_north_01() {
        let result = roll_north(
            &"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        );
        let expected = String::from(
            ".#O..#O...
....#.....
O....#O.#.
..#O....#.
#.#O..#.##
.#O.......
O..#......
O....#OO..
OOO.......
OOOO....##
",
        );
        assert_eq!(result, expected)
    }
}

fn count_weight(block: &str) -> usize {
    let mut acc = 0;
    for line in block.lines() {
        for (i, c) in line.chars().enumerate() {
            match c {
                'O' => acc += line.len() - i,
                _ => {}
            }
        }
    }
    acc
}

#[cfg(test)]
mod tests_count_weight {
    use super::*;

    #[test]
    fn count_weight_01() {
        let result = count_weight(
            &".#O..#O...
....#.....
O....#O.#.
..#O....#.
#.#O..#.##
.#O.......
O..#......
O....#OO..
OOO.......
OOOO....##
",
        );
        let expected = 136;
        assert_eq!(result, expected)
    }
}

fn main() {
    let content = get_file_content("assets/input");

    println!(
        "Result: {:?}",
        count_weight(roll_north(&content.as_str()).as_str())
    );
}
