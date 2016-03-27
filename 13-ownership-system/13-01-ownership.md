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

### **变量绑定（Binding）**
好了，先来看下Rust中如何定义一个变量
```rust
{
    let a: i32;
    println!("{}", a);
}
```
上面定义了一个i32的变量a，如果你直接println，你会收到一个error报错：
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
其实，**let**关键字并不只是声明变量的意思，它还有一层特殊且重要的概念-**绑定**。通俗的讲，let关键字可以把一个变量和一段内存区域做“绑定”，绑定后，这段内存就被这个变量所拥有，这个变量也成为这段内存的唯一**所有者**。
所以，“a = 100”发生了这么几个动作，首先在栈上分配一个i32的内存区域，并填充值100，随后，把这个内存区域与变量a做绑定，让a成为其所有者。

### **作用域**
像C语言一样，Rust通过**{}**定义作用域：
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
不能对不可变变量赋值。

待续。。。。



请记住如下规则：
* **通过let指令，完成一段内存区域和一个变量的绑定，绑定后变量成为其所有者。**
* **一段内存区域有且仅有一个所有者**

待续。。。。



===============分隔线 =========下面为老内容，要删掉 =======================



与很多编程语言不同，Rust通过Ownership标识某一变量是否拥有值(内存块)的所有限，同时只允许最多一个变量可以拥有这个所有权，这个操作也称为变量绑定（Variable Bindings）。
在Rust中，一块内存（存储int, float, string,对象...）同一时刻只能拥有一个所有者（Owner），这个所有者对这块内存拥有所有权（Ownership）。只有所有者可以读写该内存（除非被借用Borrow），一旦所有者被释放（如离开作用域），那么对应的这块内存也同时被释放。
Rust通过**let**语句完成所有权的绑定：
```rust
fn main() {
	let x: Vec<i32> = vec!(1i32, 2, 3);
}
```
### **移动语义**
通过直接把一个变量赋值为另一个变量，完成所有权的移交操作，称为移动语义。被移动语义的变量失去了所有权，后续不可对此变量进行任何访问，否则无法通过编译，因为失去所有权的变量是不可预测的；例如，x的所有权移动给了y，那么y后续甚至可以释放这个变量的内存，如果再访问x就会出现非法访问内存，所以在编译阶段所有权系统会拒绝这种情况通过。这就是rust如何通过ownership保证了内存的安全性。
```rust
fn main() {
	let x: Vec<i32> = vec!(1i32, 2, 3);
	let y = x;              //ownership从x moved给了y
	// println!("{:?}", x); //不可以访问ownership被moved的变量x
	println!("{:?}", y);
}
```
>官方解释
>When we move v to v2, it creates a copy of that pointer, for v2. Which means that there would be two pointers to the content of the vector on the heap. It would violate Rust's safety guarantees by introducing a data race. Therefore, Rust forbids using v after we’ve done the move.

意思就是说，move后有两个指针同时指向了该内存区域，为了避免数据竞争，Rust是不允许使用move后的源变量。
上例中，第2行为向量（vector）对象x和它包含的数据分配了内存。向量对象储存在栈上并包含一个指向堆上[1, 2, 3]内容的指针。当我们从x移动到y，它为y创建了一个那个指针的拷贝。这意味着这将会有两个指向向量内容的指针。这将会因为引入了一个数据竞争而违反Rust的安全保证。因此，Rust禁止我们在移动后使用x。

会发生所有权移交moved的情况：

- 一个变量直接绑定给另一个变量，如 let y = x。
- **直接**作为参数，而不是引用（后面会详解）的方式传递给函数。
- 在其他作用域直接绑定给其他变量
>	let x = vec!(i32, 2, 3);
>	{ let y = x;  }       //ownership moved

- move关键字显式移交所有权（后面章节详解）。

### **Copy特性**
针对不能访问被moved的变量，有的人觉得有时不是这样的：
```rust
fn main() {
	let x = 1;
	let y = x; //都是拷贝，只不过这儿拷贝了值，而不是指针。因为i32实现了Copy trait
	println!("x = {}", x);
	println!("y = {}", y);
}
```
上面确实是可以编译过去的，这是为什么呢？
通过上面章节我们知道，当所有权被转移给另一个绑定以后，你不能再使用原始绑定。然而，这里有一个trait会改变这个行为，它叫做Copy。因为i32类型实现了的Copy triat，这个Copy仅仅拷贝了值。最后把新值的所有权绑定给了y。

默认实现Copy trait的类型：
```rust
clone_impl! { isize }
clone_impl! { i8 }
clone_impl! { i16 }
clone_impl! { i32 }
clone_impl! { i64 }

clone_impl! { usize }
clone_impl! { u8 }
clone_impl! { u16 }
clone_impl! { u32 }
clone_impl! { u64 }

clone_impl! { f32 }
clone_impl! { f64 }

clone_impl! { () }
clone_impl! { bool }
clone_impl! { char }
```

### **可变性**
默认的变量绑定是只读的不可变的(immutable)，如要修改变量的值，必须在变量名前加上**mut**关键字。

####错误：尝试变更不可变绑定的变量
```rust
fn main() {
	let x: Vec<i32> = vec!(1i32, 2, 3); //不可变绑定
	x.push(4);      //x只读，不允许变更。
	println!("{:?}", x);
}
```
>error: cannot borrow immutable local variable `x` as mutable

试图修改不可变绑定的变量会导致编译无法通过。

####正确：变更可变绑定的变量
```rust
fn main() {
	let mut x: Vec<i32> = vec!(1i32, 2, 3); //可变绑定
	x.push(4);
	println!("{:?}", x);
}
```

### **move关键字**
move关键字常用在闭包中，强制闭包获取所有权。
```rust
fn main() {
	let x: i32 = 100;
	let some_closure = move |i: i32| i + x;
	let y = some_closure(2);
	println!("x={}, y={}", x, y);
}
```
>结果： x=100, y=102

注意，由于move强制move包体外依赖的变量的所有权，依照上面i32的Copy特性，实际上闭包获取的是x值拷贝的所有权，所以在最后依然可以访问x。

####**无move关键字的闭包**
如果没有move关键字：
```rust
fn main() {
	let x: i32 = 100;
	let some_closure =|i: i32| i + x;
	let y = some_closure(2);
	println!("x={}, y={}", x, y);
}
```
那么闭包以引用的方式借用外部变量，并根据是否对外部变量的读写情况推断为可变借用还是不可变借用，上例中的x在包体中推断为不可变借用。

如果尝试在包体中修改x的值，可以这样：
```rust
fn main() {
	let mut x: i32 = 100;
	{
		let mut some_closure =|i: i32| {x += i; i};
		let y = some_closure(2);
		println!("y={}", y);
	}
	println!("x={}", x);
}
```
> 输出：
> 　　y=2
> 　　x=102

上例中，在没有move关键字的情况下，编译器自动推断包体内的x为可变引用。

通过上面两个例子的对比，可以清晰的看到move关键字存在的区别。另外，如果x没有实现Copy triat，move后，在包体外不可再访问源变量x。

###**交回所有权**
所有权从一个变量x  moved到另一个变量y后，x可以通过再绑定的方式让y交回所有权
```rust
fn do_something(mut v: Vec<i32>) -> Vec<i32> {
	println!("{:?}", v);
	v.push(100);
	v   //返回，为了交回所有权
}

fn main() {
	let mut x: Vec<i32> = vec!(1i32, 2, 3);
	x = do_something(x);
	println!("{:?}", x);  //ok
}
```
当x作为参数传递给do函数时，所有权转移到了函数中的参数v，最后函数把vector返回，调用let x = show(x)重新把所有权交回给x。

为了交回所有权，上面还需要通过把变量作为返回值返回，这是多么的麻烦！幸运的是，Rust为我们提供了一个叫做Borrowing（借用）的trait，它让我们轻松地可以解决这种问题。下面的小节让我们来看看强大的References&Borrowing。
