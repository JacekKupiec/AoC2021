use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const STEPS_NUM : u32 = 20;

fn main() {
    let input = File::open("D:\\source\\Rust\\AoC 2021\\day_14\\src\\big_input.txt").unwrap();
    let mut reader = BufReader::new(input);
    let mut buffer = String::new();

    reader.read_line(&mut buffer).expect("There has to be a initial polymer");
    let initial_polymer = Vec::from(buffer.trim_end());
    reader.read_line(&mut buffer).expect("There has to be a blank line"); //read a separator line
    buffer.clear(); // prepare buffer for reading rules

    let mut mappings = HashMap::new();

    while let Ok(bytes_read) = reader.read_line(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        let line_to_parse = buffer.trim_end().as_bytes();
        let pattern = encode_tuple(line_to_parse[0], line_to_parse[1]);
        let transformation = line_to_parse[6];

        mappings.insert(pattern, transformation);

        buffer.clear();
    }

    println!("Mapping read");

    let mut element_pair_count_ = HashMap::new();

    for key in mappings.keys() {
        let high = (*key >> 8) as u8;
        let low = *key as u8;
        let start_polymer = vec![high, low];
        let transformed_polymer = apply_polymerization_steps(&start_polymer, &mappings, STEPS_NUM);
        let mut quantities = calculate_characters_quantity(&transformed_polymer);
        
        if let Some(lows_count) = quantities.get_mut(&low) {
            *lows_count -= 1;
        }

        element_pair_count_.insert(*key, quantities);

        println!("Polymer ready {:?}", start_polymer);
    }

    println!("Polymers precomputed");

    let half_of_polymer = apply_polymerization_steps(&initial_polymer, &mappings, STEPS_NUM);

    println!("Half of polymer ready");

    let mut unique_characters = mappings.values().map(|v| *v).collect::<Vec<u8>>();
    unique_characters.sort();
    unique_characters.dedup();

    let initial_dictionary: HashMap<u8, u64> = unique_characters.iter().map(|element| (*element, 0u64)).collect();

    println!("Unique characters find: {:?}", initial_dictionary);

    let mut final_counts = half_of_polymer.windows(2)
        .map(|particles_pair| {
            let key = encode_tuple(particles_pair[0], particles_pair[1]);
            let message = format!("There has to be a key {}{}", particles_pair[0] as char, particles_pair[1] as char);

            element_pair_count_.get(&key).expect(&message)
        })
        .fold(initial_dictionary, |accumulator, current| {
            let mut result_dict = HashMap::new();

            for (acc_key, acc_value) in accumulator {
                let final_sum = if let Some(curr_sum) = current.get(&acc_key) { *curr_sum } else { 0 };

                result_dict.insert(acc_key, acc_value + final_sum);
            }

            result_dict
        });

    let last_character = half_of_polymer.last().expect("The half-polymer has to have the last element (not to be empty)");

    if let Some(last_char_count) = final_counts.get_mut(last_character) {
        *last_char_count += 1;
    }

    let most_common_element_quantity = final_counts.values().max().unwrap();
    let least_common_element_quantity = final_counts.values().min().unwrap();
    println!("Answer: {}", most_common_element_quantity - least_common_element_quantity);
    println!("Final counts:\n{:?}\n\n", final_counts);

    // debug code
    /*let correct_answer = apply_polymerization_steps(&initial_polymer, &mappings, 2*STEPS_NUM);
    let correct_counts = calculate_characters_quantity(&correct_answer);
    let most_common_element_quantity = correct_counts.values().max().unwrap();
    let least_common_element_quantity = correct_counts.values().min().unwrap();

    println!("Answer: {}", most_common_element_quantity - least_common_element_quantity);
    println!("Correct counts:\n{:?}", correct_counts);*/
}

fn encode_tuple(high: u8, low: u8) -> u16 {
    ((high as u16) << 8) + low as u16
}

fn apply_polymerization_step(initial_polymer: &Vec<u8>, mappings: &HashMap<u16, u8>) -> Vec<u8> {
    let mut new_polymer = Vec::with_capacity(2*initial_polymer.len() - 1);

    for slice in initial_polymer.windows(2) {
        let patter = encode_tuple(slice[0], slice[1]);

        new_polymer.push(slice[0]);
        new_polymer.push(mappings[&patter]);
    }

    new_polymer.push(initial_polymer[initial_polymer.len() - 1]);

    return new_polymer;
}

fn apply_polymerization_steps(initial_polymer: &Vec<u8>, mappings: &HashMap<u16, u8>, steps: u32) -> Vec<u8> {
    let mut polymer = initial_polymer.clone();

    for _ in 0..steps {
        polymer = apply_polymerization_step(&polymer, &mappings);
    }

    return polymer;
}

fn calculate_characters_quantity(polymer : &Vec<u8>) -> HashMap<u8, u64> {
    let mut char_counts = HashMap::new();

    for c in polymer {
        if char_counts.contains_key(c) {
            let counter = char_counts.get_mut(c).expect("I have to get mutable reference when value exists");
            *counter = *counter + 1;
        } else {
            char_counts.insert(*c, 1u64);
        }
    }

    return char_counts;
}