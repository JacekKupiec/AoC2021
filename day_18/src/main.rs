use std::io::stdin;

fn main() {
    let mut buffer = String::new();

    while let Ok(bytes_read) = stdin().read_line(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        buffer.clear();
    }

    println!("Hello, world!");
}
