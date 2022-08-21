use std::collections::{HashMap, HashSet};
use std::io;

fn dfs(graph: &HashMap<String, Vec<String>>, label: &str, visited: &mut HashSet<String>, counter: &mut i32, key: &str) {
    if label == "end" {
        *counter = *counter + 1;
    }
    else {
        if label.chars().any(|c| c.is_lowercase()) {
            visited.insert(label.to_string());
        }

        if let Some(neighbours) = graph.get(label) {
            for neighbour in neighbours {
                if !visited.contains(neighbour) {
                    dfs(graph, neighbour, visited, counter, key);
                }
            }
        }

        visited.remove(label);
    }
}

fn main() {
    let mut buffer = String::new();
    let input = io::stdin();
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    while let Ok(bytes_read) = input.read_line(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        if let Some((source_verticle_name, target_verticle_name)) = buffer.trim_end().split_once('-') {
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

    println!("Wczytany graf:\n{:?}", graph);

    let mut visited : HashSet<String> = HashSet::new();
    let mut counter_all_cases = 0;
    let can_be_visited_twice = graph.keys().filter(|key|
            key.chars().any(|c|
                    c.is_lowercase())
                    && *key != "end"
                    && *key != "start");

    for key in can_be_visited_twice {
        let mut counter = 0;
        dfs(&graph, "start", &mut visited, &mut counter, key);

        counter_all_cases += counter;
    }

    println!("Różne ścieżki: {}", counter_all_cases);
}
