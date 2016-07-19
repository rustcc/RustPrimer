# 变量绑定与原生类型

## 变量绑定
Rust 通过 let 关键字进行变量绑定。

```rust
fn main() {
    let a1 = 5;
    let a2:i32 = 5;
    assert_eq!(a1, a2);
    //let 绑定 整数变量默认类型推断是 i32

    let b1:u32 = 5;
    //assert_eq!(a1, b1);
    //去掉上面的注释会报错，因为类型不匹配
    //errer: mismatched types
}
```

这里的 assert_eq! 宏的作用是判断两个参数是不是相等的，但如果是两个不匹配的类型，就算字面值相等也会报错。

## 可变绑定
rust 在声明变量时，在变量前面加入 mut 关键字，变量就会成为可变绑定的变量。

```rust
fn main() {
    let mut a: f64 = 1.0;
    let b = 2.0f32;

    //改变 a 的绑定
    a = 2.0;
    println!("{:?}", a);

    //重新绑定为不可变
    let a = a;

    //不能赋值
    //a = 3.0;

    //类型不匹配
    //assert_eq!(a, b);
}
```

这里的 b 变量，绑定了 2.0f32。这是 Rust 里面值类型显式标记的语法，规定为`value`+`type`的形式。

**例如：**
固定大小类型：
> 1u8 1i8  
> 1u16 1i16  
> 1u32 1i32  
> 1u64 1i64

可变大小类型：
> 1usize 1isize

浮点类型：
> 1f32 1f64

## let解构
为什么在 Rust 里面声明一个变量的时候要采用 let 绑定表达式？
那是因为 let 绑定表达式的表达能力更强，而且 let 表达式实际上是一种模式匹配。

**例如：**

```rust
fn main() {
    let (a, mut b): (bool,bool) = (true, false);
    println!("a = {:?}, b = {:?}", a, b);
    //a 不可变绑定
    //a = false;

    //b 可变绑定
    b = true;
    assert_eq!(a, b);
}
```

这里使用了 bool，只有true和false两个值，通常用来做逻辑判断的类型。

## 原生类型

Rust内置的原生类型 (primitive types) 有以下几类：

* 布尔类型：有两个值`true`和`false`。
* 字符类型：表示单个Unicode字符，存储为4个字节。
* 数值类型：分为有符号整数 (`i8`, `i16`, `i32`, `i64`, `isize`)、
无符号整数 (`u8`, `u16`, `u32`, `u64`, `usize`) 以及浮点数 (`f32`, `f64`)。
* 字符串类型：最底层的是不定长类型`str`，更常用的是字符串切片`&str`和堆分配字符串`String`，
其中字符串切片是静态分配的，有固定的大小，并且不可变，而堆分配字符串是可变的。
* 数组：具有固定大小，并且元素都是同种类型，可表示为`[T; N]`。
* 切片：引用一个数组的部分数据并且不需要拷贝，可表示为`&[T]`。
* 元组：具有固定大小的有序列表，每个元素都有自己的类型，通过解构或者索引来获得每个元素的值。
* 指针：最底层的是裸指针`*const T`和`*mut T`，但解引用它们是不安全的，必须放到`unsafe`块里。
* 函数：具有函数类型的变量实质上是一个函数指针。
* 元类型：即`()`，其唯一的值也是`()`。

```rust
// boolean type
let t = true;
let f: bool = false;

// char type
let c = 'c';

// numeric types
let x = 42;
let y: u32 = 123_456;
let z: f64 = 1.23e+2;
let zero = z.abs_sub(123.4);
let bin = 0b1111_0000;
let oct = 0o7320_1546;
let hex = 0xf23a_b049;

// string types
let str = "Hello, world!";
let mut string = str.to_string();

// arrays and slices
let a = [0, 1, 2, 3, 4];
let middle = &a[1..4];
let mut ten_zeros: [i64; 10] = [0; 10];

// tuples
let tuple: (i32, &str) = (50, "hello");
let (fifty, _) = tuple;
let hello = tuple.1;

// raw pointers
let x = 5;
let raw = &x as *const i32;
let points_at = unsafe { *raw };

// functions
fn foo(x: i32) -> i32 { x }
let bar: fn(i32) -> i32 = foo;
```

有几点是需要特别注意的：

* 数值类型可以使用`_`分隔符来增加可读性。
* Rust还支持单字节字符`b'H'`以及单字节字符串`b"Hello"`，仅限制于ASCII字符。
此外，还可以使用`r#"..."#`标记来表示原始字符串，不需要对特殊字符进行转义。
* 使用`&`符号将`String`类型转换成`&str`类型很廉价，
但是使用`to_string()`方法将`&str`转换到`String`类型涉及到分配内存，
除非很有必要否则不要这么做。
* 数组的长度是不可变的，动态的数组称为Vec (vector)，可以使用宏`vec!`创建。
* 元组可以使用`==`和`!=`运算符来判断是否相同。
* 不多于32个元素的数组和不多于12个元素的元组在值传递时是自动复制的。
* Rust不提供原生类型之间的隐式转换，只能使用`as`关键字显式转换。
* 可以使用`type`关键字定义某个类型的别名，并且应该采用驼峰命名法。

```rust
// explicit conversion
let decimal = 65.4321_f32;
let integer = decimal as u8;
let character = integer as char;

// type aliases
type NanoSecond = u64;
type Point = (u8, u8);
```
