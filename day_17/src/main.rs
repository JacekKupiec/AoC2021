/* target area: x=185..221, y=-122..-74
    The probe's x position increases by its x velocity.
    The probe's y position increases by its y velocity.
    Due to drag, the probe's x velocity changes by 1 toward the value 0; that is, it decreases by 1 if it is greater than 0, increases by 1 if it is less than 0, or does not change if it is already 0.
    Due to gravity, the probe's y velocity decreases by 1.
*/

use std::cmp::max;

fn solve_quadratic_equation(a: f64, b: f64, c: f64) -> (f64, f64) {
    let delta = b*b - 4.0*a*c; // assume there is a solution

    ((-b - delta.sqrt()) / 2.0 / a, (-b + delta.sqrt()) / 2.0 / a)
}

fn simulate_probe(mut vx: i32, mut vy: i32, x_min: i32, x_max: i32, y_min: i32, y_max: i32, f: i32, g: i32) -> bool {
    let mut x_pos = 0;
    let mut y_pos = 0;

    loop {
        x_pos += vx;
        y_pos += vy;

        if x_pos >= x_min && x_pos <= x_max && y_pos >= y_min && y_pos <= y_max {
            break true;
        }

        if y_pos < y_min || x_pos > x_max {
            break false;
        }

        vx = max(vx - f, 0);
        vy -= g;
    }
}

fn main() {
    let x_min = 185i32;
    let x_max = 221i32;
    let y_max = -74i32;
    let y_min = -122i32;

    let (vx_start1, vx_limit2) = solve_quadratic_equation(1.0, 1.0, (2*x_min) as f64);
    let vx_start = vx_start1.max(vx_limit2).ceil() as i32;
    let mut counter = 0;

    for vx in vx_start..=x_max {
        for vy in (-y_min.abs())..=y_min.abs() {
            counter += if simulate_probe(vx, vy, x_min, x_max, y_min, y_max, 1, 1) { 1 } else { 0 };

        }
    }

    println!("Number of possible vectors {}", counter);
}
