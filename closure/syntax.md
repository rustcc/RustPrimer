# 闭包的语法
## 基本形式
闭包看起来像这样：

```rust
let plus_one = |x: i32| x + 1;

assert_eq!(2, plus_one(1));
```

我们创建了一个绑定，`plus_one`，并把它赋予一个闭包。闭包的参数位于管道（`|`）之中，而闭包体是一个表达式，在这个例子中，`x + 1`。记住`{}`是一个表达式，所以我们也可以拥有包含多行的闭包：

```rust
let plus_two = |x| {
    let mut result: i32 = x;

    result += 1;
    result += 1;

    result
};

assert_eq!(4, plus_two(2));
```

你会注意到闭包的一些方面与用`fn`定义的常规函数有点不同。第一个是我们并不需要标明闭包接收和返回参数的类型。我们可以：

```rust
let plus_one = |x: i32| -> i32 { x + 1 };

assert_eq!(2, plus_one(1));
```

不过我们并不需要这么写。为什么呢？基本上，这是出于“人体工程学”的原因。因为为命名函数指定全部类型有助于像文档和类型推断，而闭包的类型则很少有文档因为它们是匿名的，并且并不会产生像推断一个命名函数的类型这样的“远距离错误”。

第二个的语法大同小异。我会增加空格来使它们看起来更像一点：

```rust
fn  plus_one_v1   (x: i32) -> i32 { x + 1 }
let plus_one_v2 = |x: i32| -> i32 { x + 1 };
let plus_one_v3 = |x: i32|          x + 1  ;
```

## 捕获变量
之所以把它称为“闭包”是因为它们“包含在环境中”（close over their environment）。这看起来像：

```rust
let num = 5;
let plus_num = |x: i32| x + num;

assert_eq!(10, plus_num(5));
```

这个闭包，`plus_num`，引用了它作用域中的`let`绑定：`num`。更明确的说，它借用了绑定。如果我们做一些会与这个绑定冲突的事，我们会得到一个错误。比如这个：

```rust
let mut num = 5;
let plus_num = |x: i32| x + num;

let y = &mut num;
```

错误是：

```text
error: cannot borrow `num` as mutable because it is also borrowed as immutable
    let y = &mut num;
                 ^~~
note: previous borrow of `num` occurs here due to use in closure; the immutable
  borrow prevents subsequent moves or mutable borrows of `num` until the borrow
  ends
    let plus_num = |x| x + num;
                   ^~~~~~~~~~~
note: previous borrow ends here
fn main() {
    let mut num = 5;
    let plus_num = |x| x + num;

    let y = &mut num;
}
^
```

一个啰嗦但有用的错误信息！如它所说，我们不能取得一个`num`的可变借用因为闭包已经借用了它。如果我们让闭包离开作用域，我们可以：

```rust
let mut num = 5;
{
    let plus_num = |x: i32| x + num;

} // plus_num goes out of scope, borrow of num ends

let y = &mut num;
```

如果你的闭包需要它，Rust会取得所有权并移动环境：

```rust
let nums = vec![1, 2, 3];

let takes_nums = || nums;

println!("{:?}", nums);
```

这会给我们：

```text
note: `nums` moved into closure environment here because it has type
  `[closure(()) -> collections::vec::Vec<i32>]`, which is non-copyable
let takes_nums = || nums;
                    ^~~~~~~
```

`Vec<T>`拥有它内容的所有权，而且由于这个原因，当我们在闭包中引用它时，我们必须取得`nums`的所有权。这与我们传递`nums`给一个取得它所有权的函数一样。

## move闭包
我们可以使用`move`关键字强制使我们的闭包取得它环境的所有权：

```rust
let num = 5;

let owns_num = move |x: i32| x + num;
```

现在，即便关键字是`move`，变量遵循正常的移动语义。在这个例子中，`5`实现了`Copy`，所以`owns_num`取得一个`5`的拷贝的所有权。那么区别是什么呢？

```rust
let mut num = 5;

{
    let mut add_num = |x: i32| num += x;

    add_num(5);
}

assert_eq!(10, num);
```

那么在这个例子中，我们的闭包取得了一个`num`的可变引用，然后接着我们调用了`add_num`，它改变了其中的值，正如我们期望的。我们也需要将`add_num`声明为`mut`，因为我们会改变它的环境。

如果我们加上`move`修饰闭包，会发生些不同：

```rust
let mut num = 5;

{
    let mut add_num = move |x: i32| num += x;

    add_num(5);
}

assert_eq!(5, num);
```

我们只会得到`5`。这次我们没有获取到外部的`num`的可变借用，我们实际上是把 `num` move 进了闭包。因为 `num` 具有 Copy 属性，因此发生 move 之后，以前的变量生命周期并未结束，还可以继续在 `assert_eq!` 中使用。我们打印的变量和闭包内的变量是独立的两个变量。如果我们捕获的环境变量不是 Copy 的，那么外部环境变量被 move 进闭包后，
它就不能继续在原先的函数中使用了，只能在闭包内使用。

不过在我们讨论获取或返回闭包之前，我们应该更多的了解一下闭包实现的方法。作为一个系统语言，Rust给予你了大量的控制你代码的能力，而闭包也是一样。

> ### 这部分引用自[The Rust Programming Language中文版](https://github.com/KaiserY/rust-book-chinese/blob/master/content/Closures%20%E9%97%AD%E5%8C%85.md)
