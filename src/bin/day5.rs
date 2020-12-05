const INPUT: &str = include_str!("../../input/day5.txt");

fn get_seat_id(s: &str) -> u16 {
    s.bytes()
        // turn the specifications into ones and zeros: F -> 0, B -> 1, L -> 0, R -> 1
        .map(|b| u16::from(b == b'B' || b == b'R'))
        // then, turn the list of bits/bitmap into a number
        .fold(0_u16, |acc, bit| (acc << 1) + bit)
}

fn main() {
    let answer1 = INPUT.split('\n').map(get_seat_id).max().unwrap();

    println!(
        "Answer 1: {} is the highest seat ID on a boarding pass",
        answer1
    )
}

#[cfg(test)]
mod test {
    use crate::get_seat_id;

    #[test]
    fn test_get_seat_id() {
        let sample = "FBFBBFFRLR";
        assert_eq!(get_seat_id(sample), 357);

        assert_eq!(get_seat_id("BFFFBBFRRR"), 567);
        assert_eq!(get_seat_id("FFFBBBFRRR"), 119);
        assert_eq!(get_seat_id("BBFFBBFRLL"), 820);
    }
}
