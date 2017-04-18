extern crate phrases;

use phrases::chinese::greetings;
use phrases::chinese::farewells::goodbye;

fn main() {
    println!("Hello in Chinese: {}", greetings::hello());
    println!("Goodbye in Chinese: {}", goodbye());
}