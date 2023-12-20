use std::{borrow::Cow, collections::HashMap, sync::OnceLock};

use itertools::Itertools;
use regex::Regex;

pub struct Day19;

crate::impl_day!("19", true);

#[derive(Debug)]
struct Part(usize, usize, usize, usize);
#[derive(Debug)]
struct Workflow(String, Vec<String>);

impl Part {
    pub fn compute_score(&self) -> usize {
        self.0 + self.1 + self.2 + self.3
    }
}

static WORKFLOW_REGEX: OnceLock<Regex> = OnceLock::new();

fn process_input(input: &str) -> (Vec<Workflow>, Vec<Part>) {
    let mut it = input.trim_end().split("\n\n");

    let workflows_str = it.next().expect("no workflows input");
    let parts_str = it.next().expect("no parts input");

    let workflows = workflows_str
        .lines()
        .map(|l| {
            let re = WORKFLOW_REGEX.get_or_init(|| Regex::new(r"([a-z]+)\{(.*?)\}").unwrap());
            let captures = re.captures(l).expect("no regex captures");
            let start = captures.get(1).expect("no first capture group").as_str();
            let rest = captures.get(2).expect("no second capture group").as_str();

            Workflow(
                start.to_owned(),
                rest.split(',')
                    .map(std::borrow::ToOwned::to_owned)
                    .collect_vec(),
            )
        })
        .collect_vec();

    let parts = parts_str
        .lines()
        .flat_map(|l| sscanf::sscanf!(l, "{{x={usize},m={usize},a={usize},s={usize}}}"))
        .map(|v| Part(v.0, v.1, v.2, v.3))
        .collect_vec();

    (workflows, parts)
}

fn part_matches_criteria(part: &Part, criteria: char, operation: char, value: usize) -> bool {
    let part_value = match criteria {
        'x' => part.0,
        'm' => part.1,
        'a' => part.2,
        's' => part.3,
        _ => unreachable!(),
    };

    match operation {
        '>' => part_value > value,
        '<' => part_value < value,
        _ => unreachable!(),
    }
}

fn get_points(h: &HashMap<String, Vec<String>>, part: &Part, is_part_1: bool) -> usize {
    let mut flow = h.get("in").unwrap();
    let mut flow_index: usize = 0;

    loop {
        let subflow: &str = flow[flow_index].as_ref();

        let dest = if let Ok((criteria, operation, value, destination)) =
            sscanf::sscanf!(subflow, "{char}{char}{usize}:{String}")
        {
            if part_matches_criteria(part, criteria, operation, value) {
                Some(Cow::Owned(destination))
            } else {
                None
            }
        } else {
            Some(Cow::Borrowed(subflow))
        };

        match dest.as_deref() {
            Some("A") => {
                if is_part_1 {
                    return part.compute_score();
                }

                todo!()
            }
            Some("R") => {
                break 0;
            }
            Some(other) => {
                flow = h.get(other).unwrap();
                flow_index = 0;
                continue;
            }
            None => {}
        }

        flow_index += 1;
    }
}

fn solve<I: Iterator<Item = Part>>(workflows: &[Workflow], parts: I) -> usize {
    let h = workflows
        .iter()
        .map(|v| (v.0.clone(), v.1.clone()))
        .collect::<HashMap<String, Vec<String>>>();

    parts.map(|p| get_points(&h, &p, true)).sum()
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let (workflows, parts) = process_input(input);
    let score = solve(&workflows, parts.into_iter());

    Box::new(score)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let (workflows, _) = process_input(input);

    Box::new(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"px{a<2006:qkq,m>2090:A,rfg}
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
{x=2127,m=1623,a=2188,s=1013}
";

    #[test]
    fn test_part1() {
        assert_eq!(19114.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            167_409_079_868_000_usize.to_string(),
            *solve_part2(INPUT).to_string()
        );
    }
}
