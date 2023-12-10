use std::{fs, vec};

fn get_file_content(file_path: &String) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

fn parse_line(line: &str) -> Vec<i32> {
    let mut out = vec![];
    for block in line.split_whitespace() {
        out.push(block.parse().unwrap());
    }
    out
}

#[cfg(test)]
mod tests_parse_line {
    use super::*;

    #[test]
    fn parse_line_01() {
        assert_eq!(
            parse_line(&String::from("0 -3 6 9 12 15")),
            vec![0, -3, 6, 9, 12, 15]
        );
    }
}

fn get_diff_list(input: Vec<i32>) -> Vec<Vec<i32>> {
    let mut tmp = input.clone();
    let mut out = vec![];
    out.push(tmp.clone());
    let mut acc: i32 = 0;
    while tmp.clone().iter().filter(|e| **e != 0).count() > 0 {
        let mut inner_tmp = vec![];
        for (index, element) in tmp.iter().enumerate() {
            if index > 0 {
                inner_tmp.push(element - acc);
            }
            acc = *element;
        }
        out.push(inner_tmp.clone());
        tmp = inner_tmp;
        acc = 0;
    }
    out
}

#[cfg(test)]
mod tests_get_diff_list {
    use super::*;

    #[test]
    fn get_diff_list_01() {
        assert_eq!(
            get_diff_list(vec![0, 3, 6, 9, 12, 15]),
            vec![
                vec![0, 3, 6, 9, 12, 15],
                vec![3, 3, 3, 3, 3],
                vec![0, 0, 0, 0]
            ]
        );
    }
    #[test]
    fn get_diff_list_02() {
        assert_eq!(
            get_diff_list(vec![10, 13, 16, 21, 30, 45, 68]),
            vec![
                vec![10, 13, 16, 21, 30, 45, 68],
                vec![3, 3, 5, 9, 15, 23],
                vec![0, 2, 4, 6, 8],
                vec![2, 2, 2, 2],
                vec![0, 0, 0]
            ]
        );
    }
}

fn predict_next_number(diff_list: Vec<Vec<i32>>) -> i32 {
    diff_list
        .iter()
        .rev()
        .fold(0, |acc, list| list.last().unwrap() + acc)
}

#[cfg(test)]
mod tests_predict_next_number {
    use super::*;

    #[test]
    fn predict_next_number_01() {
        assert_eq!(
            predict_next_number(vec![
                vec![0, 3, 6, 9, 12, 15],
                vec![3, 3, 3, 3, 3],
                vec![0, 0, 0, 0]
            ]),
            18
        );
    }

    #[test]
    fn predict_next_number_02() {
        assert_eq!(
            predict_next_number(vec![
                vec![10, 13, 16, 21, 30, 45],
                vec![3, 3, 5, 9, 15],
                vec![0, 2, 4, 6],
                vec![2, 2, 2],
                vec![0, 0]
            ]),
            68
        );
    }
}

fn main() {
    println!(
        "\nResult: {}",
        get_file_content(&String::from("assets/input"))
            .lines()
            .map(parse_line)
            .map(get_diff_list)
            .fold(0, |acc, line| acc + predict_next_number(line))
    );
}
