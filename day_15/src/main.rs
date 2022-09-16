use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::{min, Ordering};
use std::collections::BinaryHeap;

/*
    small: 40
    medium: 315
    big: 403
    large: ???
 */

#[derive(Debug)]
struct Edge {
    weight: u32,
    target: usize
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct PriorityQueueItem {
    distance: u32,
    vertex: usize
}

impl Ord for PriorityQueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}

fn main() {
    let input = File::open("D:\\source\\Rust\\AoC 2021\\day_15\\src\\medium_input.txt").unwrap();
    let mut reader = BufReader::new(input);
    let mut buffer = String::new();
    let mut risk_level_map = Vec::new();

    while let Ok(bytes_read) = reader.read_line(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        let row : Vec<u8> = buffer.trim_end().bytes().map(|c| c - b'0').collect();
        risk_level_map.push(row);

        buffer.clear();
    }

    let graph = build_graph(&risk_level_map);
    println!("The distance between source and destination: {}", calculate_distance_priority_queue(&graph, 0, graph.len() - 1));
}

fn build_graph(risk_level_map : &Vec<Vec<u8>>) -> Vec<Vec<Edge>> {
    let row_count = risk_level_map.len();
    let column_count = risk_level_map[0].len();
    let mut graph = Vec::with_capacity(row_count * column_count);

    for row_idx in 0..row_count {
        for column_idx in 0..column_count {
            let mut edges = Vec::with_capacity(4);

            if let Some(risk_level) =  risk_level_map[row_idx].get(column_idx + 1) {
                edges.push(Edge {
                    weight: *risk_level as u32,
                    target: row_idx* column_count + column_idx + 1
                });
            }

            if column_idx > 0 {
                if let Some(risk_level) = risk_level_map[row_idx].get(column_idx - 1) {
                    edges.push(Edge {
                        weight: *risk_level as u32,
                        target: row_idx*column_count + column_idx - 1
                    });
                }
            }

            if let Some(risk_level_row) = risk_level_map.get(row_idx + 1) {
                edges.push(Edge {
                    weight: risk_level_row[column_idx] as u32,
                    target: (row_idx + 1)*column_count + column_idx
                });
            }

            if row_idx > 0 {
                if let Some(risk_level_row) = risk_level_map.get(row_idx - 1) {
                    edges.push(Edge {
                        weight: risk_level_row[column_idx] as u32,
                        target: (row_idx - 1)*column_count + column_idx
                    })
                }
            }

            graph.push(edges);
        }
    }

    return graph;
}

fn calculate_distance_priority_queue(graph: &Vec<Vec<Edge>>, source: usize, target: usize) -> u32 {
    let mut distances = vec![u32::MAX; graph.len()];
    let mut visit_statuses = vec![false; graph.len()];
    let mut current_vertex = source;
    let mut priority_queue = BinaryHeap::with_capacity(graph.len());

    distances[source] = 0;
    priority_queue.push(PriorityQueueItem {
        distance: 0,
        vertex: source
    });

    while !priority_queue.is_empty() {
        let current_vertex = priority_queue.pop().expect("Pop must work on not empty queue");

        if current_vertex.vertex == target {
            return current_vertex.distance;
        }

        visit_statuses[current_vertex.vertex] = true;

        for edge in &graph[current_vertex.vertex] {
            let current_distance = distances[edge.target];
            let new_distance = distances[current_vertex.vertex] + edge.weight;

            distances[edge.target] = min(current_distance, new_distance);

            if !visit_statuses[edge.target] {
                priority_queue.push(PriorityQueueItem {
                    distance: distances[edge.target],
                    vertex: edge.target
                });
            }
        }
    }

    return u32::MAX;
}

fn calculate_distance_find_max(graph: &Vec<Vec<Edge>>, source: usize, target: usize) -> u32 {
    let mut distances = vec![u32::MAX; graph.len()];
    let mut visit_statuses = vec![false; graph.len()];
    let mut current_vertex = source;

    distances[source] = 0;

    return loop {
        visit_statuses[current_vertex] = true;

        for edge in &graph[current_vertex] {
            let current_distance = distances[edge.target];
            let new_distance = distances[current_vertex] + edge.weight;

            distances[edge.target] = min(current_distance, new_distance);
        }

        if let Some(vertex) = find_minimal(&distances, &visit_statuses) {
            if vertex == target {
                break distances[target];
            }

            current_vertex = vertex;
        }
        else {
            break u32::MAX;
        }
    }
}

fn find_minimal(distances: &Vec<u32>, visit_statuses: &Vec<bool>) -> Option<usize> {
    let result = distances.iter()
        .enumerate()
        .map(|(idx, d)| if visit_statuses[idx] == true { (idx, u32::MAX) } else { (idx, *d) })
        .min_by(|x, y| x.1.cmp(&(y.1)));

    match result {
        Some((_, u32::MAX)) | None => None,
        Some((idx, _)) => Some(idx)
    }
}
