use lazy_static::lazy_static;
use regex::Regex;

const INPUT: &str = include_str!("../../input/day2.txt");

#[derive(Debug, PartialEq)]
struct InputLine {
    min: i32,
    max: i32,
    character: char,
    password: String,
}

impl InputLine {
    fn new(min: i32, max: i32, character: char, password: String) -> Self {
        InputLine {
            min,
            max,
            character,
            password,
        }
    }

    fn is_valid(&self) -> bool {
        self.validate(0, self.password.as_str())
    }

    fn validate(&self, acc: i32, password: &str) -> bool {
        let head = password.chars().next();

        match head {
            // string is empty, we've walked through the whole string. let's check if acc is within the boundaries.
            None => self.min <= acc, // no need to check for max, because if we would have hit max, we would have bailed out already
            // the character we're looking at is not the character to validate/count, so just go ahead and check the rest of the password
            Some(c) if c != self.character => self.validate(acc, &password[1..]),
            // ok, so this is a relevant character, now we need to increase the accumulator and do checks!
            Some(_) => {
                let acc = acc + 1;
                // more counts than max? this password is invalid
                if self.max < acc {
                    false
                } else {
                    // everything is awesome… let's continue with the increased acc and check the rest of the password
                    self.validate(acc, &password[1..])
                }
            }
        }
    }
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?m)(\d+)-(\d+) ([[:alpha:]]): ([[:alpha:]]+)").unwrap();
}

fn parse_input(input: &str) -> Vec<InputLine> {
    RE.captures_iter(input)
        .map(|c| {
            InputLine::new(
                c[1].parse().unwrap(),
                c[2].parse().unwrap(),
                c[3].parse().unwrap(),
                c[4].to_owned(),
            )
        })
        .collect()
}

fn main() {
    let input = parse_input(INPUT);
    let answer = input.iter().filter(|i| i.is_valid()).count();

    println!("{} passwords are valid", answer);
}

#[cfg(test)]
mod test {
    use crate::{parse_input, InputLine};

    const SAMPLE: &str = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";

    #[test]
    pub fn test_parse() {
        let actual = parse_input(SAMPLE);
        let expected = vec![
            InputLine::new(1, 3, 'a', "abcde".to_string()),
            InputLine::new(1, 3, 'b', "cdefg".to_string()),
            InputLine::new(2, 9, 'c', "ccccccccc".to_string()),
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    pub fn test_valid() {
        let is_valid1 = InputLine::new(1, 3, 'a', "abcde".to_string()).is_valid();
        let is_valid2 = InputLine::new(2, 9, 'c', "ccccccccc".to_string()).is_valid();

        assert!(is_valid1);
        assert!(is_valid2);
    }

    #[test]
    pub fn test_invalid() {
        let is_valid = InputLine::new(1, 3, 'b', "cdefg".to_string()).is_valid();

        assert!(!is_valid);
    }
}
