use std::fs;

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

fn hash(input: &str) -> usize {
    input.chars().fold(0, |acc, c| {
        let result = ((acc + (c as usize)) * 17) % 256;
        println!(
            "Acc: {}, c: {}, ASCII: {}, result: {}",
            acc, c, c as usize, result
        );
        result
    })
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
        assert_eq!(hash("rn=1"), 30);
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

fn main() {
    let content = get_file_content("assets/input");

    println!("Result: {:?}", content.lines().next().unwrap().split(',').map(hash).sum::<usize>());
}
