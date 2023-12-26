use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
    fs,
};

use regex::Regex;

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
enum Pulse {
    High,
    Low,
}

impl Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pulse::High => write!(f, "HIGH"),
            Pulse::Low => write!(f, "low"),
        }
    }
}

type ModuleName = String;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct FlipFlop {
    name: String,
    outputs: Vec<ModuleName>,
    state: bool,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Conjunction {
    name: ModuleName,
    outputs: Vec<ModuleName>,
    inputs: Vec<(ModuleName, Pulse)>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Broadcaster {
    name: String,
    outputs: Vec<ModuleName>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
enum Module {
    F(FlipFlop),
    C(Conjunction),
    B(Broadcaster),
}

type ModuleMap = HashMap<ModuleName, Module>;

#[derive(Debug, PartialEq)]
struct Content {
    modules: ModuleMap,
}

fn parse_content(lines: &str) -> Content {
    let re_flipflop = Regex::new(r"%(?<name>.+) -> (?<targets>.*)").unwrap();
    let re_conjunction = Regex::new(r"&(?<name>.+) -> (?<targets>.*)").unwrap();
    let re_broadcaster = Regex::new(r"broadcaster -> (?<targets>.*)").unwrap();

    let mut modules: ModuleMap = HashMap::new();
    lines
        .split("\n")
        .filter(|line| line != &"") // Final \n in file
        .for_each(|line| {
            let maybe_caps = re_flipflop.captures(&line);
            match maybe_caps {
                Some(caps) => {
                    modules.insert(
                        caps["name"].to_string(),
                        Module::F(FlipFlop {
                            name: caps["name"].to_string(),
                            outputs: caps["targets"]
                                .split(",")
                                .map(|e| e.replace(" ", ""))
                                .collect::<Vec<ModuleName>>(),
                            state: false,
                        }),
                    );
                }
                None => {}
            };

            let maybe_caps = re_conjunction.captures(&line);
            match maybe_caps {
                Some(caps) => {
                    modules.insert(
                        caps["name"].to_string(),
                        Module::C(Conjunction {
                            name: caps["name"].to_string(),
                            outputs: caps["targets"]
                                .split(",")
                                .map(|e| e.replace(" ", ""))
                                .collect::<Vec<ModuleName>>(),
                            inputs: vec![
                            // Will be handled next
                            ],
                        }),
                    );
                }
                None => {}
            };

            let maybe_caps = re_broadcaster.captures(&line);
            match maybe_caps {
                Some(caps) => {
                    modules.insert(
                        "broadcaster".to_string(),
                        Module::B(Broadcaster {
                            name: "broadcaster".to_string(),
                            outputs: caps["targets"]
                                .split(",")
                                .map(|e| e.replace(" ", ""))
                                .collect::<Vec<ModuleName>>(),
                        }),
                    );
                }
                None => {}
            };
        });

    let tmp_modules = modules.clone();

    // Conjunction input update
    tmp_modules
        .iter()
        .map(|(_, m)| match m {
            Module::C(c) => Some(c),
            _ => None,
        })
        .filter(|x| if let Some(_) = x { true } else { false })
        .map(|x| if let Some(y) = x { y } else { panic!("WAT") })
        .for_each(|c| {
            let list = modules
                .clone()
                .iter()
                .map(|(_, n)| match n {
                    Module::F(f) => {
                        if f.outputs.contains(&c.name) {
                            f.name.clone()
                        } else {
                            "".to_owned()
                        }
                    }
                    Module::C(d) => {
                        if d.outputs.contains(&c.name) {
                            d.name.clone()
                        } else {
                            "".to_owned()
                        }
                    }
                    Module::B(b) => {
                        if b.outputs.contains(&c.name) {
                            "broadcaster".to_owned()
                        } else {
                            "".to_owned()
                        }
                    }
                })
                .filter(|name| name != "")
                .map(|n| (n, Pulse::Low))
                .collect::<Vec<(String, Pulse)>>();

            let mut_c = modules.get_mut(&c.name).unwrap();
            if let Module::C(m_c) = mut_c {
                m_c.inputs = list.clone();
            }
        });

    Content { modules }
}

#[cfg(test)]
mod tests_parse_content {
    use super::*;

    #[test]
    fn parse_content_01() {
        let content = parse_content(
            &"\
%vh -> qc, pb
&pb -> gf, gv
broadcaster -> hd, zj
",
        );
        assert_eq!(
            content,
            Content {
                modules: HashMap::from([
                    (
                        "vh".to_string(),
                        Module::F(FlipFlop {
                            name: "vh".to_string(),
                            outputs: vec!["qc".to_string(), "pb".to_string()],
                            state: false
                        })
                    ),
                    (
                        "pb".to_string(),
                        Module::C(Conjunction {
                            name: "pb".to_string(),
                            outputs: vec!["gf".to_string(), "gv".to_string()],
                            inputs: vec![("vh".to_string(), Pulse::Low)]
                        })
                    ),
                    (
                        "broadcaster".to_string(),
                        Module::B(Broadcaster {
                            name: "broadcaster".to_string(),
                            outputs: vec!["hd".to_string(), "zj".to_string()]
                        })
                    )
                ])
            }
        );
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct SendPulse {
    to: ModuleName,
    pulse: Pulse,
    from: ModuleName,
}

impl Display for SendPulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}->{}", self.from, self.pulse, self.to)
    }
}

type PulseQueue = VecDeque<SendPulse>;

trait ReceivePulse {
    fn receive_pulse(&mut self, queue: &mut PulseQueue, pulse: &Pulse, input: String);
}

impl ReceivePulse for Module {
    fn receive_pulse(&mut self, queue: &mut PulseQueue, pulse: &Pulse, input: String) {
        match self {
            Module::F(f) => {
                match pulse {
                    Pulse::High => {
                        // Nothing happens
                    }
                    // Flip the flop
                    Pulse::Low => {
                        let tmp = {
                            if f.state {
                                f.state = false;
                                &Pulse::Low
                            } else {
                                f.state = true;
                                &Pulse::High
                            }
                        };
                        // And send the appropriate pulse to outputs
                        f.outputs.iter().for_each(|m| {
                            queue.push_back(SendPulse {
                                to: m.to_owned(),
                                pulse: tmp.clone(),
                                from: f.name.clone(),
                            })
                        });
                    }
                }
            }
            Module::C(c) => {
                // Mutate the appropriate input on the Conjunction with given pulse
                c.inputs.iter_mut().find(|m| *m.0 == input).unwrap().1 = pulse.clone();

                c.outputs.iter().for_each(|m| {
                    let toto = SendPulse {
                        to: m.to_owned(),
                        pulse: if c.inputs.iter().filter(|i| i.1 == Pulse::Low).count() == 0 {
                            Pulse::Low
                        } else {
                            Pulse::High
                        },
                        from: c.name.clone(),
                    };
                    queue.push_back(toto)
                });
            }
            Module::B(b) => {
                // Retransmit the same pulse to outputs
                b.outputs.iter().for_each(|m| {
                    queue.push_back(SendPulse {
                        to: m.to_owned(),
                        pulse: pulse.clone(),
                        from: b.name.clone(),
                    })
                });
            }
        };
    }
}

fn handle_pulse_queue(map: &mut ModuleMap, queue: &mut PulseQueue) -> Option<String> {
    while let Some(element) = queue.pop_front() {
        println!("Element: {}", element);
        // Here we have some fuckery afoot: there are four different "blocs" in
        // input data, and they all loop, so by isolating each bloc and
        // searching for how many button pushes it takes to make the common target
        // (named "ns"), you can determine how many steps each bloc loop have.
        //
        // Then, when you have all four loop counts, just LCM them.
        // For added benefits, each four loop count is prime so LCMing them mean
        // that you just have to multiply them
        //
        // TODO: Convert every thing above in code
        if element.to == "ns" && element.pulse == Pulse::High {
            return Some(element.to_string());
        }
        if let Some(x) = map.get_mut(&element.to) {
            x.receive_pulse(queue, &element.pulse, element.from)
        }
    }
    return None;
}

fn push_the_button(map: &mut ModuleMap) -> usize {
    let mut button_count = 0;
    loop {
        button_count += 1;
        let mut queue: PulseQueue = VecDeque::from(vec![SendPulse {
            to: "broadcaster".to_string(),
            pulse: Pulse::Low,
            from: "BUTTON".to_string(),
        }]);
        if let Some(found) = handle_pulse_queue(map, &mut queue) {
            println!("Module {} goes HIGH at {} count", found, button_count);
            return button_count;
        }
    }
}

fn main() {
    let content = get_file_content("assets/input");

    let mut data = parse_content(&content);
    let button_count = push_the_button(&mut data.modules);

    println!("Result: {:?}", button_count);
}
