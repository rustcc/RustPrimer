# 文件输入与输出

文件 `std::fs::File` 本身实现了 `Read` 和 `Write` trait，所以文件的输入输出非常简单，只要得到一个 `File` 类型实例就可以调用读写接口进行文件输入与输出操作了。而要得到 `File` 就得让操作系统打开(open)或新建(create)一个文件。还是拿例子来说明

```rust
use std::io;
use std::io::prelude::*;
use std::fs::File;

// create file and write something
fn create_file(filename: &str, buf: &[u8]) -> io::Result<()> {
	let mut f = try!(File::create(filename));
	try!(f.write(&buf));
	Ok(())
}

// read from file to String
fn read_file(filename: &str, mut buf: &mut String) -> io::Result<()> {
	let mut f = try!(File::open(filename));
	try!(f.read_to_string(&mut buf));
	Ok(())
}

fn main() {
	let f = "foo.txt";
	let mut buf = String::new();
	match create_file(f, b"Hello, World!") {
		Ok(()) => {
		    match read_file(f, &mut buf) {
		        Ok(()) => {println!("{}", buf);},
		        Err(e) => {println!("{}", e);},
            };
		},
		Err(e) => {println!("{}", e);},
	}
}
```

文件操作上面 Rust 与其它语言处理方式有些不一样，其它语言一般把读写选项作为函数参数传给 open 函数，而 Rust 则是在 option 上面调用 open 函数。 [`std::fs::OpenOptions`](http://doc.rust-lang.org/stable/std/fs/struct.OpenOptions.html) 是一个 builder，通过 new 函数创建后，可以链式调用设置打开文件的选项，是 read, write, append, truncate 还是 create 等，OpenOptions 构建完成后就可以再接着调用 open 方法了，看下下面的例子就明白了

```rust
use std::fs::OpenOptions;

let file = OpenOptions::new().write(true).truncate(true).open("foo.txt");
```

Rust 这种用 builder pattern 来设置打开文件选项，相比于将选项以字符作为参数传给 open 函数的一个优点是可以让编译器保证检查选项合法性，不用等到运行时才发现手抖把 read-mode 的 `r` 写成了 `t`。
