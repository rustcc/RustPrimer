# 测试

> 程序测试是一种找到缺陷的有效方式，但是它对证明没有缺陷却无能为力。
>
>    Edsger W. Dijkstra, "The Humble Programmer" (1972)

作为软件工程质量保障体系的重要一环，测试是应该引起我们充分注意并重视的事情。前面说过，Rust 语言的设计集成了最近十多年中总结出来的大量最佳工程实践，而对测试的原生集成也正体现了这一点。下面来看 Rust 是怎么设计测试特性的。

Rust 的测试特性按精细度划分，分为 3 个层次：

1. 函数级；
2. 模块级；
3. 工程级；

另外，Rust 还支持对文档进行测试。

## 函数级测试

在本章中，我们用创建一个库的实操来讲解测试的内容。我们先用 cargo 建立一个库工程：`adder`

```
$ cargo new adder
$ cd adder
```

### `#[test]` 标识
打开 `src/lib,rs` 文件，可以看到如下代码

```rust
#[test]
fn it_works() {
    // do test work
}
```

Rust 中，只需要在一个函数的上面，加上 `#[test]` 就标明这是一个测试用的函数。

有了这个属性之后，在使用 `cargo build` 编译时，就会忽略这些函数。使用 `cargo test` 可以运行这些函数。类似于如下效果：

```
$ cargo test
   Compiling adder v0.0.1 (file:///home/you/projects/adder)
     Running target/adder-91b3e234d4ed382a

running 1 test
test it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

Rust 提供了两个宏来执行测试断言：

```rust
assert!(expr)               测试表达式是否为 true 或 false
assert_eq!(expr, expr)      测试两个表达式的结果是否相等
```
比如

```rust
#[test]
fn it_works() {
    assert!(false);
}
```

运行 `cargo test`，你会得到类似下面这样的提示

```
$ cargo test
   Compiling adder v0.0.1 (file:///home/you/projects/adder)
     Running target/adder-91b3e234d4ed382a

running 1 test
test it_works ... FAILED

failures:

---- it_works stdout ----
        thread 'it_works' panicked at 'assertion failed: false', /home/steve/tmp/adder/src/lib.rs:3



failures:
    it_works

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured

thread '<main>' panicked at 'Some tests failed', /home/steve/src/rust/src/libtest/lib.rs:247
```

### `#[should_panic]` 标识

如果你的测试函数没完成，或没有更新，或是故意让它崩溃，但为了让测试能够顺利完成，我们主动可以给测试函数加上 `#[should_panic]` 标识，就不会让 `cargo test` 报错了。

如

```rust
#[test]
#[should_panic]
fn it_works() {
    assert!(false);
}
```

运行 `cargo test`，结果类似如下：

```
$ cargo test
   Compiling adder v0.0.1 (file:///home/you/projects/adder)
     Running target/adder-91b3e234d4ed382a

running 1 test
test it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

### `#[ignore]` 标识

有时候，某个测试函数非常耗时，或暂时没更新，我们想不让它参与测试，但是又不想删除它，这时， `#[ignore]` 就派上用场了。

```rust
#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}
```

写上这个，运行 `cargo test` 的时候，就不会测试这个函数。

## 模块级测试

有时，我们会组织一批测试用例，这时，模块化的组织结构就有助于建立结构性的测试体系。Rust 中，可以类似如下写法：

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::add_two;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }
}
```

也即在 `mod` 的上面写上 `#[cfg(test)]` ，表明这个模块是个测试模块。一个测试模块中，可以包含若干测试函数，测试模块中还可以继续包含测试模块，即模块的嵌套。

如此，就形式了结构化的测试体系，甚是方便。


## 工程级测试

函数级和模块级的测试，代码是与要测试的模块（编译单元）写在相同的文件中，一般做的是白盒测试。工程级的测试，一般做的就是黑盒集成测试了。

我们看一个工程的目录，在这个目录下，有一个 `tests` 文件夹（没有的话，就手动建立）

```
Cargo.toml
Cargo.lock
examples
src
tests
```

我们在 tests 目录下，建立一个文件 `testit.rs` ，名字随便取皆可。内容为：

```rust
extern crate adder;

#[test]
fn it_works() {
    assert_eq!(4, adder::add_two(2));
}
```

这里，比如，我们 src 中，写了一个库，提供了一个 `add_two` 函数，现在进行集成测试。

首先，用 `extern crate` 的方式，引入这个库，由于是同一个项目，cargo 会自动找。引入后，就按模块的使用方法调用就行了，其它的测试标识与前面相同。

写完后，运行一下 `cargo test`，提示类似如下：

```
$ cargo test
   Compiling adder v0.0.1 (file:///home/you/projects/adder)
     Running target/adder-91b3e234d4ed382a

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

     Running target/lib-c18e7d3494509e74

running 1 test
test it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

## 文档级测试

Rust 对文档的哲学，是不要单独写文档，一是代码本身是文档，二是代码的注释就是文档。Rust 不但可以自动抽取代码中的文档，形成标准形式的文档集合，还可以对文档中的示例代码进行测试。

比如，我们给上面库加点文档：

``````rust
//! The `adder` crate provides functions that add numbers to other numbers.
//!
//! # Examples
//!
//! ```
//! assert_eq!(4, adder::add_two(2));
//! ```

/// This function adds two to its argument.
///
/// # Examples
///
/// ```
/// use adder::add_two;
///
/// assert_eq!(4, add_two(2));
/// ```

pub fn add_two(a: i32) -> i32 {
   a + 2
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn it_works() {
      assert_eq!(4, add_two(2));
   }
}
``````


运行 `cargo test`，结果如下：

```
$ cargo test
   Compiling adder v0.0.1 (file:///home/steve/tmp/adder)
     Running target/adder-91b3e234d4ed382a

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

     Running target/lib-c18e7d3494509e74

running 1 test
test it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests adder

running 2 tests
test add_two_0 ... ok
test _0 ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured
```

看到了吧，多了些测试结果。

## 结语

我们可以看到，Rust 对测试，对文档，对文档中的示例代码测试，都有特性支持。从这些细节之处，可以看出 Rust 设计的周密性和严谨性。

但是，光有好工具是不够的，工程的质量更重要的是写代码的人决定的。我们应该在 Rust 严谨之风的熏陶下，养成良好的编码和编写测试的习惯，掌握一定的分析方法，把质量要求贯彻到底。
