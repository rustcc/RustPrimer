# print! 宏

我们在快速入门中就提到过标准输出的行缓冲。它一个表现就是 `print!` 宏。如果你在 `print!` 宏后面接上一个输入就会发现这种按行缓冲的机制。

```rust
fn main() {
	print!("hello!\ninput:");
	let mut input = String::new();
		std::io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line");
	println!("line:{}",input);
}
```

您可以编译并运行这段程序试一试，您会发现我们并没有得到预期的（下划线代表光标的位置）：

```
hello!
input:_
```

而是得到了：

```
hello!
_
```

这就是由于标准输出中的这种行缓冲机制，在遇到换行符之前，输出的内容并不会隐式的刷新，这就导致 `print!` 宏和 `println!` 宏实际上并不完全相同。在标准库中 `print!` 宏是这样的：

```rust
macro_rules! print {
    ($($arg:tt)*) => { ... };
}
```

由此，我们可以对它进行改进，使它和 `println!` 宏被自动刷新，不过这种刷新是一种显式的刷新。

```rust
use std::io::{self, Write};

macro_rules! printf {
	($($arg:tt)*) =>{
		print!($($arg)*);
		io::stdout().flush().unwrap();
	}
}
```

此外，当您需要刷新还没有遇到换行符的一行内容的时候您都可以使用 `io::stdout().flush().unwrap();` 进行刷新，不过需要注意的是要先 `use std::io::{self, Write};` 如果您不这样做，将会得到一个错误。
