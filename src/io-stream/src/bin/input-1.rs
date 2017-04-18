use std::io;

fn read_input() -> io::Result<()> {
    let mut input = String::new();

    try!(io::stdin().read_line(&mut input));

    println!("You typed: {}", input.trim());

    Ok(())
}

fn main() {
    read_input().unwrap();
}
