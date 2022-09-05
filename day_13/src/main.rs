use std::collections::HashSet;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use sscanf::scanf;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x : i32,
    y: i32
}

fn fold_along_y(points: &HashSet<Point>, fold_line: i32) -> HashSet<Point> {
    points.iter().filter_map(|point| {
        let mapped_point = if point.y < fold_line {
            point.clone()
        } else {
            Point {
                x: point.x,
                y: fold_line - point.y + fold_line
        }};

        if mapped_point.y >= 0 {
            Some(mapped_point)
        } else {
            None
        }
    }).collect()
}

fn fold_along_x(points: &HashSet<Point>, fold_line: i32) -> HashSet<Point> {
    points.iter().filter_map(|point| {
        let mapped_point = if point.x < fold_line {
            point.clone()
        } else {
            Point {
                x:  2*fold_line - point.x, //fold_line - point.x + fold_line
                y: point.y
        }};

        if mapped_point.x >= 0 {
            Some(mapped_point)
        } else {
            None
        }
    }).collect()
}

fn main() {
    let input = File::open("D:\\source\\Rust\\AoC 2021\\day_13\\src\\big_input.txt").unwrap();
    let mut reader = BufReader::new(input);
    let mut buffer = String::new();
    let mut all_points : HashSet<_> = HashSet::new();

    while let Ok(bytes_read) = reader.read_line(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        if let Some((left_coordinate, right_coordinate)) = buffer.trim_end().split_once(',') {
            let point = Point {
                x: left_coordinate.parse().unwrap(),
                y: right_coordinate.parse().unwrap()
            };
            all_points.insert(point);
        } else {
            break;
        }

        buffer.clear();
    }

    buffer.clear();
    let mut dots_left: HashSet<Point> = all_points;

    while let Ok(bytes_read) = reader.read_line(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        let (axis, fold_coordinate) = scanf!(buffer.trim_end(), "fold along {}={}", str, i32).unwrap();

        dots_left = match axis {
            "x" => {
                fold_along_x(&dots_left, fold_coordinate)
            },
            "y" => {
                fold_along_y(&dots_left, fold_coordinate)
            },
            _ => {
                panic!("Wrong input! Axis: {}, coordinate: {}", axis, fold_coordinate);
            }
        };

        buffer.clear();
    }

    let mut array = [[0; 40]; 7];

    for point in &dots_left {
        array[point.y as usize][point.x as usize] = 1;
    }

    for row in &array {
        for digit in row {
            if *digit == 1 {
                print!("#");
            } else {
                print!(".");
            }
        }

        print!("\n");
    }
}
