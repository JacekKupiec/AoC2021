use std::cmp::max;
use std::io;
use std::io::Read;

const DAYS : usize = 256;

fn lantern_fish_recurse(days: i32) -> usize {
    if days <= 0 {
        return 1;
    }

    return lantern_fish_recurse(days - 7) + lantern_fish_recurse(days - 9);
}

fn main() {
    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Nie udało sie odczytać wejścia");

    let initial_fish_states : Vec<_> = buffer.trim_end()
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect();

    let mut lantern_fish_population: [u64; DAYS + 2] = [1; DAYS + 2];
    let last_day_of_simulation = DAYS +  1;

    for idx in 1..=last_day_of_simulation {
        let idx_7 = max(0, idx as i32 - 7) as usize;
        let idx_9 = max(0, idx as i32 - 9) as usize;

        lantern_fish_population[idx] = lantern_fish_population[idx_7] + lantern_fish_population[idx_9];
    }

    let lantern_fish_population : u64 = initial_fish_states.iter().map(|age| lantern_fish_population[DAYS - age]).sum();

    println!("{}", lantern_fish_population);
}
