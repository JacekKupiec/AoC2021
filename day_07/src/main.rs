use std::io;
use std::io::Read;

fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer)
        .expect("Nie udało się wczytać danych");

    let crab_positions: Vec<i32> = buffer.trim_end()
        .split(',')
        .map(|position| position.parse().unwrap())
        .collect();

    let min_position = *crab_positions.iter().min().unwrap();
    let max_position = *crab_positions.iter().max().unwrap();

    let sum_of_distances : i32 = (min_position..=max_position).map(|position_aligned|
        crab_positions.iter().map(|crab_position|
            (1 + (crab_position - position_aligned).abs())*(crab_position - position_aligned).abs() / 2).sum()
    ).min().unwrap();

    println!("{}", sum_of_distances);
}
