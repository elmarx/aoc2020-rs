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
fn main() {
    let answer1 = count_groups(INPUT).iter().sum::<usize>();
    println!("Answer 1: {}", answer1);
}

#[cfg(test)]
mod test {
    use crate::count_groups;

    const SAMPLE: &str = include_str!("../../input/day6_sample.txt");

    #[test]
    fn test_count_groups() {
        let actual = count_groups(SAMPLE);
        let expected = vec![3, 3, 3, 1, 1];

        assert_eq!(actual, expected)
    }
}
