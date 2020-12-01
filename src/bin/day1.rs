use rayon::prelude::*;

const INPUT: &str = include_str!("../../input/day1_1.txt");

fn main() {
    let input: Vec<i32> = INPUT.split("\n").map(|l| l.parse().unwrap()).collect();

    let (a, b) = find_2020_tuple(&input).expect("did not find any tuple summing up to 2020 :'(");
    let answer = a * b;

    println!("Answer: {0}. ({1} * {2} = {0})", answer, a, b);
}

/// from the list of input, find the two numbers that add up to 2020
fn find_2020_tuple(input: &[i32]) -> Option<(i32, i32)> {
    // simply walk through all numbers…
    input.par_iter().enumerate().find_map_any(|(index, &a)| {
        input
            .par_iter()
            // if we checked a + b, no need to check b + a, so skip all previous values
            .skip(index)
            .find_any(|&b| a + b == 2020)
            // if an actual number has been found, return a and b as tuple
            .map(|&b| (a, b))
    })
}

#[cfg(test)]
mod test {
    use crate::find_2020_tuple;

    #[test]
    fn test_sample() {
        let sample = &[1721, 979, 366, 299, 675, 1456];
        let actual = find_2020_tuple(sample);
        assert_eq!(actual, Some((1721, 299)));
    }
}
