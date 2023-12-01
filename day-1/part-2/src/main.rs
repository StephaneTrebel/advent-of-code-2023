use std::fs;
use std::str;
use std::usize::MAX;

fn get_calibration(line: &str) -> u32 {
    let mut ascending_numbers_position: Vec<(&str, usize)> = vec![];
    let ascending_numbers = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for number in ascending_numbers {
        ascending_numbers_position.push((number, line.find(&number).unwrap_or(MAX)));
    }
    ascending_numbers_position.sort_by_key(|t| t.1);

    let reversed_line = line.chars().rev().collect::<String>();
    let mut descending_numbers_position: Vec<(&str, usize)> = vec![];
    let descending_numbers = [
        "zero", "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
    ];
    for number in descending_numbers {
        descending_numbers_position.push((number, reversed_line.find(&number).unwrap_or(MAX)));
    }
    descending_numbers_position.sort_by_key(|t| t.1);

    let cleaned_line = {
        let temp = line.clone();
        let first = ascending_numbers_position.get(0);
        let temp2 = match first {
            Some(i) => {
                if i.0 == "one" {
                    temp.replace("one", "o1e")
                } else if i.0 == "two" {
                    temp.replace("two", "t2o")
                } else if i.0 == "three" {
                    temp.replace("three", "t3e")
                } else if i.0 == "four" {
                    temp.replace("four", "f4r")
                } else if i.0 == "five" {
                    temp.replace("five", "f5e")
                } else if i.0 == "six" {
                    temp.replace("six", "s6x")
                } else if i.0 == "seven" {
                    temp.replace("seven", "s7n")
                } else if i.0 == "eight" {
                    temp.replace("eight", "e8t")
                } else {
                    temp.replace("nine", "n9e")
                }
            }
            None => temp.replace("nothing", "0"),
        };
        let last = descending_numbers_position.get(0);
        let temp3 = match last {
            Some(i) => {
                if i.0 == "eno" {
                    temp2.replace("one", "1")
                } else if i.0 == "owt" {
                    temp2.replace("two", "2")
                } else if i.0 == "eerht" {
                    temp2.replace("three", "3")
                } else if i.0 == "ruof" {
                    temp2.replace("four", "4")
                } else if i.0 == "evif" {
                    temp2.replace("five", "5")
                } else if i.0 == "xis" {
                    temp2.replace("six", "6")
                } else if i.0 == "neves" {
                    temp2.replace("seven", "7")
                } else if i.0 == "thgie" {
                    temp2.replace("eight", "8")
                } else {
                    temp2.replace("nine", "9")
                }
            }
            None => temp2.replace("nothing", "0"),
        };
        temp3
    };

    let mut test = cleaned_line
        .chars()
        .flat_map(|c| c.to_digit(10))
        .map(|i| char::from_digit(i, 10).unwrap());

    let mut temp: Vec<char> = vec![];
    let first = test.next().unwrap_or('0');
    temp.push(first);
    temp.push(test.last().unwrap_or(first));

    let calibration: u32 = str::parse(&(temp.iter().collect::<String>())).expect("Ouch");
    println!("Word '{}' calibration is: {:?}", line, calibration);
    return calibration;
}

fn main() {
    let file_path = "assets/input";
    println!("Loading input file: {}", file_path);
    let file_content = fs::read_to_string(file_path).expect("Cannot load file");

    let mut calibrations_sum = 0;

    for line in file_content.split("\n") {
        calibrations_sum += get_calibration(&line);
    }

    println!("Calibration sum: {}", calibrations_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_calibration_01() {
        assert_eq!(get_calibration(""), 0);
    }

    #[test]
    fn test_get_calibration_02() {
        assert_eq!(get_calibration("two1nine"), 29);
    }

    #[test]
    fn test_get_calibration_03() {
        assert_eq!(get_calibration("eightwothree"), 83);
    }

    #[test]
    fn test_get_calibration_04() {
        assert_eq!(get_calibration("abcone2threexyz"), 13);
    }

    #[test]
    fn test_get_calibration_05() {
        assert_eq!(get_calibration("xtwone3four"), 24);
    }

    #[test]
    fn test_get_calibration_06() {
        assert_eq!(get_calibration("4nineeightseven2"), 42);
    }

    #[test]
    fn test_get_calibration_07() {
        assert_eq!(get_calibration("zoneight234"), 14);
    }

    #[test]
    fn test_get_calibration_08() {
        assert_eq!(get_calibration("7pqrstsixteen"), 76);
    }

    #[test]
    fn test_get_calibration_09() {
        assert_eq!(get_calibration("eighthree"), 83);
    }

    #[test]
    fn test_get_calibration_10() {
        assert_eq!(get_calibration("sevenine"), 79);
    }
}
