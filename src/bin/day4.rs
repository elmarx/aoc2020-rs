use lazy_static::lazy_static;
use maplit::hashset;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

const INPUT: &str = include_str!("../../input/day4.txt");

type Passport = HashMap<String, String>;

lazy_static! {
    static ref INPUT_RE: Regex = Regex::new(r"(\w+):(\S+?)( |\n|$)").unwrap();
    static ref HGT_RE: Regex = Regex::new(r"^(\d+)(in|cm)$").unwrap();
    static ref HCL_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref ECL: HashSet<&'static str> =
        hashset! {"amb", "blu", "brn", "gry", "grn", "hzl", "oth"};
    static ref PID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
}

fn parse_input(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .map(|passport| {
            INPUT_RE
                .captures_iter(passport)
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

fn is_valid_extended(passport: &Passport) -> bool {
    is_valid(passport)
        && is_byr_valid(passport.get("byr").unwrap())
        && is_iyr_valid(passport.get("iyr").unwrap())
        && is_eyr_valid(passport.get("eyr").unwrap())
        && is_hgt_valid(passport.get("hgt").unwrap())
        && is_hcl_valid(passport.get("hcl").unwrap())
        && is_ecl_valid(passport.get("ecl").unwrap())
        && is_pid_valid(passport.get("pid").unwrap())
}

fn is_v_within(v: &str, min: i32, max: i32) -> bool {
    v.parse::<i32>().map_or(false, |n| (min..=max).contains(&n))
}

fn is_byr_valid(v: &str) -> bool {
    is_v_within(v, 1920, 2002)
}

fn is_iyr_valid(v: &str) -> bool {
    is_v_within(v, 2010, 2020)
}

fn is_eyr_valid(v: &str) -> bool {
    is_v_within(v, 2020, 2030)
}

fn is_hgt_valid(v: &str) -> bool {
    HGT_RE.captures(v).map_or(false, |c| {
        let hgt = c[1].parse::<u32>().unwrap();

        match &c[2] {
            "in" => (59..=76).contains(&hgt),
            "cm" => (150..=193).contains(&hgt),
            _ => unreachable!(),
        }
    })
}

fn is_hcl_valid(v: &str) -> bool {
    HCL_RE.is_match(v)
}

fn is_ecl_valid(v: &str) -> bool {
    ECL.contains(v)
}

fn is_pid_valid(v: &str) -> bool {
    PID_RE.is_match(v)
}

fn main() {
    let answer1 = parse_input(INPUT).iter().filter(|p| is_valid(p)).count();
    println!("Answer 1: {} passwords are valid", answer1);

    let answer2 = parse_input(INPUT)
        .iter()
        .filter(|p| is_valid_extended(p))
        .count();
    println!(
        "Answer 2: {} passwords are valid and have valid values",
        answer2
    );
}

#[cfg(test)]
mod test {
    use crate::{
        is_byr_valid, is_ecl_valid, is_hcl_valid, is_hgt_valid, is_pid_valid, is_valid,
        is_valid_extended, parse_input,
    };
    use itertools::Itertools;

    const SAMPLE: &str = include_str!("../../input/day4_sample.txt");
    const SAMPLE_INVALID: &str = include_str!("../../input/day4_sample_invalid.txt");
    const SAMPLE_VALID: &str = include_str!("../../input/day4_sample_valid.txt");

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

    #[test]
    fn test_byr_valid() {
        assert!(is_byr_valid("2002"));
        assert!(!is_byr_valid("2003"));
    }

    #[test]
    fn test_hgt_valid() {
        assert!(is_hgt_valid("60in"));
        assert!(is_hgt_valid("190cm"));
        assert!(!is_hgt_valid("190in"));
        assert!(!is_hgt_valid("190"));
    }

    #[test]
    fn test_hcl_valid() {
        assert!(is_hcl_valid("#123abc"));
        assert!(!is_hcl_valid("#123abz"));
        assert!(!is_hcl_valid("123abc"));
    }

    #[test]
    fn test_ecl_valid() {
        assert!(is_ecl_valid("brn"));
        assert!(!is_ecl_valid("wat"));
    }

    #[test]
    fn test_pid_valid() {
        assert!(is_pid_valid("000000001"));
        assert!(!is_pid_valid("0123456789"));
    }

    #[test]
    fn test_valid_passports() {
        let input = parse_input(SAMPLE_VALID);
        assert!(input.iter().all(is_valid_extended));
    }

    #[test]
    fn test_invalid_passports() {
        let input = parse_input(SAMPLE_INVALID);
        assert!(!input.iter().any(is_valid_extended));
    }
}
