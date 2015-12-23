RustPrimer
===================


所有权系统。。。

----------

概述
-------------
所有权系统（Ownership System）是Rust语言最基本最独特也是最重要的特性之一。
Rust语言号称，只要你能编译通过程序就不会崩溃（内存安全）；拥有着零或者极小的运行时开销（运行速度）。这些优点也全部得益于Rust的所有权系统。

所有权系统，包括三个重要的组成部分：

- **Owership**（所有权）
- **Borrowing**（借用），以及它关联的特性: "references(引用)" 
- **Lifetimes**（生命周期）

这三个特性之间相互关联，后面章节会依次全面讲解，下文会以英文Ownership, Borrowing和Liftetimes代指它们。

> **提示:**
> 很多Rust初学者会经历我们所谓的“与借用检查器（borrow checker）作斗争”的过程，也就是指Rust编译器拒绝编译一个开发者认为合理的程序。这种“斗争”会因为开发者关于所有权系统如何工作的基本模型与Rust实现的实际规则不匹配而经常发生。当你刚开始尝试Rust的时候，你很可能会有相似的经历。然而有一个好消息：更有经验的Rust开发者反应，一旦他们适应所有权系统一段时间之后，与借用检查器的冲突会越来越少。


所有权（Owership）
-------------

与很多编程语言不同，Rust通过所有权特指某一变量是否拥有变量值(内存块)的所有权，同时只允许最多一个变量可以拥有这个所有权，这个操作也称之为变量绑定（Variable Bindings）。
Rust通过**let**语句完成所有权的绑定：
```rust
fn main() {
	let x: Vec<i32> = vec!(1i32, 2, 3);
}
```
### 移动语义
通过直接把一个变量赋值为另一个变量，完成所有权的移交操作，称为移动语义。被移动语义的变量失去了所有权，后续不可对此变量进行任何访问，否则会panic掉，因为失去所有权的变量是不可预测的；例如，x的所有权移动给了y，那么y后续甚至可以释放这个变量的内存，如果再访问x就会非法访问内存panic掉。这就是rust如何通过ownership保证了内存的安全性。
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
	{ let y = x;  }       //ownership moved

- move关键字显式移交所有权（后面章节详解）。



### Copy特性
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

### 可变性
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

###交回所有权
所有权从一个变量x  moved到另一个变量y后，x可以通过再绑定的方式让y交回所有权
```rust
fn do(mut v: Vec<i32>) -> Vec<i32> {
        println!("{:?}", v);
        v.push(100);
        v   //返回，为了交回所有权
}

fn main() {
        let x: Vec<i32> = vec!(1i32, 2, 3);
        let x = show(x);
        println!("{:?}", x);  //ok
}
```
当x作为参数传递给do函数时，所有权转移到了函数中的参数v，最后函数把vector返回，调用let x = show(x)重新把所有权交回给x。

为了交回所有权，上面还需要通过把变量作为返回值返回，这是多么的麻烦！幸运的是，Rust为我们提供了一个叫做Borrowing（借用）的trait，它让我们轻松地可以解决这种问题。下面的小节让我们来看看强大的References&Borrowing。


引用&借用（References&Borrowing）
-------------

在上一章节的最后，我们演示了回收所有权那种糟糕的方式。其获取Vec<i32>作为我们的参数，我们获取一个引用：&Vec<i32>。与其直接传递x，我们传递&x。我们称**&T**类型为一个“**引用**”，而与其拥有这个资源，它**借用了所有权**。一个借用变量的绑定在它离开作用域时并不释放资源。这意味着foo()调用之后，我们可以再次使用原始的绑定。
