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

fn is_symbol(c: &char) -> bool {
    return !c.is_ascii_digit() && c != &'.';
}

const LEN_MAX: i32 = 140;
const HEIGHT_MAX: i32 = 140;

fn has_symbol_around(schematic: &Schematic, coordinates: &Coordinates) -> bool {
    let casted_coordinates: (i32, i32) = (coordinates.0 as i32, coordinates.1 as i32);
    for h in (casted_coordinates.1 - 1).clamp(0, HEIGHT_MAX) as u32
        ..=(casted_coordinates.1 + 1).clamp(0, HEIGHT_MAX) as u32
    {
        for w in (casted_coordinates.0 - 1).clamp(0, LEN_MAX) as u32
            ..=(casted_coordinates.0 + 1).clamp(0, LEN_MAX) as u32
        {
            if is_symbol(schematic.get(&(w, h)).unwrap_or(&'.')) {
                return true;
            }
        }
    }
    return false;
}

fn get_part_numbers(schematic: &Schematic) -> Vec<u32> {
    let mut part_numbers: Vec<u32> = vec![];

    // The flag that will tell us to keep accumulating
    // digits to form the part number
    let mut acc = false;

    // The flag that will tell us that the part is valid
    // (it has a symbol around it)
    let mut should_add_part = false;

    // Temporary vector to accumulate digits to form the part number
    let mut temp_chars: Vec<char> = vec![];

    for h in 1..=HEIGHT_MAX as u32 {
        for w in 1..=LEN_MAX as u32 {
            let coordinates = (w, h);
            let c = schematic.get(&coordinates).unwrap();
            if c.is_ascii_digit() {
                if !acc {
                    acc = true;
                }
                temp_chars.push(c.clone());
                if has_symbol_around(&schematic, &coordinates) {
                    should_add_part = true;
                }
            } else {
                if acc {
                    let new_part = temp_chars
                        .iter()
                        .collect::<String>()
                        .parse()
                        .expect("Not a valid number");
                    print!("New part: {}", new_part);
                    if should_add_part {
                        println!(" will be added");
                        part_numbers.push(new_part);
                    } else {
                        println!(" will NOT be added");
                    }
                    // Clean up for next part
                    acc = false;
                    should_add_part = false;
                    temp_chars.clear();
                }
            }
        }
    }

    return part_numbers;
}

fn main() {
    let file_path: String = String::from("assets/input");
    let file_content = get_file_content(&file_path);

    let schematic = parse_schematic(&file_content);

    let part_numbers = get_part_numbers(&schematic);

    let mut part_numbers_sum: u32 = 0;
    for part in part_numbers {
        part_numbers_sum += part;
    }

    println!("Part numbers sum: {}", part_numbers_sum);
}
