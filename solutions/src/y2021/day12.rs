use crate::solver::Solver;
use petgraph::graphmap::UnGraphMap;

pub struct Day12;

crate::impl_day!("12", true);

enum Node {
    Start,
    End,
    Small(String),
    Big(String),
}

impl From<&str> for Node {
    fn from(s: &str) -> Self {
        match s {
            "start" => Node::Start,
            "end" => Node::End,
            s => {
                if s.len() != 2 {
                    panic!("Doesn't have a size of two chars: {}", s);
                }

                let mut chars = s.chars();
                let c0 = chars.next().unwrap();
                let c1 = chars.next().unwrap();

                if (c0.is_uppercase() && !c1.is_uppercase())
                    || (!c0.is_uppercase() && c1.is_uppercase())
                {
                    panic!("Chars have different cases: {}", s);
                }

                if c0.is_uppercase() {
                    Node::Big(s.to_owned())
                } else {
                    Node::Small(s.to_owned())
                }
            }
        }
    }
}

fn process_input(input: &str) -> UnGraphMap<&str, ()> {
    let mut edges = vec![];
    for line in input.trim().lines() {
        let v = line.split('-').collect::<Vec<&str>>();
        let e1 = v[0];
        let e2 = v[1];

        edges.push((e1, e2));
    }

    UnGraphMap::<_, ()>::from_edges(edges)
}

fn is_small_cave(s: &str) -> bool {
    return s.chars().next().unwrap().is_lowercase();
}

fn count(graph: &UnGraphMap<&str, ()>, small_caves: usize, seen: Vec<&str>, node: &str) -> usize {
    let mut part = small_caves;

    if node == "end" {
        return 1;
    }

    if seen.contains(&node) {
        if node == "start" {
            return 0;
        }

        if is_small_cave(node) {
            if small_caves == 1 {
                return 0;
            } else {
                part = 1;
            }
        }
    }

    graph
        .edges(node)
        .map(|(_, e, _)| {
            let mut v = seen.clone();
            v.push(node);

            count(graph, part, v, e)
        })
        .sum()
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let graph = process_input(input);
    let res = count(&graph, 1, vec![], "start");

    Box::new(res)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let graph = process_input(input);
    let res = count(&graph, 2, vec![], "start");

    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = r#"
start-A
start-b
A-c
A-b
b-d
A-end
b-end
"#;

    const INPUT2: &str = r#"
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
"#;

    const INPUT3: &str = r#"
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
"#;

    #[test]
    fn test_part1() {
        assert_eq!(10.to_string(), *solve_part1(INPUT1).to_string());
        assert_eq!(19.to_string(), *solve_part1(INPUT2).to_string());
        assert_eq!(226.to_string(), *solve_part1(INPUT3).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(36.to_string(), *solve_part2(INPUT1).to_string());
    }
}
