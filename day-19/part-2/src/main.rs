use std::fs;

use eval::{to_value, Expr};
use regex::Regex;

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Instruction<'a> {
    destination: &'a str,
    predicate: Option<String>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Workflow<'a> {
    name: &'a str,
    instructions: Vec<Instruction<'a>>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Content<'a> {
    workflows: Vec<Workflow<'a>>,
}

fn parse_content<'a>(lines: &'a str) -> Content {
    let mut blocks = lines.split("\n\n");
    let re_workflow = Regex::new(r"(.+)\{(.*)\}").unwrap();

    Content {
        workflows: blocks
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                let Some(caps) = re_workflow.captures(line) else {
                    panic!("Invalid workflow: {}", line);
                };

                Workflow {
                    name: caps.get(1).unwrap().as_str(),
                    instructions: caps
                        .get(2)
                        .unwrap()
                        .as_str()
                        .split(",")
                        .map(|e| {
                            if e.contains(":") {
                                let mut s = e.split(":");
                                let c = s.next().unwrap();
                                let tmp = c.replace("<", " < ");
                                let p = tmp.replace(">", " > ");

                                Instruction {
                                    predicate: Some(p),
                                    destination: s.next().unwrap(),
                                }
                            } else {
                                Instruction {
                                    predicate: None,
                                    destination: e,
                                }
                            }
                        })
                        .collect::<Vec<Instruction>>(),
                }
            })
            .collect::<Vec<Workflow>>(),
    }
}

#[cfg(test)]
mod tests_parse_content {
    use super::*;

    #[test]
    fn parse_content_01() {
        assert_eq!(
            parse_content(
                &"\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}"
            ),
            Content {
                workflows: vec![
                    Workflow {
                        name: "px",
                        instructions: vec![
                            Instruction {
                                predicate: Some("a < 2006".to_string()),
                                destination: "qkq"
                            },
                            Instruction {
                                predicate: Some("m > 2090".to_string()),
                                destination: "A"
                            },
                            Instruction {
                                predicate: None,
                                destination: "rfg"
                            }
                        ]
                    },
                    Workflow {
                        name: "pv",
                        instructions: vec![
                            Instruction {
                                predicate: Some("a > 1716".to_string()),
                                destination: "R"
                            },
                            Instruction {
                                predicate: None,
                                destination: "A"
                            }
                        ]
                    }
                ],
            }
        );
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Combinations {
    x: Vec<usize>,
    m: Vec<usize>,
    a: Vec<usize>,
    s: Vec<usize>,
}

fn get_combo<'a>(
    base: &Vec<usize>,
    variable: &'a str,
    predicate: &'a str,
) -> (Vec<usize>, Vec<usize>) {
    (
        base.iter()
            .filter(|e| match Expr::new(predicate).value(variable, e).exec() {
                Ok(value) => {
                    if value == to_value(true) {
                        true
                    } else {
                        false
                    }
                }
                Err(e) => {
                    panic!("Error {e}");
                }
            })
            .map(|e| e.clone())
            .collect::<Vec<usize>>(),
        base.iter()
            .filter(|e| match Expr::new(predicate).value(variable, e).exec() {
                Ok(value) => {
                    if value == to_value(false) {
                        true
                    } else {
                        false
                    }
                }
                Err(e) => {
                    panic!("Error {e}");
                }
            })
            .map(|e| e.clone())
            .collect::<Vec<usize>>(),
    )
}

fn count_parts_that_satisfies_instructions<'a>(
    instructions: &Vec<Instruction<'a>>,
    current_combinations: &Combinations,
) -> Vec<(&'a str, Combinations)> {
    let re_predicate = Regex::new(r"([xmas]) ([<>]) (.*)").unwrap();
    let mut tmp_combinations = current_combinations.clone();

    instructions
        .iter()
        .flat_map(|instruction| {
            let next_destination = instruction.destination.clone();
            match &instruction.predicate {
                Some(predicate) => {
                    let caps = re_predicate.captures(predicate).unwrap();
                    let variable = &caps[1];

                    match variable {
                        "x" => {
                            let (if_x, else_x) = get_combo(&tmp_combinations.x, "x", &predicate);
                            let mut if_tmp = tmp_combinations.clone();
                            if_tmp.x = if_x;
                            tmp_combinations.x = else_x;
                            vec![(next_destination, if_tmp)]
                        }
                        "m" => {
                            let (if_m, else_m) = get_combo(&tmp_combinations.m, "m", &predicate);
                            let mut if_tmp = tmp_combinations.clone();
                            if_tmp.m = if_m;
                            tmp_combinations.m = else_m;
                            vec![(next_destination, if_tmp)]
                        }
                        "a" => {
                            let (if_a, else_a) = get_combo(&tmp_combinations.a, "a", &predicate);
                            let mut if_tmp = tmp_combinations.clone();
                            if_tmp.a = if_a;
                            tmp_combinations.a = else_a;
                            vec![(next_destination, if_tmp)]
                        }
                        "s" => {
                            let (if_s, else_s) = get_combo(&tmp_combinations.s, "s", &predicate);
                            let mut if_tmp = tmp_combinations.clone();
                            if_tmp.s = if_s;
                            tmp_combinations.s = else_s;
                            vec![(next_destination, if_tmp)]
                        }
                        e => {
                            panic!("NOPE: {}", e)
                        }
                    }
                }
                None => vec![(next_destination, tmp_combinations.clone())],
            }
        })
        .collect::<Vec<(&'a str, Combinations)>>()
}

#[cfg(test)]
mod tests_count_parts_that_satisfies_instructions {
    use super::*;

    #[test]
    fn count_parts_that_satisfies_instructions_01() {
        let collection = count_parts_that_satisfies_instructions(
            &vec![
                Instruction {
                    predicate: Some("x > 10".to_string()),
                    destination: "A",
                },
                Instruction {
                    predicate: None,
                    destination: "R",
                },
            ],
            &Combinations {
                x: (1..=4000).collect(),
                m: (1..=4000).collect(),
                a: (1..=4000).collect(),
                s: (1..=4000).collect(),
            },
        );
        assert_eq!(collection[0].1.x.len(), 3990);
        assert_eq!(collection[0].1.m.len(), 4000);
        assert_eq!(collection[0].1.a.len(), 4000);
        assert_eq!(collection[0].1.s.len(), 4000);
    }

    #[test]
    fn count_parts_that_satisfies_instructions_02() {
        let collection = count_parts_that_satisfies_instructions(
            &vec![
                Instruction {
                    predicate: Some("x < 10".to_string()),
                    destination: "A",
                },
                Instruction {
                    predicate: None,
                    destination: "R",
                },
            ],
            &Combinations {
                x: (1..=4000).collect(),
                m: (1..=4000).collect(),
                a: (1..=4000).collect(),
                s: (1..=4000).collect(),
            },
        );
        assert_eq!(collection[0].1.x.len(), 9);
        assert_eq!(collection[0].1.m.len(), 4000);
        assert_eq!(collection[0].1.a.len(), 4000);
        assert_eq!(collection[0].1.s.len(), 4000);
    }

    #[test]
    fn count_parts_that_satisfies_instructions_03() {
        let collection = count_parts_that_satisfies_instructions(
            &vec![
                Instruction {
                    predicate: Some("x < 10".to_string()),
                    destination: "png",
                },
                Instruction {
                    predicate: Some("m > 10".to_string()),
                    destination: "R",
                },
                Instruction {
                    predicate: None,
                    destination: "A",
                },
            ],
            &Combinations {
                x: (1..=4000).collect(),
                m: (1..=4000).collect(),
                a: (1..=4000).collect(),
                s: (1..=4000).collect(),
            },
        );
        assert_eq!(collection.len(), 3);

        assert_eq!(collection[0].1.x.len(), 9);
        assert_eq!(collection[0].1.m.len(), 4000);
        assert_eq!(collection[0].1.a.len(), 4000);
        assert_eq!(collection[0].1.s.len(), 4000);

        assert_eq!(collection[1].1.x.len(), 3991);
        assert_eq!(collection[1].1.m.len(), 3990);
        assert_eq!(collection[1].1.a.len(), 4000);
        assert_eq!(collection[1].1.s.len(), 4000);

        assert_eq!(collection[2].1.x.len(), 3991);
        assert_eq!(collection[2].1.m.len(), 10);
        assert_eq!(collection[2].1.a.len(), 4000);
        assert_eq!(collection[2].1.s.len(), 4000);
    }
}

fn calc_combinations<'a>(input: &Vec<(&'a str, Combinations)>) -> usize {
    input
        .iter()
        .filter(|(a_or_r, _)| *a_or_r == "A")
        .fold(0, |acc, (_, c)| {
            acc + c.x.len() * c.m.len() * c.a.len() * c.s.len()
        })
}

fn get_combinations<'a>(
    workflows: &Vec<Workflow<'a>>,
    workflow_name: &'a str,
    combinations: &Combinations,
) -> Vec<(&'a str, Combinations)> {
    if workflow_name == "A" || workflow_name == "R" {
        return vec![(workflow_name, combinations.clone())];
    }

    count_parts_that_satisfies_instructions(
        &workflows
            .iter()
            .find(|w| w.name == workflow_name)
            .unwrap()
            .instructions,
        &combinations,
    )
    .iter()
    .flat_map(|(w, c)| get_combinations(&workflows, w, c))
    .collect::<Vec<(&'a str, Combinations)>>()
}

#[cfg(test)]
mod tests_get_combinations {
    use super::*;

    #[test]
    fn get_combinations_01() {
        let content = parse_content(
            &"\
in{x<1351:A,qqz}
qqz{m<10:A,R}",
        );
        let collection = get_combinations(
            &content.workflows,
            &"in",
            &Combinations {
                x: (1..=4000).collect(),
                m: (1..=4000).collect(),
                a: (1..=4000).collect(),
                s: (1..=4000).collect(),
            },
        );
        assert_eq!(collection.len(), 3);

        assert_eq!(collection[0].0, "A");
        assert_eq!(collection[0].1.x.len(), 1350);
        assert_eq!(collection[0].1.m.len(), 4000);
        assert_eq!(collection[0].1.a.len(), 4000);
        assert_eq!(collection[0].1.s.len(), 4000);

        assert_eq!(collection[1].0, "A");
        assert_eq!(collection[1].1.x.len(), 2650);
        assert_eq!(collection[1].1.m.len(), 9);
        assert_eq!(collection[1].1.a.len(), 4000);
        assert_eq!(collection[1].1.s.len(), 4000);

        assert_eq!(collection[2].0, "R");
        assert_eq!(collection[2].1.x.len(), 2650);
        assert_eq!(collection[2].1.m.len(), 3991);
        assert_eq!(collection[2].1.a.len(), 4000);
        assert_eq!(collection[2].1.s.len(), 4000);

        assert_eq!(calc_combinations(&collection), 86781600000000);
    }

    #[test]
    fn get_combinations_02() {
        let content = parse_content(
            &"\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}",
        );
        let collection = get_combinations(
            &content.workflows,
            &"in",
            &Combinations {
                x: (1..=4000).collect(),
                m: (1..=4000).collect(),
                a: (1..=4000).collect(),
                s: (1..=4000).collect(),
            },
        );
        assert_eq!(calc_combinations(&collection), 167409079868000);
    }
}

fn main() {
    let content = get_file_content("assets/input");

    let data = parse_content(&content);
    println!(
        "Result: {:?}",
        calc_combinations(&get_combinations(
            &data.workflows,
            &"in",
            &Combinations {
                x: (1..=4000).collect(),
                m: (1..=4000).collect(),
                a: (1..=4000).collect(),
                s: (1..=4000).collect(),
            },
        ))
    );
}
