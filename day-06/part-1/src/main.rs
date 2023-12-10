use std::fs;

fn get_file_content(file_path: &String) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

fn get_ways_to_beat_record(race: &Race) -> u32 {
    (1..=race.time)
        .into_iter()
        .map(|i| i * (race.time - i))
        .filter(|d| d > &race.distance)
        .count() as u32
}

#[cfg(test)]
mod tests_parse_seeds {
    use super::*;

    #[test]
    fn get_ways_to_beat_record_01() {
        assert_eq!(
            get_ways_to_beat_record(&Race {
                time: 7,
                distance: 9
            }),
            4
        );
    }

    #[test]
    fn get_ways_to_beat_record_02() {
        assert_eq!(
            get_ways_to_beat_record(&Race {
                time: 15,
                distance: 40
            }),
            8
        );
    }

    #[test]
    fn get_ways_to_beat_record_03() {
        assert_eq!(
            get_ways_to_beat_record(&Race {
                time: 30,
                distance: 200
            }),
            9
        );
    }
}

fn main() {
    let content = get_file_content(&String::from("assets/input"));

    let mut lines = content.lines();
    let times: Vec<u32> = lines
        .next()
        .unwrap()
        .replace("Time:", "")
        .split(' ')
        .map(|e| e.parse::<u32>().unwrap_or(0))
        .filter(|e| *e != 0)
        .collect();

    let distances: Vec<u32> = lines
        .next()
        .unwrap()
        .replace("Distance:", "")
        .split(' ')
        .map(|e| e.parse::<u32>().unwrap_or(0))
        .filter(|e| *e != 0)
        .collect();

    let mut races: Vec<Race> = vec![];
    for i in 0..times.len() {
        races.push(Race {
            time: times[i],
            distance: distances[i],
        });
    }

    println!(
        "Number of ways you can beat the record: {}",
        races
            .iter()
            .map(get_ways_to_beat_record)
            .fold(1, |acc, e| acc * e)
    );
}
