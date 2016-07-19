# 泛型


我们在编程中，通常有这样的需求，为多种类型的数据编写一个功能相同的函数，如两个数的加法，希望这个函数既支持i8、i16、 i32 ....float64等等，甚至自定义类型，在不支持泛型的编程语言中，我们通常要为每一种类型都编写一个函数，而且通常情况下函数名还必须不同，例如：

```rust
fn add_i8(a:i8, b:i8) -> i8 {
	a + b
}
fn add_i16(a:i16, b:i16) -> i16 {
	a + b
}
fn add_f64(a:f64, b:f64) -> f64 {
	a + b
}

// 各种其他add函数
// ...

fn main() {
	println!("add i8: {}", add_i8(2i8, 3i8));
	println!("add i16: {}", add_i16(20i16, 30i16));
	println!("add f64: {}", add_f64(1.23, 1.23));
}
```

如果有很多地方都需要支持多种类型，那么代码量就会非常大，而且代码也会非常臃肿，编程就真的变成了苦逼搬砖的工作，枯燥而乏味:D。
学过C++的人也许很容易理解泛型，但本教程面向的是Rust初学者，所以不会拿C++的泛型、多态和Rust进行对比，以免增加学习的复杂度和不必要的困扰，从而让Rust初学者更容易理解和接受Rust泛型。


## 概念

泛型程序设计是程序设计语言的一种风格或范式。允许程序员在强类型程序设计语言中编写代码时使用一些以后才指定的类型，在实例化时（instantiate）作为参数指明这些类型（在Rust中，有的时候类型还可以被编译器推导出来）。各种程序设计语言和其编译器、运行环境对泛型的支持均不一样。Ada, Delphi, Eiffel, Java, C#, F#, Swift, and Visual Basic .NET称之为泛型（generics）；ML, Scala and Haskell称之为参数多态（parametric polymorphism）；C++与D称之为模板。具有广泛影响的1994年版的《Design Patterns》一书称之为参数化类型（parameterized type）。

>提示：
>以上概念摘自[《维基百科-泛型》](https://zh.wikipedia.org/wiki/%E6%B3%9B%E5%9E%8B)

在编程的时候，我们经常利用多态。通俗的讲，多态就是好比坦克的炮管，既可以发射普通弹药，也可以发射制导炮弹（导弹），也可以发射贫铀穿甲弹，甚至发射子母弹，大家都不想为每一种炮弹都在坦克上分别安装一个专用炮管，即使生产商愿意，炮手也不愿意，累死人啊。所以在编程开发中，我们也需要这样“通用的炮管”，这个“通用的炮管”就是多态。

需要知道的是，泛型就是一种多态。

泛型主要目的是为程序员提供了编程的便利，减少代码的臃肿,同时极大丰富了语言本身的表达能力, 为程序员提供了一个合适的炮管。想想，一个函数，代替了几十个，甚至数百个函数，是一件多么让人兴奋的事情。
泛型，可以理解为具有某些功能共性的集合类型，如i8、i16、u8、f32等都可以支持add，甚至两个struct Point类型也可以add形成一个新的Point。

先让我们来看看标准库中常见的泛型Option<T>，它的原型定义：

```rust
enum Option<T> {
	Some(T),
	None,
}
```

T就是泛型参数，这里的T可以换成A-Z任何你自己喜欢的字母。不过习惯上，我们用T表示Type，用E表示Error。T在具体使用的时候才会被实例化：

```rust
let a = Some(100.111f32);
```

编译器会自行推导出a为Option<f32>类型，也就是说Option中的T在这里是f32类型。

当然，你也可以显式声明a的类型，但必须保证和右值的类型一样，不然编译器会报"mismatched types"类型不匹配错误。

```rust
let a:Option<f32> = Some(100.111);  //编译自动推导右值中的100.111为f32类型。
let b:Option<f32> = Some(100.111f32);
let c:Option<f64> = Some(100.111);
let d:Option<f64> = Some(100.111f64);
```


### 泛型函数
至此，我们已经了解到泛型的定义和简单的使用了。
现在让我们用函数重写add操作：

```rust
use std::ops::Add;

fn add<T: Add<T, Output=T>>(a:T, b:T) -> T {
	a + b
}

fn main() {
	println!("{}", add(100i32, 1i32));
	println!("{}", add(100.11f32, 100.22f32));
}
```

>**输出:**
>101
>200.33

```add<T: Add<T, Output=T>>(a:T, b:T) -> T```就是我们泛型函数，返回值也是泛型T，Add<>中的含义可以暂时忽略，大体意思就是只要参数类型实现了Add trait，就可以被传入到我们的add函数，因为我们的add函数中有相加+操作，所以要求传进来的参数类型必须是可相加的，也就是必须实现了Add trait（具体参考std::ops::Add）。

### 自定义类型
上面的例子，add的都是语言内置的基础数据类型，当然我们也可以为自己自定义的数据结构类型实现add操作。

```rust
use std::ops::Add;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// 为Point实现Add trait
impl Add for Point {
    type Output = Point; //执行返回值类型为Point
    fn add(self, p: Point) -> Point {
        Point{
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

fn add<T: Add<T, Output=T>>(a:T, b:T) -> T {
	a + b
}

fn main() {
	println!("{}", add(100i32, 1i32));
	println!("{}", add(100.11f32, 100.22f32));

	let p1 = Point{x: 1, y: 1};
	let p2 = Point{x: 2, y: 2};
	println!("{:?}", add(p1, p2));
}
```

>**输出:**
>101
200.33
Point { x: 3, y: 3 }

上面的例子稍微更复杂些了，只是我们增加了自定义的类型，然后让add函数依然可以在上面工作。如果对trait不熟悉，请查阅trait相关章节。

大家可能会疑问，那我们是否可以让Point也变成泛型的，这样Point的x和y也能够支持float类型或者其他类型，答案当然是可以的。

```rust
use std::ops::Add;

#[derive(Debug)]
struct Point<T: Add<T, Output = T>> { //限制类型T必须实现了Add trait，否则无法进行+操作。
    x: T,
    y: T,
}

impl<T: Add<T, Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, p: Point<T>) -> Point<T> {
        Point{
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

fn add<T: Add<T, Output=T>>(a:T, b:T) -> T {
	a + b
}

fn main() {
	let p1 = Point{x: 1.1f32, y: 1.1f32};
	let p2 = Point{x: 2.1f32, y: 2.1f32};
	println!("{:?}", add(p1, p2));

	let p3 = Point{x: 1i32, y: 1i32};
	let p4 = Point{x: 2i32, y: 2i32};
	println!("{:?}", add(p3, p4));
}
```

>**输出：**
>Point { x: 3.2, y: 3.2 }
Point { x: 3, y: 3 }

上面的列子更复杂了些，我们不仅让自定义的Point类型支持了add操作，同时我们也为Point做了泛型化。

当```let p1 = Point{x: 1.1f32, y: 1.1f32};```时，Point的T推导为f32类型，这样Point的x和y属性均成了f32类型。因为p1.x+p2.x，所以T类型必须支持Add trait。

### 总结
上面区区几十行的代码，却实现了非泛型语言百行甚至千行代码才能达到的效果，足见泛型的强大。

### 习题

#### 1. Generic lines iterator

##### 问题描述
有时候我们可能做些文本分析工作, 数据可能来源于外部或者程序内置的文本.

请实现一个 `parse` 函数, 只接收一个 lines iterator 为参数, 并输出每行.

要求既能输出内置的文本, 也能输出文件内容.

##### 调用方式及输出参考

```
let lines = "some\nlong\ntext".lines()
parse(do_something_or_nothing(lines))
```

```
some
long
text
```

```
use std::fs:File;
use std::io::prelude::*;
use std::io::BufReader;
let lines = BufReader::new(File::open("/etc/hosts").unwrap()).lines()
parse(do_some_other_thing_or_nothing(lines))
```

```
127.0.0.1       localhost.localdomain   localhost
::1             localhost.localdomain   localhost
...
```

##### Hint
本书`类型系统中的几个常见 trait`章节中介绍的 AsRef, Borrow 等 trait 应该能派上用场.
