# 代码风格

## 空白

* 每行不能超出99个字符。
* 缩进用个空格，不用TAB。
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

* 对于多行的函数签名，每一新行一个和第一个参数对齐。允许每行多个参数：

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

* 多行函数调用一般遵循和签名同意的规则。然而，如果最后的参数开始了一个语句块，块的内容可以开始一个新行，缩进一层：

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
// Good
struct Foo {
    short: f64,
    really_long: f64,
}

// Bad
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
它们支持 [Markdown 语法](https://en.wikipedia.org/wiki/Markdown)
而且是注释你的公开API的主要方式。

支持的 markdown 语法包括列在 [GitHub Flavored Markdown]
(https://help.github.com/articles/github-flavored-markdown) 文档中的所有扩展，加上上角标。

### 总结行

任何文档注释中的第一行应该是一行总结代码的单行短句。该行用于在 Rustdoc 输出中的一个简短的总结性描述，所以，让它短比较好。

### 句子结构

所有的文档注释，包括总结行，一个以大写字母开始，以句号、问号，或者感叹号结束。最好使用完整的句子而不是片段。

总结行应该以 [第三人称单数陈述句形式]
(http://en.wikipedia.org/wiki/English_verbs#Third_person_singular_present) 来写。
基本上，这意味着用 "Returns" 而不是 "Return"。

例如：

``` rust
/// Sets up a default runtime configuration, given compiler-supplied arguments.
///
/// This function will block until the entire pool of M:N schedulers has
/// exited. This function also requires a local thread to be available.
///
/// # Arguments
///
/// * `argc` & `argv` - The argument vector. On Unix this information is used
///                     by `os::args`.
/// * `main` - The initial procedure to run inside of the M:N scheduling pool.
///            Once this procedure exits, the scheduling pool will begin to shut
///            down. The entire pool (and this function) will only return once
///            all child threads have finished executing.
///
/// # Return value
///
/// The return value is used as the process return code. 0 on success, 101 on
/// error.
```

### 避免文档内注释

Use inner doc comments _only_ to document crates and file-level modules:

``` rust
//! The core library.
//!
//! The core library is a something something...
```

### 解释上下文

Rust doesn't have special constructors, only functions that return new
instances.  These aren't visible in the automatically generated documentation
for a type, so you should specifically link to them:

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

% Braces, semicolons, and commas [FIXME: needs RFC]

### Opening braces always go on the same line.

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

### `match` arms get braces, except for single-line expressions.

``` rust
match foo {
    bar => baz,
    quux => {
        do_something();
        do_something_else()
    }
}
```

### `return` statements get semicolons.

``` rust
fn foo() {
    do_something();

    if condition() {
        return;
    }

    do_something_else();
}
```

### Trailing commas

> **[FIXME]** We should have a guideline for when to include trailing
> commas in `struct`s, `match`es, function calls, etc.
>
> One possible rule: a trailing comma should be included whenever the
> closing delimiter appears on a separate line:

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

% Naming conventions

### General conventions [RFC #430]

> The guidelines below were approved by [RFC #430](https://github.com/rust-lang/rfcs/pull/430).

In general, Rust tends to use `CamelCase` for "type-level" constructs
(types and traits) and `snake_case` for "value-level" constructs. More
precisely:

| Item | Convention |
| ---- | ---------- |
| Crates | `snake_case` (but prefer single word) |
| Modules | `snake_case` |
| Types | `CamelCase` |
| Traits | `CamelCase` |
| Enum variants | `CamelCase` |
| Functions | `snake_case` |
| Methods | `snake_case` |
| General constructors | `new` or `with_more_details` |
| Conversion constructors | `from_some_other_type` |
| Local variables | `snake_case` |
| Static variables | `SCREAMING_SNAKE_CASE` |
| Constant variables | `SCREAMING_SNAKE_CASE` |
| Type parameters | concise `CamelCase`, usually single uppercase letter: `T` |
| Lifetimes | short, lowercase: `'a` |

<p>
In `CamelCase`, acronyms count as one word: use `Uuid` rather than
`UUID`.  In `snake_case`, acronyms are lower-cased: `is_xid_start`.

In `snake_case` or `SCREAMING_SNAKE_CASE`, a "word" should never
consist of a single letter unless it is the last "word". So, we have
`btree_map` rather than `b_tree_map`, but `PI_2` rather than `PI2`.

### Referring to types in function/method names [RFC 344]

> The guidelines below were approved by [RFC #344](https://github.com/rust-lang/rfcs/pull/344).

Function names often involve type names, the most common example being conversions
like `as_slice`. If the type has a purely textual name (ignoring parameters), it
is straightforward to convert between type conventions and function conventions:

Type name | Text in methods
--------- | ---------------
`String`  | `string`
`Vec<T>`  | `vec`
`YourType`| `your_type`

Types that involve notation follow the convention below. There is some
overlap on these rules; apply the most specific applicable rule:

Type name | Text in methods
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

Names of items within a module should not be prefixed with that module's name:

Prefer

``` rust
mod foo {
    pub struct Error { ... }
}
```

over

``` rust
mod foo {
    pub struct FooError { ... }
}
```

This convention avoids stuttering (like `io::IoError`). Library clients can
rename on import to avoid clashes.

### Getter/setter 方法

> The guidelines below were approved by [RFC #344](https://github.com/rust-lang/rfcs/pull/344).

Some data structures do not wish to provide direct access to their fields, but
instead offer "getter" and "setter" methods for manipulating the field state
(often providing checking or other functionality).

The convention for a field `foo: T` is:

* A method `foo(&self) -> &T` for getting the current value of the field.
* A method `set_foo(&self, val: T)` for setting the field. (The `val` argument
  here may take `&T` or some other type, depending on the context.)

Note that this convention is about getters/setters on ordinary data types, *not*
on [builder objects](../ownership/builders.html).

### Predicates

* Simple boolean predicates should be prefixed with `is_` or another
  short question word, e.g., `is_empty`.
* Common exceptions: `lt`, `gt`, and other established predicate names.

### Imports

The imports of a crate/module should consist of the following
sections, in order, with a blank space between each:

* `extern crate` directives
* external `use` imports
* local `use` imports
* `pub use` imports

For example:

```rust
// Crates.
extern crate getopts;
extern crate mylib;

// Standard library imports.
use getopts::{optopt, getopts};
use std::os;

// Import from a library that we wrote.
use mylib::webserver;

// Will be reexported when we import this module.
pub use self::types::Webdata;
```

### 避免 `use *`，除非在测试里

Glob imports have several downsides:
* They make it harder to tell where names are bound.
* They are forwards-incompatible, since new upstream exports can clash
  with existing names.

When writing a [`test` submodule](../testing/README.md), importing `super::*` is appropriate
as a convenience.

### Prefer fully importing types/traits while module-qualifying functions.

For example:

```rust
use option::Option;
use mem;

let i: isize = mem::transmute(Option(0));
```

### Reexport the most important types at the crate level.

Crates `pub use` the most common types for convenience, so that clients do not
have to remember or write the crate's module hierarchy to use these types.

### Define types and operations together.

Type definitions and the functions/methods that operate on them should be
defined together in a single module, with the type appearing above the
functions/methods.
