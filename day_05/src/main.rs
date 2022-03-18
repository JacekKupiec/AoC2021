use std::cmp::{max, min};
use std::io;
use num_iter::range_step_inclusive;

struct Point {
    x : i32,
    y : i32
}

impl Point {
    fn is_horizontal(&self, point: &Point) -> bool {
        self.y == point.y
    }

    fn is_vertical(&self, point: &Point) -> bool {
        self.x == point.x
    }

    fn is_classic_diagonal(&self, point: &Point) -> bool {
        let x_abs_diff = if self.x < point.x { point.x - self.x } else {self.x - point.x};
        let y_abs_diff = if self.y < point.y { point.y - self.y } else { self.y - point.y};

        return x_abs_diff == y_abs_diff;
    }
}

const MAP_SIZE: usize = 1000;

fn main() {
    let mut map  = [[0u16; MAP_SIZE]; MAP_SIZE];
    let mut buffer = String::new();

    while let Ok(bytes_read) = io::stdin().read_line(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        let split_by = [' ', '-', '>', ','];
        let coordinates : Vec<_> = buffer
            .trim_end()
            .split(&split_by[..])
            .filter(|number| !number.is_empty())
            .map(|number| number.parse().unwrap())
            .collect();

        let point1 = if coordinates[0] <= coordinates[2] {
            Point {
                x: coordinates[0],
                y: coordinates[1]
            }
        } else {
            Point {
                x: coordinates[2],
                y: coordinates[3]
        } };

        let point2 = if coordinates[0] > coordinates[2] {
            Point {
                x: coordinates[0],
                y: coordinates[1]
            }
        } else {
            Point {
                x: coordinates[2],
                y: coordinates[3]
            }
        };

        if point1.is_horizontal(&point2) {
            let begin = min(point1.x, point2.x) as usize;
            let end = max(point1.x, point2.x) as usize;
            let vertical_dimension = point1.y as usize;

            for horizontal_idx in begin..=end {
                map[vertical_dimension][horizontal_idx] += 1;
            }
        } else if point1.is_vertical(&point2) {
            let begin = min(point1.y, point2.y) as usize;
            let end = max(point1.y, point2.y) as usize;
            let horizontal_dimension = point1.x as usize;

            for vertical_idx in begin..=end {
                map[vertical_idx][horizontal_dimension] += 1;
            }
         } else if point1.is_classic_diagonal(&point2) {
            let begin_x = min(point1.x, point2.x);
            let end_x = max(point1.x, point2.x);
            let x_range : Vec<_> = (begin_x..=end_x).collect();

            let step = if point1.y < point2.y { 1 } else {-1};
            let y_range: Vec<_> = range_step_inclusive(point1.y, point2.y, step).collect();

            let xy_indices = x_range.iter().zip(y_range.iter());

            for (x_idx, y_idx) in xy_indices {
                map[*y_idx as usize][*x_idx as usize] += 1;
            }
        }
        else {
            println!("Mam jeszcze jeden przypadek !!!");
        }

        buffer.clear();
    }

    let mut counter = 0;

    for idx_y in 0..MAP_SIZE {
        for idx_x in 0..MAP_SIZE {
            if map[idx_y][idx_x] >= 2 {
                counter += 1;
            }
        }
    }

    println!("{}", counter);
}
