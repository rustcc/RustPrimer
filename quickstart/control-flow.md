# 控制流(control flow)

## If

If是分支 (branch) 的一种特殊形式，也可以使用`else`和`else if`。
与C语言不同的是，逻辑条件不需要用小括号括起来，但是条件后面必须跟一个代码块。
Rust中的`if`是一个表达式 (expression)，可以赋给一个变量：

```rust
let x = 5;

let y = if x == 5 { 10 } else { 15 };
```

Rust是基于表达式的编程语言，有且仅有两种语句 (statement)：

1. **声明语句** (declaration statement)，比如进行变量绑定的`let`语句。
2. **表达式语句** (expression statement)，它通过在末尾加上分号`;`来将表达式变成语句，
丢弃该表达式的值，一律返回unit`()`。

表达式如果返回，总是返回一个值，但是语句不返回值或者返回`()`，所以以下代码会报错：

```rust
let y = (let x = 5);

let z: i32 = if x == 5 { 10; } else { 15; };
```

值得注意的是，在Rust中赋值 (如`x = 5`) 也是一个表达式，返回unit的值`()`。

## For

Rust中的`for`循环与C语言的风格非常不同，抽象结构如下：

```rust
for var in expression {
    code
}
```

其中`expression`是一个迭代器 (iterator)，具体的例子为`0..10` (不包含最后一个值)，
或者`[0, 1, 2].iter()`。

## While

Rust中的`while`循环与C语言中的类似。对于无限循环，Rust有一个专用的关键字`loop`。
如果需要提前退出循环，可以使用关键字`break`或者`continue`，
还允许在循环的开头设定标签 (同样适用于`for`循环)：

```rust
'outer: loop {
   println!("Entered the outer loop");

   'inner: loop {
       println!("Entered the inner loop");
       break 'outer;
   }

   println!("This point will never be reached");
}

println!("Exited the outer loop");
```

## Match

Rust中的`match`表达式非常强大，首先看一个例子：

```rust
let day = 5;

match day {
  0 | 6 => println!("weekend"),
  1 ... 5 => println!("weekday"),
  _ => println!("invalid"),
}
```

其中`|`用于匹配多个值，`...`匹配一个范围 (包含最后一个值)，并且`_`在这里是必须的，
因为`match`强制进行穷尽性检查 (exhaustiveness checking)，必须覆盖所有的可能值。
如果需要得到`|`或者`...`匹配到的值，可以使用`@`绑定变量：

```rust
let x = 1;

match x {
    e @ 1 ... 5 => println!("got a range element {}", e),
    _ => println!("anything"),
}
```

使用`ref`关键字来得到一个引用：

```rust
let x = 5;
let mut y = 5;

match x {
    // the `r` inside the match has the type `&i32`
    ref r => println!("Got a reference to {}", r),
}

match y {
    // the `mr` inside the match has the type `&i32` and is mutable
    ref mut mr => println!("Got a mutable reference to {}", mr),
}
```

再看一个使用`match`表达式来解构元组的例子：

```rust
let pair = (0, -2);

match pair {
    (0, y) => println!("x is `0` and `y` is `{:?}`", y),
    (x, 0) => println!("`x` is `{:?}` and y is `0`", x),
    _ => println!("It doesn't matter what they are"),
}
```

`match`的这种解构同样适用于结构体或者枚举。如果有必要，还可以使用`..`来忽略域或者数据：

```rust
struct Point {
    x: i32,
    y: i32,
}

let origin = Point { x: 0, y: 0 };

match origin {
    Point { x, .. } => println!("x is {}", x),
}

enum OptionalInt {
    Value(i32),
    Missing,
}

let x = OptionalInt::Value(5);

match x {
    // 这里是 match 的 if guard 表达式，我们将在以后的章节进行详细介绍
    OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than five!"),
    OptionalInt::Value(..) => println!("Got an int!"),
    OptionalInt::Missing => println!("No such luck."),
}
```

此外，Rust还引入了`if let`和`while let`进行模式匹配：

```rust
let number = Some(7);
let mut optional = Some(0);

// If `let` destructures `number` into `Some(i)`, evaluate the block.
if let Some(i) = number {
    println!("Matched {:?}!", i);
} else {
    println!("Didn't match a number!");
}

// While `let` destructures `optional` into `Some(i)`, evaluate the block.
while let Some(i) = optional {
    if i > 9 {
        println!("Greater than 9, quit!");
        optional = None;
    } else {
        println!("`i` is `{:?}`. Try again.", i);
        optional = Some(i + 1);
    }
}
```

