# 运算符重载

Rust可以让我们对某些运算符进行重载，这其中大部分的重载都是对`std::ops`下的trait进行重载而实现的。

## 重载加法

我们现在来实现一个只支持加法的阉割版[复数](https://zh.wikipedia.org/wiki/%E5%A4%8D%E6%95%B0_%28%E6%95%B0%E5%AD%A6%29)：

```rust
use std::ops::Add;

#[derive(Debug)]
struct Complex {
    a: f64,
    b: f64,
}

impl Add for Complex {
    type Output = Complex;
    fn add(self, other: Complex) -> Complex {
        Complex {a: self.a+other.a, b: self.b+other.b}
    }
}

fn main() {
    let cp1 = Complex{a: 1f64, b: 2.0};
    let cp2 = Complex{a: 5.0, b:8.1};
    let cp3 = cp1 + cp2;
    print!("{:?}", cp3);
}
```

输出:

```
Complex { a: 6, b: 10.1}
```

这里我们实现了`std::ops::Add`这个trait。这时候有同学一拍脑门，原来如此，没错……其实Rust的大部分运算符都是`std::ops`下的trait的语法糖！

我们来看看`std::ops::Add`的具体结构

```rust
impl Add<i32> for Point {
    type Output = f64;

    fn add(self, rhs: i32) -> f64 {
        // add an i32 to a Point and get an f64
    }
}
```

## 神奇的Output以及动态分发
有的同学会问了，这个`Output`是肿么回事？答，类型转换哟亲！
举个不太恰当的栗子，我们在现实中会出现`0.5+0.5=1`这样的算式，用Rust的语言来描述就是： 两个`f32`相加得到了一个`i8`。显而易见，Output就是为这种情况设计的。

还是看代码：

```rust
use std::ops::Add;

#[derive(Debug)]
struct Complex {
    a: f64,
    b: f64,
}

impl Add for Complex {
    type Output = Complex;
    fn add(self, other: Complex) -> Complex {
        Complex {a: self.a+other.a, b: self.b+other.b}
    }
}

impl Add<i32> for Complex {
    type Output = f64;
    fn add(self, other: i32) -> f64 {
        self.a + self.b + (other as f64)
    }
}

fn main() {
    let cp1 = Complex{a: 1f64, b: 2.0};
    let cp2 = Complex{a: 5.0, b:8.1};
    let cp3 = Complex{a: 9.0, b:20.0};
    let complex_add_result = cp1 + cp2;
    print!("{:?}\n", complex_add_result);
    print!("{:?}", cp3 + 10i32);
}
```

输出结果：

```
Complex { a: 6, b: 10.1 }
39
```

## 对范型的限制

Rust的运算符是基于trait系统的，同样的，运算符可以被当成一种对范型的限制，我们可以这么要求`范型T必须实现了trait Mul<Output=T>`。
于是，我们得到了如下的一份代码：

```rust
use std::ops::Mul;

trait HasArea<T> {
    fn area(&self) -> T;
}

struct Square<T> {
    x: T,
    y: T,
    side: T,
}

impl<T> HasArea<T> for Square<T>
        where T: Mul<Output=T> + Copy {
    fn area(&self) -> T {
        self.side * self.side
    }
}

fn main() {
    let s = Square {
        x: 0.0f64,
        y: 0.0f64,
        side: 12.0f64,
    };

    println!("Area of s: {}", s.area());
}
```

对于trait `HasArea<T>`和 struct `Square<T>`，我们通过`where T: Mul<Output=T> + Compy` 限制了`T`必须实现乘法。同时Copy则限制了Rust不再将self.side给move到返回值里去。

写法简单，轻松愉快。
