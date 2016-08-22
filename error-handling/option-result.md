# 17.错误处理
错误处理是保证程序健壮性的前提，在编程语言中错误处理的方式大致分为两种：抛出异常（exceptions）和作为值返回。

**Rust** 将错误作为值返回并且提供了原生的优雅的错误处理方案。

熟练掌握错误处理是软件工程中非常重要的环节，让我一起来看看**Rust**展现给我们的错误处理艺术。

## 17.1 Option和Result
谨慎使用`panic`：

```rust
fn guess(n: i32) -> bool {
    if n < 1 || n > 10 {
        panic!("Invalid number: {}", n);
    }
    n == 5
}

fn main() {
    guess(11);
}
```

`panic`会导致当前线程结束，甚至是整个程序的结束，这往往是不被期望看到的结果。（编写示例或者简短代码的时候`panic`不失为一个好的建议）


### Option

```rust
enum Option<T> {
    None,
    Some(T),
}
```

**Option** 是Rust的系统类型，用来表示值不存在的可能，这在编程中是一个好的实践，它强制**Rust**检测和处理值不存在的情况。例如：

```rust
fn find(haystack: &str, needle: char) -> Option<usize> {
    for (offset, c) in haystack.char_indices() {
        if c == needle {
            return Some(offset);
        }
    }
    None
}
```

`find`在字符串`haystack`中查找`needle`字符，事实上结果会出现两种可能，有（`Some(usize)`)或无（`None`）。

```rust
fn main() {
    let file_name = "foobar.rs";
    match find(file_name, '.') {
        None => println!("No file extension found."),
        Some(i) => println!("File extension: {}", &file_name[i+1..]),
    }
}
```

**Rust** 使用模式匹配来处理返回值，调用者必须处理结果为`None`的情况。这往往是一个好的编程习惯，可以减少潜在的bug。**Option** 包含一些方法来简化模式匹配，毕竟过多的`match`会使代码变得臃肿，这也是滋生bug的原因之一。

#### unwrap

```rust
impl<T> Option<T> {
    fn unwrap(self) -> T {
        match self {
            Option::Some(val) => val,
            Option::None =>
              panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}
```

`unwrap`当遇到`None`值时会panic，如前面所说这不是一个好的工程实践。不过有些时候却非常有用：

* **在例子和简单快速的编码中** 有的时候你只是需要一个小例子或者一个简单的小程序，输入输出已经确定，你根本没必要花太多时间考虑错误处理，使用`unwrap`变得非常合适。
* **当程序遇到了致命的bug，panic是最优选择**


#### map

假如我们要在一个字符串中找到文件的扩展名，比如`foo.rs`中的`rs`， 我们可以这样：

```rust
fn extension_explicit(file_name: &str) -> Option<&str> {
    match find(file_name, '.') {
        None => None,
        Some(i) => Some(&file_name[i+1..]),
    }
}

fn main() {
    match extension_explicit("foo.rs") {
        None => println!("no extension"),
        Some(ext) =>  assert_eq!(ext, "rs"),
    }
}
```

我们可以使用`map`简化：

```rust
// map是标准库中的方法
fn map<F, T, A>(option: Option<T>, f: F) -> Option<A> where F: FnOnce(T) -> A {
    match option {
        None => None,
        Some(value) => Some(f(value)),
    }
}
// 使用map去掉match
fn extension(file_name: &str) -> Option<&str> {
    find(file_name, '.').map(|i| &file_name[i+1..])
}
```

`map`如果有值`Some(T)`会执行`f`，反之直接返回`None`。

#### unwrap_or

```rust
fn unwrap_or<T>(option: Option<T>, default: T) -> T {
    match option {
        None => default,
        Some(value) => value,
    }
}
```
`unwrap_or`提供了一个默认值`default`，当值为`None`时返回`default`：
```rust
fn main() {
    assert_eq!(extension("foo.rs").unwrap_or("rs"), "rs");
    assert_eq!(extension("foo").unwrap_or("rs"), "rs");
}
```

#### and_then

```rust
fn and_then<F, T, A>(option: Option<T>, f: F) -> Option<A>
        where F: FnOnce(T) -> Option<A> {
    match option {
        None => None,
        Some(value) => f(value),
    }
}
```

看起来`and_then`和`map`差不多，不过`map`只是把值为`Some(t)`重新映射了一遍，`and_then`则会返回另一个`Option`。如果我们在一个文件路径中找到它的扩展名，这时候就会变得尤为重要：

```rust
use std::path::Path;
fn file_name(file_path: &str) -> Option<&str> {
    let path = Path::new(file_path);
    path.file_name().to_str()
}
fn file_path_ext(file_path: &str) -> Option<&str> {
    file_name(file_path).and_then(extension)
}
```

### Result

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`Result`是`Option`的更通用的版本，比起`Option`结果为`None`它解释了结果错误的原因，所以：

```rust
type Option<T> = Result<T, ()>;
```

这样的别名是一样的（`()`标示空元组，它既是`()`类型也可以是`()`值）
#### unwrap

```rust
impl<T, E: ::std::fmt::Debug> Result<T, E> {
    fn unwrap(self) -> T {
        match self {
            Result::Ok(val) => val,
            Result::Err(err) =>
              panic!("called `Result::unwrap()` on an `Err` value: {:?}", err),
        }
    }
}
```

没错和`Option`一样，事实上它们拥有很多类似的方法，不同的是，`Result`包括了错误的详细描述，这对于调试人员来说，这是友好的。

#### Result我们从例子开始

```rust
fn double_number(number_str: &str) -> i32 {
    2 * number_str.parse::<i32>().unwrap()
}

fn main() {
    let n: i32 = double_number("10");
    assert_eq!(n, 20);
}
```

`double_number`从一个字符串中解析出一个`i32`的数字并`*2`，`main`中调用看起来没什么问题，但是如果把`"10"`换成其他解析不了的字符串程序便会panic

```rust
impl str {
    fn parse<F: FromStr>(&self) -> Result<F, F::Err>;
}

```

`parse`返回一个`Result`，但让我们也可以返回一个`Option`，毕竟一个字符串要么能解析成一个数字要么不能，但是`Result`给我们提供了更多的信息（要么是一个空字符串，一个无效的数位，太大或太小），这对于使用者是友好的。当你面对一个Option和Result之间的选择时。如果你可以提供详细的错误信息，那么大概你也应该提供。

这里需要理解一下`FromStr`这个**trait**:

```rust
pub trait FromStr {
    type Err;
    fn from_str(s: &str) -> Result<Self, Self::Err>;
}

impl FromStr for i32 {
    type Err = ParseIntError;
    fn from_str(src: &str) -> Result<i32, ParseIntError> {

    }
}
```

`number_str.parse::<i32>()`事实上调用的是`i32`的`FromStr`实现。

我们需要改写这个例子：

```rust
use std::num::ParseIntError;

fn double_number(number_str: &str) -> Result<i32, ParseIntError> {
    number_str.parse::<i32>().map(|n| 2 * n)
}

fn main() {
    match double_number("10") {
        Ok(n) => assert_eq!(n, 20),
        Err(err) => println!("Error: {:?}", err),
    }
}
```

不仅仅是`map`，`Result`同样包含了`unwrap_or`和`and_then`。也有一些特有的针对错误类型的方法`map_err`和`or_else`。

#### Result别名
在**Rust**的标准库中会经常出现Result的别名，用来默认确认其中`Ok(T)`或者`Err(E)`的类型，这能减少重复编码。比如`io::Result`

```rust
use std::num::ParseIntError;
use std::result;

type Result<T> = result::Result<T, ParseIntError>;

fn double_number(number_str: &str) -> Result<i32> {
    unimplemented!();
}
```

### 组合Option和Result
`Option`的方法`ok_or`：

```rust
fn ok_or<T, E>(option: Option<T>, err: E) -> Result<T, E> {
    match option {
        Some(val) => Ok(val),
        None => Err(err),
    }
}
```

可以在值为`None`的时候返回一个`Result::Err(E)`，值为`Some(T)`的时候返回`Ok(T)`，利用它我们可以组合`Option`和`Result`：

```rust
use std::env;

fn double_arg(mut argv: env::Args) -> Result<i32, String> {
    argv.nth(1)
        .ok_or("Please give at least one argument".to_owned())
        .and_then(|arg| arg.parse::<i32>().map_err(|err| err.to_string()))
        .map(|n| 2 * n)
}

fn main() {
    match double_arg(env::args()) {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }
}
```

`double_arg`将传入的命令行参数转化为数字并翻倍，`ok_or`将`Option`类型转换成`Result`，`map_err`当值为`Err(E)`时调用作为参数的函数处理错误

#### 复杂的例子

```rust
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    File::open(file_path)
         .map_err(|err| err.to_string())
         .and_then(|mut file| {
              let mut contents = String::new();
              file.read_to_string(&mut contents)
                  .map_err(|err| err.to_string())
                  .map(|_| contents)
         })
         .and_then(|contents| {
              contents.trim().parse::<i32>()
                      .map_err(|err| err.to_string())
         })
         .map(|n| 2 * n)
}

fn main() {
    match file_double("foobar") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }
}
```

`file_double`从文件中读取内容并将其转化成`i32`类型再翻倍。
这个例子看起来已经很复杂了，它使用了多个组合方法，我们可以使用传统的`match`和`if let`来改写它：

```rust
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => return Err(err.to_string()),
    };
    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        return Err(err.to_string());
    }
    let n: i32 = match contents.trim().parse() {
        Ok(n) => n,
        Err(err) => return Err(err.to_string()),
    };
    Ok(2 * n)
}

fn main() {
    match file_double("foobar") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }
}
```

这两种方法个人认为都是可以的，依具体情况来取舍。

### try!

```rust
macro_rules! try {
    ($e:expr) => (match $e {
        Ok(val) => val,
        Err(err) => return Err(::std::convert::From::from(err)),
    });
}

```

`try!`事实上就是`match Result`的封装，当遇到`Err(E)`时会提早返回，
`::std::convert::From::from(err)`可以将不同的错误类型返回成最终需要的错误类型，因为所有的错误都能通过`From`转化成`Box<Error>`，所以下面的代码是正确的：

```rust
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, Box<Error>> {
    let mut file = try!(File::open(file_path));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents));
    let n = try!(contents.trim().parse::<i32>());
    Ok(2 * n)
}

```

#### 组合自定义错误类型

```rust
use std::fs::File;
use std::io::{self, Read};
use std::num;
use std::io;
use std::path::Path;

// We derive `Debug` because all types should probably derive `Debug`.
// This gives us a reasonable human readable description of `CliError` values.
#[derive(Debug)]
enum CliError {
    Io(io::Error),
    Parse(num::ParseIntError),
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(err: num::ParseIntError) -> CliError {
        CliError::Parse(err)
    }
}

fn file_double_verbose<P: AsRef<Path>>(file_path: P) -> Result<i32, CliError> {
    let mut file = try!(File::open(file_path).map_err(CliError::Io));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents).map_err(CliError::Io));
    let n: i32 = try!(contents.trim().parse().map_err(CliError::Parse));
    Ok(2 * n)
}
```

`CliError`分别为`io::Error`和`num::ParseIntError`实现了`From`这个trait，所有调用`try!`的时候这两种错误类型都能转化成`CliError`。

### 总结

熟练使用`Option`和`Result`是编写 **Rust** 代码的关键，**Rust** 优雅的错误处理离不开值返回的错误形式，编写代码时提供给使用者详细的错误信息是值得推崇的。
