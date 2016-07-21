# 高阶函数
  高阶函数与普通函数的不同在于，它可以使用一个或多个函数作为参数，可以将函数作为返回值。rust的函数是first class type，所以支持高阶函数。而，由于rust是一个强类型的语言，如果要将函数作为参数或返回值，首先需要搞明白函数的类型。下面先说函数的类型，再说函数作为参数和返回值。

## 函数类型
  前面说过，关键字`fn`可以用来定义函数。除此以外，它还用来构造函数类型。与函数定义主要的不同是，构造函数类型不需要函数名、参数名和函数体。在Rust Reference中的描述如下：
  > The function type constructor fn forms new function types. A function type consists of a possibly-empty set of function-type modifiers (such as unsafe or extern), a sequence of input types and an output type.

  来看一个简单例子：
  
  ```rust
fn inc(n: i32) -> i32 {//函数定义
  n + 1
}

type IncType = fn(i32) -> i32;//函数类型

fn main() {
  let func: IncType = inc;
  println!("3 + 1 = {}", func(3));
}
  ```
  
  上例首先使用`fn`定义了`inc`函数，它有一个`i32`类型参数，返回`i32`类型的值。然后再用`fn`定义了一个函数类型，这个函数类型有i32类型的参数和i32类型的返回值，并用`type`关键字定义了它的别名`IncType`。在`main`函数中定义了一个变量`func`，其类型就为`IncType`，并赋值为`inc`，然后在`pirntln`宏中调用：`func(3)`。可以看到，`inc`函数的类型其实就是`IncType`。
  这里有一个问题，我们将`inc`赋值给了`func`，而不是`&inc`，这样是将`inc`函数的拥有权转给了`func`吗，赋值后还可以以`inc()`形式调用`inc`函数吗？先来看一个例子：
  
  ```rust
fn main() {
  let func: IncType = inc;
  println!("3 + 1 = {}", func(3));
  println!("3 + 1 = {}", inc(3));
}

type IncType = fn(i32) -> i32;

fn inc(n: i32) -> i32 {
  n + 1
}
  ```
  
  我们将上例保存在rs源文件中，再用rustc编译，发现并没有报错，并且运行也得到我们想要的结果：
  
  ```
3 + 1 = 4
3 + 1 = 4
  ```
  
  这说明，赋值时，`inc`函数的所有权并没有被转移到`func`变量上，而是更像不可变引用。在rust中，函数的所有权是不能转移的，我们给函数类型的变量赋值时，赋给的一般是函数的指针，所以rust中的函数类型，就像是C/C++中的函数指针，当然，rust的函数类型更安全。可见，rust的函数类型，其实应该是属于指针类型（Pointer Type）。rust的Pointer Type有两种，一种为引用（Reference`&`），另一种为原始指针（Raw pointer `*`），详细内容请看[Rust Reference 8.18 Pointer Types](http://doc.rust-lang.org/reference.html#pointer-types)。而rust的函数类型应是引用类型，因为它是安全的，而原始指针则是不安全的，要使用原始指针，必须使用`unsafe`关键字声明。

## 函数作为参数
  函数作为参数，其声明与普通参数一样。看下例：
  
  ```rust
fn main() {
  println!("3 + 1 = {}", process(3, inc));
  println!("3 - 1 = {}", process(3, dec));
}

fn inc(n: i32) -> i32 {
  n + 1
}

fn dec(n: i32) -> i32 {
  n - 1
}

fn process(n: i32, func: fn(i32) -> i32) -> i32 {
  func(n)
}
  ```
  
  例子中，`process`就是一个高阶函数，它有两个参数，一个类型为`i32`的`n`，另一个类型为`fn(i32)->i32`的函数`func`，返回一个`i32`类型的参数；它在函数体内以`n`作为参数调用`func`函数，返回`func`函数的返回值。运行可以得到以下结果：
  
  ```
3 + 1 = 4
3 - 1 = 2
  ```
  
  不过，这不是函数作为参数的唯一声明方法，使用泛型函数配合特质（`trait`）也是可以的，因为rust的函数都会实现一个`trait`:`FnOnce`、`Fn`或`FnMut`。将上例中的`process`函数定义换成以下形式是等价的：
  
  ```rust
fn process<F>(n: i32, func: F) -> i32
    where F: Fn(i32) -> i32 {
    func(n)
}
  ```

## 函数作为返回值
  函数作为返回值，其声明与普通函数的返回值类型声明一样。看例子：
  
  ```rust
fn main() {
   let a = [1,2,3,4,5,6,7];
   let mut b = Vec::<i32>::new();
   for i in &a {
       b.push(get_func(*i)(*i));
   }
   println!("{:?}", b);
}

fn get_func(n: i32) -> fn(i32) -> i32 {
    fn inc(n: i32) -> i32 {
        n + 1
    }
    fn dec(n: i32) -> i32 {
        n - 1
    }
    if n % 2 == 0 {
        inc
    } else {
        dec
    }
}
  ```
  
  例子中的高阶函数为`get_func`，它接收一个i32类型的参数，返回一个类型为`fn(i32) -> i32`的函数，若传入的参数为偶数，返回`inc`，否则返回`dec`。这里需要注意的是，`inc`函数和`dec`函数都定义在`get_func`内。在函数内定义函数在很多其他语言中是不支持的，不过rust支持，这也是rust灵活和强大的一个体现。不过，在函数中定义的函数，不能包含函数中（环境中）的变量，若要包含，应该闭包（详看13章 闭包）。
  所以下例：
  
  ```rust
fn main() {
  let f = get_func();
  println!("{}", f(3));
}

fn get_func() -> fn(i32)->i32 {
  let a = 1;
  fn inc(n:i32) -> i32 {
    n + a
  }
  inc
}
  ```
  
  使用rustc编译，会出现如下错误：
  ![error](../images/high-order-function.png)
