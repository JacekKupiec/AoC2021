use std::io;
use crate::VertexStatus::{NotVisited, Visited};

fn is_low_point(board : &Vec<Vec<u8>>, first_dim_idx : usize, second_dim_idx: usize) -> bool {
    let low_point_candidate = board[first_dim_idx][second_dim_idx];

    // one step left
    if second_dim_idx >= 1 {
        if board[first_dim_idx][second_dim_idx - 1] <= low_point_candidate {
            return false;
        }
    }

    // one step right
    if let Some(height) = board[first_dim_idx].get(second_dim_idx + 1) {
        if *height <= low_point_candidate {
            return false;
        }
    }

    // one step up
    if first_dim_idx >= 1  {
        if board[first_dim_idx -1][second_dim_idx] <= low_point_candidate {
            return false;
        }
    }

    // one step down
    if let Some(row) = board.get(first_dim_idx + 1) {
        if row[second_dim_idx] <= low_point_candidate {
            return false;
        }
    }

    return true;
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum VertexStatus {
    NotVisited,
    Visited
}

fn get_basin_size(board : &Vec<Vec<u8>>, vertices_statuses : &mut Vec<Vec<VertexStatus>>, first_dim_idx : usize, second_dim_idx : usize) -> usize {
    let mut stack = vec![(first_dim_idx, second_dim_idx)];
    let mut counter = 0;

    while !stack.is_empty() {
        if let Some((x, y)) = stack.pop() {
            if board[x][y] >= 9 || vertices_statuses[x][y] == Visited {
                continue;
            }

            vertices_statuses[x][y] = Visited;
            counter += 1;

            if y >= 1 {// one step left
                stack.push((x, y - 1));
            }

            if x >= 1 { // one step up
                stack.push((x -1, y));
            }

            if y + 1 < board[x].len() {  // one step right
                stack.push((x, y + 1));
            }

            if x + 1 < board.len() {
                stack.push((x + 1, y));
            }
        }
        else {
            panic!("Can't pop from stack despite being not empty!");
        }
    }

    return counter;
}

fn main() {
    let mut board : Vec<_> = Vec::new();
    let mut buffer = String::new();

    while let Ok(bytes_read) = io::stdin().read_line(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        let row_parsed : Vec<u8> = buffer.trim_end().bytes().map(|b| b - b'0').collect();
        board.push(row_parsed);
        buffer.clear();
    }

    let mut vertices_statuses: Vec<Vec<VertexStatus>> = Vec::with_capacity(board.len());
    let row_size = board[0].len();

    for _ in 0..board.len() {
        vertices_statuses.push(vec![NotVisited; row_size]);
    }

    let mut basin_sizes : Vec<usize> = Vec::new();

    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if is_low_point(&board, i, j) {
                let basin_size = get_basin_size(&board, &mut vertices_statuses, i, j);
                basin_sizes.push(basin_size);
            }
        }
    }

    basin_sizes.sort();
    basin_sizes.reverse();
    println!("{} * {} * {} = {}", basin_sizes[0], basin_sizes[1], basin_sizes[2], basin_sizes[0] * basin_sizes[1] * basin_sizes[2]);
}
