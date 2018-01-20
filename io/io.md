# 标准输入与输出

回顾一下我们写的第一个 Rust 程序就是带副作用的，其副作用就是向标准输出(stdout)，通常是终端或屏幕，输出了 Hello, World! 让屏幕上这几个字符的地方点亮起来。`println!` 宏是最常见的输出，用宏来做输出的还有 `print!`，两者都是向标准输出(stdout)输出，两者的区别也一眼就能看出。至于格式化输出，[基础运算符和字符串格式化小节](../type/operator-and-formatting.md)有详细说明，这里就不再啰嗦了。

更通用的标准输入与输出定义在 `std::io` 模块里，调用 `std::io::stdin()` 和 `std::io::stdout()` 两个函数分别会得到输入句柄和输出句柄(哎，句柄这个词是计算机史上最莫名其妙的翻译了)，这两个句柄默认会通过互斥锁同步，也就是说不让多个进程同时读或写标准输入输出，不然的话如果一个进程要往标准输出画马，一个进程要画驴，两个进程同时写标准输出的话，最后可能就给画出一头骡子了，如果更多进程画不同的动物最后可能就成四不像了。除了隐式地用互斥锁，我们还可以显式地在句柄上调用 `.lock()`。输入输出句柄实现了前面讲的读写 Trait，所以是 reader/writer，就可以调接口来读写标准输入与输出了。举几个栗子：

```rust
use std::io;

fn read_from_stdin(buf: &mut String) -> io::Result<()> {
	try!(io::stdin().read_line(buf));
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

可以看到上面的例子都是返回了 `io::Result<()>` 类型，这不是偶然，而是 IO 操作通用的写法，因为 IO 操作是程序与外界打交道，所以都是有可能失败的，用 `io::Result<T>` 把结果包起来，`io::Result<T>` 只是标准 `Result<T,E>` 中 `E` 固定为 `io::Error` 后类型的别名，而作为有副作用的操作我们一般是不用关心其返回值的，因为执行这类函数其真正的意义都体现在副作用上面了，所以返回值只是用来表示是否成功执行，而本身 `Result` 类型本身已经可以表示执行状态了，里面的 `T` 是什么则无关紧要，既然 `T` 没什么意义，那我们就选没什么意义的 `unit` 类型好了，所以 IO 操作基本上都是使用 `io::Result<()>`。

另外有一个地方需要注意的是由于 IO 操作可能会失败所以一般都是和 `try!` 宏一起使用的，但是 `try!` 在遇到错误时会把错误 `return` 出去的，所以需要保证包含 `try!` 语句的函数其返回类型是 `io::Result<T>`，很多新手文档没仔细看就直接查 std api 文档，然后照着 api 文档里面的例子把带 IO 操作的 `try!` 宏写到了 `main` 函数里。结果一编译，擦，照着文档写都编译不过，什么烂文档。其实点一下 api 文档上面的运行按钮就会发现文档里面的例子都是把 `try!` 放在另一个函数里面的，因为 `main` 函数是没有返回值的，而 `try!` 会返回 `io::Result<T>`，所以直接把 `try!` 放 `main` 函数里面肯定要跪。比如下面的从标准输入读取一行输入，由于把 `try!` 放在了 main 函数里，所以是编译不过的。

```rust
use std::io;

fn main() {
	let mut input = String::new();
	try!(io::stdin().read_line(&mut input));
	println!("You typed: {}", input.trim());
}
```

这里有一件事需要主要的是 Rust 里面没有办法从键盘获取一个数字类型的值。实际上像 C 这样的语言也不是直接获取了数字类型，它只不过是做了一种转换。那么我们如果想要从键盘获取一个数字类型应该怎么做呢？

```rust
fn main() {
	let mut input = String::new();
		std::io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line");
    // 这里等效的写法是：
    // let num: i32 = input.trim().parse().unwrap(); 
	let num = input.trim().parse::<i32>().unwrap();
	println!("您输入的数字是：{}", num);
}
```

如果有很多地方都需要输入数字可以自行编写一个 `numin` 宏:

```rust
macro_rules! numin {
	  () =>{
	      {
            let mut input = String::new();
	          std::io::stdin()
	              .read_line(&mut input)
                .expect("Failed to read line");
	          input.trim().parse().unwrap()
        }
    };
}
```

于是上面的程序可以被改写成：

```

fn main() {
    let num: i32 = numin!();
	println!("您输入的数字是：{}", num);
}
```

不过如果用户输入的不是数字，那么就会导致错误。这一点和 C 里面是非常相似的。当然您可以把程序写得再复杂一点儿来保证用户输入的一定是数字。不过这些就不是我们这一节要讨论的内容了。

还有一点一些从其它语言转过来的程序员可能会疑惑的是，如何从命令行接受输入参数，因为 C 里面的 main 函数可以带参数所以可以直接从 main 函数的参数里获取输入参数。但其实这类输入与我们这里讲的有很大的差别的，它在 Rust 里面被归为环境变量，可以通过 `std::env::args()` 获取，这个函数返回一个 `Args` 迭代器，其中第一个就是程序名，后面的都是输入给程序的命令行参数。

```rust
use std::env;

fn main() {
	let args = env::args();
	for arg in args {
		println!("{}", arg);
	}
}
```

将上面的程序存为 *args.rs* 然后编译执行，结果如下

```
$ rustc args.rs
$ ./args a b c
./args
a
b
c
```

