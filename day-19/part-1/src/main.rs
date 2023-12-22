use std::fs;

use eval::{to_value, Expr};
use regex::Regex;

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Instruction {
    destination: String,
    predicate: Option<String>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Workflow {
    name: String,
    instructions: Vec<Instruction>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Part {
    x_rating: usize,
    m_rating: usize,
    a_rating: usize,
    s_rating: usize,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Content {
    workflows: Vec<Workflow>,
    parts: Vec<Part>,
}

fn parse_content(lines: &str) -> Content {
    let mut blocks = lines.split("\n\n");
    let re_workflow = Regex::new(r"(.+)\{(.*)\}").unwrap();
    let re_part = Regex::new(r"\{x=(?<x>.+),m=(?<m>.+),a=(?<a>.+),s=(?<s>.+)\}").unwrap();

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
                    name: (caps[1]).to_owned(),
                    instructions: caps[2]
                        .to_string()
                        .split(",")
                        .map(|e| {
                            if e.contains(":") {
                                let mut s = e.split(":");
                                Instruction {
                                    predicate: Some(
                                        s.next()
                                            .unwrap()
                                            .to_string()
                                            // For eval purposes
                                            .replace("<", " < ")
                                            .replace(">", " > "),
                                    ),
                                    destination: s.next().unwrap().to_string(),
                                }
                            } else {
                                Instruction {
                                    predicate: None,
                                    destination: e.to_owned(),
                                }
                            }
                        })
                        .collect::<Vec<Instruction>>(),
                }
            })
            .collect::<Vec<Workflow>>(),
        parts: blocks
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                let Some(caps) = re_part.captures(line) else {
                    panic!("Invalid part: {}", line);
                };

                Part {
                    x_rating: caps["x"].parse::<usize>().unwrap(),
                    m_rating: caps["m"].parse::<usize>().unwrap(),
                    a_rating: caps["a"].parse::<usize>().unwrap(),
                    s_rating: caps["s"].parse::<usize>().unwrap(),
                }
            })
            .collect::<Vec<Part>>(),
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
                        name: "px".to_string(),
                        instructions: vec![
                            Instruction {
                                predicate: Some("a < 2006".to_string()),
                                destination: "qkq".to_string()
                            },
                            Instruction {
                                predicate: Some("m > 2090".to_string()),
                                destination: "A".to_string()
                            },
                            Instruction {
                                predicate: None,
                                destination: "rfg".to_string()
                            }
                        ]
                    },
                    Workflow {
                        name: "pv".to_string(),
                        instructions: vec![
                            Instruction {
                                predicate: Some("a > 1716".to_string()),
                                destination: "R".to_string()
                            },
                            Instruction {
                                predicate: None,
                                destination: "A".to_string()
                            }
                        ]
                    }
                ],
                parts: vec![
                    Part {
                        x_rating: 787,
                        m_rating: 2655,
                        a_rating: 1222,
                        s_rating: 2876
                    },
                    Part {
                        x_rating: 1679,
                        m_rating: 44,
                        a_rating: 2067,
                        s_rating: 496
                    }
                ]
            }
        );
    }
}

fn get_next_destination(instructions: &Vec<Instruction>, part: &Part) -> String {
    let mut current_destination = String::new();

    for instruction in instructions {
        match &instruction.predicate {
            Some(p) => {
                match Expr::new(p)
                    .value("x", part.x_rating)
                    .value("m", part.m_rating)
                    .value("a", part.a_rating)
                    .value("s", part.s_rating)
                    .exec()
                {
                    Ok(value) => {
                        if value == to_value(true) {
                            current_destination = instruction.destination.clone();
                            break;
                        }
                    }
                    Err(e) => {
                        println!("{e}");
                    }
                };
            }
            None => {
                current_destination = instruction.destination.clone();
                break;
            }
        }
    }

    current_destination
}

#[cfg(test)]
mod tests_get_next_destination {
    use super::*;

    #[test]
    fn execute_instructions_01() {
        assert_eq!(
            get_next_destination(
                &vec![
                    Instruction {
                        predicate: Some("x > 10".to_string()),
                        destination: "A".to_string()
                    },
                    Instruction {
                        predicate: None,
                        destination: "R".to_string()
                    }
                ],
                &Part {
                    x_rating: 11,
                    m_rating: 0,
                    a_rating: 0,
                    s_rating: 0
                }
            ),
            "A"
        );
    }

    #[test]
    fn execute_instructions_02() {
        assert_eq!(
            get_next_destination(
                &vec![
                    Instruction {
                        predicate: Some("x < 10".to_string()),
                        destination: "A".to_string()
                    },
                    Instruction {
                        predicate: None,
                        destination: "R".to_string()
                    }
                ],
                &Part {
                    x_rating: 11,
                    m_rating: 0,
                    a_rating: 0,
                    s_rating: 0
                }
            ),
            "R"
        );
    }

    #[test]
    fn execute_instructions_03() {
        assert_eq!(
            get_next_destination(
                &vec![
                    Instruction {
                        predicate: Some("x < 10".to_string()),
                        destination: "A".to_string()
                    },
                    Instruction {
                        predicate: None,
                        destination: "R".to_string()
                    },
                    Instruction {
                        predicate: None,
                        destination: "A".to_string()
                    }
                ],
                &Part {
                    x_rating: 11,
                    m_rating: 0,
                    a_rating: 0,
                    s_rating: 0
                }
            ),
            "R"
        );
    }
}

fn execute_workflows(workflows: &Vec<Workflow>, part: &Part) -> String {
    dbg!(&part);
    let mut current_destination = workflows
        .iter()
        .find(|w| w.name == "in")
        .unwrap()
        .clone()
        .name;

    while current_destination != "A" && current_destination != "R" {
        let maybe_workflow = workflows.iter().find(|w| w.name == current_destination);
        match maybe_workflow {
            None => break,
            Some(workflow) => {
                dbg!(&workflow.name);
                current_destination = get_next_destination(&workflow.instructions, &part);
            }
        };
    }

    current_destination
}

#[cfg(test)]
mod tests_execute_workflows {
    use super::*;

    #[test]
    fn execute_workflows_01() {
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
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}",
        );
        assert_eq!(
            execute_workflows(&content.workflows, &content.parts[0]),
            "A"
        )
    }
}

fn main() {
    let content = get_file_content("assets/input");

    let data = parse_content(&content);
    println!(
        "Result: {:?}",
        // For each part
        data.parts
            .iter()
            // Retrieve its ultimate destination (either "A" or "R")
            .map(|part| (part, execute_workflows(&data.workflows, part)))
            // Only keep the `A`ccepted parts
            .filter(|(_, r)| r == "A")
            // Sum their internal rating
            .map(|(p, _)| p.x_rating + p.m_rating + p.a_rating + p.s_rating)
            // Sum this for all accepted parts
            .sum::<usize>()
    );
}
