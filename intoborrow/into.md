# Into/From 及其在 String 和 &str 互转上的应用

`std::convert` 下面，有两个 Trait，`Into/From`，它们是一对孪生姐妹。它们的作用是配合泛型，进行一些设计上的归一化处理。

它们的基本形式为： `From<T>` 和 `Into<T>`。

## From<T>

对于类型为 `U` 的对象 `foo`，如果它实现了 `From<T>`，那么，可以通过 `let foo = U::from(bar)` 来生成自己。这里，`bar` 是类型为 `T` 的对象。

下面举一例，因为 `String` 实现了 `From<&str>`，所以 `String` 可以从 `&str` 生成。

```rust
let string = "hello".to_string();
let other_string = String::from("hello");

assert_eq!(string, other_string);
```

## Into<T>

对于一个类型为 `U: Into<T>` 的对象 `foo`，`Into` 提供了一个函数：`.into(self) -> T`，调用 `foo.into()` 会消耗自己（转移资源所有权），生成类型为 `T` 的另一个新对象 `bar`。

这句话，说起来有点抽象。下面拿一个具体的实例来辅助理解。

```rust
fn is_hello<T: Into<Vec<u8>>>(s: T) {
   let bytes = b"hello".to_vec();
   assert_eq!(bytes, s.into());
}

let s = "hello".to_string();
is_hello(s);
```

因为 `String` 类型实现了 `Into<Vec<u8>>`。

下面拿一个实际生产中字符串作为函数参数的例子来说明。

在我们设计库的 `API` 的时候，经常会遇到一个恼人的问题，函数参数如果定为 `String`，则外部传入实参的时候，对字符串字面量，必须要做 `.to_string()` 或 `.to_owned()` 转换，参数一多，就是一件又乏味又丑的事情。（而反过来设计的话，对初学者来说，又会遇到一些生命周期的问题，比较麻烦，这个后面论述）

那存不存在一种方法，能够使传参又能够接受 `String` 类型，又能够接受 `&str` 类型呢？答案就是**泛型**。而仅是泛型的话，太宽泛。因此，标准库中，提供了 `Into<T>` 来为其做约束，以便方便而高效地达到我们的目的。

比如，我们有如下结构体：

```rust
struct Person {
    name: String,
}

impl Person {
    fn new (name: String) -> Person {
        Person { name: name }
    }
}
```

我们在调用的时候，是这样的：

```rust
let name = "Herman".to_string();
let person = Person::new(name);
```

如果直接写成：

```rust
let person = Person::new("Herman");
```

就会报类型不匹配的错误。

好了，下面 `Into` 出场。我们可以定义结构体为

```rust
struct Person {
    name: String,
}

impl Person {
    fn new<S: Into<String>>(name: S) -> Person {
        Person { name: name.into() }
    }
}
```

然后，调用的时候，下面两种写法都是可以的：

```rust
fn main() {
    let person = Person::new("Herman");
    let person = Person::new("Herman".to_string());
}
```

我们来仔细分析一下这一块的写法

```rust
impl Person {
    fn new<S: Into<String>>(name: S) -> Person {
        Person { name: name.into() }
    }
}
```

参数类型为 `S`， 是一个泛型参数，表示可以接受不同的类型。`S: Into<String>` 表示 `S` 类型必须实现了 `Into<String>`（约束）。而 `&str` 类型，符合这个要求。因此 `&str` 类型可以直接传进来。

而 `String` 本身也是实现了 `Into<String>` 的。当然也可以直接传进来。

然后，下面 `name: name.into()` 这里也挺神秘的。它的作用是将 `name` 转换成 `String` 类型的另一个对象。当 name 是 `&str` 时，它会转换成 `String` 对象，会做一次字符串的拷贝（内存的申请、复制）。而当 name 本身是 `String` 类型时，`name.into()` 不会做任何转换，代价为零（有没有恍然大悟）。

根据参考资料，上述内容通过下面三式获得：

```rust
impl<'a> From<&'a str> for String {}
impl<T> From<T> for T {}
impl<T, U> Into<U> for T where U: From<T> {}
```

更多内容，请参考：

- [http://doc.rust-lang.org/std/convert/trait.Into.html](http://doc.rust-lang.org/std/convert/trait.Into.html)
- [http://doc.rust-lang.org/std/convert/trait.From.html](http://doc.rust-lang.org/std/convert/trait.From.html)
- [http://hermanradtke.com/2015/05/06/creating-a-rust-function-that-accepts-string-or-str.html](http://hermanradtke.com/2015/05/06/creating-a-rust-function-that-accepts-string-or-str.html)
