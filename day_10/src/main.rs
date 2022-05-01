use std::collections::HashMap;
use std::io;

fn main() {
    let mut line = String::new();
    let mut stack : Vec<char> = Vec::new();
    let mut points_counter = 0;
    let points_for_character = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    while let Ok(bytes_read) = io::stdin().read_line(&mut line) {
        if bytes_read == 0 {
            break;
        }

        for c in line.trim_end().chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    if let Some(ch) = stack.last() {
                        if *ch == c {
                            stack.pop();
                        } else {
                            points_counter += points_for_character[&c];
                            break; // Find the first occurrence of incorrect closing character
                        }
                    }
                    else { // When there are more closing characters than opening ones
                        points_counter += points_for_character[&c];
                        break;// Find the first occurrence of incorrect closing character
                    }
                },
                _ => panic!("Niewłaściwy znak")
            }
        }

        line.clear();
        stack.clear();
    }

    println!("{}", points_counter);
}
