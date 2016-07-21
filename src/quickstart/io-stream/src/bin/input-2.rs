use std::io;
fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("WTF!");

    println!("You typed: {}", input.trim());
}
