**所有权（Owership）**
-------------
在进入正题之前，大家先回忆下一般的编程语言知识。
对于一般的编程语言，通常会先声明一个变量，然后并初始化它。
例如在C语言中：
```c
int* foo {
    int a;          // 变量a的作用域开始
    a = 100;        
    char *c = "xyz";   // 变量c的作用域开始
    return &a;
}                   // 变量a和c的作用域结束
````

尽管可以编译通过，但这是一段非常糟糕的代码，现实中我相信大家都不会这么去写。变量a和c都是局部变量，函数介乎后吧局部变量a的地址返回，但局部变量a存在栈中，在离开作用域后，局部变量所申请的栈上内存都会被系统回收，从而造成了**悬空指针**的问题。**这是一个非常典型的内存安全问题。很多编程语言都存在类似这样的内存安全问题**。再来看变量c，c的值是常量字符串，存储于常量区，可能这个函数我们只调用了一次，我们可能不再想使用这个字符串，但"xyz"只有当整个程序结束后系统才能回收这片内存，这点让程序员是不是也很无奈？
> 备注：对于"xyz"，可根据实际情况，通过堆的方式，手动管理（申请和释放）内存。

所以，内存安全和内存管理通常是程序员眼中的两大头疼问题。令人兴奋的是，但Rust却再让你担心内存安全问题，也不用再操心内存管理的麻烦，那Rust是如何做到这一点的？请往下看。

### **绑定（Binding）**
好了，先来看下以下一段Rust代码：
```rust
{
    let a: i32;
    println!("{}", a);
}
```
上面定义了一个i32类型的标识符a，如果你直接println，你会收到一个error报错：
> error: use of possibly uninitialized variable: `a`

这是**因为Rust并不会像其他语言一样可以为变量默认初始化值，Rust明确规定变量的初始值必须有程序员自己决定**。

正确的做法：
```rust
{
    let a: i32;
    a = 100; //必须初始化a
    println!("{}", a);
}
```
其实，**let**关键字并不只是声明变量的意思，它还有一层特殊且重要的概念-**绑定**。通俗的讲，let关键字可以把一个标识符和一段内存区域做“绑定”，绑定后，这段内存就被这个标识符所拥有，这个标识符也成为这段内存的唯一**所有者**。
所以，“a = 100”发生了这么几个动作，首先在栈上分配一个i32的内存区域，并填充值100，随后，把这个内存区域与a做绑定，让a成为其所有者。

### **作用域**
像C语言一样，Rust通过“{}”大括号定义作用域：
```rust
{
    {
        let a: i32 = 100;
    }
    println!("{}", a);
}
```
编译后会得到如下error错误：
>b.rs:3:20: 3:21 error: unresolved name `a` [E0425]  
b.rs:3     println!("{}", a);

像C语言一样，在局部变量离开作用域后，变量随即会被销毁；**但不同是，Rust会连同变量绑定的内存，不管是否为常量字符串，连同所有者变量一起被销毁释放**。所以上面的例子，a销毁后再次访问a就会提示无法找到变量a的错误。这些所有的一切都是在编译过程中完成的。

### **移动语义（move）**
先看如下代码：
```rust
{
    let a: String = String::new("xyz");
    let b = a;
    println!("{}", a);
}
```
编译后会得到如下的报错：
> c.rs:4:20: 4:21 error: use of moved value: `a` [E0382]  
c.rs:4     println!("{}", a);  

错误的意思是在println中使用了被moved的变量a。那为什么会有这种报错呢？具体含义是什么？
在Rust中，和“绑定”概念相辅相成的另一个机制就是“转移move所有权”，意思是，**可以把value对应的内存的所有权从一个变量转移（move）到另一个变量**，这个操作同样通过**let**关键字完成，和绑定不同的是，“=”两边的左值和右值均为两个变量名：
```rust
语法：
    let 变量A = 变量B;  // 把“变量A”对应value的所有权转移给“变量B”
```
Move前后的内存示意如下：
> **Before move:**  
a  <=> 内存(地址：**A**，内容："xyz")  
**After move:**  
a  
b  <=> 内存(地址：**A**，内容："xyz")  


被move的变量不可以继续被使用。否则提示错误“error: use of moved value”。

这里有些人可能会疑问，move后，如果变量A和变量B离开作用域，所对应的内存会不会造成“Double Free”的问题？答案是否定的，**Rust规定，只有value内存区域的所有者销毁后才释放内存，而无论这块内存区域是不是被多次move，同一时刻只会有一个所有者，所以内存也只会被释放一次**。
通过这个机制，就保证了内存安全。是不是觉得很强大？


### **Copy特性**
有读者仿照“move”小节中的例子写了下面一个例子，然后说“a被move后是可以访问的”：
```rust
    let a: i32 = 100;
    let b = a;
    println!("{}", a);
```
编译确实可以通过，输出为100。这是为什么呢，是不是跟move小节里的结论相悖了？
其实不然，这其实是根据变量类型是否实现Copy特性决定的。对于实现Copy特性的变量，在move时会拷贝value到新内存区域，并把新内存区域绑定为新变量b。
> **Before move:**  
a  <=> 内存(地址：**A**，内容：100)  
**After move:**  
a  <=> 内存(地址：**A**，内容：100)  
b  <=> 内存(地址：**B**，内容：100)  

move前后的a和b对应内存的地址不同。

在Rust中，基本数据类型(Primitive Types)均实现了Copy特性，包括i8, i16, i32, i64, usize, u8, u16, u32, u64, f32, f64, (), bool, char等等。其他支持Copy的数据类型可以参考官方文档的[Copy章节](https://doc.rust-lang.org/std/marker/trait.Copy.html "Copy Trait")。

### **浅拷贝与深拷贝**
对于上面move String和i32的两个例子的区别，其实很多面向对象编程语言中“浅拷贝”和“深拷贝”的区别。对于基本数据类型磊说，“深拷贝”和“浅拷贝“产生的效果相同。对于引用对象类型来说，”浅拷贝“更像仅仅拷贝了对象的内存地址。  
如果我们想实现对**String**的”深拷贝“怎么办？  可以直接调用**String**的Clone特性实现对内存的值拷贝而不是简单的地址拷贝。
```rust
{
    let a: String = String::new("xyz");
    let b = a.clone();  // <-注意此处的clone
    println!("{}", a);
}
```
这个时候可以编译通过，并且成功打印"xyz"。

clone后的效果等同如下：
> **Before move:**  
a  <=> 内存(地址：**A**，内容："xyz")  
**After move:**  
a  <=> 内存(地址：**A**，内容："xyz")  
b  <=> 内存(地址：**B**，内容："xyz")  
注意，然后a和b对应的value内容相同，但是内存地址并不一样。


### **可变性**
通过上面，我们已经已经了解了变量声明、值绑定、以及移动move语义等等相关知识，但是还没有进行过修改变量值这么简单的操作，在其他语言中看似简单到不值得一提的事却在Rust中暗藏玄机。
按照其他编程语言思维，修改一个变量的值：
```rust
let a: i32 = 100;
a = 200;
```
很抱歉，这么简单的操作依然还会报错：
> error: re-assignment of immutable variable `a` [E0384]  
<anon>:3     a = 200;  

不能对**不可变绑定**赋值。如果要修改值，必须用关键字mut申明绑定为可变的：
```rust
let mut a: i32 = 100;  // 通过关键字mut申明a是可变的
a = 200;
```

**想到“不可变”我们第一时间想到了const常量，但不可变绑定与const常量是完全不同的两种概念；首先，“不可变”确切地应该称为“不可变绑定”，是用来约束绑定行为的，“不可变绑定”后不能通过这个“所有者”变更所绑定“value内存区域”的内容，所以，依然可以把目标value内存区域变为“可变绑定”，目标“value内存区域”由数据类型本身决定。**  
**例如：**
```rust
let a = vec![1, 2, 3];  //不可变绑定, a <=> 内存区域A(1,2,3)
let mut a = a;  //可变绑定， a <=> 内存区域A(1,2,3), 注意此a已非上句a，只是名字一样而已
a.push(4);
println!("{:?}", a);  //打印：[1, 2, 3, 4]
```
“可变绑定”后，目标内存还是同一块，只不过，可以通过新绑定的a去修改这片内存了。  

```rust
let mut a: &str = "abc";  //可变绑定, a <=> 内存区域A("abc")  
a = "xyz";    //绑定到另一内存区域, a <=> 内存区域B("xyz")  
println!("{:?}", a);  //打印："xyz"  
```
上面这种情况不要混淆了，a = "xyz"表示a绑定目标内存的区域发生了变化。  

其实，Rust中也有const常量，常量不存在“绑定”之说，和其他语言的常量含义相同：  
```rust
const PI:f32 = 3.14;
```

可变性的目的就是严格区分绑定的可变性，以便编译器可以更好的优化，也提高了内存安全性。  

### **高级Copy特性**
在前面的小节有简单了解Copy特性，接下来我们来深入了解下这个特性。  
Copy特性定义在标准库[std::marker::Copy](https://doc.rust-lang.org/std/marker/trait.Copy.html "")中：  
```rust
pub trait Copy: Clone { }
```
一旦一种类型实现了Copy特性，这就意味着这种类型可以通过的简单的位(bits)拷贝实现拷贝。从前面知识我们知道“绑定”存在move语义（所有权转移），但是，一旦这种类型实现了Copy特性，会先拷贝内容到新内存区域，然后把新内存区域和这个标识符做绑定。

**哪些情况下我们自定义的类型（如某个Struct等）可以实现Copy特性？**  
只要这种类型是属性类型都实现了Copy特性，那么这个类型就可以实现Copy特性。  
例如：
```rust
struct Foo {  //可实现Copy特性
    a: i32,
    b: bool,
}

struct Bar {  //不可实现Copy特性
    l: Vec<i32>,
}
```

因为Foo的属性a和b的类型i32和bool均实现了Copy特性，所以Foo也是可以实现Copy特性的。但对于Bar来说，它的属性l是Vec<T>类型，这种类型并没有实现Copy特性，所以Bar也是无法实现Copy特性的。

**那么我们如何来实现Copy特性呢？**  
有两种方式可以实现。  
1. **通过derive让Rust编译器自动实现**  
```rust
#derive(Copy, Clone)]
struct Foo {
    a: i32,
    b: bool,
}
```  
编译器会自动检查Foo的所有属性是否实现了Copy特性，一旦检查通过，便会为Foo自动实现Copy特性。
2. **自己实现Clone和Copy trait**
```rust
#[derive(Debug)]
struct Foo {
    a: i32,
    b: bool,
}

impl Copy for Foo {}
impl Clone for Foo {
    fn clone(&self) -> Foo {
        Foo{a: self.a, b: self.b}
    }
}

fn main() {
    let x = Foo{ a: 100, b: true};
    let mut y = x;
    y.b = false;
    
    println!("{:?}", x);  //打印：Foo { a: 100, b: true }
    println!("{:?}", y);  //打印：Foo { a: 100, b: false }
}
```    
从结果我们发现let mut y = x后，x并没有因为所有权move而出现不可访问错误。  
因为Copy特性继承了Clone特性，所以我们均需要手动实现这两个特性。


### **高级move**
我们从前面的小节了解到，let绑定会发生所有权转移的情况，但所有权转移却因为变量类型是否实现Copy特性而行为不同：
```rust
let x: T = something;
let y = x;
```  
* 类型T没有实现Copy特性：x所有权转移到y。
* 类型T实现了Copy特性：拷贝x，并把拷贝的所有权绑定为y，x依然拥有原来value内存区域的所有权。

##### **move关键字**
move关键字常用在闭包中，强制闭包获取所有权。  
**例子1：**
```rust
fn main() {
	let x: i32 = 100;
	let some_closure = move |i: i32| i + x;
	let y = some_closure(2);
	println!("x={}, y={}", x, y);
}
```
>结果： x=100, y=102  
  
**例子2：**
```rust
fn main() {
	let mut x: String = String::from("abc");
	let mut some_closure = move |c: char| x.push(c);
	let y = some_closure('d');
	println!("x={:?}", x);
}
```  
> **报错：**  
error: use of moved value: `x` [E0382]  
<anon>:5 	println!("x={:?}", x);    

这是因为move关键字，会把闭包中的外部变量的所有权move到包体内，发生了所有权转移的问题，所以println访问x会如上错误。如果我们去掉println就可以编译通过。  

那么，如果我们想在包体外依然访问x，即x不失去所有权，怎么办？
```rust
fn main() {
	let mut x: String = String::from("abc");
	{
    	let mut some_closure = |c: char| x.push(c);
	    some_closure('d');
	}
	println!("x={:?}", x);  //成功打印：x="abcd"
}
```
我们只是去掉了move，去掉move后，包体内就会对x进行了**可变借用**，而不是“剥夺”x的所有权，细心的同学还注意到我们在前后还加了"{}"大括号作用域，是为了作用域结束后让**可变借用**失效，这样println才可以成功访问并打印我们期待的内容。  
关于“**Borrowing借用**”知识我们会在下一个大节中详细讲解。

