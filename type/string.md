# String

这章我们来着重介绍一下字符串。

刚刚学习Rust的同学可能会被Rust的字符串搞混掉，比如`str`，`String`， `OsStr`， `CStr`，`CString`等等……
事实上，如果你不做FFI的话，常用的字符串类型就只有前两种。我们就来着重研究一下Rust的前两种字符串。

你要明白的是，Rust中的字符串实际上是被编码成UTF-8的一个字节数组。这么说比较拗口，简单来说，Rust字符串内部存储的是一个u8数组，但是这个数组是Unicode字符经过UTF-8编码得来的。因此，可以看成Rust原生就支持Unicode字符集（Python2的码农泪流满面）。

## str

首先我们先来看一下`str`， 从字面意思上，Rust的string被表达为： `&'static str`(看不懂这个表达式没关系，&表示引用你知道吧，static表示静态你知道吧，好了，齐了)，即，你在代码里写的，所有的用`""`包裹起来的字符串，都被声明成了一个不可变，静态的字符串。而我们的如下语句：

```rust
let x = "Hello";
let x:&'static str = "Hello";
```

实际上是将 `"Hello"` 这个静态变量的引用传递给了`x`。同时，这里的字符串不可变！

字符串也支持转义字符：
比如如下：

```rust
let z = "foo
bar";
let w = "foo\nbar";
assert_eq!(z, w);
```

也可以在字符串字面量前加上`r`来避免转义

    //没有转义序列
    let d: &'static str = r"abc \n abc";
    //等价于
    let c: &'static str = "abc \\n abc";

## String

光有`str`，确实不够什么卵用，毕竟我们在实际应用中要的更多的还是一个可变的，不定长的字符串。这时候，一种在堆上声明的字符串`String`被设计了出来。
它能动态的去增长或者缩减，那么怎么声明它呢？我们先介绍一种简单的方式，从`str`中转换：

```rust
let x:&'static str = "hello";

let mut y:String = x.to_string();
println!("{}", y);
y.push_str(", world");
println!("{}", y);
```

我知道你一定会问：——
    那么如何将一个`String`重新变成`&str`呢？
    答：用 `&*` 符号

```rust
fn use_str(s: &str) {
    println!("I am: {}", s);
}

fn main() {
    let s = "Hello".to_string();
    use_str(&*s);
}
```

我们来分析一下，以下部分将涉及到部分`Deref`的知识，可能需要你预习一下，如果不能理解大可跳过下一段：

首先呢， `&*`是两个符号`&`和`*`的组合，按照Rust的运算顺序，先对`String`进行`Deref`,也就是`*`操作。

由于`String`实现了 `impl Deref<Target=str> for String`，这相当于一个运算符重载，所以你就能通过`*`获得一个`str`类型。但是我们知道，单独的`str`是不能在Rust里直接存在的，因此，我们需要先给他进行`&`操作取得`&str`这个结果。

有人说了，我发现只要用`&`一个操作符就能将使上面的编译通过。
这其实是一个编译器的锅，因为Rust的编译器会在`&`后面插入足够多的`*`来尽可能满足`Deref`这个特性。这个特性会在某些情况下失效，因此，为了不给自己找麻烦，还是将操作符写全为好。


需要知道的是，将`String`转换成`&str`是非常轻松的，几乎没有任何开销。但是反过来，将`&str`转换成`String`是需要在堆上请求内存的，因此，要慎重。

我们还可以将一个UTF-8编码的字节数组转换成String，如

```rust
// 存储在Vec里的一些字节
let miao = vec![229,150,181];

// 我们知道这些字节是合法的UTF-8编码字符串，所以直接unwrap()
let meow = String::from_utf8(miao).unwrap();

assert_eq!("喵", meow);
```

## 索引访问

有人会把Rust中的字符串和其惯用的字符串等同起来，于是就出现了如下代码

```rust
let x = "hello".to_string();
x[1]; //编译错误！
```

Rust的字符串实际上是不支持通过下标访问的，但是呢，我们可以通过将其转变成数组的方式访问

```rust
let x = "哎哟我去".to_string();
for i in x.as_bytes() {
    print!("{} ", i);
}

println!("");

for i in x.chars() {
    print!("{}", i);
}

x.chars().nth(2);
```

## 字符串切片

对字符串切片是一件非常危险的事，虽然Rust支持，但是我并不推荐。因为Rust的字符串Slice实际上是切的bytes。这也就造成了一个严重后果，如果你切片的位置正好是一个Unicode字符的内部，Rust会发生Runtime的panic，导致整个程序崩溃。
因为这个操作是如此的危险，所以我就不演示了……
