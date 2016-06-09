use std::borrow::Borrow;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse<I>(lines: I)
    where I: IntoIterator, I::Item: Borrow<str>
{
    for line in lines {
        println!("{}", line.borrow());
    }
}

pub fn demo(path: &str) {
    if path.is_empty() {
        println!("==== Parsing long string ====");
        let content = "some\nlong\ntext";
        parse(content.lines());
    } else {
        println!("==== Parsing text file ====");
        let f = File::open(path).unwrap();
        let b = BufReader::new(f);
        parse(b.lines().map(|l| l.unwrap_or("".to_owned())));
    }
}
