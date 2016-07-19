# 注释与文档

## 注释
在 Rust 里面注释分成两种，行注释和块注释。它的形式和 C 语言是一样的。
两种注释分别是：
> 1. 行注释使用 `//` 放在注释前面。比如:

```
// I love Rust, but I hate Rustc.
```

> 2. 块注释分别使用`/*`和`*/`包裹需要注释的内容。比如：

```
/* W-Cat 是个大胖猫，N-Cat 是个高度近视猫。*/
```

## 文档
Rust 自带有文档功能的注释，分别是`///`和`//!`。支持 Markdown 格式
1. `///`用来描述的它后面接着的项。
2. `//!`用来描述包含它的项，一般用在模块文件的头部。
比如在 main.rs 文件中输入以下内容：

```
        //! # The first line
        //! The second line
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
```    

### 生成 html 文档
* `rustdoc main.rs`

或者

* `cargo doc`
