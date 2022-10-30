use std::fs::File;
use std::io::{BufRead, BufReader, Read, stdin};

struct BitMap {
    bits: Vec<u8>
}

impl BitMap {
    fn get_integer(&self, start: usize, stop: usize) -> usize {
        let mut result = 0;
        let bit_range = start..stop;

        for bit_idx in bit_range {
            result = result * 2 + self.get_bit(bit_idx);
        }

        return result;
    }

    fn get_bit(&self, position: usize) -> usize {
        let byte_idx = position >> 3;
        let bit_idx = 7 - (position & 7);

        if self.bits[byte_idx] & (1 << bit_idx) != 0 {
            1
        } else {
            0
        }
    }
}

impl From<&[u8]> for BitMap {
    fn from(array: &[u8]) -> Self {
        Self {
            bits: Vec::from(array)
        }
    }
}

enum PacketTypeId {
    Sum = 0,
    Product,
    Minimum,
    Maximum,
    Literal,
    GreaterThan,
    LessThan,
    EqualTo
}

fn main() {
    let input = File::open("D:\\source\\Rust\\AoC 2021\\day_16\\src\\big_input.txt").unwrap();
    let mut reader = BufReader::new(input);
    let mut buffer = String::new();

    reader.read_line(&mut buffer).expect("Podany plik z danymi wejściowymi nie istnieje");

    let hex_string_without_whitespaces: Vec<u8> = buffer.trim_end().bytes().collect();
    let mut parsed_input = vec![0u8; hex_string_without_whitespaces.len() / 2];

    for (idx, c) in hex_string_without_whitespaces.iter().enumerate() {
        let converted_from_hex = match c {
            b'0'..=b'9' => c - b'0',
            b'A'..=b'F' => c - b'A' + 10,
            _ => panic!("Unexpected character found: {}", c)
        };

        if idx & 1 == 0 {
            parsed_input[idx / 2] |= converted_from_hex << 4;
        } else {
            parsed_input[idx / 2] |= converted_from_hex;
        }
    }

    let bit_map = BitMap::from(parsed_input.as_slice());
    let (result, _) = compute_packet(&bit_map, 0);

    println!("Operator result: {}", result);
}

fn compute_packet(bit_map: &BitMap, start: usize) -> (isize, usize) {
    let type_id = bit_map.get_integer(start + 3, start + 6);

    if type_id == PacketTypeId::Literal as usize {
        let (literal_value, literal_length) = compute_literal(&bit_map, start + 6);

        (literal_value, literal_length + 6)
    } else {
        let (operator_result, bits_subpackets) = compute_operator(&bit_map, type_id, start + 6);
        (operator_result, 6 + bits_subpackets)
    }
}

fn compute_operator(bit_map: &BitMap, type_id : usize, mut start: usize) -> (isize, usize) {
    let length_type_id = bit_map.get_bit(start);
    let mut subresults = Vec::new();

    if length_type_id == 0 {
        let sub_packages_bit_length = bit_map.get_integer(start + 1, start + 16);
        let mut bits_count = 0;

        start += 16; // 1 + 15

        while bits_count < sub_packages_bit_length {
            let (subpacket_result, sub_package_length) = compute_packet(bit_map, start);

            subresults.push(subpacket_result);
            start += sub_package_length;
            bits_count += sub_package_length;
        }

        let operator_result = process_operator(&subresults, type_id);

        return (operator_result, 1 + 15 + sub_packages_bit_length);
    } else {
        let sub_packages_cnt = bit_map.get_integer(start + 1, start + 12);
        let mut bits_count = 12; // 1 + 11

        start += 12;

        for _ in 0..sub_packages_cnt {
            let (subpacket_result, sub_length) = compute_packet(bit_map, start);

            subresults.push(subpacket_result);
            start += sub_length;
            bits_count += sub_length;
        }

        let operator_result = process_operator(&subresults, type_id);

        return (operator_result, bits_count);
    }
}

fn compute_literal(bit_map: &BitMap, mut start: usize) -> (isize, usize) {
    let mut bits_to_drop = 0;
    let mut literal_value = 0isize;

    loop {
        let literal_part = bit_map.get_integer(start, start + 5) as isize;

        bits_to_drop += 5;
        literal_value = literal_value << 4 | literal_part & 0xF; // << - the highest priority, & - lower, | - the lowest
        start += 5;

        if literal_part & 0x10 == 0 {
            break (literal_value, bits_to_drop);
        }
    }
}

fn process_operator(subresults : &Vec<isize>, type_id : usize) -> isize {
    if type_id == PacketTypeId::Sum as usize {
        return subresults.iter().sum();
    } else if type_id == PacketTypeId::Product as usize {
        return subresults.iter().product();
    } else if type_id == PacketTypeId::Minimum as usize {
        return *subresults.iter().min().unwrap();
    } else if type_id == PacketTypeId::Maximum as usize {
        return *subresults.iter().max().unwrap();
    } else if type_id == PacketTypeId::GreaterThan as usize {
        return (subresults[0] > subresults[1]) as isize;
    } else if type_id == PacketTypeId::LessThan as usize {
        return (subresults[0] < subresults[1]) as isize;
    } else if type_id == PacketTypeId::EqualTo as usize {
        return (subresults[0] == subresults[1]) as isize;
    }

    panic!("Unknown operation");
}