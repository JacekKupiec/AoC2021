use std::collections::HashMap;
use std::io;

fn main() {
    let mut buffer = String::new();
    let input = io::stdin();
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    while let Ok(bytes_read) = input.read_line(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        if let Some((source_verticle_name, target_verticle_name)) = buffer.split_once('-') {
            if let Some(neighbour_list) = graph.get_mut(source_verticle_name) {
                neighbour_list.push(target_verticle_name.to_string());
            } else {
                graph.insert(source_verticle_name.to_string(), vec![target_verticle_name.to_string()]);
            }

            if let Some(neighbour_list) = graph.get_mut(target_verticle_name) {
                neighbour_list.push(source_verticle_name.to_string());
            } else {
                graph.insert(target_verticle_name.to_string(), vec![source_verticle_name.to_string()]);
            }
        }

        buffer.clear();
    }
}
