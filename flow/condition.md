# 条件分支

- if
- if let
- match

## if 表达式

Rust 中的 if 表达式基本就是如下几种形式：

```rust
// 形式 1
if expr1 {

}

// 形式 2
if expr1 {

}
else {

}

// 形式 3
if expr1 {

}
else if expr2 {
    // else if 可多重
}
else {

}

```

相对于 C 系语言，Rust 的 if 表达式的显著特点是：

1. 判断条件不用小括号括起来；
2. 它是表达式，而不是语句。

鉴于上述第二点，因为是表达式，所以我们可以写出如下代码：

```rust
let x = 5;

let y = if x == 5 {
    10
} else {
    15
}; // y: i32
```

或者压缩成一行：

```rust
let x = 5;

let y = if x == 5 { 10 } else { 15 }; // y: i32
```

## if let

我们在代码中常常会看到 `if let` 成对出现，这实际上是一个 match 的简化用法。直接举例来说明：

```rust
let x = Some(5);

if let Some(y) = x {
    println!("{}", y);      // 这里输出为：5
}

let z = if let Some(y) = x {
    y
}
else {
    0
};
// z 值为 5

```

上面代码等价于

```rust
let x = Some(5);
match x {
    Some(y) => println!("{}", y),
    None => ()
}

let z = match x {
    Some(y) => y,
    None => 0
};
```

设计这个特性的目的是，在条件判断的时候，直接做一次模式匹配，方便代码书写，使代码更紧凑。

## match

Rust 中没有类似于 C 的 `switch` 关键字，但它有用于模式匹配的 `match`，能实现同样的功能，并且强大太多。

match 的使用非常简单，举例如下：

```rust
let x = 5;

match x {
    1 => {
        println!("one")
    },
    2 => println!("two"),
    3 => println!("three"),
    4 => println!("four"),
    5 => println!("five"),
    _ => println!("something else"),
}
```
注意，match 也是一个表达式。match 后面会专门论述，请参见 **模式匹配** 这一章。
