use std::fs;
use std::collections::{HashMap, VecDeque};

#[derive(PartialEq, Clone)]
enum Prefix {
    Broadcast,
    FlipFlop,
    Conjunction
}

#[derive(Clone, PartialEq)]
enum Signal {
    Low,
    High
}

#[derive(Clone)]
struct Module {
    prefix: Prefix,
    code: String,
    recipients: Vec<String>,
    strength: Option<Signal>,
    memory: Option<HashMap<String, Signal>>
}

impl Signal {
    fn toggle(&mut self) {
        match self {
            Signal::Low => *self = Signal::High,
            Signal::High => *self = Signal::Low
        }
    }
}

fn build_modules(modules: String) -> Vec<Module> {
    // Parse modules into their separate types.
    let mut modules: Vec<_> = modules
        .lines()
        .filter_map(|module| {
            let prefix = match module.chars().nth(0) {
                Some('b') => Prefix::Broadcast,
                Some('%') => Prefix::FlipFlop,
                Some('&') => Prefix::Conjunction,
                _ => unreachable!()
            };

            let memory = match prefix {
                Prefix::Conjunction => Some(HashMap::new()),
                _ => None
            };

            let strength = match prefix {
                Prefix::FlipFlop => Some(Signal::Low),
                _ => None
            };

            let module = module
                .replace("%", "")
                .replace("&", "");

            let (code, recipients) = module.split_once(" -> ").unwrap();

            let recipients = recipients.split(", ").map(|recipient| recipient.to_string()).collect();

            return Some(Module {
                prefix,
                memory,
                strength,
                code: code.to_string(),
                recipients
            });
        }).collect();

    // Find all inputs for each conjunction.
    for idx in 0..modules.len() {
        if modules[idx].prefix == Prefix::Conjunction {
            let to_find = modules[idx].code.clone();

            modules
                .clone()
                .iter()
                .filter(|modu| modu.recipients.contains(&to_find))
                .map(|modu| modu.code.clone())
                .for_each(|code| {
                    let memory = modules[idx].memory.as_mut().unwrap();
                    memory.insert(code, Signal::Low);
                });
        }
    }

    return modules;
}

fn signals_sent() -> usize {
    // Each module sends a signal to its recipients when it receives a signal.
    //  Broadcast sends a low signal to all recipients.
    //  FlipFlops start off false, but flip when receiving a low pulse, then
    //      sends high if on and low if off.
    //  Conjunction modules have a memory of each sender, defaulted to 0
    //
    //  My plan is to encode this system as a list of modules, alongside an agenda of payloads.
    //  Each

    if let Some(modules) = fs::read_to_string("data/20.input").ok() {
        let mut modules = build_modules(modules);
        let mut highs = 0;
        let mut lows = 0;

        for _ in 0..1000 {
            // Send a singular low signal to the broadcaster.
            let mut agenda = VecDeque::from([("button".to_string(), "broadcaster".to_string(), Signal::Low)]);

            while let Some((sender, recipient, signal)) = agenda.pop_front() {
                if signal == Signal::Low {
                    highs += 1;
                } else {
                    lows += 1;
                }

                if let Some(idx) = modules.iter().position(|module| module.code == recipient) {
                    match modules[idx].prefix {
                        Prefix::Broadcast => {
                            // Copy the signal to all recipients.
                            for new_recipient in modules[idx].recipients.iter() {
                                agenda.push_back((recipient.clone(), new_recipient.to_string(), signal.clone()));
                            }
                        },
                        Prefix::FlipFlop => {
                            if signal == Signal::Low {
                                let strength = modules[idx].strength.as_mut().unwrap();

                                // Toggle state.
                                strength.toggle();

                                // Getting an owned copy of strength
                                let strength_to_send = strength.clone();

                                // Send the stored strength.
                                for new_recipient in modules[idx].recipients.iter() {
                                    agenda.push_back((recipient.clone(), new_recipient.to_string(), strength_to_send.clone()));
                                }
                            }
                        },
                        Prefix::Conjunction => {
                            let memory = modules[idx].memory.as_mut().unwrap();

                            // Update our memory of the sender.
                            memory.entry(sender).and_modify(|strength| {
                                *strength = signal;
                            });

                            // If all sender’s memories are favourable, send low.
                            if memory.values().all(|value| *value == Signal::High) {
                                for new_recipient in modules[idx].recipients.iter() {
                                    agenda.push_back((recipient.clone(), new_recipient.to_string(), Signal::Low));
                                }
                            } else {
                                // Else, send high.
                                for new_recipient in modules[idx].recipients.iter() {
                                    agenda.push_back((recipient.clone(), new_recipient.to_string(), Signal::High));
                                }

                            }
                        }
                    }
                }
            }
        }

        return highs * lows;
    }

    panic!("file not found")
}

fn main() {
    println!("part one: {}", signals_sent())
}
