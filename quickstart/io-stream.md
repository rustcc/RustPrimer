# 输入输出流
**输入输出**是人机交互的一种方式。最常见的输入输出是标准输入输出和文件输入输出（当然还有数据库输入输出，本节不讨论这部分）。

## 标准输入
标准输入也叫作控制台输入，是常见输入的一种。

**例子1：**

```rust
use std::io;

fn read_input() -> io::Result<()> {
    let mut input = String::new();

    try!(io::stdin().read_line(&mut input));

    println!("You typed: {}", input.trim());

    Ok(())
}

fn main() {
    read_input();
}
```

**例子2：**

```rust
use std::io;
fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("WTF!");

    println!("You typed: {}", input.trim());
}
```

这里体现了常见的标准输入的处理方式。两个例子都是声明了一个可变的字符串来保存输入的数据。
他们的不同之处在在于处理潜在输入异常的方式。

1. 例子 1 使用了 `try!` 宏。这个宏会返回 `Result<(), io::Error>` 类型，`io::Result<()>` 就是这个类型的别名。所以例子 1 需要单独使用一个 `read_input` 函数来接收这个类型，而不是在 `main` 函数里面，因为 `main` 函数并没有接收 `io::Result<()>` 作为返回类型。

2. 例子 2 使用了 `Result<(), io::Error>` 类型的 `expect` 方法来接收 `io::stdin().read_line` 的返回类型。并处理可能潜在的 io 异常。

## 标准输出
标准输出也叫控制台输出，Rust 里面常见的标准输出宏有 `print!` 和 `println!`。它们的区别是后者比前者在末尾多输出一个换行符。

**例子1：**

```rust
fn main() {
    print!("this ");
    print!("will ");
    print!("be ");
    print!("on ");
    print!("the ");
    print!("same ");
    print!("line ");

    print!("this string has a newline, why not choose println! instead?\n");
}
```

**例子2：**

```rust
fn main() {
    println!("hello there!");
    println!("format {} arguments", "some");
}
```

这里两个例子都比较简单。读者可以运行一下查看输出结果对比一下他们的区别。
值得注意的是例子 2 中，`{ }` 会被 `"some"` 所替换。这是 rust 里面的一种格式化输出。

标准化的输出是行缓冲(line-buffered)的,这就导致标准化的输出在遇到一个新行之前并不会被隐式刷新。
换句话说  `print!` 和 `println!` 二者的效果并不总是相同的。
如果说得更简单明了一点就是，您不能把 `print!` 当做是C语言中的 `printf` 譬如：

```rust
use std::io;
fn main() {
    print!("请输入一个字符串：");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("读取失败");
    print!("您输入的字符串是：{}\n", input);
}
```

在这段代码运行时则不会先出现预期的提示字符串，因为行没有被刷新。
如果想要达到预期的效果就要显示的刷新：

```rust
use std::io::{self, Write};
fn main() {
    print!("请输入一个字符串：");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("读取失败");
    print!("您输入的字符串是：{}\n", input);
}
```

## 文件输入

文件输入和标准输入都差不多，除了输入流指向了文件而不是控制台。下面例子采用了模式匹配来处理潜在的输入错误

**例子：**

```rust
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
```

## 文件输出
文件输出和标准库输出也差不多，只不过是把输出流重定向到文件中。下面详细看例子。

**例子：**

```rust
// 输出文本
static LOREM_IPSUM: &'static str =
"Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn main() {
    let path = Path::new("out/lorem_ipsum.txt");
    let display = path.display();

    // 用只写模式打开一个文件，并返回 `io::Result<File>` 类型
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           Error::description(&why)),
        Ok(file) => file,
    };

    // 写入 `LOREM_IPSUM` 字符串到文件中, 并返回 `io::Result<()>` 类型
    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                                               Error::description(&why))
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
```
