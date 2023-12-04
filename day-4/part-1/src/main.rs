use std::fs;

fn get_file_content(file_path: &String) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(Debug)]
struct Scratchcard {
    winning_numbers: Vec<u32>,
    our_numbers: Vec<u32>,
}

fn parse_scratchcard(line: &str) -> Scratchcard {
    let mut scratchcard: Scratchcard = Scratchcard {
        winning_numbers: vec![],
        our_numbers: Vec::new(),
    };

    let mut split = line.split("|");

    if let Some(winning_numbers) = split.next() {
        for winning_number in winning_numbers.split(" ") {
            let temp = winning_number.parse().unwrap_or(0);
            dbg!(temp);
            if temp != 0 {
                scratchcard.winning_numbers.push(temp);
            }
        }
    }

    if let Some(our_numbers) = split.next() {
        for our_number in our_numbers.split(" ") {
            let temp = our_number.parse().unwrap_or(0);
            dbg!(temp);
            if temp != 0 {
                scratchcard.our_numbers.push(temp);
            }
        }
    }

    return scratchcard;
}

#[cfg(test)]
mod tests_parse_scratchcard {
    use super::*;

    #[test]
    fn parse_scratchcard_01() {
        let scratchcard = parse_scratchcard(&String::from(
            "Scratchcard 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
        ));

        assert_eq!(scratchcard.winning_numbers, vec![41, 48, 83, 86, 17]);
        assert_eq!(scratchcard.our_numbers, vec![83, 86, 6, 31, 17, 9, 48, 53]);
    }
}

fn compute_scratchcard_score(scratchcard: Scratchcard) -> u32 {
    let mut power_count: u32 = 0;
    let mut sum: u32 = 0;
    let base: u32 = 2;
    for our_number in scratchcard.our_numbers.iter() {
        scratchcard
            .winning_numbers
            .iter()
            .find(|n| n == &our_number)
            .iter()
            .for_each(|_| {
                sum = base.pow(power_count);
                power_count += 1;
            })
    }
    return sum;
}

#[cfg(test)]
mod tests_compute_score {
    use super::*;

    #[test]
    fn compute_score_01() {
        assert_eq!(
            compute_scratchcard_score(Scratchcard {
                winning_numbers: vec![41, 48, 83, 86, 17],
                our_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
            }),
            8
        );
    }

    #[test]
    fn compute_score_02() {
        assert_eq!(
            compute_scratchcard_score(Scratchcard {
                winning_numbers: vec![13, 32, 20, 16, 61],
                our_numbers: vec![61, 30, 68, 82, 17, 32, 24, 19],
            }),
            2
        );
    }

    #[test]
    fn compute_score_03() {
        assert_eq!(
            compute_scratchcard_score(Scratchcard {
                winning_numbers: vec![1, 21, 53, 59, 44],
                our_numbers: vec![69, 82, 63, 72, 16, 21, 14, 1],
            }),
            2
        );
    }

    #[test]
    fn compute_score_04() {
        assert_eq!(
            compute_scratchcard_score(Scratchcard {
                winning_numbers: vec![41, 92, 73, 84, 69],
                our_numbers: vec![59, 84, 76, 51, 58, 5, 54, 83],
            }),
            1
        );
    }

    #[test]
    fn compute_score_05() {
        assert_eq!(
            compute_scratchcard_score(Scratchcard {
                winning_numbers: vec![87, 83, 26, 28, 32],
                our_numbers: vec![88, 30, 70, 12, 93, 22, 82, 36],
            }),
            0
        );
    }

    #[test]
    fn compute_score_06() {
        assert_eq!(
            compute_scratchcard_score(Scratchcard {
                winning_numbers: vec![31, 18, 13, 56, 72],
                our_numbers: vec![74, 77, 10, 23, 35, 67, 36, 11],
            }),
            0
        );
    }
}

fn main() {
    println!("Scratchcard points sum: {}", get_file_content(&String::from("assets/input"))
        .lines()
        .map(parse_scratchcard)
        .map(compute_scratchcard_score)
        .fold(0, |acc, e| acc + e));
}
