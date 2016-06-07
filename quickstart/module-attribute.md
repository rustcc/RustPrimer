# 模块与属性

Rust有两个与模块 (module) 系统相关的独特术语：`crate`和`module`，
其中包装箱 (crate) 与其它语言中的 libary 或者 package 作用一样。
每个包装箱都有一个隐藏的根模块，在根模块下可以定义一个子模块树，
其路径采用`::`作为分隔符。包装箱由条目 (item) 构成，多个条目通过模块组织在一起。

## 定义模块

使用`mod`关键字定义我们的模块：

```rust
// in src/lib.rs

mod chinese {
    mod greetings {

    }

    mod farewells {

    }
}

mod english {
    mod greetings {

    }

    mod farewells {

    }
}
```
定义了四个子模块`chinese::{greetings, farewells}`和`english::{greetings, farewells}`。
模块默认是私有的，可以使用`pub`关键字将其设置成公开，只有公开的条目才允许在模块外部访问。

实践中更好的组织方式是将一个包装箱分拆到多个文件：

```rust
// in src/lib.rs

pub mod chinese;

pub mod english;
```
这两句声明告诉Rust查看`src/chinese.rs`和`src/english.rs`，
或者`src/chinese/mod.rs`和`src/english/mod.rs`。
先添加一些函数：

```rust
// in src/chinese/greetings.rs

pub fn hello() -> String {
    "你好！".to_string()
}
```

```rust
// in src/chinese/farewells.rs

pub fn goodbye() -> String {
    "再见！".to_string()
}
```

```rust
// in src/english/greetings.rs

pub fn hello() -> String {
    "Hello!".to_string()
}
```

```rust
// in src/english/farewells.rs

pub fn goodbye() -> String {
    "Goodbye!".to_string()
}
```
函数默认也是私有的，为了后面的使用我们需要`pub`关键字使其成为公有。

## 导入 crate

为了使用我们前面创建的名为`phrases`的包装箱，需要先声明导入

```rust
// in src/main.rs

extern crate phrases;

fn main() {
    println!("Hello in Chinese: {}", phrases::chinese::greetings::hello());
}
```

Rust还有一个`use`关键字，允许我们导入包装箱中的条目到当前的作用域内：

```rust
// in src/main.rs

extern crate phrases;

use phrases::chinese::greetings;
use phrases::chinese::farewells::goodbye;

fn main() {
    println!("Hello in Chinese: {}", greetings::hello());
    println!("Goodbye in Chinese: {}", goodbye());
}
```
但是，我们不推荐直接导入函数，这样更容易导致命名空间冲突，只导入模块是更好的做法。
如果要导入来自同一模块的多个条目，可以使用大括号简写：

```rust
use phrases::chinese::{greetings, farewells};
```
如果是导入全部，可以使用通配符`*`。重命名可以使用`as`关键字：

```rust
use phrases::chinese::greetings as chinese_greetings;
```

有时我们需要将外部包装箱里面的函数导入到另一个模块内，
这时可以使用`pub use`来提供扩展接口而不映射代码层级结构。
比如

```rust
// in src/english/mod.rs

pub use self::greetings::hello;
pub use self::farewells::goodbye;

mod greetings;

mod farewells;
```
其中`pub use`声明将函数带入了当前模块中，
使得我们现在有了`phrases::english::hello()`函数和`phrases::english::goodbye()`函数，
即使它们的定义位于`phrases::english::greetings::hello()`
和`phrases::english::farewells::goodbye()`中，
内部代码的组织结构不能反映我们的扩展接口。

默认情况下，`use`声明表示从根包装箱开始的绝对路径。
此外，我们可以使用`use self::`表示相对于当前模块的位置，
`use super::`表示当前位置的上一级，以`::`为前缀的路径表示根包装箱路径。

```rust
use foo::baz::foobaz; // foo is at the root of the crate

mod foo {
    use foo::bar::foobar; // foo is at crate root
    use self::baz::foobaz; // self refers to module 'foo'

    pub mod bar {
        pub fn foobar() { }
    }

    pub mod baz {
        use super::bar::foobar; // super refers to module 'foo'
        pub fn foobaz() { }
    }
}
```

## 属性

在Rust中，属性 (attribute) 是应用于包装箱、模块或者条目的元数据 (metadata)，
主要用于：

* 实现条件编译 (conditional compilation)
* 设置包装箱名字、版本以及类型
* 取消可疑代码的警告
* 设置编译器选项
* 链接外部库
* 标记测试函数

属性有两种用法：`#![crate_attribute]`应用于整个包装箱，
而`#[crate_attribute]`应用于紧邻的一个模块或者条目。
属性的参数也有三种不同的形式：

* `#[attribute = "value"]`
* `#[attribute(key = "value")]`
* `#[attribute(value)]`

下面列举几个经常用到的属性：

* `#[path="foo.rs"]`用于设置一个模块需要载入的文件路径。
* `#[allow(dead_code)]`用于取消对死代码的默认lint检查。
* `#[derive(PartialEq, Clone)]`用于自动推导`PartialEq`和`Clone`这两个特性的实现。

