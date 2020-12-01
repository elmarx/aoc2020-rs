use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day1_1.txt");

fn main() {
    let input: Vec<i32> = INPUT.split("\n").map(|l| l.parse().unwrap()).collect();

    let (a_1, b_2) =
        find_2020_tuple(&input).expect("did not find any tuple summing up to 2020 :'(");
    let answer_1 = a_1 * b_2;

    let (a_2, b_2, c_2) =
        find_2020_triple(&input).expect("did not find any triple summing up to 2020 :'(");
    let answer_2 = a_2 * b_2 * c_2;

    println!("Answer Part 1: {0}. ({1} * {2} = {0})", answer_1, a_1, b_2);
    println!(
        "Answer Part 2: {0}. ({1} * {2} * {3} = {0})",
        answer_2, a_2, b_2, c_2
    );
}

/// from the list of input, find the two numbers that add up to 2020
fn find_2020_tuple(input: &[i32]) -> Option<(i32, i32)> {
    input
        .iter()
        .tuple_combinations()
        .find(|(&a, &b)| a + b == 2020)
        .map(|(&a, &b)| (a, b))
}

/// from the list of input, find the three numbers that add up to 2020
/// almost the same as part 1/tuples, except… You have to go deeper!
fn find_2020_triple(input: &[i32]) -> Option<(i32, i32, i32)> {
    input
        .iter()
        .tuple_combinations()
        .find(|(&a, &b, &c)| a + b + c == 2020)
        .map(|(&a, &b, &c)| (a, b, c))
}

#[cfg(test)]
mod test {
    use crate::{find_2020_triple, find_2020_tuple};

    const SAMPLE: &[i32] = &[1721, 979, 366, 299, 675, 1456];

    #[test]
    fn test_sample_tuple() {
        let actual = find_2020_tuple(SAMPLE);
        assert_eq!(actual, Some((1721, 299)));
    }

    #[test]
    fn test_sample_triple() {
        let actual = find_2020_triple(SAMPLE);
        assert_eq!(actual, Some((979, 366, 675)));
    }
}
