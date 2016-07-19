# 10.1 trait关键字

## trait与具体类型

使用**trait**定义一个特征：

```rust
trait HasArea {
    fn area(&self) -> f64;
}
```

**trait**里面的函数可以没有函数体，实现代码交给具体实现它的类型去补充：

```rust
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

fn main() {
    let c = Circle {
        x: 0.0f64,
        y: 0.0f64,
        radius: 1.0f64,
    };
    println!("circle c has an area of {}", c.area());
}
```

**注**: **&self**表示的是**area**这个函数会将调用者的借代引用作为参数

这个程序会输出：

```
circle c has an area of 3.141592653589793
```

## trait与泛型

> 我们了解了Rust中trait的定义和使用，接下来我们介绍一下它的使用场景，从中我们可以窥探出接口这特性带来的惊喜

我们知道泛型可以指任意类型，但有时这不是我们想要的，需要给它一些约束。

#### 泛型的trait约束

```rust
use std::fmt::Debug;
fn foo<T: Debug>(s: T) {
    println!("{:?}", s);
}
```

`Debug`是**Rust**内置的一个trait，为"{:?}"实现打印内容，函数`foo`接受一个泛型作为参数，并且约定其需要实现`Debug`

#### 多trait约束

可以使用多个trait对泛型进行约束：

```rust
use std::fmt::Debug;
fn foo<T: Debug + Clone>(s: T) {
    s.clone();
    println!("{:?}", s);
}
```

`<T: Debug + Clone>`中`Debug`和`Clone`使用`+`连接，标示泛型`T`需要同时实现这两个trait。

#### where关键字

约束的trait增加后，代码看起来就变得诡异了，这时候需要使用`where`从句：

```rust
use std::fmt::Debug;
fn foo<T: Clone, K: Clone + Debug>(x: T, y: K) {
    x.clone();
    y.clone();
    println!("{:?}", y);
}

// where 从句
fn foo<T, K>(x: T, y: K) where T: Clone, K: Clone + Debug {
    x.clone();
    y.clone();
    println!("{:?}", y);
}

// 或者
fn foo<T, K>(x: T, y: K)
    where T: Clone,
          K: Clone + Debug {
    x.clone();
    y.clone();
    println!("{:?}", y);
}
```

## trait与内置类型

内置类型如：`i32`, `i64`等也可以添加trait实现，为其定制一些功能：

```rust
trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for i32 {
    fn area(&self) -> f64 {
        *self as f64
    }
}

5.area();
```

这样的做法是有限制的。Rust 有一个“孤儿规则”：当你为某类型实现某 trait 的时候，必须要求类型或者 trait 至少有一个是在当前 crate 中定义的。你不能为第三方的类型实现第三方的 trait 。

在调用 trait 中定义的方法的时候，一定要记得让这个 trait 可被访问。

```rust
let mut f = std::fs::File::open("foo.txt").ok().expect("Couldn’t open foo.txt");
let buf = b"whatever"; //  buf: &[u8; 8]
let result = f.write(buf);
# result.unwrap();
```

这里是错误：

```
error: type `std::fs::File` does not implement any method in scope named `write`
let result = f.write(buf);
               ^~~~~~~~~~
```

我们需要先use这个Write trait：

```rust
use std::io::Write;

let mut f = std::fs::File::open("foo.txt").expect("Couldn’t open foo.txt");
let buf = b"whatever";
let result = f.write(buf);
# result.unwrap(); // ignore the error
```

这样就能无错误的编译了。


## trait的默认方法


```rust
trait Foo {
    fn is_valid(&self) -> bool;

    fn is_invalid(&self) -> bool { !self.is_valid() }
}
```

`is_invalid`是默认方法，`Foo`的实现者并不要求实现它，如果选择实现它，会覆盖掉它的默认行为。

## trait的继承

```rust
trait Foo {
    fn foo(&self);
}

trait FooBar : Foo {
    fn foobar(&self);
}
```

这样`FooBar`的实现者也要同时实现`Foo`：

```rust
struct Baz;

impl Foo for Baz {
    fn foo(&self) { println!("foo"); }
}

impl FooBar for Baz {
    fn foobar(&self) { println!("foobar"); }
}
```

## derive属性

**Rust**提供了一个属性`derive`来自动实现一些trait，这样可以避免重复繁琐的实现他们，能被`derive`使用的trait包括：`Clone`, `Copy`, `Debug`, `Default`, `Eq`, `Hash`, `Ord`, `PartialEq`, `PartialOrd`

```rust
#[derive(Debug)]
struct Foo;

fn main() {
    println!("{:?}", Foo);
}
```


