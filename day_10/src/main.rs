use std::collections::HashMap;
use std::io;

fn is_complementary_pair(left : char, right: char) -> bool {
    return (left == '(' && right == ')')
        || (left == '[' && right == ']')
        || (left == '{' && right == '}')
        || (left == '<' && right == '>');
}

fn get_complementary_character(c : char) -> char {
    match c {
        '(' =>  ')',
        '{' => '}',
        '[' => ']',
        '<' => '>',
        _ => panic!("Unexpected symbol")
    }
}

fn is_valid_string(line: &str) -> Result<Vec<char>, String> {
    let mut stack : Vec<char> = Vec::new();

    for c in line.trim_end().chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                if let Some(ch) = stack.last() {
                    if is_complementary_pair(*ch, c) {
                        stack.pop();
                    } else {
                        return Err(String::from("Invalid string")); // Find the first occurrence of incorrect closing character
                    }
                }
                else { // When there are more closing characters than opening ones
                    return Err(String::from("Invalid string"));// Find the first occurrence of incorrect closing character
                }
            },
            _ => panic!("Invalid character!")
        }
    }

    return Ok(stack);
}

fn main() {
    let mut line = String::new();
    let mut all_results = Vec::new();

    while let Ok(bytes_read) = io::stdin().read_line(&mut line) {
        if bytes_read == 0 {
            break;
        }

        if let Ok(mut stack) = is_valid_string(&line) {
            let chars_points = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
            let mut points_counter: u64 = 0;

            while let Some(c) = stack.pop() {
                points_counter = points_counter*5 + chars_points[&c];
            }

            all_results.push(points_counter);
        }

        line.clear();
    }

    all_results.sort();
    println!("{}", all_results[all_results.len() / 2]);
}
