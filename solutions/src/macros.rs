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
                        $solvers.push(Box::new(crate::[<y $year>]::[<day N>]::[<Day N>]));
                }
            }}
        )*
    };
}
