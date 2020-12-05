use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day5.txt");

fn get_seat_id(s: &str) -> u16 {
    s.bytes()
        // turn the specifications into ones and zeros: F -> 0, B -> 1, L -> 0, R -> 1
        .map(|b| u16::from(b == b'B' || b == b'R'))
        // then, turn the list of bits/bitmap into a number
        .fold(0_u16, |acc, bit| (acc << 1) + bit)
}

fn find_my_seat(boarding_pass_ids: &[u16]) -> u16 {
    boarding_pass_ids
        .iter()
        // boarding passes are unordered, so bring them in order before checking for our seat
        .sorted()
        // look at our potential neighbors
        .tuple_windows::<(&u16, &u16)>()
        .find_map(|(a, b)| {
            // if there is exactly one seat (ID) between, that's our seat!
            let my_seat_a = *a + 1;
            let my_seat_b = *b - 1;
            if my_seat_a == my_seat_b {
                Some(my_seat_a)
            } else {
                None
            }
        })
        .unwrap()
}

fn main() {
    let boarding_pass_ids = INPUT.split('\n').map(get_seat_id).collect_vec();

    let answer1 = *boarding_pass_ids.iter().max().unwrap();
    println!(
        "Answer 1: {} is the highest seat ID on a boarding pass",
        answer1
    );

    let answer2 = find_my_seat(&boarding_pass_ids);
    println!("Answer 2: {} is my seat ID", answer2);
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
