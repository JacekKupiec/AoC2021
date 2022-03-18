use std::collections::{HashMap, HashSet};
use std::io;

fn parse_signals(signal_sequence : &str) -> Vec<HashSet<u8>> {
    signal_sequence.split(' ')
        .map(|segment| -> HashSet<u8> {
            let mut set = HashSet::new();

            for byte in segment.as_bytes() {
                set.insert(*byte);
            }

            return set;
        })
        .collect()
}

fn build_translation(signal_digits : &Vec<HashSet<u8>>) -> Vec<(i32, &HashSet<u8>)> {
    let digit_one = signal_digits.iter().find(|digit| digit.len() == 2).unwrap();
    let digit_seven = signal_digits.iter().find(|digit| digit.len() == 3).unwrap();
    let digit_four = signal_digits.iter().find(|digit| digit.len() == 4).unwrap();
    let digit_eight = signal_digits.iter().find(|digit| digit.len() == 7).unwrap();

    let digit_nine = signal_digits.iter().find(|digit| digit.len() == 6 && digit_four.is_subset(*digit)).unwrap();
    let digit_six = signal_digits.iter().find(|digit| digit.len() == 6 && !digit_one.is_subset(*digit)).unwrap();
    let digit_zero = signal_digits.iter().find(|digit| digit.len() == 6 && *digit != digit_nine && *digit != digit_six).unwrap();

    let digit_three = signal_digits.iter().find(|digit| digit.len() == 5 && digit_one.is_subset(*digit)).unwrap();
    let digit_five = signal_digits.iter().find(|digit| digit.len() == 5 && digit.is_subset(digit_six)).unwrap();
    let digit_two = signal_digits.iter().find(|digit| digit.len() == 5 && *digit != digit_five && *digit != digit_three).unwrap();

    return Vec::from([
        (0, digit_zero),
        (1, digit_one),
        (2, digit_two),
        (3, digit_three),
        (4, digit_four),
        (5, digit_five),
        (6, digit_six),
        (7, digit_seven),
        (8, digit_eight),
        (9, digit_nine),
    ]);
}

fn main() {
    let mut buffer = String::new();
    let mut sum = 0;

    while let Ok(bytes_read) = io::stdin().read_line(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        if let Some((left_half, right_half)) = buffer.trim_end().split_once(" | ") {
            let signal_digits = parse_signals(left_half);
            let digit_translation = build_translation(&signal_digits);
            let digits_to_recognize: Vec<_> = parse_signals(right_half);
            let mut result = 0;

            for digit in digits_to_recognize {
                if let Some((idx, _)) = digit_translation.iter().find(|(_, dgt)| **dgt == digit) {
                    result = result*10 + idx;
                }
            }
            sum += result;
        }

        buffer.clear();
    }

    println!("{}", sum);
}
