# 复合类型

## 元组 Tuples

在别的语言里，你可能听过元组这个词，它表示一个大小、类型固定的有序数据组。在Rust中，情况并没有什么本质上的不同。不过Rust为我们提供了一系列简单便利的语法来让我们能更好的使用他。

```
let y = (2, "hello world");
let x: (3, &str) = (3, "world hello");

// 然后呢，你能用很简单的方式去访问他们：

// 用let表达式
let (w, z) = y; // w=2, z="hello world"

// 用下标

let f = x.0; // f = 3
let e = x.1; // e = "world hello"

```

## 结构体 struct

在Rust中，结构体是一个跟 `tuple` 类似 的概念。我们同样可以将一些常用的数据、属性聚合在一起，就形成了一个结构体。

所不同的是，Rust的结构体有三种最基本的形式。

### 具名结构体

这种结构体呢，他可以大致看成这样的一个声明形式:

```
struct A {
    attr1: i32,
    atrr2: String,
}
```

内部每个成员都有自己的名字和类型。

### 元组类型结构体

元组类型结构体使用小括号，类似 `tuple`。

```
struct B(i32, u16, bool);
```

它可以看作是一个有名字的元组，具体使用方法和一般的元组基本类似。

### 空结构体

结构体内部也可以没有任何成员。

```
struct D;
```

空结构体的内存占用为0。但是我们依然可以针对这样的类型实现它的“成员函数”。

不过到目前为止，空结构体后面不能加大括号，这么写会编译错误：

补充： 注意，最新的nightly版本已经允许这么写编译通过了！

```
struct C {

}
```

不过也有人提出来，这样的写法应该编译通过：https://github.com/rust-lang/rfcs/pull/1506 

### 实现结构体 impl

Rust没有继承，它和Golang不约而同的选择了trait(Golang叫Interface)作为其实现多态的基础。当然了，我们要想对一个结构体写一些专门的成员函数应该怎么写呢？

答： impl

talk is cheap ,举个栗子：

```
struct Person {
    name: String,
}

impl Person {
    fn new(n: &str) -> Person {
        Person {
            name: n.to_string(),
        }
    }
    
    fn greeting(&self) {
        println!("{} say hello .", self.name);
    }
}

fn main() {
    let peter = Person::new("Peter");
    peter.greeting();
}

```

看见了self，Python程序员不厚道的笑了。

我们来分析一下，上面的`impl`中，new被Person这个结构体自身所调用，其特征是`::`的调用，Java程序员站出来了：类函数！ 而带有`self`的`greeting`，更像是一个成员函数。

恩，回答正确，然而不加分。

### 关于各种ref的讨论

Rust对代码有着严格的安全控制，因此对一个变量也就有了所有权和借用的概念。所有权同一时间只能一人持有，可变引用也只能同时只能一个实例持有，不可变引用则可以被多个示例持有。同时所有权能被转移，在Rust中被称为`move`。

以上是所有权的基本概念，事实上，在整个软件的运行周期内，所有权的转换是一件及其恼人和烦琐的事情，尤其对那些初学Rust的同学来说。同样的，Rust的结构体作为其类型系统的基石，也有着比较严格的所有权控制限制。具体来说，关于结构体的所有权，有两种你需要考虑的情况。

#### 字段的ref和owner

在以上的结构体中，我们定义了不少结构体，但是如你所见，结构体的每个字段都是完整的属于自己的。也就是说，每个字段的owner都是这个结构体。每个字段的生命周期最终都不会超过这个结构体。

但是有些时候，我只是想要持有一个(可变)引用的值怎么办？
如下代码：
```
struct RefBoy {
    loc: &i32,
}
```

这时候你会得到一个编译错误：

```
<anon>:6:14: 6:19 error: missing lifetime specifier [E0106]
<anon>:6         loc: & i32,
```
这种时候，你将持有一个值的引用，因为它本身的生命周期在这个struct之外，所以对这个结构体而言，它无法准确的判断获知这个引用的生命周期，这在Rust编译器而言是不被接受的。
因此，这个时候就需要我们给这个结构体人为的写上一个生命周期，并显式的表明这个引用的生命周期。写法如下：
```
struct RefBoy<'a> {
    loc: &'a i32,
}
```
这里解释一下这个符号`<>`，它表示的是一个`属于`的关系，无论其中描述的是*生命周期*还是*泛型*。即： `RefBoy in 'a `。最终我们可以得出个结论，`RefBoy`这个结构体，其生命周期一定不能比`'a`更长才行。

写到这里，可能有的人还是对生命周期比较迷糊，不明白其中缘由，其实你只需要知道两点即可：

1. 结构体里的引用字段必须要有显式的生命周期
2. 一个被显式写出生命周期的结构体，其自身的生命周期一定小于等于其显示写出的任意一个生命周期

关于第二点，其实生命周期是可以写多个的，用 `,` 分隔。

注：生命周期和泛型都写在`<>`里，先生命周期后泛型，用`;`分隔。

#### impl中的三种self

前面我们知道，Rust中，通过impl可以对一个结构体添加成员方法。同时我们也看到了`self`这样的语法，同时，这个self也有好几种需要你仔细记忆的情况。

impl中的self,常见的有三种形式：`self`、`&self`、`&mut self` ，我们分别来说。

##### 被move的self

正如上面例子中的impl，我们实现了一个以`self`为第一个参数的函数，但是这样的函数实际上是有问题的。
问题在于Rust的所有权转移机制。

我曾经见过一个关于Rust的笑话："你调用了一下别人，然后你就不属于你了"。

比如下面代码就会报出一个错误：
```
struct A {
    a: i32,
}
impl A {
    pub fn show(self) {
        println!("{}", self.a);
    }
}

fn main() {
    let ast = A{a: 12i32};
    ast.show();
    println!("{}", ast.a);
}
```
错误：
```
13:25 error: use of moved value: `ast.a` [E0382]
<anon>:13     println!("{}", ast.a);
```
为什么呢？因为Rust本身，在你调用一个函数的时候，如果传入的不是一个引用，那么无疑，这个参数的owner将被move掉。同理，impl中的self，如果你写的不是一个引用的话，也是会被默认的move掉哟！

那么如何避免这种情况呢？答案是`Copy`和`Clone`：

```
#[derive(Copy, Clone)]
struct A {
    a: i32,
}
```

这么写的话，会使编译通过。但是这么写实际上也是有其缺陷的。其缺陷就是：你不能在一个被copy的impl函数里改变它！事实上，被move的self其实是相对少用的一种情况，更多的时候，我们需要的是ref和ref mut。

###### ref和ref mut

关于ref和mut ref的写法和被move的self写法类似，只不过多了一个引用修饰符号，上面有例子写法，不多说。

需要注意的一点是，你不能在一个ref的方法里调用一个mut ref，任何情况下都不行！

但是，反过来是可以的。代码如下：

```
#[derive(Copy, Clone)]
struct A {
    a: i32,
}
impl A {
    pub fn show(&self) {
        println!("{}", self.a);
        // compile error: cannot borrow immutable borrowed content `*self` as mutable
        // self.add_one();
    }
    pub fn add_two(&mut self) {
        self.add_one();
        self.add_one();
        self.show();
    }
    pub fn add_one(&mut self) {
        self.a += 1;
    }
}

fn main() {
    let mut ast = A{a: 12i32};
    ast.show();
    ast.add_two();
}
```

需要注意的是，一旦你的结构体持有一个可变引用，你，只能在`&mut self`的实现里去改变他！

Rust允许我们灵活的对一个struct进行你想要的实现，在编程的自由度上无疑有了巨大的提高。

至于更高级的关于trait和泛型的用法，我们将在以后的章节进行详细介绍。

## 枚举类型 enum

同struct一样，Rust的枚举(enum)类型也具有两种不同的实现。一种极具Rust特点的枚举类型

以下两种名字也是我瞎掰的：

### 有特点的枚举

```
enum SpecialPoint {
    Point(i32, i32),
    Special(String),
}
```

枚举里面居然能包含一些你需要的，特定的数据信息！

这是常规的枚举所无法做到的，更像枚举类，不是么？

### 有别人特点的枚举

```
enum Direction {
    West,
    North,
    Sourth,
    East,
}

```

至少这种我以前见过，程序员如是说。

### 使用枚举

和struct的成员访问符号`.`不同的是，枚举类型要想访问其成员，几乎无一例外的必须要用到模式匹配。并且， 你可以写一个 Direction::West，但是你绝对不能写成Direction.West。虽然编译器足够聪明能发现你这个粗心的毛病。

关于模式匹配，我不会说太多，还是举个栗子

```
enum SpecialPoint {
    Point(i32, i32),
    Special(String),
}

fn main() {
    let sp = SpecialPoint::Point(0, 0);
    match sp {
        SpecialPoint::Point(x, y) => {
            println!("I'am SpecialPoint(x={}, y={})", x, y);
        }
        SpecialPoint::Special(why) => {
            println!("I'am Special because I am {}", why);
        }
    }
}

```

呐呐呐，这就是模式匹配取值啦。
当然了，`enum`其实也是可以`impl`的，一般人我不告诉他！
