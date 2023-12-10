use std::{collections::HashMap, fs};

fn get_file_content(file_path: &String) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

type Coordinates = (u32, u32);

type Schematic = HashMap<Coordinates, char>;

fn parse_schematic(file_content: &String) -> Schematic {
    let mut schematic: Schematic = HashMap::new();
    let mut w = 1;
    let mut h = 1;
    for line in file_content.lines() {
        for char in line.chars() {
            schematic.insert((w, h), char);
            w += 1;
        }
        h += 1;
        w = 1;
    }
    return schematic;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_schematic_01() {
        let schematic = parse_schematic(&String::from(".6+\n54*"));
        assert_eq!(schematic.get(&(1, 1)), Some(&'.'));
        assert_eq!(schematic.get(&(2, 1)), Some(&'6'));
        assert_eq!(schematic.get(&(3, 1)), Some(&'+'));
        assert_eq!(schematic.get(&(1, 2)), Some(&'5'));
        assert_eq!(schematic.get(&(2, 2)), Some(&'4'));
        assert_eq!(schematic.get(&(3, 2)), Some(&'*'));
        assert_eq!(schematic.get(&(1, 3)), None);
    }
}

const LEN_MAX: i32 = 140;
const HEIGHT_MAX: i32 = 140;

/// If applicable, retrieve the gear center of a possible gear part
fn get_gear_center(schematic: &Schematic, coordinates: &Coordinates) -> Option<Coordinates> {
    let casted_coordinates: (i32, i32) = (coordinates.0 as i32, coordinates.1 as i32);
    for h in (casted_coordinates.1 - 1).clamp(0, HEIGHT_MAX) as u32
        ..=(casted_coordinates.1 + 1).clamp(0, HEIGHT_MAX) as u32
    {
        for w in (casted_coordinates.0 - 1).clamp(0, LEN_MAX) as u32
            ..=(casted_coordinates.0 + 1).clamp(0, LEN_MAX) as u32
        {
            if schematic.get(&(w, h)).unwrap_or(&'.') == &'*' {
                return Some((w, h));
            }
        }
    }
    return None;
}

#[derive(Debug)]
struct GearPart {
    number: u32,
    gear_center: Coordinates,
}

fn get_potential_gear_parts(schematic: &Schematic) -> Vec<GearPart> {
    let mut gear_parts: Vec<GearPart> = vec![];

    // The flag that will tell us to keep accumulating
    // digits to form the part number
    let mut acc = false;

    // The flag that will tell us that the part is valid
    // (it has a symbol around it)
    let mut should_add_part = false;

    // Temporary vector to accumulate digits to form the part number
    let mut temp_chars: Vec<char> = vec![];
    let mut temp_gear_center: Coordinates = (0, 0);

    for h in 1..=HEIGHT_MAX as u32 {
        for w in 1..=LEN_MAX as u32 {
            let coordinates = (w, h);
            let c = schematic.get(&coordinates).unwrap();
            if c.is_ascii_digit() {
                if !acc {
                    acc = true;
                }
                temp_chars.push(c.clone());
                if let Some(gear_center) = get_gear_center(&schematic, &coordinates) {
                    should_add_part = true;
                    temp_gear_center = gear_center;
                }
            } else {
                if acc {
                    let new_gear_part = GearPart {
                        number: temp_chars
                            .iter()
                            .collect::<String>()
                            .parse()
                            .expect("Not a valid number"),
                        gear_center: temp_gear_center,
                    };

                    print!("New gear part: {:?}", new_gear_part);
                    if should_add_part {
                        println!(" will be added");
                        gear_parts.push(new_gear_part);
                    } else {
                        println!(" will NOT be added");
                    }
                    // Clean up for next part
                    acc = false;
                    should_add_part = false;
                    temp_chars.clear();
                    temp_gear_center = (0, 0);
                }
            }
        }
    }

    return gear_parts;
}

fn get_gear_centers(schematic: &Schematic) -> Vec<Coordinates> {
    let mut gear_centers: Vec<Coordinates> = vec![];
    for h in 1..=HEIGHT_MAX as u32 {
        for w in 1..=LEN_MAX as u32 {
            let coordinates = (w, h);
            if schematic.get(&coordinates).unwrap() == &'*' {
                gear_centers.push(coordinates);
            }
        }
    }
    return gear_centers;
}

fn main() {
    let file_path: String = String::from("assets/input");
    let file_content = get_file_content(&file_path);

    let schematic = parse_schematic(&file_content);

    let potential_gear_parts = get_potential_gear_parts(&schematic);
    let potential_gear_centers = get_gear_centers(&schematic);

    let mut gear_ratio_sum = 0;
    for potential_gear_center in potential_gear_centers {
        let gear_parts_for_gear_center = potential_gear_parts
            .iter()
            .filter(|e| e.gear_center == potential_gear_center);
        if gear_parts_for_gear_center.clone().count() == 2 {
            let gear_ratio: u32 = gear_parts_for_gear_center.fold(1, |acc, e| acc * e.number);
            gear_ratio_sum += gear_ratio;
        }
    }

    println!("Gear ratio sum: {}", gear_ratio_sum);
}
