use lazy_static::lazy_static;
use regex::Regex;

const INPUT: &str = include_str!("../../input/day2.txt");

#[derive(Debug, PartialEq)]
struct InputLine {
    a: i32,
    b: i32,
    character: char,
    password: String,
}

impl InputLine {
    fn new(a: i32, b: i32, character: char, password: String) -> Self {
        InputLine {
            a,
            b,
            character,
            password,
        }
    }

    fn is_valid_min_max(&self) -> bool {
        self.validate_min_max(0, self.password.as_str())
    }

    fn validate_min_max(&self, acc: i32, password: &str) -> bool {
        let head = password.chars().next();

        match head {
            // string is empty, we've walked through the whole string. let's check if acc is within the boundaries.
            None => self.a <= acc, // no need to check for max, because if we would have hit max, we would have bailed out already
            // the character we're looking at is not the character to validate/count, so just go ahead and check the rest of the password
            Some(c) if c != self.character => self.validate_min_max(acc, &password[1..]),
            // ok, so this is a relevant character, now we need to increase the accumulator and do checks!
            Some(_) => {
                let acc = acc + 1;
                // more counts than max? this password is invalid
                if self.b < acc {
                    false
                } else {
                    // everything is awesome… let's continue with the increased acc and check the rest of the password
                    self.validate_min_max(acc, &password[1..])
                }
            }
        }
    }

    fn is_valid_positions(&self) -> bool {
        let a = self.password.chars().nth(self.a as usize - 1).unwrap();
        let b = self.password.chars().nth(self.b as usize - 1).unwrap();

        (a == self.character) ^ (b == self.character)
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
    let answer_1 = input.iter().filter(|i| i.is_valid_min_max()).count();
    let answer_2 = input.iter().filter(|i| i.is_valid_positions()).count();

    println!("Answer 1: {} passwords are valid", answer_1);
    println!("Answer 2: {} passwords are valid", answer_2);
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
    pub fn test_valid_min_max() {
        let is_valid1 = InputLine::new(1, 3, 'a', "abcde".to_string()).is_valid_min_max();
        let is_valid2 = InputLine::new(2, 9, 'c', "ccccccccc".to_string()).is_valid_min_max();

        assert!(is_valid1);
        assert!(is_valid2);
    }

    #[test]
    pub fn test_invalid_min_max() {
        let is_valid = InputLine::new(1, 3, 'b', "cdefg".to_string()).is_valid_min_max();

        assert!(!is_valid);
    }

    #[test]
    pub fn test_valid_positions() {
        let is_valid = InputLine::new(1, 3, 'a', "abcde".to_string()).is_valid_positions();

        assert!(is_valid)
    }

    #[test]
    pub fn test_invalid_positions() {
        let is_valid1 = InputLine::new(1, 3, 'b', "cdefg".to_string()).is_valid_positions();
        let is_valid2 = InputLine::new(2, 9, 'c', "ccccccccc".to_string()).is_valid_positions();

        assert!(!is_valid1);
        assert!(!is_valid2);
    }
}
