use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // 创建一个文件路径
    let path = Path::new("hello.txt");
    let display = path.display();

    // 打开文件只读模式, 返回一个 `io::Result<File>` 类型
    let mut file = match File::open(&path) {
        // 处理打开文件可能潜在的错误
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
    };

    // 文件输入数据到字符串，并返回 `io::Result<usize>` 类型
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
}
