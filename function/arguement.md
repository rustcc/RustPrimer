# 函数参数
## 参数声明
  rust的函数参数声明和一般的变量声明相仿，也是参数名后加冒号，冒号后跟参数类型，不过不需要`let`关键字。需要注意的是，普通变量声明(let语句)是可以省略变量类型的，而函数参数的声明则不能省略参数类型。
  来看一个简单例子：
  
  ```rust
fn main() {
  say_hi("ruster");
}

fn say_hi(name: &str) {
  println!("Hi, {}", name);
}
  ```
  
  上例中，`say_hi`函数拥有一个参数，名为`name`，类型为`&str`。

## 将函数作为参数
  在rust中，函数是一等公民（可以储存在变量/数据结构中，可以作为参数传入函数，可以作为返回值），所以rust的函数参数不仅可以是一般的类型，也可以是函数。如：
  
  ```rust
fn main() {
  let xm = "xiaoming";
  let xh = "xiaohong";
  say_what(xm, hi);
  say_what(xh, hello);
}

fn hi(name: &str) {
  println!("Hi, {}.", name);
}

fn hello(name: &str) {
  println!("Hello, {}.", name);
}

fn say_what(name: &str, func: fn(&str)) {
  func(name)
}
  ```
  
  上例中，`hi`函数和`hello`函数都是只有一个`&str`类型的参数且没有返回值。而`say_what`函数则有两个参数，一个是`&str`类型，另一个则是函数类型（function type），它是只有一个`&str`类型参数且没有返回值的函数类型。

## 模式匹配
  支持模式匹配，让rust平添了许多的灵活性，用起来也是十分的舒爽。模式匹配不仅可以用在变量声明（let语句）中，也可以用在函数参数声明中，如：
  
  ```rust
fn main() {
  let xm = ("xiaoming", 54);
  let xh = ("xiaohong", 66);
  print_id(xm);
  print_id(xh);
  print_name(xm);
  print_age(xh);
  print_name(xm);
  print_age(xh);
}

fn print_id((name, age): (&str, i32)) {
  println!("I'm {},age {}.", name, age);
}

fn print_age((_, age): (&str, i32)) {
  println!("My age is  {}", age);
}

fn print_name((name,_): (&str, i32)) {
  println!("I am  {}", name);
}
  ```
  
  上例是一个元组(Tuple)匹配的例子，当然也可以是其他可在let语句中使用的类型。参数的模式匹配跟let语句的匹配一样，也可以使用下划线来表示丢弃一个值。
