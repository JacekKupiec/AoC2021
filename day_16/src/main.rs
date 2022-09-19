use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let input = File::open("D:\\source\\Rust\\AoC 2021\\day_16\\src\\big_input.txt").unwrap();
    let mut reader = BufReader::new(input);
    let mut buffer = [0; 400];

    reader.read(&mut buffer).expect("Podany plik z danymi wejściowymi nie istnieje");

    for element in &mut buffer {
        match *element {
            b'0'..=b'9' => *element -= b'0',
            b'A'..=b'F' => *element -= b'A' - 10,
            _ => panic!("Unexpected character found: {}", *element)
        }
    }

    let mut bit_position = 0;

    loop {
        
    }
}
