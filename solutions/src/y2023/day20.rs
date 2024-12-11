use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

pub struct Day20;

crate::impl_day!("20", true);

#[derive(Debug)]
enum Module {
    Broadcaster(Vec<String>),
    FlipFlop(Vec<String>, bool),
    Conjunction(Vec<String>, Vec<(String, bool)>),
}

fn collect_destination_modules(s: &str) -> Vec<String> {
    s.split(" -> ")
        .skip(1)
        .flat_map(|list| {
            list.split(", ")
                .map(std::borrow::ToOwned::to_owned)
                .collect_vec()
        })
        .collect_vec()
}

fn collect_module_name(s: &str) -> String {
    if s.starts_with("broadcaster") {
        "broadcaster".to_string()
    } else {
        let first_space_index = s.find(' ').expect("first space not found");

        s[1..first_space_index].to_owned()
    }
}

fn process_input(input: &str) -> HashMap<String, Module> {
    let mut conjunction_module_names = vec![];

    let mut h: HashMap<String, Module> = input
        .trim_end()
        .lines()
        .map(|l| {
            let destination_modules = collect_destination_modules(l);
            let module_name = collect_module_name(l);

            if module_name.as_str() == "broadcaster" {
                (module_name, Module::Broadcaster(destination_modules))
            } else {
                let is_flip_flop = l.starts_with('%');

                (
                    module_name.clone(),
                    if is_flip_flop {
                        Module::FlipFlop(destination_modules, false)
                    } else {
                        conjunction_module_names.push(module_name);

                        Module::Conjunction(destination_modules, vec![])
                    },
                )
            }
        })
        .collect();

    input.lines().for_each(|l| {
        let destination_modules = collect_destination_modules(l);
        let module_name = collect_module_name(l);

        for dest in destination_modules {
            if conjunction_module_names.contains(&dest) {
                h.entry(dest).and_modify(|v| match v {
                    Module::Conjunction(_, b) => {
                        b.push((module_name.clone(), false));
                    }
                    _ => panic!("not a conjunction module"),
                });
            }
        }
    });

    h
}

fn push_signals(
    targets: &[String],
    signal: bool,
    sender: &str,
    queue: &mut VecDeque<(String, bool, Option<String>)>,
    low_count: &mut usize,
    high_count: &mut usize,
) {
    for target in targets {
        queue.push_back((target.to_owned(), signal, Some(sender.to_owned())));
        if signal {
            *high_count += 1;
        } else {
            *low_count += 1;
        }
    }
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let mut modules = process_input(input);
    // println!("modules: {modules:?}");

    let mut low_signals_count = 0;
    let mut high_signals_count = 0;
    let mut queue = VecDeque::new();

    for _ in 0..1000 {
        let mut tmp_low = 0;
        let mut tmp_high = 0;

        // Push the button!
        assert!(queue.is_empty());
        queue.push_back(("broadcaster".to_string(), false, None));
        tmp_low += 1;

        loop {
            if queue.is_empty() {
                break;
            }

            // println!("queue state: {queue:?}");

            let (name, signal, sender) = queue.pop_front().expect("no queued signal");

            modules.get_mut(&name).map_or_else(
                || { /* Nothing to do, the pulse was already counted. */ },
                |module| match module {
                    Module::Broadcaster(targets) => {
                        push_signals(
                            targets,
                            signal,
                            &name,
                            &mut queue,
                            &mut tmp_low,
                            &mut tmp_high,
                        );
                    }
                    Module::FlipFlop(targets, state) => {
                        if signal {
                            /* Nothing to do, the pulse was already counted. */
                        } else {
                            *state = !*state;
                            let new_signal = *state;

                            push_signals(
                                targets,
                                new_signal,
                                &name,
                                &mut queue,
                                &mut tmp_low,
                                &mut tmp_high,
                            );
                        }
                    }
                    Module::Conjunction(targets, inputs) => {
                        let sender = sender.expect("no sender in conjunction module");

                        let input_index = inputs
                            .iter()
                            .position(|(n, _)| *n == sender)
                            .unwrap_or_else(|| panic!("conjunction input not found: {}", name));

                        let entry = inputs.get_mut(input_index).unwrap();
                        entry.1 = signal;

                        let new_signal = inputs[0].1;
                        if inputs.iter().all(|(_, v)| *v == new_signal) {
                            push_signals(
                                targets,
                                !new_signal,
                                &name,
                                &mut queue,
                                &mut tmp_low,
                                &mut tmp_high,
                            );
                        }
                    }
                },
            );
        }

        // println!("{tmp_low}, {tmp_high}");

        low_signals_count += tmp_low;
        high_signals_count += tmp_high;
    }

    let res = low_signals_count * high_signals_count;
    Box::new(res)
}

#[allow(unused)]
fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let res = "Part 2 not done";
    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = r"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";

    #[test]
    fn test_part1_simple() {
        assert_eq!(32_000_000.to_string(), *solve_part1(INPUT1).to_string());
    }

    #[test]
    fn test_part1_complex() {
        const INPUT2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
";
        assert_eq!(11_687_500.to_string(), *solve_part1(INPUT2).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(0.to_string(), *solve_part2(INPUT1).to_string());
    }
}
