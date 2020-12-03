// TreeMap is the integer grid: a list of rows
type TreeMap = Vec<Vec<bool>>;

const INPUT: &[u8] = include_bytes!("../../input/day3.txt");

const HASH: u8 = '#' as u8;
const NL: u8 = '\n' as u8;

fn parse_treemap(input: &[u8]) -> TreeMap {
    let mut output = vec![vec![]];

    for &c in input {
        if c == NL {
            // create a new line
            output.push(Vec::new())
        } else {
            output.last_mut().unwrap().push(c == HASH)
        }
    }

    output
}

fn count_trees(map: &TreeMap) -> i32 {
    // walk through all the rows, skip the first row, as the first step will lead directly to the second row
    map.iter().enumerate().skip(1).fold(0, |acc, (step, row)| {
        // index/coordinate where we enter this row is number of steps * 3 (cause we go 3 to the left)
        // modulo width of a row (since we're repeating the pattern)
        if row.get((step * 3) % row.len()).unwrap().to_owned() {
            acc + 1
        } else {
            acc
        }
    })
}

fn main() {
    let input = parse_treemap(INPUT);
    let answer1 = count_trees(&input);

    println!("Answer 1: {} trees would be encountered", answer1);
}

#[cfg(test)]
mod test {
    use crate::{count_trees, parse_treemap};
    use itertools::Itertools;

    const SAMPLE: &[u8] = include_bytes!("../../input/day3_sample.txt");

    #[test]
    pub fn test_parse_simple() {
        let sample = ".#.\n#.#".bytes().collect_vec();
        let expected = vec![vec![false, true, false], vec![true, false, true]];

        let actual = parse_treemap(&sample);

        assert_eq!(actual, expected);
    }

    #[test]
    pub fn test_parse_sample() {
        let actual = parse_treemap(&SAMPLE);
        let expected = vec![
            vec![
                false, false, true, true, false, false, false, false, false, false, false,
            ],
            vec![
                true, false, false, false, true, false, false, false, true, false, false,
            ],
            vec![
                false, true, false, false, false, false, true, false, false, true, false,
            ],
            vec![
                false, false, true, false, true, false, false, false, true, false, true,
            ],
            vec![
                false, true, false, false, false, true, true, false, false, true, false,
            ],
            vec![
                false, false, true, false, true, true, false, false, false, false, false,
            ],
            vec![
                false, true, false, true, false, true, false, false, false, false, true,
            ],
            vec![
                false, true, false, false, false, false, false, false, false, false, true,
            ],
            vec![
                true, false, true, true, false, false, false, true, false, false, false,
            ],
            vec![
                true, false, false, false, true, true, false, false, false, false, true,
            ],
            vec![
                false, true, false, false, true, false, false, false, true, false, true,
            ],
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    pub fn test_count_trees() {
        let sample = parse_treemap(SAMPLE);
        let actual = count_trees(&sample);

        assert_eq!(actual, 7)
    }
}
