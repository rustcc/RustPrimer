# Borrow, BorrowMut, ToOwned

## Borrow<T>

`use std::borrow::Borrow;`

`Borrow` 提供了一个方法 `.borrow()`。

对于一个类型为 `T` 的值 `foo`，如果 `T` 实现了 `Borrow<U>`，那么，`foo` 可执行 `.borrow()` 操作，即 `foo.borrow()`。操作的结果，我们得到了一个类型为 `&U` 的新引用。

`Borrow` 可以认为是 `AsRef` 的严格版本，它对普适引用操作的前后类型之间附加了一些其它限制。

`Borrow` 的前后类型之间要求必须有内部等价性。不具有这个等价性的两个类型之间，不能实现 `Borrow`。

`AsRef` 更通用，更普遍，覆盖类型更多，是 `Borrow` 的超集。

举例：

```rust
use std::borrow::Borrow;

fn check<T: Borrow<str>>(s: T) {
    assert_eq!("Hello", s.borrow());
}

let s = "Hello".to_string();

check(s);

let s = "Hello";

check(s);
```

## BorrowMut<T>

`use std::borrow::BorrowMut;`

`BorrowMut<T>` 提供了一个方法 `.borrow_mut()`。它是 `Borrow<T>` 的可变（mutable）引用版本。

对于一个类型为 `T` 的值 `foo`，如果 `T` 实现了 `BorrowMut<U>`，那么，`foo` 可执行 `.borrow_mut()` 操作，即 `foo.borrow_mut()`。操作的结果我们得到类型为 `&mut U` 的一个可变（mutable）引用。

注：在转换的过程中，`foo` 会被可变（mutable）借用。

## ToOwned

`use std::borrow::ToOwned;`

`ToOwned` 为 `Clone` 的普适版本。它提供了 `.to_owned()` 方法，用于类型转换。

有些实现了 `Clone` 的类型 `T` 可以从引用状态实例 `&T` 通过 `.clone()` 方法，生成具有所有权的 `T` 的实例。但是它只能由 `&T` 生成 `T`。而对于其它形式的引用，`Clone` 就无能为力了。

而 `ToOwned` trait 能够从任意引用类型实例，生成具有所有权的类型实例。

## 参考

- [http://doc.rust-lang.org/std/borrow/trait.Borrow.html](http://doc.rust-lang.org/std/borrow/trait.Borrow.html)
