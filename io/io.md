# 输入与输出

输入与输出可以说是一个实用程序的最基本要求，没有输入输出的程序是没有什么卵用的。虽然输入输出被函数式编程语言鄙称为副作用，但正是这个副作用才赋予了程序实用性，君不见某著名函数式语言之父称他主导设计的函数式语言"[is useless](https://www.youtube.com/watch?v=iSmkqocn0oQ)"。这章我们就来谈谈输入输出副作用。


## 读写 Trait

输入最基本的功能是读(Read)，输出最基本的功能是写(Write)。标准库里面把怎么读和怎么写抽象出来归到了 `Read` 和 `Write` 两个接口里面，实现了 `Read` 接口的叫 reader，而实现了 `Write` 的叫 writer。Rust里面的 Trait 比其它语言里面的接口更好的一个地方是 Trait 可以带默认实现，比如用户定义的 reader 只需要实现 `read` 一个方法就可以调用 `Read` trait 里面的任意其它方法，而 writer 也只需要实现 `write` 和 `flush` 两个方法。

Read 和 Write 这两个 Trait 都有定义了好多方法，具体可以参考标准库 API 文档中的[Read](http://doc.rust-lang.org/stable/std/io/trait.Read.html) 和 [Write](http://doc.rust-lang.org/stable/std/io/trait.Write.html)

Read 是基于字节读的，每读一个字节都要调用一次操作系统 API 与内核交互，效率很低，尤其是连续读时。这种情况我们把读请求攒起来，像过马路一样等凑足一波再让操作系统真正的读一次，这就是 `BufRead` Trait。一个普通的 reader 通过 `io::BufReader::new(reader)` 或者 `io::BufReader::with_capacity(bufSize, reader)` 就可以得到一个 BufReader 了，显然这两个创建 BufReader 的函数一个是使用默认大小的 buffer 一个可以指定 buffer 大小。BufReader 比较常用的两个方法是按行读： `read_line(&mut self, buf: &mut String) -> Result<usize>` 和 `lines(&mut self) -> Lines<Self>`，从函数签名上就可以大概猜出函数的用法所以就不啰嗦了，需要注意的是后者返回的是一个迭代器。详细说明直接看 API 文档中的[BufRead](http://doc.rust-lang.org/stable/std/io/trait.BufRead.html)

输入与输出接口有了，我们接下来看看实际应用中最常用的两类 reader 和 writer：标准输入/输出，文件输入/输出


## 标准输入与输出

回顾一下我们写的第一个 Rust 程序就是带副作用的，其副作用就是向标准输出(stdout)，通常是终端或屏幕，输出了 Hello, World! 让屏幕上这几个字符的地方点亮起来。`println!` 宏是最常见的输出，用宏来做输出的还有 `print!`，两者都是向标准输出(stdout)输出，两者的区别也一眼就能看出。至于格式化输出，[基础运算符和字符串格式化小节](../type/operator-and-formatting.md)有详细说明，这里就不再啰嗦了。

更通用的标准输入与输出定义在 `std::io` 模块里，调用 `std::io::stdin()` 和 `std::io::stdout()` 两个函数分别会得到输入句柄和输出句柄(哎，句柄这个词是计算机史上最莫名其妙的翻译了)，这两个句柄默认会通过互斥锁同步，也就是说不让多个进程同时读或写标准输入输出，不然的话如果一个进程要往标准输出画马，一个进程要画驴，两个进程同时写标准输出的话，最后可能就给画出一头骡子了，如果更多进程画不同的动物最后可能就成四不像了。除了隐式地用互斥锁，我们还可以显式地在句柄上调用 `.lock()`。输入输出句柄实现了前面讲的读写 Trait，所以是 reader/writer，就可以调接口来读写标准输入与输出了。举几个栗子：

```rust
use std::io;

fn read_from_stdin(buf: &mut String) -> io::Result<()> {
	try!(io::stdin().read_line(&mut buf));
	Ok(())
}
```

```rust
use std::io;

fn write_to_stdout(buf: &[u8]) -> io::Result<()> {
	try!(io::stdout().write(&buf));
	Ok(())
}
```

可以看到上面的例子都是返回了 `io::Result<()>` 类型，这不是偶然，而是 IO 操作通用的写法，因为 IO 操作是程序与外界打交道，所以都是有可能失败的，用 `io::Result<T>` 把结果包起来，`io::Result<T>` 是标准 `Result<T,E>` 中 `E` 特化为 `std::io::Error` 的版本，而作为有副作用的操作我们一般是不用关心其返回值的，因为执行这类函数其真正的意义都体现在副作用上面了，所以返回值只是用来表示是否成功执行，而本身 `Result` 类型本身已经可以表示执行状态了，里面的 `T` 是什么则无关紧要，既然 `T` 没什么意义，那我们就选没什么意义的 `unit` 类型好了，所以 IO 操作基本上都是使用 `io::Result<()>`。

另外有一个地方需要注意的是由于 IO 操作可能会失败所以一般都是和 `try!` 宏一起使用的，但是 `try!` 在遇到错误时会把错误 `return` 出去的，所以需要保证包含 `try!` 语句的函数其返回类型是 `io::Result<T>`，很多新手文档没仔细看就直接查 std api 文档，然后照着 api 文档里面的例子把带 IO 操作的 `try!` 宏写到了 `main` 函数里。结果一编译，擦，照着文档写都编译不过，什么烂文档。其实点一下 api 文档上面的运行按钮就会发现文档里面的例子都是把 `try!` 放在另一个函数里面的，因为 `main` 函数是没有返回值的，而 `try!` 会返回 `io::Result<T>`，所以直接把 `try!` 放 `main` 函数里面肯定要跪。

还有一点一些从其它语言转过来的程序猿可能会疑惑的是，如何从命令行接受输入参数，因为 C 里面的 main 函数可以带参数所以可以直接从 main 函数的参数里获取输入参数。但其实这类输入与我们这里讲的有很大的差别的，它在 Rust 里面被归为环境变量，可以通过 `std::env::args()` 获取，这个函数返回一个 `Args` 迭代器，其中第一个就是程序名，后面的都是输入给程序的命令行参数。


## 文件输入与输出

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
