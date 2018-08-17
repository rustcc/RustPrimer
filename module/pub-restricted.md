# pub restricted

## 概览

这是 rust1.18 新增的一个语法。在此之前的版本，`item` 只有 `pub`/non-`pub` 两种分类，pub restricted 这个语法用来扩展 `pub` 的使用，使其能够指定想要的作用域\(可见范围\)，详情参见RFC [1422-pub-restricted.md](https://github.com/rust-lang/rfcs/blob/master/text/1422-pub-restricted.md)。

在 Rust 中 `crate` 是一个模块树，可以通过表达式 `pub(crate) item;` 来限制 `item` 只在当前 `crate` 中可用，在当前 `crate` 的其他子树中，可以通过 `use + path` 的语法来引用 `item`。

## 设计动因

Rust1.18 之前，如果我们想要设计一个 item `x` 可以在多处使用，那么有两种方法：

* 在根目录中定义一个非 `pub` item；
* 在子模块中定义一个 `pub` item，同时通过 `use` 将这个项目引用到根目录。 

但是，有时候这两种方法都并不是我们想要的。在一些情况下，我们希望对于某些特定的模块，该item可见，而其他模块不可见。

下面我们来看一个例子：

```Rust
// Intent: `a` exports `I`, `bar`, and `foo`, but nothing else.
pub mod a {
    pub const I: i32 = 3;

    // `semisecret` will be used "many" places within `a`, but
    // is not meant to be exposed outside of `a`.
    fn semisecret(x: i32) -> i32  { use self::b::c::J; x + J }

    pub fn bar(z: i32) -> i32 { semisecret(I) * z }
    pub fn foo(y: i32) -> i32 { semisecret(I) + y }

    mod b {
        mod c {
            const J: i32 = 4; // J is meant to be hidden from the outside world.
        }
    }
}
```

这段代码编译无法通过，因为 `J` 无法在 `mod c` 的外部访问，而 `fn semisecret` 尝试在 `mod a` 中访问 `J`.

在 rust1.18 之前，保持`J`私有，并能够让 `a` 使用 `fn semisecret` 的正确写法是，将 `fn semisecret` 移动到 `mod c` 中，并将其 `pub`，之后根据需要可以重新导出 `semisecret`。(如果不需要保持 `J` 的私有化，那么可以对其进行 `pub`，之后可以在 `b` 中 `pub use self::c::J` 或者直接 `pub c`)

```Rust
// Intent: `a` exports `I`, `bar`, and `foo`, but nothing else.
pub mod a {
    pub const I: i32 = 3;

    // `semisecret` will be used "many" places within `a`, but
    // is not meant to be exposed outside of `a`.
    // (If we put `pub use` here, then *anyone* could access it.)
    use self::b::semisecret;

    pub fn bar(z: i32) -> i32 { semisecret(I) * z }
    pub fn foo(y: i32) -> i32 { semisecret(I) + y }

    mod b {
        pub use self::c::semisecret;
        mod c {
            const J: i32 = 4; // J is meant to be hidden from the outside world.
            pub fn semisecret(x: i32) -> i32  { x + J }
        }
    }
}
```

这种情况可以正常工作，但是，这里有个严重的问题：没有人能够十分清晰的说明 `pub fn semisecret` 使用到了哪些地方，需要通过上下文进行判断：

1. 所有可以访问 `semisecret` 的模块；
2. 在所有可以访问 `semisecret` 的模块中，是否存在 `semisecret` 的 re-export;

同时，如果在 `a` 中使用 `pub use self::b::semisecret` ，那么所有人都可以通过 `use` 访问 `fn semisecret`，但是实际上，这个函数只需要让 `mod a` 访问就可以了。

## pub restricted 的使用

### Syntax

old:

    VISIBILITY ::= <empty> | `pub`

new:

    VISIBILITY ::= <empty> | `pub` | `pub` `(` USE_PATH `)` | `pub` `(` `crate` `)`

pub\(restriction\) 意味着对 item，method，field等的定义加以可见范围（作用域）的限制。

可见范围（作用域）分为所有 crate \(无限制\)，当前 crate，当前 crate 中的子模块的绝对路径。被限制的东西不能在其限制范围之外直接使用。

* `pub` 无明确指定意味着无限制；
* `pub(crate)` 当前 crate 有效；
* `pub(in <path>)` 在 `<path>` 表示的模块中有效。

### 修改示例

```Rust
// Intent: `a` exports `I`, `bar`, and `foo`, but nothing else.
pub mod a {
    pub const I: i32 = 3;

    // `semisecret` will be used "many" places within `a`, but
    // is not meant to be exposed outside of `a`.
    // (`pub use` would be *rejected*; see Note 1 below)
    use self::b::semisecret;

    pub fn bar(z: i32) -> i32 { semisecret(I) * z }
    pub fn foo(y: i32) -> i32 { semisecret(I) + y }

    mod b {
        pub(in a) use self::c::semisecret;
        mod c {
            const J: i32 = 4; // J is meant to be hidden from the outside world.

            // `pub(in a)` means "usable within hierarchy of `mod a`, but not
            // elsewhere."
            pub(in a) fn semisecret(x: i32) -> i32  { x + J }
        }
    }
}
```

Note 1: 如果改成下面这种方式，编译器会报错:

```Rust
pub mod a { [...] pub use self::b::semisecret; [...] }
```

因为 `pub(in a) fn semisecret` 说明这个函数只能在 `a` 中使用，不允许 `pub` 出 `a` 的范围。

### 限制字段示例

```Rust
mod a {
    #[derive(Default)]
    struct Priv(i32);

    pub mod b {
        use a::Priv as Priv_a;

        #[derive(Default)]
        pub struct F {
            pub    x: i32,
                   y: Priv_a,
            pub(in a) z: Priv_a,
        }

        #[derive(Default)]
        pub struct G(pub i32, Priv_a, pub(in a) Priv_a);

        // ... accesses to F.{x,y,z} ...
        // ... accesses to G.{0,1,2} ...
    }
    // ... accesses to F.{x,z} ...
    // ... accesses to G.{0,2} ...
}

mod k {
    use a::b::{F, G};
    // ... accesses to F and F.x ...
    // ... accesses to G and G.0 ...
}
```

### Crate 限制示例

Crate `c1`:

```Rust
pub mod a {
    struct Priv(i32);

    pub(crate) struct R { pub y: i32, z: Priv } // ok: field allowed to be more public
    pub        struct S { pub y: i32, z: Priv }

    pub fn to_r_bad(s: S) -> R { ... } //~ ERROR: `R` restricted solely to this crate

    pub(crate) fn to_r(s: S) -> R { R { y: s.y, z: s.z } } // ok: restricted to crate
}

use a::{R, S}; // ok: `a::R` and `a::S` are both visible

pub use a::R as ReexportAttempt; //~ ERROR: `a::R` restricted solely to this crate
```

Crate `c2`:

```Rust
extern crate c1;

use c1::a::S; // ok: `S` is unrestricted

use c1::a::R; //~ ERROR: `c1::a::R` not visible outside of its crate
```
