# Rust旅程

## HelloWorld
按照编程语言的传统，学习第一门编程语言的第一个程序都是打印 Hello World！
下面根据我们的步骤创建 Rust 的 Hello World！程序：

**下面的命令操作，如果没有特别说明，都是在shell下运行。本文为了简单统一，所有例子都在 win10 的 powershell 下运行，所有命令都运行在`ps:`标识符之后**

- 创建一个 Doing 目录和 helloworld.rs 文件

> ps: mkdir ~/Doing  
> ps: cd ~/Doing  
> ps: notepad helloworld.rs # 作者偏向于使用 sublime 作为编辑器  
> ps: subl helloworld.rs # 本章以后使用 subl 代替 notepad  

注意这里用的后缀名是.rs，一般编程语言的代码文件都有惯用的后缀名，比如：
    C语言是.c，java是.java，python是.py等等，**请务必记住Rust语言的惯用后缀名是.rs**（虽然用别的后缀名也能通过rustc的编译）。

- 在 helloworld.rs 文件中输入 Rust 代码

```rust
fn main() {
    println!("Hello World!");
}
```

- 编译 helloworld.rs 文件

> ps: rustc helloworld.rs  
> ps: rustc helloworld.rs -O # 也可以选择优化编译  

- 运行程序

> ps: ./helloworld.exe # windows 平台下需要加 .exe 后缀  
> Hello World!  

没有`ps:`前缀的表示为控制台打印输出。

我们已经用rust编写第一个可执行程序，打印出了'hello world!'，很酷，对吧！
但是这段代码到底是什么意思呢，作为新手的你一定云里雾里吧，让我们先看一下这个程序：

1. 第一行中 fn 表示定义一个**函数**，main是这个函数的名字，花括号{}里的语句则表示这个函数的内容。
2. 名字叫做**main**的函数有特殊的用途，那就是作为程序的入口，也就是说程序每次都从这个函数开始运行。
3. 函数中只有一句 ```println!("Hello World!");```，这里```println!```是一个Rust语言自带的**宏**，
这个宏的功能就是打印文本(结尾会换行)，而"Hello World!"这个用引号包起来的东西是一个**字符串**，就是我们要打印的文本。
4. 你一定注意到了```;```吧， 在Rust语言中，分号```;```用来把语句分隔开，也就是说语句的末尾一般用分号做为结束标志。

## HelloRust

- 创建项目 hellorust

> ps: cargo new hellorust --bin

- 查看目录结构

> ps: tree # win10 powershell 自带有 tree 查看文件目录结构的功能  
> └─hellorust  
> ----└─src

这里显示的目录结构，在hellorust目录下有 src 文件夹和 Cargo.toml 文件，同时这个目录会初始化为 git 项目

- 查看Cargo.toml文件

> ps: cat Cargo.toml  
> [package]  
name = "hellorust"  
version = "0.1."  
authors = ["YourName <YourEmail>"]  
[dependencies]  

- 编辑src目录下的main.rs文件

> ps: subl ./src/main.rs

cargo 创建的项目，在src目录下会有一个初始化的main.rs文件，内容为：

```rust
fn main() {
    println!("Hello, world!");
}
```

现在我们编辑这个文件，改为：

```rust
fn main() {
    let rust = "Rust";
    println!("Hello, {}!", rust);
}
```

这里的 `let rust = "Rust"` 是把 rust 变量绑定为 "Rust" ，
`println!("Hello, {}!", rust);`里把 rust 变量的值代入到`"Hello, {}!"`中的`{}`。

- 编译和运行

> ps: cargo build  
> ps: cargo build --release # 这个属于优化编译  
> ps: ./target/debug/hellorust.exe  
> ps: ./target/release/hellorust.exe # 如果前面是优化编译，则这样运行  
> ps: cargo run # 编译和运行合在一起  
> ps: cargo run --release # 同上，区别是是优化编译的  
