#[macro_export]
macro_rules! days_gen {
    () => {
        use paste::paste;
        use seq_macro::seq;

        seq! { N in 1..=25 {
            paste! {
                pub mod [<day N>];
            }
        }}
    };
}

#[macro_export]
macro_rules! solvers_gen {
    ($solvers: ident, $($year: expr), *) => {
        use paste::paste;
        use seq_macro::seq;

        $(
            seq! { N in 1..=25 {
                paste! {
                        $solvers.push(Box::new($crate::[<y $year>]::[<day N>]::[<Day N>]));
                }
            }}
        )*
    };
}

#[macro_export]
macro_rules! impl_day {
    ($day: expr, $done: expr) => {
        use paste::paste;
        use $crate::solver::Solver;

        paste! {
            impl Solver for [<Day $day>] {
                fn solve_part1(&self, input: &str) -> Box<dyn std::fmt::Display> {
                    solve_part1(input)
                }

                fn solve_part2(&self, input: &str) -> Box<dyn std::fmt::Display> {
                    solve_part2(input)
                }

                fn done(&self) -> bool {
                    $done
                }
            }
        }
    };
}

// #[macro_export]
// macro_rules! impl_day_v2 {
//     ($day: expr, $done: expr) => {
//         use crate::solver::SolverWithResult;
//         use paste::paste;

//         paste! {
//             impl SolverWithResult for [<Day $day>] {
//                 fn solve_part1(&self, input: &str) -> anyhow::Result<Box<dyn std::fmt::Display>> {
//                     solve_part1(input)
//                 }

//                 fn solve_part2(&self, input: &str) -> anyhow::Result<Box<dyn std::fmt::Display>> {
//                     solve_part2(input)
//                 }

//                 fn done(&self) -> bool {
//                     $done
//                 }
//             }
//         }
//     };
// }
