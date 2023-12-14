use itertools::{repeat_n, Itertools};
use memoize::memoize;
use std::fs;

fn get_file_content(file_path: &String) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Row {
    spring_list: String,
    expected_broken_list: Vec<usize>,
}

fn parse_line(line: &str) -> Row {
    let mut split = line.split_whitespace();

    Row {
        spring_list: split.next().unwrap().to_string(),
        expected_broken_list: split
            .next()
            .unwrap()
            .split(',')
            .map(|c| c.parse().unwrap())
            .collect_vec(),
    }
}

#[cfg(test)]
mod tests_parse_line {
    use super::*;

    #[test]
    fn tests_parse_line_01() {
        assert_eq!(
            parse_line(&"#.#.### 1,1,3"),
            Row {
                spring_list: "#.#.###".to_string(),
                expected_broken_list: vec![1, 1, 3]
            }
        );
    }
}

trait Unfold {
    fn unfold(&mut self, fold_size: usize);
}

impl Unfold for Row {
    fn unfold(&mut self, fold_size: usize) {
        if fold_size > 0 {
            self.spring_list = repeat_n(self.spring_list.clone(), fold_size).join("?");

            self.expected_broken_list = repeat_n(self.expected_broken_list.clone(), fold_size)
                .map(|e| e.iter().join(","))
                .join(",")
                .split(',')
                .map(|c| c.parse().unwrap())
                .collect_vec();
        }
    }
}

#[cfg(test)]
mod tests_unfold {
    use super::*;

    #[test]
    fn unfold_01() {
        let mut row = parse_line(".# 1");
        row.unfold(5);
        assert_eq!(row.spring_list, ".#?.#?.#?.#?.#");
        assert_eq!(row.expected_broken_list, vec![1, 1, 1, 1, 1]);
    }

    #[test]
    fn unfold_02() {
        let mut row = parse_line("???.### 1,1,3");
        row.unfold(5);
        assert_eq!(row.spring_list, "???.###????.###????.###????.###????.###");
        assert_eq!(
            row.expected_broken_list,
            vec![1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3]
        );
    }
}

#[memoize]
fn recursive_count(spring_list: String, group_list: Vec<usize>) -> u64 {
    let next_char = spring_list.chars().next();
    let next_group = group_list.get(0);

    match next_group {
        None => {
            // We compare nothing and nothing, a Tail Call means 1
            if next_char == None {
                // println!("and spring list is empty, too, returning 1");
                return 1;
            }
            // If we don't have a remaining group, but we have at least one broken
            // spring, then it's an error
            if spring_list.contains("#") {
                return 0;
            }
            // If not, we have a combination of "." and "?" springs,
            // which is fine: if we consider '?' as '.', it matches
            else {
                return 1;
            }
        }
        Some(group) => {
            return match next_char {
                Some('#') => {
                    // This is the meat of the algorithm.
                    // We'll attempt to build a possible group and check that
                    // against the expected group
                    let possible_group = spring_list
                        .chars()
                        .take(*group)
                        .collect_vec()
                        .iter()
                        .map(|e| if *e == '?' { '#' } else { *e })
                        .collect_vec();

                    // Could not take enough char to create a possible group
                    // -> Failure
                    if possible_group.len() != *group {
                        return 0;
                    }

                    // let expected_group = (0..).take(*group).map(|_| '#').collect_vec();
                    let expected_group = vec!['#'; *group];

                    // The resulting group should be the same the expected,
                    // or else we have a failure
                    if possible_group != expected_group {
                        return 0;
                    }

                    if spring_list.len() == *group {
                        // If our current list is the same size as the group,
                        // we ensure that there are no group afterward to check
                        if group_list.len() == 1 {
                            // If there aren't, we have finished -> Success !
                            return 1;
                        } else {
                            // If there are there won't be enough spring to make
                            // a possible group so -> Failure
                            return 0;
                        }
                    }

                    // If we have a broker spring right after the group it's a
                    // failure since it would have been included in the expected
                    // group -> Failure
                    if let Some('#') = spring_list.chars().skip(*group).next() {
                        return 0;
                    }

                    // If this group is ok, iterate on the next group
                    // (included here is the assumed "." that separates groups)
                    return recursive_count(
                        spring_list.chars().skip(group + 1).collect(),
                        group_list.iter().skip(1).map(|e| *e).collect_vec(),
                    );
                }

                // Simplest case: There is nothing to do except ignoring a
                // functionning spring, and moving on
                Some('.') => recursive_count(spring_list.chars().skip(1).collect(), group_list),

                Some('?') => {
                    // It is here that we will Add arrangements, based on the forking
                    // on '?' chararcters
                    recursive_count(spring_list.replacen("?", "#", 1), group_list.clone())
                        + recursive_count(spring_list.replacen("?", ".", 1), group_list.clone())
                }
                _ => 0,
            };
        }
    }
}

#[cfg(test)]
mod tests_recursive_count {
    use super::*;

    #[test]
    fn recursive_count_01() {
        assert_eq!(recursive_count(".".to_string(), vec![1]), 0);
    }

    #[test]
    fn recursive_count_02() {
        assert_eq!(recursive_count("#".to_string(), vec![1]), 1);
    }

    #[test]
    fn recursive_count_03() {
        assert_eq!(recursive_count("#.#".to_string(), vec![1, 1]), 1);
    }

    #[test]
    fn recursive_count_04() {
        assert_eq!(recursive_count("#?#".to_string(), vec![1]), 0);
    }

    #[test]
    fn recursive_count_05() {
        let row = parse_line(".###.##..#.. 3,2,1");
        assert_eq!(
            recursive_count(row.spring_list, row.expected_broken_list),
            1
        );
    }

    #[test]
    fn recursive_count_06() {
        let row = parse_line("???#.##..#.. 3,2,1");
        assert_eq!(
            recursive_count(row.spring_list, row.expected_broken_list),
            1
        );
    }
}

fn calc(row: &Row) -> u64 {
    let result = recursive_count(row.spring_list.clone(), row.expected_broken_list.clone());
    // print!("Result is: {}", result);
    result
}

#[cfg(test)]
mod tests_calc {
    use super::*;

    #[test]
    fn calc_01() {
        assert_eq!(calc(&parse_line("???.### 1,1,3")), 1);
    }

    #[test]
    fn calc_02() {
        assert_eq!(calc(&parse_line(".??..??...?##. 1,1,3")), 4);
    }

    #[test]
    fn calc_03() {
        assert_eq!(calc(&parse_line("?###???????? 3,2,1")), 10);
    }
}

fn main() {
    let content = get_file_content(&String::from("assets/input"));
    let mut row_list: Vec<Row> = content.lines().map(parse_line).collect();

    print!("Unfolding…");
    for row in row_list.iter_mut() {
        row.unfold(5);
    }
    println!("…Done !");

    println!("Computing permutations calculations");

    println!(
        "Permutation sum: {:?}",
        row_list
            .iter()
            .enumerate()
            .map(|(index, row)| {
                println!("Computing permutations for row #{}", index);
                calc(&row)
            })
            .sum::<u64>()
    );
}
