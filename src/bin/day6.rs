use itertools::Itertools;
use std::collections::HashSet;

const INPUT: &str = include_str!("../../input/day6.txt");

/// count the number of unique "yes" per group
fn count_groups(i: &str) -> Vec<usize> {
    // split the input into the groups
    i.split("\n\n")
        .map(|group| {
            // then, put all chars of the group into a set (and remove '\n', as it just separates passengers)
            // and return the hash size
            let group: HashSet<u8> = group.bytes().filter(|b| *b != b'\n').collect();
            group.len()
        })
        .collect()
}

fn count_common_answers(i: &str) -> Vec<usize> {
    i.split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|passenger| passenger.bytes().collect::<HashSet<u8>>())
                .fold1(|a, b| a.intersection(&b).map(|e| *e).collect())
                .unwrap()
                .len()
        })
        .collect()
}

fn main() {
    let answer1 = count_groups(INPUT).iter().sum::<usize>();
    println!("Answer 1: {}", answer1);

    let answer2 = count_common_answers(INPUT).iter().sum::<usize>();
    println!("Answer 2: {}", answer2);
}

#[cfg(test)]
mod test {
    use crate::{count_common_answers, count_groups};

    const SAMPLE: &str = include_str!("../../input/day6_sample.txt");

    #[test]
    fn test_count_groups() {
        let actual = count_groups(SAMPLE);
        let expected = vec![3, 3, 3, 1, 1];

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_count_common_answers() {
        let actual = count_common_answers(SAMPLE);
        let expected = vec![3, 0, 1, 1, 1];

        assert_eq!(actual, expected);
    }
}
