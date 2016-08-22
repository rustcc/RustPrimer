# 注释

Rust 代码文件中，通常我们可以看到 3 种注释。

- 行注释
- 文档注释
- 模块注释

## 行注释

 `//` 后的，直到行尾，都属于注释，不会影响程序的行为。

```rust
// 创建一个绑定
let x = 5;

let y = 6; // 创建另一个绑定
```

## 文档注释

文档注释使用 ```///```，一般用于函数或结构体（字段）的说明，置于要说明的对象上方。文档注释内部可使用markdown格式的标记语法，可用于 rustdoc 工具的自动文档提取。

    /// Adds one to the number given.
    ///
    /// # Examples
    ///
    /// ```
    /// let five = 5;
    ///
    /// assert_eq!(6, add_one(5));
    /// # fn add_one(x: i32) -> i32 {
    /// #     x + 1
    /// # }
    /// ```
    fn add_one(x: i32) -> i32 {
        x + 1
    }


## 模块注释

模块注释使用 ```//!```，用于说明本模块的功能。一般置于模块文件的头部。

```rust
//! # The Rust Standard Library
//!
//! The Rust Standard Library provides the essential runtime
//! functionality for building portable Rust software.
```

PS: 相对于 `///`, `//!` 用来注释包含它的项（也就是说，crate，模块或者函数），而不是位于它之后的项。


## 其它：兼容C语言的注释

Rust 也支持兼容 C 的块注释写法：`/* */`。但是不推荐使用，请尽量不要使用这种注释风格（会被鄙视的）。

```rust
/*
    let x = 42;
    println!("{}", x);
*/
```
