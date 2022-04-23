use std::borrow::Borrow;
use std::cmp::Ordering;
use std::io;
use std::io::Read;

fn main() {
    task2();
}

fn count_bits(lines : &Vec<&[u8]>, position : usize, length : usize) -> (usize, usize) {
    let mut counter_ones = 0;

    for line_idx in 0..length {
        counter_ones = if lines[line_idx][position] == b'1' { counter_ones + 1 } else { counter_ones };
    }

    return (length - counter_ones, counter_ones);
}

fn get_oxygen_bit(count_zeroes: usize, count_ones: usize) -> u8 {
    return match count_ones.cmp(&count_zeroes) {
        Ordering::Less => b'0',
        _ => b'1'
    };
}

fn get_co2_bit(count_zeroes: usize, count_ones: usize) -> u8 {
    return match count_ones.cmp(&count_zeroes) {
        Ordering::Less => b'1',
        _ => b'0'
    };
}

fn pick_by_bit_criterion(lines: &mut Vec<&[u8]>, length: &mut usize, position: usize, bit_criterion : u8) {
    let vec_size = *length;
    let mut vec_pos = 0;

    for _ in 0..vec_size {
        if lines[vec_pos][position] != bit_criterion {
            let tmp_buffer = lines[vec_pos];
            lines[vec_pos] = lines[*length - 1];
            lines[*length - 1] = tmp_buffer;
            *length = *length - 1;
        } else {
            vec_pos += 1;
        }
    }
}

fn task2() {
    let mut entire_input = String::new();

    io::stdin()
        .read_to_string(&mut entire_input)
        .expect("Error getting input");


    let mut samples: Vec<&[u8]> = entire_input.lines().map(|x| x.trim_end().as_bytes()).collect();
    let mut real_vec_len = samples.len();
    let mut oxygen_generator_rating : u32 = 1;

    for column_idx in 0..12 {
        let (count_zeroes, count_ones) = count_bits(&samples, column_idx, real_vec_len);
        let oxygen_bit = get_oxygen_bit(count_zeroes, count_ones);

        pick_by_bit_criterion(&mut samples, &mut real_vec_len, column_idx, oxygen_bit);

        if real_vec_len <= 1 {
            let binary_code = String::from_utf8_lossy(samples[0]);
            oxygen_generator_rating = u32::from_str_radix(binary_code.borrow(), 2).unwrap();
            println!("{0} {1}", binary_code, oxygen_generator_rating);
            break;
        }
    }

    let mut co2_scrubber_rating : u32 = 1;
    real_vec_len = samples.len();

    for column_idx in 0..12 {
        let (count_zeroes, count_ones) = count_bits(&samples, column_idx, real_vec_len);
        let co2_bit = get_co2_bit(count_zeroes, count_ones);

        pick_by_bit_criterion(&mut samples, &mut real_vec_len, column_idx, co2_bit);

        if real_vec_len <= 1 {
            let binary_code = String::from_utf8_lossy(samples[0]);
            co2_scrubber_rating = u32::from_str_radix(binary_code.borrow(), 2).unwrap();
            println!("{} {}", binary_code, co2_scrubber_rating);
            break;
        }
    }

    println!("Answer = {}", oxygen_generator_rating*co2_scrubber_rating);
}

fn task1() {
    const SAMPLE_LENGTH: usize = 12;
    const BITMASK: i32 = (1 << SAMPLE_LENGTH) - 1;
    let mut counts = [0; SAMPLE_LENGTH];
    let mut counter = 0;

    loop {
        let mut input_line = String::new();
        let bytes_read = io::stdin().read_line(&mut input_line).unwrap();

        if bytes_read == 0 {
            break;
        }

        for (idx, character) in input_line.trim_end().chars().enumerate() {
            if character == '1' {
                counts[idx] += 1;
            }
        }

        counter += 1;
    }

    let treshold = counter / 2;
    let mut gamma_rate = 0;
    counts.reverse();

    for (idx, count) in counts.iter().enumerate() {
        if *count > treshold {
            gamma_rate += 1 << idx;
        }
    }

    let epsilon_rate = !gamma_rate & BITMASK;

    println!("GR: {} ER: {} Product: {}", gamma_rate, epsilon_rate, gamma_rate * epsilon_rate);
}
