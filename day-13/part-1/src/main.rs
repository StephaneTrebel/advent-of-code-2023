use std::fs;

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

fn count_above(unclean_block: &str) -> usize {
    let block = unclean_block.replace(" ", "");

    let lines: Vec<&str> = block.split_whitespace().collect();

    // There's a trick here: due to the skip, index is actually
    // the real index - 1
    for (index, line) in lines.iter().skip(1).enumerate() {
        // Comparing the current line and the one directly above it will give us
        // the possible mirror position as an index.
        if *line == lines[index] {
            let mirror_position = index + 1;
            let mut spread = 1;
            let mut ok = true;
            loop {
                if (mirror_position as i32) - 1 - (spread as i32) < 0 {
                    break;
                }
                match (
                    lines.get(mirror_position - 1 - spread),
                    lines.get(mirror_position + spread),
                ) {
                    (Some(a), Some(b)) => {
                        if a != b {
                            ok = false;
                            break;
                        } else {
                            spread += 1;
                        }
                    }
                    _ => {
                        // No reason to pursue since at least one side is missing
                        // for a comparison
                        break;
                    }
                }
            }
            if ok {
                return mirror_position;
            }
        }
    }

    return 0;
}

#[cfg(test)]
mod tests_count_above {
    use super::*;

    #[test]
    fn count_above_01() {
        assert_eq!(
            count_above(
                "
#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
            ),
            4
        );
    }

    #[test]
    fn count_above_02() {
        assert_eq!(
            count_above(
                "
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."
            ),
            0
        );
    }

    #[test]
    fn count_above_03() {
        assert_eq!(
            count_above(
                "
#..##.#
...##..
###..##
.#....#
#.#..#.
#.#..#.
.#....#
###..##
...##.."
            ),
            5
        );
    }
}

fn count_left(unclean_block: &str) -> usize {
    let block = unclean_block.replace(" ", "");

    let lines = block.split_whitespace();
    let first_line = lines.clone().nth(0).unwrap();
    let mut columns: Vec<String> = vec![];

    // We rotate the block
    for i in 0..first_line.len() {
        let column = lines
            .clone()
            .map(|l| l.to_string().chars().nth(i).unwrap().to_string())
            .rev()
            .collect();
        columns.push(column);
    }

    // And then count his "lines"
    return count_above(columns.join("\n").as_str());
}

#[cfg(test)]
mod tests_count_left {
    use super::*;

    #[test]
    fn count_left_01() {
        assert_eq!(
            count_left(
                "
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."
            ),
            5
        );
    }

    #[test]
    fn count_left_02() {
        assert_eq!(
            count_left(
                "
#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
            ),
            0
        );
    }

    #[test]
    fn count_left_03() {
        assert_eq!(
            count_left(
                "
#....####.##.####
.##.###...##...##
#..#....######...
#####..#.####.#..
.##.#..#.#..#.#..
#..###.##....##.#
#..##...#.##.#...
#..#...###..###..
.##.#..#.#..#.#..
.##.##....##....#
#..##.###....###."
            ),
            11
        );
    }
}

fn main() {
    let content = get_file_content("assets/input");
    let blocks = content.split("\n\n");

    println!(
        "Result: {:?}",
        blocks
            .enumerate()
            .map(|(index, b)| {
                let left = count_left(b);
                let above = count_above(b);
                println!("Block: {}, Left: {}, Above: {}", index, left, above);
                left + 100 * above
            })
            .sum::<usize>()
    );
}
