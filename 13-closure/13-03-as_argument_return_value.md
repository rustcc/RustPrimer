# 闭包作为参数和返回值
## 闭包作为参数（Taking closures as arguments）

现在我们知道了闭包是 trait，我们已经知道了如何接受和返回闭包；就像任何其它的 trait！

这也意味着我们也可以选择静态或动态分发。首先，让我们写一个获取可调用结构的函数，调用它，然后返回结果：

```rust
fn call_with_one<F>(some_closure: F) -> i32
    where F : Fn(i32) -> i32 {

    some_closure(1)
}

let answer = call_with_one(|x| x + 2);

assert_eq!(3, answer);
```

我们传递我们的闭包，`|x| x + 2`，给`call_with_one`。它正做了我们说的：它调用了闭包，`1`作为参数。

让我们更深层的解析`call_with_one`的签名：

```rust
fn call_with_one<F>(some_closure: F) -> i32
#    where F : Fn(i32) -> i32 {
#    some_closure(1) }
```

我们获取一个参数，而它有类型`F`。我们也返回一个`i32`。这一部分并不有趣。下一部分是：

```rust
# fn call_with_one<F>(some_closure: F) -> i32
    where F : Fn(i32) -> i32 {
#   some_closure(1) }
```

因为`Fn`是一个trait，我们可以用它限制我们的泛型。在这个例子中，我们的闭包取得一个`i32`作为参数并返回`i32`，所以我们用泛型限制是`Fn(i32) -> i32`。

还有一个关键点在于：因为我们用一个trait限制泛型，它会是单态的，并且因此，我们在闭包中使用静态分发。这是非常简单的。在很多语言中，闭包固定在堆上分配，所以总是进行动态分发。在Rust中，我们可以在栈上分配我们闭包的环境，并静态分发调用。这经常发生在迭代器和它们的适配器上，它们经常取得闭包作为参数。

当然，如果我们想要动态分发，我们也可以做到。trait对象处理这种情况，通常：

```rust
fn call_with_one(some_closure: &Fn(i32) -> i32) -> i32 {
    some_closure(1)
}

let answer = call_with_one(&|x| x + 2);

assert_eq!(3, answer);
```

现在我们取得一个trait对象，一个`&Fn`。并且当我们将我们的闭包传递给`call_with_one`时我们必须获取一个引用，所以我们使用`&||`。

## 函数指针和闭包

一个函数指针有点像一个没有环境的闭包。因此，你可以传递一个函数指针给任何函数除了作为闭包参数，下面的代码可以工作：

```rust
fn call_with_one(some_closure: &Fn(i32) -> i32) -> i32 {
    some_closure(1)
}

fn add_one(i: i32) -> i32 {
    i + 1
}

let f = add_one;

let answer = call_with_one(&f);

assert_eq!(2, answer);
```

在这个例子中，我们并不是严格的需要这个中间变量`f`，函数的名字就可以了：

```rust
let answer = call_with_one(&add_one);
```

## 返回闭包（Returning closures）

对于函数式风格代码来说在各种情况返回闭包是非常常见的。如果你尝试返回一个闭包，你可能会得到一个错误。在刚接触的时候，这看起来有点奇怪，不过我们会搞清楚。当你尝试从函数返回一个闭包的时候，你可能会写出类似这样的代码：

```rust
fn factory() -> (Fn(i32) -> i32) {
    let num = 5;

    |x| x + num
}

let f = factory();

let answer = f(1);
assert_eq!(6, answer);
```

编译的时候会给出这一长串相关错误：

```text
error: the trait `core::marker::Sized` is not implemented for the type
`core::ops::Fn(i32) -> i32` [E0277]
fn factory() -> (Fn(i32) -> i32) {
                ^~~~~~~~~~~~~~~~
note: `core::ops::Fn(i32) -> i32` does not have a constant size known at compile-time
fn factory() -> (Fn(i32) -> i32) {
                ^~~~~~~~~~~~~~~~
error: the trait `core::marker::Sized` is not implemented for the type `core::ops::Fn(i32) -> i32` [E0277]
let f = factory();
    ^
note: `core::ops::Fn(i32) -> i32` does not have a constant size known at compile-time
let f = factory();
    ^
```

为了从函数返回一些东西，Rust 需要知道返回类型的大小。不过`Fn`是一个 trait，它可以是各种大小(size)的任何东西。比如说，返回值可以是实现了`Fn`的任意类型。一个简单的解决方法是：返回一个引用。因为引用的大小(size)是固定的，因此返回值的大小就固定了。因此我们可以这样写：

```rust
fn factory() -> &(Fn(i32) -> i32) {
    let num = 5;

    |x| x + num
}

let f = factory();

let answer = f(1);
assert_eq!(6, answer);
```

不过这样会出现另外一个错误：

```text
error: missing lifetime specifier [E0106]
fn factory() -> &(Fn(i32) -> i32) {
                ^~~~~~~~~~~~~~~~~
```

对。因为我们有一个引用，我们需要给它一个生命周期。不过我们的`factory()`函数不接收参数，所以省略不能用在这。我们可以使用什么生命周期呢？`'static`：

```rust
fn factory() -> &'static (Fn(i32) -> i32) {
    let num = 5;

    |x| x + num
}

let f = factory();

let answer = f(1);
assert_eq!(6, answer);
```

不过这样又会出现另一个错误：

```text
error: mismatched types:
 expected `&'static core::ops::Fn(i32) -> i32`,
    found `[closure@<anon>:7:9: 7:20]`
(expected &-ptr,
    found closure) [E0308]
         |x| x + num
         ^~~~~~~~~~~

```

这个错误让我们知道我们并没有返回一个`&'static Fn(i32) -> i32`，而是返回了一个`[closure <anon>:7:9: 7:20]`。等等，什么？

因为每个闭包生成了它自己的环境`struct`并实现了`Fn`和其它一些东西，这些类型是匿名的。它们只在这个闭包中存在。所以Rust把它们显示为`closure <anon>`，而不是一些自动生成的名字。

这个错误也指出了返回值类型期望是一个引用，不过我们尝试返回的不是。更进一步，我们并不能直接给一个对象`'static`声明周期。所以我们换一个方法并通过`Box`装箱`Fn`来返回一个 trait 对象。这个*几乎*可以成功运行：

```rust
fn factory() -> Box<Fn(i32) -> i32> {
    let num = 5;

    Box::new(|x| x + num)
}
# fn main() {
let f = factory();

let answer = f(1);
assert_eq!(6, answer);
# }
```

这还有最后一个问题：

```text
error: closure may outlive the current function, but it borrows `num`,
which is owned by the current function [E0373]
Box::new(|x| x + num)
         ^~~~~~~~~~~
```

好吧，正如我们上面讨论的，闭包借用他们的环境。而且在这个例子中。我们的环境基于一个栈分配的`5`，`num`变量绑定。所以这个借用有这个栈帧的生命周期。所以如果我们返回了这个闭包，这个函数调用将会结束，栈帧也将消失，那么我们的闭包指向了被释放的内存环境！再有最后一个修改，我们就可以让它运行了：

```rust
fn factory() -> Box<Fn(i32) -> i32> {
    let num = 5;

    Box::new(move |x| x + num)
}
# fn main() {
let f = factory();

let answer = f(1);
assert_eq!(6, answer);
# }
```

通过把内部闭包添加`move`关键字，我们强制闭包使用 move 的方式捕获环境变量。因为这里的 num 类型是 i32，实际上这里的 move 执行的是 copy, 这样一来，闭包就不再拥有指向环境的指针，而是完整拥有了被捕获的变量。并允许它离开我们的栈帧。

> ### 这部分引用自[The Rust Programming Language中文版](https://github.com/KaiserY/rust-book-chinese/blob/master/content/Closures%20%E9%97%AD%E5%8C%85.md)
