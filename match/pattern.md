# 模式
模式，是Rust另一个强大的特性。它可以被用在`let`和`match`表达式里面。相信大家应该还记得我们在[复合类型](../type/compound-types.md)中提到的关于在let表达式中解构元组的例子，实际上这就是一个模式。

```rust
let tup = (0u8, 1u8);
let (x, y) = tup;
```

而且我们需要知道的是，如果一个模式中出现了和当前作用域中已存在的同名的绑定，那么它会覆盖掉外部的绑定。比如：

```rust
let x = 1;
let c = 'c';

match c {
    x => println!("x: {} c: {}", x, c),
}

println!("x: {}", x);
```

它的输出结果是:

```
x: c c: c
x: 1
```

在以上代码中，match作用域里的`x`这个绑定被覆盖成了`'c'`，而出了这个作用域，绑定`x`又恢复为`1`。这和变量绑定的行为是一致的。

## 更强大的解构

在上一节里，我们初步了解了模式匹配在解构`enum`时候的便利性，事实上，在Rust中模式可以被用来对任何复合类型进行解构——struct/tuple/enum。现在我们要讲述一个复杂点的例子，对`struct`进行解构。

首先，我们可以对一个结构体进行标准的解构：

```rust
struct Point {
    x: i64,
    y: i64,
}
let point = Point { x: 0, y: 0 };
match point {
    Point { x, y } => println!("({},{})", x, y),
}
```

最终，我们拿到了`Point`内部的值。有人说了，那我想改个名字怎么办？
很简单，你可以使用 `:`来对一个struct的字段进行重命名，如下:

```rust
struct Point {
    x: i64,
    y: i64,
}
let point = Point { x: 0, y: 0 };
match point {
    Point { x: x1, y: y1} => println!("({},{})", x1, y1),
}
```

另外，有的时候我们其实只对某些字段感兴趣，就可以用`..`来省略其他字段。

```rust
struct Point {
    x: i64,
    y: i64,
}

let point = Point { x: 0, y: 0 };

match point {
    Point { y, .. } => println!("y is {}", y),
}
```

## 忽略和内存管理

总结一下，我们遇到了两种不同的模式忽略的情况——`_`和`..`。这里要注意，模式匹配中被忽略的字段是不会被`move`的，而且实现`Copy`的也会优先被Copy而不是被`move`。

说的有点拗口，上代码：

```rust
let tuple: (u32, String) = (5, String::from("five"));

let (x, s) = tuple;

// 以下行将导致编译错误，因为String类型并未实现Copy, 所以tuple被整体move掉了。
// println!("Tuple is: {:?}", tuple);

let tuple = (5, String::from("five"));

// 忽略String类型，而u32实现了Copy，则tuple不会被move
let (x, _) = tuple;

println!("Tuple is: {:?}", tuple);
```

## 范围和多重匹配

模式匹配可以被用来匹配单种可能，当然也就能被用来匹配多种情况：

### 范围

在模式匹配中，当我想要匹配一个数字(字符)范围的时候，我们可以用`...`来表示：

```rust
let x = 1;

match x {
    1 ... 10 => println!("一到十"),
    _ => println!("其它"),
}

let c = 'w';

match c {
    'a' ... 'z' => println!("小写字母"),
    'A' ... 'Z' => println!("大写字母"),
    _ => println!("其他字符"),
}
```

### 多重匹配

当我们只是单纯的想要匹配多种情况的时候，可以使用 `|` 来分隔多个匹配条件

```rust
let x = 1;

match x {
    1 | 2 => println!("一或二"),
    _ => println!("其他"),
}
```

## ref 和 ref mut

前面我们了解到，当被模式匹配命中的时候，未实现`Copy`的类型会被默认的move掉，因此，原owner就不再持有其所有权。但是有些时候，我们只想要从中拿到一个变量的（可变）引用，而不想将其move出作用域，怎么做呢？答：用`ref`或者`ref mut`。

```rust
let mut x = 5;

match x {
    ref mut mr => println!("mut ref :{}", mr),
}
// 当然了……在let表达式里也能用
let ref mut mrx = x;
```


## 变量绑定

在模式匹配的过程内部，我们可以用`@`来绑定一个变量名，这在复杂的模式匹配中是再方便不过的，比如一个具名的范围匹配如下：

```rust
let x = 1u32;
match x {
    e @ 1 ... 5 | e @ 10 ... 15 => println!("get:{}", e),
    _ => (),
}
```

如代码所示，e绑定了x的值。

当然，变量绑定是一个极其有用的语法，下面是一个来自官方doc里的例子：

```rust
#[derive(Debug)]
struct Person {
    name: Option<String>,
}

let name = "Steve".to_string();
let x: Option<Person> = Some(Person { name: Some(name) });
match x {
    Some(Person { name: ref a @ Some(_), .. }) => println!("{:?}", a),
    _ => {}
}
```

输出：

```
Some("Steve")
```

## 后置条件

一个后置的if表达式可以被放在match的模式之后，被称为`match guards`。例如如下代码：

```rust
let x = 4;
let y = false;

match x {
    4 | 5 if y => println!("yes"),
    _ => println!("no"),
}
```

猜一下上面代码的输出？

答案是`no`。因为guard是后置条件，是整个匹配的后置条件：所以上面的式子表达的逻辑实际上是：

```
// 伪代码表示
IF y AND (x IN List[4, 5])
```
