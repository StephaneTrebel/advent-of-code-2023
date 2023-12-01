use std::fs;
use std::str;

fn main() {
    let file_path = "assets/input";
    println!("Loading input file: {}", file_path);
    let file_content = fs::read_to_string(file_path).expect("Cannot load file");

    let mut calibrations_sum = 0;
    for word in file_content.split("\n") {
        let mut test = word
            .chars()
            .flat_map(|c| c.to_digit(10))
            .map(|i| char::from_digit(i, 10).unwrap());

        let mut temp: Vec<char> = vec![];
        let first = test.next().unwrap_or('0');
        temp.push(first);
        temp.push(test.last().unwrap_or(first));

        let calibration: u32 = str::parse(&(temp.iter().collect::<String>())).expect("Ouch");
        println!("Word '{}' calibration is: {:?}", word, calibration);
        calibrations_sum += calibration;
    }

    println!("Calibration sum: {}", calibrations_sum);
}
