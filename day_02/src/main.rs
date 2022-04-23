use std::cmp::max;
use std::io;

fn main() {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    loop {
        let mut input_line = String::new();

        let line_length_bytes = io::stdin().read_line(&mut input_line)
            .expect("Wrong line!");

        if line_length_bytes == 0 {
            println!("Horizontal position: {}\nDepth: {}\nProduct: {}", horizontal, depth, horizontal * depth);
            break;
        }

        let (command, value) = input_line.trim_end().split_once(' ').unwrap();
        let parsed_value: i32 = value.parse().unwrap();

        match command {
            "down" => aim += parsed_value,
            "up" => aim = max(0, aim - parsed_value),
            "forward" => {
                horizontal += parsed_value;
                depth += aim * parsed_value;
            },
            _ => {}
        }
    }
}
