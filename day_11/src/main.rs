use std::io;

fn propagate_flash(octopuses: &mut Vec<Vec<u8>>, row: usize, column: usize) {
    let flash_threshold = 10;

    octopuses[row][column] += 1;

    if octopuses[row][column] == flash_threshold {
        // lower row + 1 slice --- 3 cells
        if let Some(_) = octopuses.get(row + 1) {
            propagate_flash(octopuses, row + 1, column);

            if let Some(_) = octopuses[row + 1].get(column + 1) {
                propagate_flash(octopuses, row + 1, column + 1);
            }

            if column > 0 {
                propagate_flash(octopuses, row + 1, column - 1);
            }
        }

        // 1 step right in the middle slice
        if let Some(_) = octopuses[row].get(column + 1) {
            propagate_flash(octopuses, row, column + 1)
        }

        // 1 step left in the middle slice
        if column > 0 {
            propagate_flash(octopuses, row, column - 1);
        }

        // upper row - 1 slice --- 3 cells
        if row > 0 {
            propagate_flash(octopuses, row - 1, column);

            if column > 0 {
                propagate_flash(octopuses, row - 1, column - 1);
            }

            if let Some(_) = octopuses[row - 1].get(column + 1) {
                propagate_flash(octopuses, row - 1, column + 1);
            }
        }
    }
}

fn main() {
    let mut buffer = String::new();
    let input = io::stdin();

    while let Ok(bytes_read) = input.read_line(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
    }

    let mut octopuses : Vec<Vec<u8>> = buffer.lines()
        .map(|line| line.trim().bytes().map(|c| c - b'0').collect())
        .collect();

    let mut iter_counter = 0;
    let synchronized_flash_step = loop {
        let row_count = octopuses.len();
        let column_count = octopuses[0].len(); // I know that this is a square matrix

        for row_idx in 0..row_count {
            for column_idx in 0..column_count {
                propagate_flash(&mut octopuses, row_idx, column_idx);
            }
        }

        let threshold_restart_octopus = 9;
        for row_idx in 0..row_count {
            for column_idx in 0..column_count {
                if octopuses[row_idx][column_idx] > threshold_restart_octopus {
                    octopuses[row_idx][column_idx] = 0;
                }
            }
        }

        iter_counter += 1;

        if octopuses.iter().flatten().all(|e| *e == 0) {
            break iter_counter;
        }
    };

    println!("Flashes number: {}", synchronized_flash_step);
}