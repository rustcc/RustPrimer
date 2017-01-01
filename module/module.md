
# 包和模块

## 包（crate）

Rust 中，`crate` 是一个独立的可编译单元。具体说来，就是一个或一批文件（如果是一批文件，那么有一个文件是这个 crate 的入口）。它编译后，会对应着生成一个可执行文件或一个库。

执行 `cargo new foo`，会得到如下目录层级：

```
foo
├── Cargo.toml
└── src
    └── lib.rs
```

这里，`lib.rs` 就是一个 crate（入口），它编译后是一个库。一个工程下可以包含不止一个 crate，本工程只有一个。

执行 `cargo new --bin bar`，会得到如下目录层级：

```
bar
├── Cargo.toml
└── src
    └── main.rs
```

这里，`main.rs` 就是一个 crate（入口），它编译后是一个可执行文件。


## 模块（module）

Rust 提供了一个关键字 `mod`，它可以在一个文件中定义一个模块，或者引用另外一个文件中的模块。

关于模块的一些要点：

1. 每个 crate 中，默认实现了一个隐式的 `根模块（root module）`；
2. 模块的命名风格也是 `lower_snake_case`，跟其它的 Rust 的标识符一样；
3. 模块可以嵌套；
4. 模块中可以写任何合法的 Rust 代码；

### 在文件中定义一个模块

比如，在上述 `lib.rs` 中，我们写上如下代码：

```rust
mod aaa {
    const X: i32 = 10;

    fn print_aaa() {
        println!("{}", 42);
    }
}
```

我们可以继续写如下代码：

```rust
mod aaa {
    const X: i32 = 10;

    fn print_aaa() {
        println!("{}", 42);
    }

    mod BBB {
        fn print_bbb() {
            println!("{}", 37);
        }
    }
}
```

还可以继续写：

```rust
mod aaa {
    const X: i32 = 10;

    fn print_aaa() {
        println!("{}", 42);
    }

    mod bbb {
        fn print_bbb() {
            println!("{}", 37);
        }
    }
}

mod ccc {
    fn print_ccc() {
        println!("{}", 25);
    }

}

```

### 模块的可见性

我们前面写了一些模块，但实际上，我们写那些模块，目前是没有什么作用的。写模块的目的一是为了分隔逻辑块，二是为了提供适当的函数，或对象，供外部访问。而模块中的内容，默认是私有的，只有模块内部能访问。

为了让外部能使用模块中 item，需要使用 `pub` 关键字。外部引用的时候，使用 `use` 关键字。例如：

```rust
mod ccc {
    pub fn print_ccc() {
        println!("{}", 25);
    }
}

fn main() {
    use ccc::print_ccc;

    print_ccc();
    // 或者
    ccc::print_ccc();
}
```

规则很简单，一个 item（函数，绑定，Trait 等），前面加了 `pub`，那么就它变成对外可见（访问，调用）的了。


### 引用外部文件模块

通常，我们会在单独的文件中写模块内容，然后使用 `mod` 关键字来加载那个文件作为我们的模块。

比如，我们在 `src` 下新建了文件 `aaa.rs`。现在目录结构是下面这样子：

```
foo
├── Cargo.toml
└── src
    └── aaa.rs
    └── main.rs
```

我们在 `aaa.rs` 中，写上：

```rust
pub fn print_aaa() {
    println!("{}", 25);
}
```

在 `main.rs` 中，写上：

```rust
mod aaa;

use aaa::print_aaa;

fn main () {
    print_aaa();
}
```

编译后，生成一个可执行文件。

细心的朋友会发现，`aaa.rs` 中，没有使用 `mod xxx {}` 这样包裹起来，是因为 `mod xxx;` 相当于把 `xxx.rs` 文件用 `mod xxx {}` 包裹起来了。初学者往往会多加一层，请注意。


### 多文件模块的层级关系

Rust 的模块支持层级结构，但这种层级结构本身与文件系统目录的层级结构是解耦的。

`mod xxx;` 这个 `xxx` 不能包含 `::` 号。也即在这个表达形式中，是没法引用多层结构下的模块的。也即，你不可能直接使用 `mod a::b::c::d;` 的形式来引用 `a/b/c/d.rs` 这个模块。

那么，Rust 的多层模块遵循如下两条规则：

1. `mod xxx;` 默认优先查找，同级目录下的 `xxx.rs` 文件；
2. 如果 `xxx.rs` 不存在，则查找 `xxx/mod.rs` 文件，即 `xxx` 目录下的 `mod.rs` 文件。

上述两种情况，加载成模块后，效果是相同的。Rust 就凭这两条规则，通过迭代使用，结合 `pub` 关键字，实现了对深层目录下模块的加载；

下面举个例子，现在我们建了一个测试工程，目录结构如下：

```
src
├── a
│   ├── b
│   │   ├── c
│   │   │   ├── d.rs
│   │   │   └── mod.rs
│   │   └── mod.rs
│   └── mod.rs
└── main.rs

```

`a/b/c/d.rs` 文件内容：

```rust
pub fn print_ddd() {
    println!("i am ddd.");
}
```

`a/b/c/mod.rs` 文件内容：

```rust
pub mod d;
```

`a/b/mod.rs` 文件内容：

```rust
pub mod c;
```

`a/mod.rs` 文件内容：

```rust
pub mod b;
```

`main.rs` 文件内容：

```rust
mod a;

use a::b::c::d;

fn main() {
    d::print_ddd();
}

```
输出结果为：`i am ddd.`

仔细理解本例子，就明白 Rust 的层级结构模块的用法了。

至于为何 Rust 要这样设计，有几下几个原因：

1. Rust 本身模块的设计是与操作系统文件系统目录解耦的，因为 Rust 本身可用于操作系统的开发；
2. Rust 中的一个文件内，可包含多个模块，直接将 `a::b::c::d` 映射到 `a/b/c/d.rs` 会引起一些歧义；
3. Rust 一切从安全性、显式化立场出发，要求引用路径中的每一个节点，都是一个有效的模块，比如上例，`d` 是一个有效的模块的话，那么，要求 `c, b, a` 分别都是有效的模块，可单独引用。


### 路径

前面我们提到，一个 crate 是一个独立的可编译单元。它有一个入口文件，这个入口文件是这个 crate（里面可能包含若干个 module）的模块根路径。整个模块的引用，形成一个链，每个模块，都可以用一个精确的路径（比如：`a::b::c::d`）来表示；

与文件系统概念类似，模块路径也有相对路径和绝对路径的概念。为此，Rust 提供了 `self` 和 `super` 两个关键字。

`self` 在路径中，有两种意思：

1. `use self::xxx` 表示，加载当前模块中的 `xxx`。此时 self 可省略；
2. `use xxx::{self, yyy}`，表示，加载当前路径下模块 `xxx` 本身，以及模块 `xxx` 下的 `yyy`；

`super` 表示，当前模块路径的上一级路径，可以理解成父模块。
```rust
use super::xxx;
```
表示引用父模块中的 `xxx`。

另外，还有一种特殊的路径形式：
```rust
::xxx::yyy
```
它表示，引用根路径下的 `xxx::yyy`，这个根路径，指的是当前 crate 的根路径。

路径中的 `*` 符号：
```rust
use xxx::*;
```
表示导入 `xxx` 模块下的所有可见 item（加了 pub 标识的 item）。

### Re-exporting

我们可以结合使用 `pub use` 来实现 `Re-exporting`。`Re-exporting` 的字面意思就是 `重新导出`。它的意思是这样的，把深层的 item 导出到上层目录中，使调用的时候，更方便。接口设计中会大量用到这个技术。

还是举上面那个 `a::b::c::d` 的例子。我们在 `main.rs` 中，要调用 `d`，得使用 `use a::b::c::d;` 来调用。而如果我们修改 `a/mod.rs` 文件为：
`a/mod.rs` 文件内容：

```rust
pub mod b;
pub use b::c::d;
```

那么，我们在 `main.rs` 中，就可以使用 `use a::d;` 来调用了。从这个例子来看没觉得方便多少。但是如果开发的一个库中有大量的内容，而且是在不同层次的模块中。那么，通过统一导出到一个地方，就能大大方便接口使用者。

### 加载外部 crate

前面我们讲的，都是在当前 crate 中的技术。真正我们在开发时，会大量用到外部库。外部库是通过

```rust
extern crate xxx;
```

这样来引入的。

注：要使上述引用生效，还必须在 `Cargo.toml` 的 `dependecies` 段，加上 `xxx="version num"` 这种依赖说明，详情见 `Cargo 项目管理` 这一章。

引入后，就相当于引入了一个符号 `xxx`，后面可以直接以这个 `xxx` 为根引用这个 crate 中的 item：

```rust
extern crate xxx;

use xxx::yyy::zzz;
```

引入的时候，可以通过 `as` 关键字重命名。

```rust
extern crate xxx as foo;

use foo::yyy::zzz;
```
