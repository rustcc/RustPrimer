# 结构体与枚举

## 结构体

结构体 (struct) 是一种记录类型，所包含的每个域 (field) 都有一个名称。
每个结构体也都有一个名称，通常以大写字母开头，使用驼峰命名法。
元组结构体 (tuple struct) 是由元组和结构体混合构成，元组结构体有名称，
但是它的域没有。当元组结构体只有一个域时，称为新类型 (newtype)。
没有任何域的结构体，称为类单元结构体 (unit-like struct)。
结构体中的值默认是不可变的，需要使用`mut`使其可变。

```rust
// structs
struct Point {
  x: i32,
  y: i32,
}
let point = Point { x: 0, y: 0 };

// tuple structs
struct Color(u8, u8, u8);
let android_green = Color(0xa4, 0xc6, 0x39);
let Color(red, green, blue) = android_green;

// A tuple struct’s constructors can be used as functions.
struct Digit(i32);
let v = vec![0, 1, 2];
let d: Vec<Digit> = v.into_iter().map(Digit).collect();

// newtype: a tuple struct with only one element
struct Inches(i32);
let length = Inches(10);
let Inches(integer_length) = length;

// unit-like structs
struct EmptyStruct;
let empty = EmptyStruct;
```

一个包含`..`的`struct`可以用来从其它结构体拷贝一些值或者在解构时忽略一些域：

```rust
#[derive(Default)]
struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point3d::default();
let point = Point3d { y: 1, ..origin };
let Point3d { x: x0, y: y0, .. } = point;
```

需要注意，Rust在语言级别不支持域可变性 (field mutability)，所以不能这么写：

```rust
struct Point {
    mut x: i32,
    y: i32,
}
```

这是因为可变性是绑定的一个属性，而不是结构体自身的。可以使用`Cell<T>`来模拟：

```rust
use std::cell::Cell;

struct Point {
    x: i32,
    y: Cell<i32>,
}

let point = Point { x: 5, y: Cell::new(6) };

point.y.set(7);
```

此外，结构体的域默认是私有的，可以使用`pub`关键字将其设置成公开。

## 枚举
Rust有一个集合类型，称为枚举 (enum)，代表一系列子数据类型的集合。
其中子数据结构可以为空-如果全部子数据结构都是空的，就等价于C语言中的enum。
我们需要使用`::`来获得每个元素的名称。

```rust
// enums
enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

let x: Message = Message::Move { x: 3, y: 4 };
```

与结构体一样，枚举中的元素默认不能使用关系运算符进行比较 (如`==`, `!=`, `>=`)，
也不支持像`+`和`*`这样的双目运算符，需要自己实现，或者使用`match`进行匹配。

枚举默认也是私有的，如果使用`pub`使其变为公有，则它的元素也都是默认公有的。
这一点是与结构体不同的：即使结构体是公有的，它的域仍然是默认私有的。
此外，枚举和结构体也可以是递归的 (recursive)。
