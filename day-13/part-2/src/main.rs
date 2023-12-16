use std::fs;

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

type Coords = (usize, usize);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum MirrorCompare {
    NoSmudge,
    OneSmudge(Coords),
    TooManySmudges,
}

fn are_equal_modulo_one_smudge(a: &str, b: &str, height: usize) -> MirrorCompare {
    let mut maybe_smudge: MirrorCompare = MirrorCompare::NoSmudge;
    for i in 0..a.len() {
        let tmp_a = a.chars().nth(i).unwrap();
        let tmp_b = b.chars().nth(i).unwrap();
        if tmp_a != tmp_b {
            match maybe_smudge {
                MirrorCompare::NoSmudge => {
                    maybe_smudge = MirrorCompare::OneSmudge((i, height));
                }
                MirrorCompare::OneSmudge(_) => {
                    maybe_smudge = MirrorCompare::TooManySmudges;
                    break;
                }
                _ => {}
            };
        }
    }
    maybe_smudge
}

#[cfg(test)]
mod tests_are_equal_modulo_one_smudge {
    use super::*;

    #[test]
    fn equal_without_smudge() {
        assert_eq!(
            are_equal_modulo_one_smudge("....", "....", 0),
            MirrorCompare::NoSmudge
        );
    }

    #[test]
    fn equal_with_smudge() {
        assert_eq!(
            are_equal_modulo_one_smudge(".#..", "....", 0),
            MirrorCompare::OneSmudge((1, 0))
        );
    }

    #[test]
    fn equal_with_smudge_other_side() {
        assert_eq!(
            are_equal_modulo_one_smudge("....", "..#.", 0),
            MirrorCompare::OneSmudge((2, 0))
        );
    }

    #[test]
    fn equal_with_more_than_one_smudge() {
        assert_eq!(
            are_equal_modulo_one_smudge("#...", "..#.", 0),
            MirrorCompare::TooManySmudges
        );
    }

    #[test]
    fn equal_with_one_smudge_01() {
        assert_eq!(
            are_equal_modulo_one_smudge("#.##..##.", "..##..##.", 0),
            MirrorCompare::OneSmudge((0, 0))
        );
    }

    #[test]
    fn equal_with_one_smudge_02() {
        assert_eq!(
            are_equal_modulo_one_smudge("#...##..#", "#....#..#", 1),
            MirrorCompare::OneSmudge((4, 1))
        );
    }
}

fn check_mirror_hypothesis(
    index: usize,
    lines: &Vec<&str>,
    smudge: MirrorCompare,
) -> (usize, MirrorCompare) {
    let mut maybe_smudge = smudge.clone(); // Temp value for spread analysis
    let i = index + 1; // Real index
    let mirror_position = i;
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
                match are_equal_modulo_one_smudge(a, b, mirror_position - 1 - spread) {
                    MirrorCompare::TooManySmudges => {
                        ok = false;
                        break;
                    }
                    // The only case that could work is if there are no smudge on the
                    // current mirror position
                    MirrorCompare::OneSmudge(x) => match smudge {
                        MirrorCompare::NoSmudge => {
                            maybe_smudge = MirrorCompare::OneSmudge(x);
                            spread += 1;
                        }
                        _ => {
                            return (0, MirrorCompare::TooManySmudges);
                        }
                    },
                    MirrorCompare::NoSmudge => {
                        // maybe_smudge = smudge.clone();
                        spread += 1;
                    }
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
        return (mirror_position, maybe_smudge);
    } else {
        return (0, MirrorCompare::TooManySmudges);
    }
}

fn count_above(unclean_block: &str) -> (usize, MirrorCompare) {
    let block = unclean_block.replace(" ", "");

    let lines: Vec<&str> = block.split_whitespace().collect();

    // There's a trick here: due to the skip, index is actually
    // the real index - 1
    for (index, line) in lines.iter().skip(1).enumerate() {
        let maybe_smudge = are_equal_modulo_one_smudge(*line, lines[index], index + 1);
        match maybe_smudge {
            // No need to check if it's a mirror
            // if we already have too many smudged on our hands
            MirrorCompare::TooManySmudges => {
                continue;
            }
            _ => {
                let tmp = check_mirror_hypothesis(index, &lines, maybe_smudge);
                match tmp.1 {
                    // Valid mirror is returned
                    MirrorCompare::OneSmudge(_) => return tmp,
                    // Invalid mirror is ignored and another one is tested
                    MirrorCompare::NoSmudge => continue,
                    MirrorCompare::TooManySmudges => continue,
                }
            }
        }
    }
    return (0, MirrorCompare::TooManySmudges);
}

#[cfg(test)]
mod tests_count_above {
    use super::*;

    #[test]
    fn count_above_01() {
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
            (3, MirrorCompare::OneSmudge((0, 0)))
        );
    }

    #[test]
    fn count_above_02() {
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
            (1, MirrorCompare::OneSmudge((4, 1)))
        );
    }

    #[test]
    fn count_above_03() {
        assert_eq!(
            count_above(
                "
.##..#..##..#.#
..##....#.#..#.
..##....#.#..#.
.....#...#..#..
.##..##.###.###
.##..##.###.###
.....#...#..#..
..##....#.#..#.
..##....#.#..##
.##..#..##..#.#"
            ),
            (5, MirrorCompare::OneSmudge((14, 1)))
        );
    }

    #[test]
    fn count_above_04() {
        assert_eq!(
            count_above(
                "
..##....##..###
.#.##..##.#...#
.#.##..##.#...#
..##....##..#.#
##..#..#..####.
.#.##..##.#..#.
#####..#####.#.
..###..###..##.
...#.##.#....#.
#..#.##.#..#...
#..##..##..####"
            ),
            (2, MirrorCompare::OneSmudge((13, 0)))
        );
    }
}

fn rotate(block: &str) -> Vec<String> {
    let lines = block.split_whitespace();
    let first_line = lines.clone().nth(0).unwrap();
    let mut columns: Vec<String> = vec![];
    for i in 0..first_line.len() {
        let column = lines
            .clone()
            .map(|l| l.to_string().chars().nth(i).unwrap().to_string())
            .rev()
            .collect();
        columns.push(column);
    }
    columns
}

#[cfg(test)]
mod tests_rotate {
    use super::*;

    #[test]
    fn rotate_01() {
        assert_eq!(
            rotate(
                "
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."
            )
            .join("\n"),
            "#..##.#
...##..
###..##
.#....#
#.#..#.
#.#..#.
.#....#
###..##
...##.."
        );
    }

    #[test]
    fn rotate_02() {
        assert_eq!(
            rotate(
                "
##.#..###.#####
###.#.##.#..##.
###.####.#..##.
##.#..###.#####
####......#####
#.#..##.#......
##.#.....#.####
......#........
.##..#..####..#
#...####.##....
.#.###..#......
##.#.#.#.#.#..#
..##.##..##.##."
            )
            .join("\n"),
            ".#.#..#######
.##.#.#.#####
#...#..##.##.
###...#.##..#
..##......##.
#####..#..#..
#..#.#.#.####
.#.#.....####
..#.#..#.#..#
##.##.#...##.
#..##...##..#
.#..#.#.##..#
#.....#.#####
#.....#.#####
.#..#.#.##..#"
        );
    }
}

// Count left work by first rotating the block by 90 degrees and then apply the
// same algorithm as if it was a regular block. It's "above" count is thus
// its "left" count.
fn count_left(unclean_block: &str) -> (usize, MirrorCompare) {
    count_above(rotate(&unclean_block.replace(" ", "")).join("\n").as_str())
}

#[cfg(test)]
mod tests_count_left {
    use super::*;

    #[test]
    fn count_left_01() {
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
            (0, MirrorCompare::TooManySmudges)
        );
    }

    #[test]
    fn count_left_02() {
        assert_eq!(
            count_left(
                "
.########..
###.##.####
#.#....#.##
###....####
.#..##..#..
##.#..#.###
..######...
#..#..#..#.
.#.####.#.."
            ),
            (10, MirrorCompare::OneSmudge((1, 10)))
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
                println!("");
                println!("Handling block #{}", index);
                let above = count_above(b);
                let left = count_left(b);

                println!("Index: {}, Above: {:?}, Left: {:?}", index, above, left);

                let (a, l) = match (above.1, left.1) {
                    (MirrorCompare::NoSmudge, MirrorCompare::NoSmudge) => {
                        panic!("No smudge on either side !")
                    }
                    (MirrorCompare::NoSmudge, MirrorCompare::OneSmudge(_)) => (0, left.0),
                    (MirrorCompare::OneSmudge(_), MirrorCompare::NoSmudge) => (above.0, 0),

                    (MirrorCompare::NoSmudge, MirrorCompare::TooManySmudges) => panic!("DERP"),
                    (MirrorCompare::TooManySmudges, MirrorCompare::NoSmudge) => panic!("DERP"),

                    (MirrorCompare::OneSmudge(_), MirrorCompare::TooManySmudges) => (above.0, 0),
                    (MirrorCompare::TooManySmudges, MirrorCompare::OneSmudge(_)) => (0, left.0),

                    (MirrorCompare::OneSmudge(_), MirrorCompare::OneSmudge(_)) => {
                        panic!("Don't know what to do !")
                    }
                    (MirrorCompare::TooManySmudges, MirrorCompare::TooManySmudges) => {
                        panic!("Don't know what to do !")
                    }
                };

                let result = l + 100 * a;
                println!("Result is: {}", result);
                result
            })
            .sum::<usize>()
    );
}
