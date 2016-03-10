# 原生类型

像其他现代编程语言一样，Rust提供了一系列基础的类型，我们一般称之为*原生类型*。其强大的类型系统就是建立在这些原生类型之上的，因此，在写Rust代码之前，必须要对Rust的原生类型有一定的了解。

## bool

Rust自带了`bool`类型，其可能值为`true`或者`false`。
我们可以通过这样的方式去声明它：
```
let is_she_love_me = false;
let mut is_he_love_me: bool = true;
```
当然，bool类型被用的最多的地方就是在`if表达式`里了。

## char

在Rust中，一个`char`类型表示一个*Unicode*字符,这也就意味着，在某些语言里代表一个字符(8bit)的char，在Rust里实际上是四个字节(32bit)。
同时，我们可以将各种奇怪的非中文字符随心所欲的赋值给一个char类型。需要注意的是，Rust中我们要用`'`来表示一个char，如果用`"`的话你得到的实际上是一个`&'static str`。

```
let c = 'x';
let cc = '王';
```

## 数字类型

和其他类C系的语言不一样，Rust用一种*符号+位数*的方式来表示其基本的数字类型。可能你习惯了`int`、`double`、`float`之类的表示法，Rust的表示法需要你稍微适应一下。

你可用的符号有 `i`、`f`、`u`

你可用的位数，当然了，都是2的n次幂，分别为`8`、`16`、`32`、`64`及`size`。

你可以将其组合起来，形成诸如`i32`,`u16`等类型。

当然了，这样的组合并不自由，因为浮点类型最少只能用32位来表示，因此只能有`f32`和`f64`来表示。

有人说了，你看我整个`i128`行不。

答，不行，谁家电脑是128位的啊……… 

但是真的有人提议支持这个类型： https://github.com/rust-lang/rfcs/pull/1504 

### 自适应类型

看完上面你一定会对`isize`和`usize`很好奇。这两个是来干啥的。这两个嘛，其实是取决于你的操作系统的位数。简单粗暴一点比如64位电脑上就是64位，32位电脑上就是32位，16位……呵呵哒。

但是需要注意的是，你不能因为你的电脑是64位的，而强行将它等同于64，也就是说`isize != i64`，任何情况下你都需要强制转换。

## 数组 Array

Rust的数组是被表示为`[T;N]`。其中N表示数组大小，并且这个大小一定是个编译时就能获得的整数值，T表`泛型`类型，即任意类型。我们可以这么来声明和使用一个数组:

```
let a = [8, 9, 10];
let b: [u8;3] = [8, 6, 5]; 
print!("{}", a[0]);
```

和Golang一样，Rust的数组中的`N`（大小）也是类型的一部分，即`[3;0] != [4;0]`。这么设计是为了更安全和高效的使用内存，当然了，这会给第一次接触类似概念的人带来一点点困难，比如以下代码。

```
fn show(arr: [u8;3]) {
    for i in &arr {
        print!("{} ", i);
    }
}

fn main() {
    let a: [u8; 3] = [1, 2, 3];
    show(a);
    let b: [u8; 4] = [1, 2, 3, 4];
    show(b);
}
```

编译运行它你将获得一个编译错误：

```
<anon>:11:10: 11:11 error: mismatched types:
 expected `[u8; 3]`,
    found `[u8; 4]`
(expected an array with a fixed size of 3 elements,
    found one with 4 elements) [E0308]
<anon>:11     show(b);
                   ^
<anon>:11:10: 11:11 help: see the detailed explanation for E0308
error: aborting due to previous error
```

这是因为你将一个4长度的数组赋值给了一个只需要3长度数组作为参数的函数。那么如何写一个通用的show方法来展现任意长度数组呢？请看下节`Slice`

## Slice

`Slice`从直观上讲，是对一个`Array`的切片，通过`Slice`，你能获取到一个`Array`的部分或者全部的访问权限。和`Array`不同，`Slice`是可以动态的，但是呢，其范围是不能超过`Array`的大小，这点和Golang是不一样的。

一个`Slice`的表达式可以为如下: `&[T]` 或者 `&mut [T]`。

这里`&`符号是一个难点，我们不妨放开这个符号，简单的把它看成是`Slice`的甲鱼臀部——规定。另外，同样的，`Slice`也是可以通过下标的方式访问其元素，下标也是从0开始的哟。
你可以这么声明并使用一个`Slice`：

```
let arr = [1, 2, 3, 4, 5, 6];
let slice_complete = &arr[..]; // 获取全部元素
let slice_middle = &arr[1..4]; // 获取中间元素，最后取得的Slice为 [2, 3, 4] 。切片遵循左开右闭原则。
let slice_right = &arr[1..]; // 最后获得的元素为[2, 3, 4, 5, 6]，长度为5。
let slice_left = &arr[..3]; // 最后获得的元素为[1, 2, 3]，长度为3。
```

怎么样，了解了吧。
那么接下来我们用`Slice`来改造一下上面的函数

```
fn show(arr: &[u8]) {
    for i in arr {
        print!("{} ", i);
    }
    println!("");
}

fn main() {
    let a: [u8; 3] = [1, 2, 3];
    let slice_a = &a[..];
    show(slice_a);
    let b: [u8; 4] = [1, 2, 3, 4];
    show(&b[..]);
}
```
输出
```
1 2 3
1 2 3 4
```

## 可变数组 Vec

熟悉C++ std library的同学可能对C++可变数组vector很熟悉，同样的，Rust也提供了一个类似的东西。他叫`Vec`。

在基础类型里讲`Vec`貌似是不太合适的，但是本书的安排里，我们已经没有了别的地方来安排这个类型，然而它又是如此的重要，因此，我在这里将为大家介绍一下`Vec`这个类型。

在Rust里，`Vec`被表示为 `Vec<T>`， 其中T是一个范型。

下面介绍几种典型的`Vec`的用法:

```
let mut v1: Vec<i32> = vec![1, 2, 3]; // 通过vec!宏来声明
let v2 = vec![0; 10]; // 声明一个初始长度为10的值全为0的动态数组
println!("{}", v1[0]); // 通过下标来访问数组元素

for i in &v1 {
    print!("{}", i); // &Vec<i32> 可以通过 Deref 转换成 &[i32]
}

println!("");

for i in &mut v1 {
    *i = *i+1;
    print!("{}", i); // 可变访问
}

```
输出结果：
```
1
123
234
```

## 最原生字符串 str

你可以用`str`来声明一个字符串，事实上，Rust中，所有用`""`包裹起来的都可以称为`&str`(注意这个`&`,这是难点，不用管他，不是么？)，但是这个类型被单独用的情况很少，因此，我们将在下一节着重介绍字符串类型。

## 元组 Tuples

在别的语言里，你可能听过元组这个词，它表示一个大小、类型固定的有序数据组。在Rust中，情况并没有什么本质上的不同。不过Rust为我们提供了一系列简单便利的语法来让我们能更好的使用他。

```
let y = (2, "hello world");
let x: (3, &str) = (3, "world hello");

// 然后呢，你能用很简单的方式去访问他们：

// 用let表达式
let (w, z) = y; // w=2, z="hello world"

// 用下标

let f = x.0; // f = 3
let e = x.1; // e = "world hello"

```

## 结构体 struct

在Rust中，结构体是一个跟 `tuple` 类似 的概念。我们同样可以将一些常用的数据、属性聚合在一起，就形成了一个结构体。

所不同的是，Rust的结构体有三种最基本的形式。

### 具名结构体

这种结构体呢，他可以大致看成这样的一个声明形式:

```
struct A {
    attr1: i32,
    atrr2: String,
}
```

内部每个成员都有自己的名字和类型。

### 元组类型结构体

元组类型结构体使用小括号，类似 `tuple`。

```
struct B(i32, u16, bool);
```

它可以看作是一个有名字的元组，具体使用方法和一般的元组基本类似。

### 空结构体

结构体内部也可以没有任何成员。

```
struct D;
```

空结构体的内存占用为0。但是我们依然可以针对这样的类型实现它的“成员函数”。

不过到目前为止，空结构体后面不能加大括号，这么写会编译错误：

```
struct C {

}
```

不过也有人提出来，这样的写法应该编译通过：https://github.com/rust-lang/rfcs/pull/1506 

### 实现结构体 impl

Rust没有继承，它和Golang不约而同的选择了trait(Golang叫Interface)作为其实现多态的基础。当然了，我们要想对一个结构体写一些专门的成员函数应该怎么写呢？

答： impl

talk is cheap ,举个栗子：

```
struct Person {
    name: String,
}

impl Person {
    fn new(n: &str) -> Person {
        Person {
            name: n.to_string(),
        }
    }
    
    fn greeting(&self) {
        println!("{} say hello .", self.name);
    }

}

fn main() {

    let peter = Person::new("Peter");
    peter.greeting();
}

```

看见了self，Python程序员不厚道的笑了。

我们来分析一下，上面的`impl`中，new被Person这个结构体自身所调用，其特征是`::`的调用，Java程序员站出来了：类函数！ 而带有`self`的`greeting`，更像是一个成员函数。

恩，回答正确，然而不加分。

Rust允许我们灵活的对一个struct进行你想要的实现，在编程的自由度上无疑有了巨大的提高。

至于更高级的关于trait和泛型的用法，我们将在以后的章节进行详细介绍。

## 枚举类型 enum

同struct一样，Rust的枚举(enum)类型也具有两种不同的实现。一种极具Rust特点的枚举类型

同结构体，以下两种名字也是我瞎掰的：

### 有特点的枚举

```
enum SpecialPoint {
    Point(i32, i32),
    Special(String),
}
```

枚举里面居然能包含一些你需要的，特定的数据信息！

这是常规的枚举所无法做到的，更像枚举类，不是么？

### 有别人特点的枚举

```
enum Direction {
    West,
    North,
    Sourth,
    East,
}

```

至少这种我以前见过，程序员如是说。

### 使用枚举

和struct的成员访问符号`.`不同的是，枚举类型要想访问其成员，几乎无一例外的必须要用到模式匹配。并且， 你可以写一个 Direction::West，但是你绝对不能写成Direction.West。虽然编译器足够聪明能发现你这个粗心的毛病。

关于模式匹配，我不会说太多，还是举个栗子

```
enum SpecialPoint {
    Point(i32, i32),
    Special(String),
}

fn main() {
    let sp = SpecialPoint::Point(0, 0);
    match sp {
        SpecialPoint::Point(x, y) => {
            println!("I'am SpecialPoint(x={}, y={})", x, y);
        }
        SpecialPoint::Special(why) => {
            println!("I'am Special because I am {}", why);
        }
    }
}

```

呐呐呐，这就是模式匹配取值啦。
当然了，`enum`其实也是可以`impl`的，一般人我不告诉他！

## 函数类型 Functions

函数同样的是一个类型，这里只给大家普及一些基本的概念，函数类型涉及到比较高阶的应用，希望大家能在后面的`闭包`章节仔细参读

下面是一个小例子

```
fn foo(x: i32) -> i32 { x+1 }

let x: fn(i32) -> i32 = foo;

assert_eq!(11, x(10));

```
