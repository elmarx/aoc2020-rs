use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

const INPUT: &str = include_str!("../../input/day4.txt");

type Passport = HashMap<String, String>;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\w+):(\S+?)( |\n|$)").unwrap();
}

fn parse_input(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .map(|passport| {
            RE.captures_iter(passport)
                .map(|c| (c[1].to_string(), c[2].to_string()))
                .collect()
        })
        .collect()
}

fn is_valid(passport: &Passport) -> bool {
    // a passport is valid if it includes all 8 fields
    // it's also valid if it includes all 7 required fields (i.e. does not include the optional field)
    passport.len() == 8 || (passport.len() == 7 && !passport.contains_key("cid"))
}

fn main() {
    let answer1 = parse_input(INPUT).iter().filter(|p| is_valid(p)).count();
    println!("Answer 1: {} passwords are valid", answer1);
}

#[cfg(test)]
mod test {
    use crate::{is_valid, parse_input};
    use itertools::Itertools;

    const SAMPLE: &str = include_str!("../../input/day4_sample.txt");

    #[test]
    fn test_parse() {
        let actual = parse_input(SAMPLE);
        assert_eq!(actual.len(), 4);
        let len = actual.iter().map(|c| c.len()).collect_vec();
        assert_eq!(len, vec![8, 7, 7, 6])
    }

    #[test]
    fn test_is_password() {
        let input = parse_input(SAMPLE);

        assert!(is_valid(input.first().unwrap()));
        assert!(!is_valid(input.get(1).unwrap()));
        assert!(is_valid(input.get(2).unwrap()));
        assert!(!is_valid(input.get(3).unwrap()));
    }
}
