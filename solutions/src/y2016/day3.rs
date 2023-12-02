use regex::Regex;

pub struct Day3;

crate::impl_day!("3", true);

fn process_input(input: &str) -> Vec<(i32, i32, i32)> {
    let re = Regex::new(r"\s*(\d+)\s*(\d+)\s*(\d+)").unwrap();

    input
        .trim()
        .lines()
        .filter_map(|l| re.captures(l))
        .map(|c| {
            let s1 = c.get(1).map(|m| m.as_str().parse()).unwrap().unwrap();
            let s2 = c.get(2).map(|m| m.as_str().parse()).unwrap().unwrap();
            let s3 = c.get(3).map(|m| m.as_str().parse()).unwrap().unwrap();

            (s1, s2, s3)
        })
        .collect()
}

fn is_valid_triangle(edges: &(i32, i32, i32)) -> bool {
    edges.0 + edges.1 > edges.2 && edges.1 + edges.2 > edges.0 && edges.0 + edges.2 > edges.1
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let triangles = process_input(input);

    let res = triangles.iter().filter(|t| is_valid_triangle(t)).count();
    Box::new(res)
}

fn process_input_p2(input: &str) -> Vec<(i32, i32, i32)> {
    let h_triangles = process_input(input);

    h_triangles
        .chunks(3)
        .flat_map(|chunk| {
            let [l1, l2, l3] = chunk else {
                panic!("invalid chunk size")
            };

            [(l1.0, l2.0, l3.0), (l1.1, l2.1, l3.1), (l1.2, l2.2, l3.2)]
        })
        .collect()
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let triangles = process_input_p2(input);

    let res = triangles.iter().filter(|t| is_valid_triangle(t)).count();
    Box::new(res)
}
