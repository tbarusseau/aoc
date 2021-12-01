extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs};

#[derive(Default, Debug)]
struct AocArgs {
    year: Option<i32>,
    day: Option<i32>,
    part: Option<i32>,
}

#[proc_macro_attribute]
pub fn aoc(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);

    let mut parsed_args = AocArgs {
        ..Default::default()
    };

    for arg in args {
        if let syn::NestedMeta::Meta(syn::Meta::NameValue(nv)) = arg {
            let segments = nv.path.segments;
            for seg in segments {
                if seg.ident == "year" {
                    if let syn::Lit::Int(year) = nv.lit {
                        parsed_args.year = Some(year.base10_parse().unwrap());
                        break;
                    }
                } else if seg.ident == "day" {
                    if let syn::Lit::Int(day) = nv.lit {
                        parsed_args.day = Some(day.base10_parse().unwrap());
                        break;
                    }
                } else if seg.ident == "part" {
                    if let syn::Lit::Int(day) = nv.lit {
                        parsed_args.part = Some(day.base10_parse().unwrap());
                        break;
                    }
                } else {
                    panic!("Invalid identifier: {}", seg.ident);
                }
            }
        }
    }

    println!("Aoc args: {parsed_args:?}");

    match parsed_args.year {
        None => {
            panic!("Missing year");
        }
        Some(y) if y < 2015 => {
            panic!("Invalid year, must be more than 2015");
        }
        _ => {}
    }

    match parsed_args.day {
        None => {
            panic!("Missing day");
        }
        Some(d) if !(1..=25).contains(&d) => {
            panic!("Invalid day, must be between 1 and 25");
        }
        _ => {}
    }

    match parsed_args.part {
        None => {
            panic!("Missing part");
        }
        Some(p) if p != 1 && p != 2 => {
            panic!("Invalid part, must be 1 or 2");
        }
        _ => {}
    }

    item
}
