use std::fs;

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

fn hash(input: &str) -> usize {
    input
        .chars()
        .fold(0, |acc, c| ((acc + (c as usize)) * 17) % 256)
}

#[cfg(test)]
mod tests_hash {
    use super::*;

    #[test]
    fn hash_01() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn hash_02() {
        assert_eq!(hash("rn"), 0);
    }

    #[test]
    fn hash_03() {
        assert_eq!(hash("cm-"), 253);
    }
    #[test]
    fn hash_04() {
        assert_eq!(hash("qp=3"), 97);
    }
    #[test]
    fn hash_05() {
        assert_eq!(hash("cm=2"), 47);
    }
    #[test]
    fn hash_06() {
        assert_eq!(hash("qp-"), 14);
    }
    #[test]
    fn hash_07() {
        assert_eq!(hash("pc=4"), 180);
    }
    #[test]
    fn hash_08() {
        assert_eq!(hash("ot=9"), 9);
    }
    #[test]
    fn hash_09() {
        assert_eq!(hash("ab=5"), 197);
    }
    #[test]
    fn hash_10() {
        assert_eq!(hash("pc-"), 48);
    }
    #[test]
    fn hash_11() {
        assert_eq!(hash("pc=6"), 214);
    }
    #[test]
    fn hash_12() {
        assert_eq!(hash("ot=7"), 231);
    }
}

type Box<'a> = Vec<(&'a str, usize)>;

fn process<'a>(base_box: Box<'a>, instruction: &'a str) -> Box<'a> {
    let mut new_box = base_box.clone();
    if instruction.contains("=") {
        let mut split = instruction.split("=");
        let lens_name = split.next().unwrap();
        let lens_power = split.next().unwrap().parse::<usize>().unwrap();
        match new_box.iter().enumerate().find(|(_, l)| l.0 == lens_name) {
            Some((index, _)) => new_box[index] = (lens_name, lens_power),
            None => new_box.push((lens_name, lens_power)),
        };
    } else if instruction.contains("-") {
        let mut split = instruction.split("-");
        let lens_name = split.next().unwrap();
        match new_box.iter().enumerate().find(|(_, l)| l.0 == lens_name) {
            Some((index, _)) => {
                new_box.remove(index);
            }
            None => {}
        };
    } else {
        panic!("Invalid instruction");
    }
    new_box
}

#[cfg(test)]
mod tests_process {
    use super::*;

    #[test]
    fn process_add() {
        let my_box: Box = vec![];
        assert_eq!(process(my_box, &"rn=1"), vec![("rn", 1)]);
    }

    #[test]
    fn process_remove() {
        let my_box: Box = vec![("rn", 1)];
        assert_eq!(process(my_box, &"rn-"), vec![]);
    }

    #[test]
    fn process_replace() {
        let my_box: Box = vec![("rn", 1), ("qb", 2), ("fo", 3)];
        assert_eq!(
            process(my_box, &"qb=4"),
            vec![("rn", 1), ("qb", 4), ("fo", 3)]
        );
    }

    #[test]
    fn process_remove_among_other() {
        let my_box: Box = vec![("rn", 1), ("qb", 2), ("fo", 3)];
        assert_eq!(process(my_box, &"qb-"), vec![("rn", 1), ("fo", 3)]);
    }
}

fn process_multiple<'a>(boxes: &Vec<Box<'a>>, instructions: Vec<&'a str>) -> Vec<Box<'a>> {
    let mut out_boxes: Vec<Box<'a>> = boxes.clone();
    for instruction in instructions {
        println!("");
        println!("Instruction: {}", instruction);
        let split_char = {
            if instruction.contains("=") {
                "="
            } else if instruction.contains("-") {
                "-"
            } else {
                panic!("Invalid instruction: {}", instruction)
            }
        };
        let lens_name = instruction.split(split_char).next().unwrap();
        let box_index = hash(lens_name);
        let tmp_box = out_boxes[box_index].clone();
        out_boxes[box_index] = process(tmp_box, instruction);
    }
    out_boxes
}

#[cfg(test)]
mod tests_process_multiple {
    use super::*;

    #[test]
    fn process_multiple_01() {
        assert_eq!(
            process_multiple(
                &vec![vec![], vec![], vec![], vec![]],
                vec![
                    "rn=1", "cm-", "qp=3", "cm=2", "qp-", "pc=4", "ot=9", "ab=5", "pc-", "pc=6",
                    "ot=7"
                ]
            ),
            vec![
                vec![("rn", 1), ("cm", 2)],
                vec![],
                vec![],
                vec![("ot", 7), ("ab", 5), ("pc", 6)]
            ]
        );
    }
}

fn calc_lens_power(input_box: &Box, box_index: usize) -> usize {
    input_box
        .iter()
        .enumerate()
        .map(|(index, lens)| (box_index + 1) * (index + 1) * lens.1)
        .sum()
}

#[cfg(test)]
mod tests_calc_lens_power {
    use super::*;

    #[test]
    fn calc_lens_power_01() {
        let my_box: Box = vec![];
        assert_eq!(calc_lens_power(&my_box, 0), 0);
    }

    #[test]
    fn calc_lens_power_02() {
        let my_box: Box = vec![("rn", 1), ("cm", 2)];
        assert_eq!(calc_lens_power(&my_box, 0), 5);
    }

    #[test]
    fn calc_lens_power_03() {
        let my_box: Box = vec![("ot", 7), ("ab", 5), ("pc", 6)];
        assert_eq!(calc_lens_power(&my_box, 3), 140);
    }
}

fn main() {
    let content = get_file_content("assets/input");

    let mut boxes: Vec<Box> = vec![];
    for _ in 0..256 {
        boxes.push(vec![]);
    }

    let instructions: Vec<&str> = content.lines().next().unwrap().split(',').collect();

    println!(
        "Result: {:?}",
        process_multiple(&boxes, instructions)
            .iter()
            .enumerate()
            .map(|(box_index, curr_box)| calc_lens_power(curr_box, box_index))
            .sum::<usize>()
    );
}
