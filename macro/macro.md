# Macro

## 简介

学过 C 语言的人都知道 `#define` 用来定义宏(macro)，而且大学很多老师都告诉你尽量少用宏，因为 C 里面的宏是一个很危险的东西-宏仅仅是简单的文本替换，完全不管语法，类型，非常容易出错。听说过或用过 Lisp 的人觉得宏极其强大，就连美国最大的创业孵化器公司创始人 Paul Gram 也极力鼓吹 Lisp 的宏是有多么强大。那么宏究竟是什么样的东西呢？这一章通过 Rust 的宏系统带你揭开宏(Macro)的神秘面纱。

Rust 中的宏几乎无处不在，其实你写的第一个 Rust 程序里面就已经用到了宏，对，就是那个有名的 hello-world。`println!("Hello, world!")` 这句看起来很像函数调用，但是在"函数名"后面加上了感叹号，这个是专门用来区分普通函数调用和宏调用的。另外从形式上看，与函数调用的另一个区别是参数可以用圆括号(`()`)、花括号(`{}`)、方括号(`[]`)中的任意一种括起来，比如这行也可以写成 `println!["Hello, world!"]` 或 `println!{"Hello, world!"}`，不过对于 Rust 内置的宏都有约定俗成的括号，比如 `vec!` 用方括号，`assert_eq!` 用圆括号。

既然宏看起来与普通函数非常像，那么使用宏有什么好处呢？是否可以用函数取代宏呢？答案显然是否定的，首先 Rust 的函数不能接受任意多个参数，其次函数是不能操作语法单元的，即把语法元素作为参数进行操作，从而生成代码，例如 `mod`, `crate` 这些是 Rust 内置的关键词，是不可能直接用函数去操作这些的，而宏就有这个能力。

相比函数，宏是用来生成代码的，在调用宏的地方，编译器会先将宏进行展开，生成代码，然后再编译展开后的代码。

宏定义格式是： `macro_rules! macro_name { macro_body }`，其中 `macro_body` 与模式匹配很像， `pattern => do_something` ，所以 Rust 的宏又称为 Macro by example (基于例子的宏)。其中 `pattern` 和 `do_something` 都是用配对的括号括起来的，括号可以是圆括号、方括号、花括号中的任意一种。匹配可以有多个分支，每个分支以分号结束。

还是先来个简单的例子说明

```rust
macro_rules! create_function {
    ($func_name:ident) => (
        fn $func_name() {
            println!("function {:?} is called", stringify!($func_name))
        }
    )
}

fn main() {
    create_function!(foo);
	foo();
}

```

上面这个简单的例子是用来创建函数，生成的函数可以像普通函数一样调用，这个函数可以打印自己的名字。编译器在看到 `create_function!(foo)` 时会从前面去找一个叫 `create_function` 的宏定义，找到之后，就会尝试将参数 `foo` 代入 `macro_body`，对每一条模式按顺序进行匹配，只要有一个匹配上，就会将 `=>` 左边定义的参数代入右边进行替换，如果替换不成功，编译器就会报错而不会往下继续匹配，替换成功就会将右边替换后的代码放在宏调用的地方。这个例子中只有一个模式，即 `$func_name:ident`，表示匹配一个标识符，如果匹配上就把这个标识符赋值给 `$func_name`，宏定义里面的变量都是以 `$` 开头的，相应的类型也是以冒号分隔说明，这里 `ident` 是变量 `$func_name` 的类型，表示这个变量是一个 `identifier`，这是语法层面的类型(designator)，而普通的类型如 `char, &str, i32, f64` 这些是语义层面的类型。在 `main` 函数中传给宏调用 `create_function` 的参数 `foo` 正好是一个标识符(`ident`)，所以能匹配上，`$func_name` 就等于 `foo`，然后把 `$func_name` 的值代入 `=>` 右边，成了下面这样的

```rust
fn foo() {
    println!("function {:?} is called", stringify!(foo))
}
```

所以最后编译器编译的实际代码是

```rust
fn main() {
    fn foo() {
	    println!("function {:?} is called", stringify!(foo))
	}
	foo();
}
```

上面定义了 `create_function` 这个宏之后，就可以随便用来生成函数了，比如调用 `create_function!(bar)` 就得到了一个名为 `bar` 的函数

通过上面这个例子，大家对宏应该有一个大致的了解了。下面就具体谈谈宏的各个组成部分。



## 宏的结构

### 宏名

宏名字的解析与函数略微有些不同，宏的定义必须出现在宏调用之前，即与 C 里面的函数类似--函数定义或声明必须在函数调用之前，只不过 Rust 宏没有单纯的声明，所以宏在调用之前需要先定义，而 Rust 函数则可以定义在函数调用后面。宏调用与宏定义顺序相关性包括从其它模块中引入的宏，所以引入其它模块中的宏时要特别小心，这个稍后会详细讨论。

下面这个例子宏定义在宏调用后面，编译器会报错说找不到宏定义，而函数则没问题

```rust
fn main() {
    let a = 42;
    foo(a);
	bar!(a);
}

fn foo(x: i32) {
	println!("The argument you passed to function is {}", x);
}

macro_rules! bar {
	($x:ident) => { println!("The argument you passed to macro is {}", $x); }
}
```

上面例子中把宏定义挪到 `main` 函数之前或者 `main` 函数里面 `bar!(a)` 调用上面，就可以正常编译运行。

宏调用虽然与函数调用很像，但是宏的名字与函数名字是处于不同命名空间的，之所以提出来是因为在有些编程语言里面宏和函数是在同一个命名空间之下的。看过下面的例子就会明白

```rust
fn foo(x: i32) -> i32 {
    x * x
}

macro_rules! foo {
    ($x:ident) => { println!("{:?}", $x); }
}
fn main() {
    let a = 5;
	foo!(a);
    println!("{}", foo(a));
}
```

### 指示符(designator)

宏里面的变量都是以 `$` 开头的，其余的都是按字面去匹配，以 `$` 开头的变量都是用来表示语法(syntactic)元素，为了限定匹配什么类型的语法元素，需要用指示符(designator)加以限定，就跟普通的变量绑定一样用冒号将变量和类型分开，当前宏支持以下几种指示符：

* ident: 标识符，用来表示函数或变量名
* expr: 表达式
* block: 代码块，用花括号包起来的多个语句
* pat: 模式，普通模式匹配（非宏本身的模式）中的模式，例如 `Some(t)`, `(3, 'a', _)`
* path: 路径，注意这里不是操作系统中的文件路径，而是用双冒号分隔的限定名(qualified name)，如 `std::cmp::PartialOrd`
* tt: 单个语法树
* ty: 类型，语义层面的类型，如 `i32`, `char`
* item: 条目，
* meta: 元条目
* stmt: 单条语句，如 `let a = 42;`

加上这些类型限定后，宏在进行匹配时才不会漫无目的的乱匹配，例如在要求标识符的地方是不允许出现表达式的，否则编译器就会报错。而 C/C++ 语言中的宏则仅仅是简单的文本替换，没有语法层面的考虑，所以非常容易出错。

### 重复(repetition)

宏相比函数一个很大的不同是宏可以接受任意多个参数，例如 `println!` 和 `vec!`。这是怎么做到的呢？

没错，就是重复(repetition)。模式的重复不是通过程序里面的循环(for/while)去控制的，而是指定了两个特殊符号 `+` 和 `*`，类似于正则表达式，因为正则表达式也是不关心具体匹配对象是一个人名还是一个国家名。与正则表达式一样， `+` 表示一次或多次（至少一次），而 `*` 表示零次或多次。重复的模式需要用括号括起来，外面再加上 `$`，例如 `$(...)*`, `$(...)+`。需要说明的是这里的括号和宏里面其它地方一样都可以是三种括号中的任意一种，因为括号在这里仅仅是用来标记一个模式的开始和结束，大部分情况重复的模式是用逗号或分号分隔的，所以你会经常看到 `$(...),*`, `$(...);*`, `$(...),+`, `$(...);+` 这样的用来表示重复。

还是来看一个例子

```rust
macro_rules! vector {
	($($x:expr),*) => {
		{
			let mut temp_vec = Vec::new();
			$(temp_vec.push($x);)*
			temp_vec
		}
	};
}

fn main() {
	let a = vector![1, 2, 4, 8];
	println!("{:?}", a);
}
```

这个例子初看起来比较复杂，我们来分析一下。

首先看 `=>` 左边，最外层是圆括号，前面说过这个括号可以是圆括号、方括号、花括号中的任意一种，只要是配对的就行。然后再看括号里面 `$(...),*` 正是刚才提到的重复模式，重复的模式是用逗号分隔的，重复的内容是 `$x:expr`，即可以匹配零次或多次用逗号分隔的表达式，例如 `vector![]` 和 `vector![3, x*x, s-t]` 都可以匹配成功。

接着看 `=>` 右边，最外层也是一个括号，末尾是分号表示这个分支结束。里面是花括号包起来的代码块，最后一行没有分号，说明这个 macro 的值是一个表达式，`temp_vec` 作为表达式的值返回。第一条语句就是普通的用 `Vec::new()` 生成一个空 vector，然后绑定到可变的变量 `temp_vec` 上面，第二句比较特殊，跟 `=>` 左边差不多，也是用来表示重复的模式，而且是跟左边是一一对应的，即左边匹配到一个表达式(`expr`)，这里就会将匹配到的表达式用在 `temp_vec.push($x);` 里面，所以 `vector![3, x*x, s-t]` 调用就会展开成

```rust
{
	let mut temp_vec = Vec::new();
	temp_vec.push(3);
	temp_vec.push(x*x);
	temp_vec.push(s-t);
	temp_vec
}
```

看着很复杂的宏，细细分析下来是不是很简单，不要被这些符号干扰了

### 递归(recursion)

除了重复之外，宏还支持递归，即在宏定义时调用其自身，类似于递归函数。因为rust的宏本身是一种模式匹配，而模式匹配里面包含递归则是函数式语言里面最常见的写法了，有函数式编程经验的对这个应该很熟悉。下面看一个简单的例子：

```rust
macro_rules! find_min {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => (
        std::cmp::min($x, find_min!($($y),+))
    )
}

fn main() {
    println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32 + 2 , 2u32));
    println!("{}", find_min!(5u32, 2u32 * 3, 4u32));
}
```

因为模式匹配是按分支顺序匹配的，一旦匹配成功就不会再往下进行匹配（即使后面也能匹配上），所以模式匹配中的递归都是在第一个分支里写最简单情况，越往下包含的情况越多。这里也是一样，第一个分支 `($x:expr)` 只匹配一个表达式，第二个分支匹配两个或两个以上表达式，注意加号表示匹配一个或多个，然后里面是用标准库中的 `min` 比较两个数的大小，第一个表达式和剩余表达式中最小的一个，其中剩余表达式中最小的一个是递归调用 `find_min!` 宏，与递归函数一样，每次递归都是从上往下匹配，只到匹配到基本情况。我们来写写 `find_min!(5u32, 2u32 * 3, 4u32)` 宏展开过程

1. `std::cmp::min(5u32, find_min!(2u32 * 3, 4u32))`
2. `std::cmp::min(5u32, std::cmp::min(2u32 * 3, find_min!(4u32)))`
3. `std::cmp::min(5u32, std::cmp::min(2u32 * 3, 4u32))`

分析起来与递归函数一样，也比较简单。

### 卫生(hygienic Macro)

有了重复和递归，组合起来就是一个很强大的武器，可以解决很多普通函数无法抽象的东西。但是这里面会有一个安全问题，也是 C/C++ 里面宏最容易出错的地方，不过 Rust 像 Scheme 一样引入了卫生(Hygiene)宏，有效地避免了这类问题的发生。

C/C++ 里面的宏仅仅是简单的文本替换，下面的 C 经过宏预处理后，宏外面定义的变量 `a` 就会与里面定义的混在一起，从而按作用域 shadow 外层的定义，这会导致一些非常诡异的问题，不去看宏具体定义仔细分析的话，很难发现这类 bug。这样的宏是不卫生的，不过也有些奇葩的 Hacker 觉得这是一个非常棒的特性，例如 CommanLisp 语言里面的宏本身很强大，但不是卫生的，而某些 Hacker 还以这个为傲，搞一些奇技淫巧故意制造出这样的 shadow 行为实现一些很 fancy 的效果。这里不做过多评论，对 C 比较熟悉的同学可以分析一下下面这段代码运行结果与第一印象是否一样。

```c
#define INCI(i) {int a=0; ++i;}
int main(void)
{
    int a = 0, b = 0;
    INCI(a);
    INCI(b);
    printf("a is now %d, b is now %d\n", a, b);
    return 0;
}
```

卫生宏最开始是由 Scheme 语言引入的，后来好多语言基本都采用卫生宏，即编译器或运行时会保证宏里面定义的变量或函数不会与外面的冲突，在宏里面以普通方式定义的变量作用域不会跑到宏外面。

```rust
macro_rules! foo {
    () => (let x = 3);
}

macro_rules! bar {
    ($v:ident) => (let $v = 3);
}

fn main() {
    foo!();
    println!("{}", x);
	bar!(a);
	println!("{}", a);
}
```

上面代码中宏 `foo!` 里面的变量 `x` 是按普通方式定义的，所以其作用域限定在宏里面，宏调用结束后再引用 `x` 编译器就会报错。要想让宏里面定义的变量在宏调用结束后仍然有效，需要按 `bar!` 里面那样定义。不过对于 `item` 规则就有些不同，例如函数在宏里面以普通方式定义后，宏调用之后，这个函数依然可用，下面代码就可以正常编译。

```rust
macro_rules! foo {
    () => (fn x() { });
}

fn main() {
    foo!();
    x();
}
```

## 导入导出(import/export)

前面提到宏名是按顺序解析的，所以从其它模块中导入宏时与导入函数、trait 的方式不太一样，宏导入导出用 `#[macro_use]` 和 `#[macro_export]`。父模块中定义的宏对其下的子模块是可见的，要想子模块中定义的宏在其后面的父模块中可用，需要使用 `#[macro_use]`。

```rust
macro_rules! m1 { () => (()) }

// 宏 m1 在这里可用

mod foo {
    // 宏 m1 在这里可用

    #[macro_export]
    macro_rules! m2 { () => (()) }

    // 宏 m1 和 m2 在这里可用
}

// 宏 m1 在这里可用
#[macro_export]
macro_rules! m3 { () => (()) }

// 宏 m1 和 m3 在这里可用

#[macro_use]
mod bar {
    // 宏 m1 和 m3 在这里可用

    macro_rules! m4 { () => (()) }

    // 宏 m1, m3, m4 在这里均可用
}

// 宏 m1, m3, m4 均可用
```

crate 之间只有被标为 `#[macro_export]` 的宏可以被其它 crate 导入。假设上面例子是 `foo` crate 中的部分代码，则只有 `m2` 和 `m3` 可以被其它 crate 导入。导入方式是在 `extern crate foo;` 前面加上 `#[macro_use]`

```rust
#[macro_use]
extern crate foo;
// foo 中 m2, m3 都被导入
```

如果只想导入 `foo` crate 中某个宏，比如 `m3`，就给 `#[macro_use]` 加上参数
```rust
#[macro_use(m3)]
extern crate foo;
// foo 中只有 m3 被导入
```

## 调试

虽然宏功能很强大，但是调试起来要比普通代码困难，因为编译器默认情况下给出的提示都是对宏展开之后的，而不是你写的原程序，要想在编译器错误与原程序之间建立联系比较困难，因为这要求你大脑能够人肉编译展开宏代码。不过还好编译器为我们提供了 `--pretty=expanded` 选项，能让我们看到展开后的代码，通过这个展开后的代码，往上靠就与你自己写的原程序有个直接对应关系，往下靠与编译器给出的错误也是直接对应关系。

目前将宏展开需要使用 unstable option，通过 `rustc -Z unstable-options --pretty=expanded hello.rs` 可以查看宏展开后的代码，如果是使用的 cargo 则通过 `cargo rustc -- -Z unstable-options --pretty=expanded` 将项目里面的宏都展开。不过目前是没法只展开部分宏的，而且由于 hygiene 的原因，会对宏里面的名字做些特殊的处理(mangle)，所以程序里面的宏全部展开后代码的可读性比较差，不过依然比依靠大脑展开靠谱。

下面可以看看最简单的 hello-word 程序里面的 `println!("Hello, world!")` 展开结果，为了 hygiene 这里内部临时变量用了 `__STATIC_FMTSTR` 这样的名字以避免名字冲突，即使这简单的一句展开后看起来也还是不那么直观的，具体这里就不详细分析了。

```
$ rustc -Z unstable-options --pretty expanded hello.rs
#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
fn main() {
    ::std::io::_print(::std::fmt::Arguments::new_v1({
                                                        static __STATIC_FMTSTR:
                                                               &'static [&'static str]
                                                               =
                                                            &["Hello, world!\n"];
                                                        __STATIC_FMTSTR
                                                    },
                                                    &match () { () => [], }));
}
```
