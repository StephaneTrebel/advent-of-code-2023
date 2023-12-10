use std::fs;

fn get_file_content(file_path: &String) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

fn parse_seeds(line: &str) -> Vec<u32> {
    return line
        .replace("seeds: ", "")
        .split(" ")
        .map(|n| n.parse().unwrap_or(0))
        .collect();
}

#[cfg(test)]
mod tests_parse_seeds {
    use super::*;

    #[test]
    fn parse_seed_ok() {
        assert_eq!(
            parse_seeds("seeds: 3136945476 509728956 1904897211 495273540 1186343315 66026055 1381149926 11379441 4060485949 190301545 444541979 351779229 1076140984 104902451 264807001 60556152 3676523418 44140882 3895155702 111080695"),
            vec![
3136945476,509728956,1904897211,495273540,1186343315,66026055,1381149926,11379441,4060485949,190301545,444541979,351779229,1076140984,104902451,264807001,60556152,3676523418,44140882,3895155702,111080695
            ]
        );
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Info {
    destination_range_start: u32,
    source_range_start: u32,
    range_length: u32,
}

type Map = Vec<Info>;

trait Sort {
    fn sort(&mut self);
}

impl Sort for Map {
    fn sort(&mut self) {
        self.sort_by(|a, b| {
            a.source_range_start
                .partial_cmp(&b.source_range_start)
                .unwrap()
        });
    }
}

fn parse_to_map(lines: &Vec<String>) -> Map {
    let mut out = vec![];
    for line in lines {
        let mut split = line.split(' ');
        out.push(Info {
            destination_range_start: split.next().unwrap().parse().unwrap(),
            source_range_start: split.next().unwrap().parse().unwrap(),
            range_length: split.next().unwrap().parse().unwrap(),
        });
    }
    out.sort();
    return out;
}

#[cfg(test)]
mod tests_parse_to_map {
    use super::*;

    #[test]
    fn parse_to_map_01() {
        assert_eq!(
            parse_to_map(&vec![String::from("3 1 5"), String::from("6 3 2")]),
            vec![
                Info {
                    destination_range_start: 3,
                    source_range_start: 1,
                    range_length: 5
                },
                Info {
                    destination_range_start: 6,
                    source_range_start: 3,
                    range_length: 2
                }
            ]
        );
    }
}

/// map better be sorted ! Use .sort() for that
fn get_destination(source: u32, map: &Map) -> u32 {
    if map.len() == 0 {
        return source;
    }
    let entry: &Info = {
        match map.iter().filter(|x| source >= x.source_range_start).last() {
            Some(mapping) => mapping,
            _ => map.last().unwrap(),
        }
    };
    let delta = source as i64 - entry.source_range_start as i64;
    if delta <= entry.range_length as i64 && delta > 0 {
        entry.destination_range_start + (delta as u32)
    } else {
        source
    }
}

#[cfg(test)]
mod tests_get_destination {
    use super::*;

    #[test]
    fn get_destination_found() {
        assert_eq!(
            get_destination(
                3,
                &vec![
                    Info {
                        destination_range_start: 1,
                        source_range_start: 2,
                        range_length: 5,
                    },
                    Info {
                        destination_range_start: 10,
                        source_range_start: 15,
                        range_length: 9,
                    },
                ]
            ),
            2
        );
    }

    #[test]
    fn get_destination_not_found() {
        assert_eq!(
            get_destination(
                999,
                &vec![
                    Info {
                        destination_range_start: 1,
                        source_range_start: 2,
                        range_length: 5,
                    },
                    Info {
                        destination_range_start: 10,
                        source_range_start: 15,
                        range_length: 9,
                    },
                ]
            ),
            999
        );
    }

    #[test]
    fn get_destination_out_of_range() {
        assert_eq!(
            get_destination(
                22,
                &vec![
                    Info {
                        destination_range_start: 1,
                        source_range_start: 2,
                        range_length: 5,
                    },
                    Info {
                        destination_range_start: 10,
                        source_range_start: 15,
                        range_length: 3,
                    },
                    Info {
                        destination_range_start: 10,
                        source_range_start: 45,
                        range_length: 100,
                    },
                ]
            ),
            22
        );
    }

    #[test]
    fn get_destination_last_entry() {
        assert_eq!(
            get_destination(
                62,
                &vec![
                    Info {
                        destination_range_start: 1,
                        source_range_start: 2,
                        range_length: 5,
                    },
                    Info {
                        destination_range_start: 10,
                        source_range_start: 15,
                        range_length: 9,
                    },
                    Info {
                        destination_range_start: 10,
                        source_range_start: 45,
                        range_length: 100,
                    },
                ]
            ),
            27
        );
    }
}

fn main() {
    let content = get_file_content(&String::from("assets/input"));

    let mut seeds: Vec<u32> = vec![];
    let mut seed_to_soil_map: Map = vec![];
    let mut soil_to_fertilizer_map: Map = vec![];
    let mut fertilizer_to_water_map: Map = vec![];
    let mut water_to_light_map: Map = vec![];
    let mut light_to_temperature_map: Map = vec![];
    let mut temperature_to_humidity_map: Map = vec![];
    let mut humidity_to_location_map: Map = vec![];

    let mut acc: Vec<String> = vec![];
    let mut lines = content.lines();
    while let Some(line) = lines.next() {
        match line {
            l if l.starts_with("seeds:") => {
                print!("Parsing seeds");
                seeds = parse_seeds(line);
                println!("...Done !");
            }
            l if l.starts_with("seed-to-soil") => {
                while let Some(l) = lines.next() {
                    if l == "" {
                        break;
                    }
                    acc.push(String::from(l));
                }
                print!("Parsing seed_to_soil_map");
                seed_to_soil_map = parse_to_map(&acc);
                println!("...Done !");
                acc = vec![];
            }
            l if l.starts_with("soil-to-fertilizer") => {
                while let Some(l) = lines.next() {
                    if l == "" {
                        break;
                    }
                    acc.push(String::from(l));
                }
                print!("Parsing soil_to_fertilizer_map");
                soil_to_fertilizer_map = parse_to_map(&acc);
                println!("...Done !");
                acc = vec![];
            }
            l if l.starts_with("fertilizer-to-water") => {
                while let Some(l) = lines.next() {
                    if l == "" {
                        break;
                    }
                    acc.push(String::from(l));
                }
                print!("Parsing fertilizer_to_water_map");
                fertilizer_to_water_map = parse_to_map(&acc);
                println!("...Done !");
                acc = vec![];
            }
            l if l.starts_with("water-to-light") => {
                while let Some(l) = lines.next() {
                    if l == "" {
                        break;
                    }
                    acc.push(String::from(l));
                }
                print!("Parsing water_to_light_map");
                water_to_light_map = parse_to_map(&acc);
                println!("...Done !");
                acc = vec![];
            }
            l if l.starts_with("light-to-temperature") => {
                while let Some(l) = lines.next() {
                    if l == "" {
                        break;
                    }
                    acc.push(String::from(l));
                }
                print!("Parsing light_to_temperature_map");
                light_to_temperature_map = parse_to_map(&acc);
                println!("...Done !");
                acc = vec![];
            }
            l if l.starts_with("temperature-to-humidity") => {
                while let Some(l) = lines.next() {
                    if l == "" {
                        break;
                    }
                    acc.push(String::from(l));
                }
                print!("Parsing temperature_to_humidity_map");
                temperature_to_humidity_map = parse_to_map(&acc);
                println!("...Done !");
                acc = vec![];
            }
            l if l.starts_with("humidity-to-location") => {
                while let Some(l) = lines.next() {
                    if l == "" {
                        break;
                    }
                    acc.push(String::from(l));
                }
                print!("Parsing humidity_to_location_map");
                humidity_to_location_map = parse_to_map(&acc);
                println!("...Done !");
                acc = vec![];
            }
            _ => {}
        }
    }

    println!(
        "Location: {}",
        seeds
            .iter()
            .map(|seed| {
                get_destination(
                    get_destination(
                        get_destination(
                            get_destination(
                                get_destination(
                                    get_destination(
                                        get_destination(*seed, &seed_to_soil_map),
                                        &soil_to_fertilizer_map,
                                    ),
                                    &fertilizer_to_water_map,
                                ),
                                &water_to_light_map,
                            ),
                            &light_to_temperature_map,
                        ),
                        &temperature_to_humidity_map,
                    ),
                    &humidity_to_location_map,
                )
            })
            .min()
            .unwrap()
    );
}
