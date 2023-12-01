pub struct Day1;

crate::impl_day!("1", true);

fn process_input(input: &str) -> &str {
    input
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let res = input
        .chars()
        .fold(0, |acc, e| if e == '(' { acc + 1 } else { acc - 1 });
    Box::new(res)
}

#[allow(unused)]
fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let mut res = 1;
    let mut level = 0;
    for c in input.chars() {
        if c == '(' {
            level += 1;
        } else {
            level -= 1;
        }

        if level == -1 {
            break;
        }

        res += 1;
    }

    Box::new(res)
}
