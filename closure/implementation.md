# 闭包的实现

Rust 的闭包实现与其它语言有些许不同。它们实际上是trait的语法糖。在这以前你会希望阅读[trait章节](https://doc.rust-lang.org/stable/book/traits.html)，和[trait对象](https://doc.rust-lang.org/stable/book/trait-objects.html)。

都理解吗？很好。

理解闭包底层是如何工作的关键有点奇怪：使用`()`调用函数，像`foo()`，是一个可重载的运算符。到此，其它的一切都会明了。在Rust中，我们使用trait系统来重载运算符。调用函数也不例外。我们有三个trait来分别重载：

```rust
# mod foo {
pub trait Fn<Args> : FnMut<Args> {
    extern "rust-call" fn call(&self, args: Args) -> Self::Output;
}

pub trait FnMut<Args> : FnOnce<Args> {
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
}

pub trait FnOnce<Args> {
    type Output;

    extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
}
# }
```

你会注意到这些 trait 之间的些许区别，不过一个大的区别是`self`：`Fn`获取`&self`，`FnMut`获取`&mut self`，而`FnOnce`获取`self`。这包含了所有3种通过通常函数调用语法的`self`。不过我们将它们分在 3 个 trait 里，而不是单独的 1 个。这给了我们大量的对于我们可以使用哪种闭包的控制。

闭包的`|| {}`语法是上面 3 个 trait 的语法糖。Rust 将会为了环境创建一个结构体，`impl`合适的 trait，并使用它。

> ### 这部分引用自[The Rust Programming Language中文版](https://github.com/KaiserY/rust-book-chinese/blob/master/content/Closures%20%E9%97%AD%E5%8C%85.md)
