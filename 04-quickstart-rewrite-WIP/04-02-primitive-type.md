# 基本类型

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
这里的 b 变量，绑定了 2.0f32。这是 Rust 里面值类型显式标记的语发。语法规定为`value`+`tpye`的形式。

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
为什么在 Rust 里面生命一个变量的时候要采用 let 绑定语法？
那是因为 let 绑定语法的表达能力更强，而且 let 语言其实是一种模式。

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
