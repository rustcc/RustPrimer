# 函数与方法

## 函数

要声明一个函数，需要使用关键字`fn`，后面跟上函数名，比如

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}
```

其中函数参数的类型不能省略，可以有多个参数，但是最多只能返回一个值，
提前返回使用`return`关键字。Rust编译器会对未使用的函数提出警告，
可以使用属性`#[allow(dead_code)]`禁用无效代码检查。

Rust有一个特殊特性适用于发散函数 (diverging function)，它不返回：

```rust
fn diverges() -> ! {
    panic!("This function never returns!");
}
```

其中`panic!`是一个宏，使当前执行线程崩溃并打印给定信息。返回类型`!`可用作任何类型：

```rust
let x: i32 = diverges();
let y: String = diverges();
```

## 匿名函数

Rust使用闭包 (closure) 来创建匿名函数：

```rust
let num = 5;
let plus_num = |x: i32| x + num;
```

其中闭包`plus_num`借用了它作用域中的`let`绑定`num`。如果要让闭包获得所有权，
可以使用`move`关键字：

```rust
let mut num = 5;

{
    let mut add_num = move |x: i32| num += x;   // 闭包通过move获取了num的所有权

    add_num(5);
}

// 下面的num在被move之后还能继续使用是因为其实现了Copy特性
// 具体可见所有权(Owership)章节
assert_eq!(5, num);
```

## 高阶函数

Rust 还支持高阶函数 (high order function)，允许把闭包作为参数来生成新的函数：

```rust
fn add_one(x: i32) -> i32 { x + 1 }

fn apply<F>(f: F, y: i32) -> i32
    where F: Fn(i32) -> i32
{
    f(y) * y
}

fn factory(x: i32) -> Box<Fn(i32) -> i32> {
    Box::new(move |y| x + y)
}

fn main() {
    let transform: fn(i32) -> i32 = add_one;
    let f0 = add_one(2i32) * 2;
    let f1 = apply(add_one, 2);
    let f2 = apply(transform, 2);
    println!("{}, {}, {}", f0, f1, f2);

    let closure = |x: i32| x + 1;
    let c0 = closure(2i32) * 2;
    let c1 = apply(closure, 2);
    let c2 = apply(|x| x + 1, 2);
    println!("{}, {}, {}", c0, c1, c2);

    let box_fn = factory(1i32);
    let b0 = box_fn(2i32) * 2;
    let b1 = (*box_fn)(2i32) * 2;
    let b2 = (&box_fn)(2i32) * 2;
    println!("{}, {}, {}", b0, b1, b2);

    let add_num = &(*box_fn);
    let translate: &Fn(i32) -> i32 = add_num;
    let z0 = add_num(2i32) * 2;
    let z1 = apply(add_num, 2);
    let z2 = apply(translate, 2);
    println!("{}, {}, {}", z0, z1, z2);
}
```

## 方法

Rust通过`impl`关键字在`struct`、`enum`或者`trait`对象上实现方法调用语法 (method call syntax)。
关联函数 (associated function) 的第一个参数通常为`self`参数，有3种变体：
* `self`，允许实现者移动和修改对象，对应的闭包特性为`FnOnce`。
* `&self`，既不允许实现者移动对象也不允许修改，对应的闭包特性为`Fn`。
* `&mut self`，允许实现者修改对象但不允许移动，对应的闭包特性为`FnMut`。

不含`self`参数的关联函数称为静态方法 (static method)。

```rust
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

fn main() {
    let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
    println!("{}", c.area());

    // use associated function and method chaining
    println!("{}", Circle::new(0.0, 0.0, 2.0).area());
}
```
