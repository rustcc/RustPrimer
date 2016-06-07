# 代码风格

## 空白

* 每行不能超出99个字符。
* 缩进只用空格，不用TAB。
* 行和文件末尾不要有空白。

### 空格

* 二元运算符左右加空格，包括属性里的等号：

``` rust
#[deprecated = "Use `bar` instead."]
fn foo(a: usize, b: usize) -> usize {
    a + b
}
```

* 在分号和逗号后面加空格：

``` rust
fn foo(a: Bar);

MyStruct { foo: 3, bar: 4 }

foo(bar, baz);
```

* 在单行语句块或`struct`表达式的开始大括号之后和结束大括号之前加空格：

``` rust
spawn(proc() { do_something(); })

Point { x: 0.1, y: 0.3 }
```

### 折行

* 对于多行的函数签名，每个新行和第一个参数对齐。允许每行多个参数：

``` rust
fn frobnicate(a: Bar, b: Bar,
              c: Bar, d: Bar)
              -> Bar {
    ...
}

fn foo<T: This,
       U: That>(
       a: Bar,
       b: Bar)
       -> Baz {
    ...
}
```

* 多行函数调用一般遵循和签名统一的规则。然而，如果最后的参数开始了一个语句块，块的内容可以开始一个新行，缩进一层：

``` rust
fn foo_bar(a: Bar, b: Bar,
           c: |Bar|) -> Bar {
    ...
}

// 可以在同一行：
foo_bar(x, y, |z| { z.transpose(y) });

// 也可以在新一行缩进函数体：
foo_bar(x, y, |z| {
    z.quux();
    z.rotate(x)
})
```


### 对齐

常见代码不必在行中用多余的空格来对齐。


``` rust
// 好
struct Foo {
    short: f64,
    really_long: f64,
}

// 坏
struct Bar {
    short:       f64,
    really_long: f64,
}

// 好
let a = 0;
let radius = 7;

// 坏
let b        = 0;
let diameter = 7;
```

### 避免块注释

使用行注释：

``` rust
// 等待主线程返回，并设置过程错误码
// 明显地。
```

而不是：

``` rust
/*
 * 等待主线程返回，并设置过程错误码
 * 明显地。
 */
```

## 文档注释

文档注释前面加三个斜线(`///`)而且提示你希望将注释包含在 Rustdoc 的输出里。
它们支持 [Markdown 语言](https://en.wikipedia.org/wiki/Markdown)
而且是注释你的公开API的主要方式。

支持的 markdown 功能包括列在 [GitHub Flavored Markdown](https://help.github.com/articles/github-flavored-markdown) 文档中的所有扩展，加上上角标。

### 总结行

任何文档注释中的第一行应该是一行总结代码的单行短句。该行用于在 Rustdoc 输出中的一个简短的总结性描述，所以，让它短比较好。

### 句子结构

所有的文档注释，包括总结行，一个以大写字母开始，以句号、问号，或者感叹号结束。最好使用完整的句子而不是片段。

总结行应该以 [第三人称单数陈述句形式](http://en.wikipedia.org/wiki/English_verbs#Third_person_singular_present) 来写。
基本上，这意味着用 "Returns" 而不是 "Return"。

例如：

``` rust
/// 根据编译器提供的参数，设置一个缺省的运行时配置。
///
/// 这个函数将阻塞直到整个 M:N 调度器池退出了。
/// 这个函数也要求一个本地的线程可用。
///
/// # 参数
///
/// * `argc` 和 `argv` - 参数向量。在 Unix 系统上，该信息被`os::args`使用。
///
/// * `main` - 运行在 M:N 调度器池内的初始过程。
///            一旦这个过程退出，调度池将开始关闭。
///            整个池（和这个函数）将只有在所有子线程完成执行后。
///
/// # 返回值
///
/// 返回值被用作进程返回码。成功是 0，101 是错误。
```

### 避免文档内注释

内嵌文档注释 _只用于_ 注释 crates 和文件级的模块：

``` rust
//! 核心库。
//!
//! 核心库是...
```

### 解释上下文

Rust 没有特定的构造器，只有返回新实例的函数。
这些在自动生成的类型文档中是不可见的，因此你应该专门链接到它们：

``` rust
/// An iterator that yields `None` forever after the underlying iterator
/// yields `None` once.
///
/// These can be created through
/// [`iter.fuse()`](trait.Iterator.html#method.fuse).
pub struct Fuse<I> {
    // ...
}
```

### 开始的大括号总是出现的同一行。

``` rust
fn foo() {
    ...
}

fn frobnicate(a: Bar, b: Bar,
              c: Bar, d: Bar)
              -> Bar {
    ...
}

trait Bar {
    fn baz(&self);
}

impl Bar for Baz {
    fn baz(&self) {
        ...
    }
}

frob(|x| {
    x.transpose()
})
```

### `match` 分支有大括号，除非是单行表达式。

``` rust
match foo {
    bar => baz,
    quux => {
        do_something();
        do_something_else()
    }
}
```

### `return` 语句有分号。

``` rust
fn foo() {
    do_something();

    if condition() {
        return;
    }

    do_something_else();
}
```

### 行尾的逗号

```rust
Foo { bar: 0, baz: 1 }

Foo {
    bar: 0,
    baz: 1,
}

match a_thing {
    None => 0,
    Some(x) => 1,
}
```

### 一般命名约定

通常，Rust 倾向于为“类型级”结构(类型和 traits)使用 `CamelCase` 而为“值级”结构使用 `snake_case` 。更确切的约定：

| 条目 | 约定 |
| ---- | ---------- |
| Crates | `snake_case` (但倾向于单个词) |
| Modules | `snake_case` |
| Types | `CamelCase` |
| Traits | `CamelCase` |
| Enum variants | `CamelCase` |
| Functions | `snake_case` |
| Methods | `snake_case` |
| General constructors | `new` 或 `with_more_details` |
| Conversion constructors | `from_some_other_type` |
| Local variables | `snake_case` |
| Static variables | `SCREAMING_SNAKE_CASE` |
| Constant variables | `SCREAMING_SNAKE_CASE` |
| Type parameters | 简洁 `CamelCase`，通常单个大写字母：`T` |
| Lifetimes | 短的小写: `'a` |

<p>
在 `CamelCase`中, 首字母缩略词被当成一个单词：用 `Uuid` 而不是
`UUID`。在 `snake_case` 中，首字母缩略词全部是小写： `is_xid_start`。

在 `snake_case` 或 `SCREAMING_SNAKE_CASE` 中，“单词”永远不应该只包含一个字母，
除非是最后一个“单词”。所以，我们有`btree_map` 而不是 `b_tree_map`，`PI_2` 而不是 `PI2`。

### 引用函数/方法名中的类型

函数名经常涉及类型名，最常见的约定例子像 `as_slice`。如果类型有一个纯粹的文本名字（忽略参数），
在类型约定和函数约定之间转换是直截了当的：

类型名 | 方法中的文本
--------- | ---------------
`String`  | `string`
`Vec<T>`  | `vec`
`YourType`| `your_type`

涉及记号的类型遵循以下约定。这些规则有重叠；应用最适用的规则：

类型名 | 方法中的文本
--------- | ---------------
`&str`    | `str`
`&[T]`    | `slice`
`&mut [T]`| `mut_slice`
`&[u8]`   | `bytes`
`&T`      | `ref`
`&mut T`  | `mut`
`*const T`| `ptr`
`*mut T`  | `mut_ptr`

### 避免冗余的前缀

一个模块中的条目的名字不应拿模块的名字做前缀：

倾向于

``` rust
mod foo {
    pub struct Error { ... }
}
```

而不是

``` rust
mod foo {
    pub struct FooError { ... }
}
```

这个约定避免了口吃（像 `io::IoError`）。库客户端可以在导入时重命名以避免冲突。

### Getter/setter 方法

一些数据类型不希望提供对它们的域的直接访问，但是提供了 "getter" 和 "setter" 方法用于操纵域状态
（经常提供检查或其他功能）。

域 `foo: T` 的约定是：

* 方法 `foo(&self) -> &T` 用于获得该域的当前值。
* 方法 `set_foo(&self, val: T)` 用于设置域。（这里的 `val` 参数可能取 `&T` 或其他类型，取决于上下文。）

请注意，这个约定是关于通常数据类型的 getters/setters， *不是* 关于构建者对象的。

### 断言

* 简单的布尔断言应该加上 `is_` 或者其他的简短问题单词作为前缀，e.g.， `is_empty`。
* 常见的例外： `lt`， `gt`，和其他已经确认的断言名。

### 导入

一个 crate/模块的导入应该按顺序包括下面各个部分，之间以空行分隔：

* `extern crate` 指令
* 外部 `use` 导入
* 本地 `use` 导入
* `pub use` 导入

例如：

```rust
// Crates.
extern crate getopts;
extern crate mylib;

// 标准库导入。
use getopts::{optopt, getopts};
use std::os;

// 从一个我们写的库导入。
use mylib::webserver;

// 当我们导入这个模块时会被重新导出。
pub use self::types::Webdata;
```

### 避免 `use *`，除非在测试里

Glob 导入有几个缺点：
* 更难知道名字在哪里绑定。
* 它们前向不兼容，因为新的上流导出可能与现存的名字冲突。

在写 `test` 子模块时，为方便导入 `super::*` 是合适的。

### 当模块限定函数时，倾向于完全导入类型/traits。

例如：

```rust
use option::Option;
use mem;

let i: isize = mem::transmute(Option(0));
```

### 在 crate 级重新导出最重要的类型。

Crates `pub use` 最常见的类型为方便，因此，客户端不必记住或写 crate 的模块结构以使用这些类型。

### 类型和操作在一起定义。

类型定义和使用它们的函数/模块应该在同一模块中定义，类型出现在函数/模块前面。
