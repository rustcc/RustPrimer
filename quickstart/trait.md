# 特性

## 特性与接口
为了描述类型可以实现的抽象接口 (abstract interface)，
Rust引入了特性 (trait) 来定义函数类型签名 (function type signature)：

```rust
trait HasArea {
    fn area(&self) -> f64;
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

struct Square {
    x: f64,
    y: f64,
    side: f64,
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn print_area<T: HasArea>(shape: T) {
    println!("This shape has an area of {}", shape.area());
}
```

其中函数`print_area()`中的泛型参数`T`被添加了一个名为`HasArea`的特性约束 (trait constraint)，
用以确保任何实现了`HasArea`的类型将拥有一个`.area()`方法。
如果需要多个特性限定 (multiple trait bounds)，可以使用`+`：

```rust
use std::fmt::Debug;

fn foo<T: Clone, K: Clone + Debug>(x: T, y: K) {
    x.clone();
    y.clone();
    println!("{:?}", y);
}

fn bar<T, K>(x: T, y: K)
    where T: Clone,
          K: Clone + Debug
{
    x.clone();
    y.clone();
    println!("{:?}", y);
}
```

其中第二个例子使用了更灵活的`where`从句，它还允许限定的左侧可以是任意类型，
而不仅仅是类型参数。

定义在特性中的方法称为默认方法 (default method)，可以被该特性的实现覆盖。
此外，特性之间也可以存在继承 (inheritance)：

```rust
trait Foo {
    fn foo(&self);

    // default method
    fn bar(&self) { println!("We called bar."); }
}

// inheritance
trait FooBar : Foo {
    fn foobar(&self);
}

struct Baz;

impl Foo for Baz {
    fn foo(&self) { println!("foo"); }
}

impl FooBar for Baz {
    fn foobar(&self) { println!("foobar"); }
}
```

如果两个不同特性的方法具有相同的名称，可以使用通用函数调用语法 (universal function call syntax)：

```rust
// short-hand form
Trait::method(args);

// expanded form
<Type as Trait>::method(args);
```

关于实现特性的几条限制：

* 如果一个特性不在当前作用域内，它就不能被实现。
* 不管是特性还是`impl`，都只能在当前的包装箱内起作用。
* 带有特性约束的泛型函数使用单态化实现 (monomorphization)，
所以它是静态派分的 (statically dispatched)。

下面列举几个非常有用的标准库特性：

* `Drop`提供了当一个值退出作用域后执行代码的功能，它只有一个`drop(&mut self)`方法。
* `Borrow`用于创建一个数据结构时把拥有和借用的值看作等同。
* `AsRef`用于在泛型中把一个值转换为引用。
* `Deref<Target=T>`用于把`&U`类型的值自动转换为`&T`类型。
* `Iterator`用于在集合 (collection) 和惰性值生成器 (lazy value generator) 上实现迭代器。
* `Sized`用于标记运行时长度固定的类型，而不定长的切片和特性必须放在指针后面使其运行时长度已知，
比如`&[T]`和`Box<Trait>`。

## 泛型和多态

泛型 (generics) 在类型理论中称作参数多态 (parametric polymorphism)，
意为对于给定参数可以有多种形式的函数或类型。先看Rust中的一个泛型例子：  

Option在rust标准库中的定义:  

```rust
enum Option<T> {
    Some(T),
    None,
}
```
Option的典型用法:  
```rust
let x: Option<i32> = Some(5);
let y: Option<f64> = Some(5.0f64);
```

其中`<T>`部分表明它是一个泛型数据类型。当然，泛型参数也可以用于函数参数和结构体域：

```rust
// generic functions
fn make_pair<T, U>(a: T, b: U) -> (T, U) {
    (a, b)
}
let couple = make_pair("man", "female");

// generic structs
struct Point<T> {
    x: T,
    y: T,
}
let int_origin = Point { x: 0, y: 0 };
let float_origin = Point { x: 0.0, y: 0.0 };
```

对于多态函数，存在两种派分 (dispatch) 机制：静态派分和动态派分。
前者类似于C++的模板，Rust会生成适用于指定类型的特殊函数，然后在被调用的位置进行替换，
好处是允许函数被内联调用，运行比较快，但是会导致代码膨胀 (code bloat)；
后者类似于Java或Go的`interface`，Rust通过引入特性对象 (trait object) 来实现，
在运行期查找虚表 (vtable) 来选择执行的方法。特性对象`&Foo`具有和特性`Foo`相同的名称，
通过转换 (casting) 或者强制多态化 (coercing) 一个指向具体类型的指针来创建。

当然，特性也可以接受泛型参数。但是，往往更好的处理方式是使用关联类型 (associated type)：

```rust
// use generic parameters
trait Graph<N, E> {
    fn has_edge(&self, &N, &N) -> bool;
    fn edges(&self, &N) -> Vec<E>;
}

fn distance<N, E, G: Graph<N, E>>(graph: &G, start: &N, end: &N) -> u32 {

}

// use associated types
trait Graph {
    type N;
    type E;

    fn has_edge(&self, &Self::N, &Self::N) -> bool;
    fn edges(&self, &Self::N) -> Vec<Self::E>;
}

fn distance<G: Graph>(graph: &G, start: &G::N, end: &G::N) -> uint {

}

struct Node;

struct Edge;

struct SimpleGraph;

impl Graph for SimpleGraph {
    type N = Node;
    type E = Edge;

    fn has_edge(&self, n1: &Node, n2: &Node) -> bool {

    }

    fn edges(&self, n: &Node) -> Vec<Edge> {

    }
}

let graph = SimpleGraph;
let object = Box::new(graph) as Box<Graph<N=Node, E=Edge>>;

```

