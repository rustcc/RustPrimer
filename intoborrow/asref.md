# AsRef 和 AsMut

`std::convert` 下面，还有另外两个 Trait，`AsRef/AsMut`，它们功能是配合泛型，在执行引用操作的时候，进行自动类型转换。这能够使一些场景的代码实现得清晰漂亮，大家方便开发。

## AsRef<T>

`AsRef` 提供了一个方法 `.as_ref()`。

对于一个类型为 `T` 的对象 `foo`，如果 `T` 实现了 `AsRef<U>`，那么，`foo` 可执行 `.as_ref()` 操作，即 `foo.as_ref()`。操作的结果，我们得到了一个类型为 `&U` 的新引用。

注：

1. 与 `Into<T>` 不同的是，`AsRef<T>` 只是类型转换，`foo` 对象本身没有被消耗；
2. `T: AsRef<U>` 中的 `T`，可以接受 资源拥有者（owned）类型，共享引用（shared referrence）类型 ，可变引用（mutable referrence）类型。

下面举个简单的例子：

```rust
fn is_hello<T: AsRef<str>>(s: T) {
   assert_eq!("hello", s.as_ref());
}

let s = "hello";
is_hello(s);

let s = "hello".to_string();
is_hello(s);
```

因为 `String` 和 `&str` 都实现了 `AsRef<str>`。


## AsMut<T>

`AsMut<T>` 提供了一个方法 `.as_mut()`。它是 `AsRef<T>` 的可变（mutable）引用版本。

对于一个类型为 `T` 的对象 `foo`，如果 `T` 实现了 `AsMut<U>`，那么，`foo` 可执行 `.as_mut()` 操作，即 `foo.as_mut()`。操作的结果，我们得到了一个类型为 `&mut U` 的可变（mutable）引用。

注：在转换的过程中，`foo` 会被可变（mutable）借用。
