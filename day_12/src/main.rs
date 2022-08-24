use std::collections::{HashMap, HashSet};
use std::io;

/*Zamiast kombinować z liczeniem odwiedziny po prostu dodaj jeszcze jeden wierzchołek*/

fn is_node_auxiliary(node_name: &str) -> bool {
    return node_name.ends_with("0");
}

fn dfs(graph: &HashMap<String, Vec<String>>, label: &str, visited: &mut HashSet<String>, is_auxiliary_node_used: bool) -> i32 {
    if label == "end" {
        return 1;
    }
    else {
        let is_lowercase_node = label.chars().any(|c| c.is_lowercase());

        if is_lowercase_node {
            visited.insert(label.to_string());
        }

        let mut counter = 0;

        if let Some(neighbours) = graph.get(label) {
            for neighbour in neighbours {
                let is_neighbour_auxiliary_node = is_node_auxiliary(neighbour);
                let can_visit_normal_node = !visited.contains(neighbour) && !is_neighbour_auxiliary_node;
                let can_visit_auxiliary_node = is_neighbour_auxiliary_node
                    && !is_auxiliary_node_used
                    && visited.contains(&neighbour[0..(neighbour.len() - 1)]);

                if can_visit_normal_node || can_visit_auxiliary_node {
                    counter += dfs(graph, neighbour, visited, is_neighbour_auxiliary_node || is_auxiliary_node_used);
                }
            }
        }

        if is_lowercase_node {
            visited.remove(label);
        }

        return counter;
    }
}

fn main() {
    let input = io::stdin();
    let mut buffer = String::new();
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    while let Ok(bytes_read) = input.read_line(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        if let Some((source_vertex_name, target_vertex_name)) = buffer.trim_end().split_once('-') {
            if let Some(neighbour_list) = graph.get_mut(source_vertex_name) {
                neighbour_list.push(target_vertex_name.to_string());
            } else {
                graph.insert(source_vertex_name.to_string(), vec![target_vertex_name.to_string()]);
            }

            if let Some(neighbour_list) = graph.get_mut(target_vertex_name) {
                neighbour_list.push(source_vertex_name.to_string());
            } else {
                graph.insert(target_vertex_name.to_string(), vec![source_vertex_name.to_string()]);
            }
        }

        buffer.clear();
    }

    let lowercase_nodes : Vec<_> = graph.keys()
        .filter(|label| label.chars().any(|c| c.is_lowercase()) && *label != "start" && *label != "end")
        .map(|label| label.to_string())
        .collect();

    for label in lowercase_nodes {
        let neighbours = graph[&label].clone();

        for neighbour in neighbours.iter() {
            let new_label = format!("{}{}", label, 0);
            let neighbours_of_neighbour = graph.get_mut(neighbour).unwrap();

            neighbours_of_neighbour.push(new_label);
        }

        let new_label = format!("{}{}", label, 0);
        graph.insert(new_label, neighbours.clone());
    }

    println!("Wczytany graf z modyfikacjami:\n{:?}", graph);

    let mut visited : HashSet<String> = HashSet::new();
    let counter = dfs(&graph, "start", &mut visited, false);

    println!("Różne ścieżki: {}", counter);
}
