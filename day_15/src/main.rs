fn main() {
    let x = 1u64;

    let y = x.saturating_sub(1).saturating_sub(1);

    println!("Hello, world! {}", y);
}
