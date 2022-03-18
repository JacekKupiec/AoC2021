use std::io;
use std::io::Read;

#[derive(Debug)]
struct Cell {
    column: usize,
    row: usize,
    number: usize,
    selected: bool
}

#[derive(Debug)]
struct Bingo {
    in_rows: [u8; 5],
    in_columns : [u8; 5]
}

const BOARD_SIZE : usize = 5;
const BYTES_FOR_BOARD : usize = 1 + 15*5;

/* Za pierwszym razem szukalismy planszy, która wygra pierwsza, teraz szukamy planszy, która wygra ostatnia */

fn main() {
    let mut buffer = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .expect("Nie można odczytać linii");

    let bingo_numbers : Vec<_> = buffer.split(',')
        .map(|number| number.trim_end().parse().unwrap())
        .collect();

    let mut bingo_board_raw : [u8; BYTES_FOR_BOARD] = [0; BYTES_FOR_BOARD];
    let mut bingo_boards : Vec<_> = Vec::new();


     while io::stdin().read_exact(&mut bingo_board_raw).is_ok() {
         let bingo_board = String::from_utf8_lossy(&bingo_board_raw);
         let parsed_bingo_board = make_bingo_board(bingo_board.as_ref());

         bingo_boards.push(parsed_bingo_board);
    }

    let mut bingo_checker : Vec<_> = (0..bingo_boards.len()).map(|_|
        Bingo {
            in_rows: [0; BOARD_SIZE],
            in_columns: [0; BOARD_SIZE]
    }).collect();

    let mut bingo_boards_solved = vec![false; bingo_boards.len()];
    let mut boards_counter = bingo_boards.len();

    'outer: for drawn_number in bingo_numbers {
        for (board_idx, board) in bingo_boards.iter_mut().enumerate() {
            if bingo_boards_solved[board_idx] {
                continue;
            }

            if let  Some(cell) = board.iter_mut().find(|c| c.number == drawn_number) {
                cell.selected = true;
                bingo_checker[board_idx].in_rows[cell.row] += 1;

                if bingo_checker[board_idx].in_rows[cell.row] >= BOARD_SIZE as u8 {
                    if boards_counter == 1 {
                        println!("{}", calculate_score(board, drawn_number));
                        break 'outer;
                    }

                    bingo_boards_solved[board_idx] = true;
                    boards_counter -= 1;
                    continue;
                }

                bingo_checker[board_idx].in_columns[cell.column] += 1;

                if bingo_checker[board_idx].in_columns[cell.column]  >= BOARD_SIZE as u8 {
                    if boards_counter == 1 {
                        println!("{}", calculate_score(board, drawn_number));
                        break 'outer;
                    }

                    bingo_boards_solved[board_idx] = true;
                    boards_counter -= 1;
                }
            }
        }
    }
}

fn make_bingo_board(unparsed_board : &str) -> Vec<Cell> {
    unparsed_board
        .trim_start()
        .split_ascii_whitespace()
        .filter(|s| !s.is_empty())
        .enumerate()
        .map(|(idx, number)| {
            Cell {
                column: idx % BOARD_SIZE,
                row: idx / BOARD_SIZE,
                number: number.parse().unwrap(),
                selected: false
            }
        })
        .collect()
}

fn calculate_score(board : &Vec<Cell> , called_number : usize) -> usize {
    board.iter()
        .filter(|cell| !cell.selected)
        .fold(0, |acc, cell| acc + cell.number)
    * called_number
}
