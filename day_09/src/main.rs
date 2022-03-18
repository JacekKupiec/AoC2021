use std::io;

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

fn main() {
    let mut board : Vec<Vec<u8>> = Vec::new();
    let mut buffer = String::new();

    while let Ok(bytes_read) = io::stdin().read_line(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        let row_parsed : Vec<u8> = buffer.trim_end().bytes().map(|b| b - b'0').collect();
        board.push(row_parsed);
        buffer.clear();
    }

    let mut cumulative_risk = 0;

    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if is_low_point(&board, i, j) {
                cumulative_risk += board[i][j] as u32 + 1;
            }
        }
    }

    println!("{}", cumulative_risk);
}
