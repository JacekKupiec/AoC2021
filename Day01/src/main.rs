use std::io;
use std::io::Read;

fn main() {
    let mut input_text = String::new();

    io::stdin()
        .read_to_string(&mut input_text)
        .expect("Nie udało się wczytać nowej linii");

    let parsed_numbers : Vec<u32> =  input_text.lines().map( |x| x.trim().parse().unwrap()).collect();
    let range = 0..(parsed_numbers.len() - 2);
    let mut previous_sum = u32::MAX;
    let mut counter = 0;

    for idx in range {
        let sum = parsed_numbers[idx] + parsed_numbers[idx + 1] + parsed_numbers[idx + 2];

        if sum > previous_sum {
            counter += 1;
        }

        previous_sum = sum;
    }

    println!("Liczba wzniesień: {}", counter);
}
