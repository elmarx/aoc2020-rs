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

fn count_trees(map: &TreeMap, right: usize, down: usize) -> i32 {
    // walk through all the rows, skip the first row, as we never hit it anyway
    map.iter()
        .enumerate()
        .skip(1)
        // skip rows that we do not hit at all
        .filter(|(row_number, _)| row_number % down == 0)
        .fold(0, |acc, (row_number, row)| {
            // the current step depends on the number of "downs" to go per step.
            let step = row_number / down;

            // index/coordinate where we enter this row is number of steps * #right (cause we go #right to the right)
            // modulo width of a row (since we're repeating the pattern)
            if row.get((step * right) % row.len()).unwrap().to_owned() {
                acc + 1
            } else {
                acc
            }
        })
}

fn main() {
    let input = parse_treemap(INPUT);
    let answer1 = count_trees(&input, 3, 1);

    println!("Answer 1: {} trees would be encountered", answer1);

    let slopes = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let answer2 = slopes
        .iter()
        .map(|(right, down)| count_trees(&input, *right as usize, *down as usize))
        .fold(1 as i64, |acc, v| acc * (v as i64));

    println!("Answer 2: {}", answer2);
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
        let actual = count_trees(&sample, 3, 1);

        assert_eq!(actual, 7)
    }

    #[test]
    pub fn test_variable_slope() {
        let sample = parse_treemap(SAMPLE);
        assert_eq!(count_trees(&sample, 1, 1), 2);
        assert_eq!(count_trees(&sample, 5, 1), 3);
        assert_eq!(count_trees(&sample, 7, 1), 4);
        assert_eq!(count_trees(&sample, 1, 2), 2)
    }
}
