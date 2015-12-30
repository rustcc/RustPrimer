**13.1 所有权（Owership）**
-------------

与很多编程语言不同，Rust通过Ownership标识某一变量是否拥有值(内存块)的所权限，同时只允许最多一个变量可以拥有这个所有权，这个操作也称之为变量绑定（Variable Bindings）。
在Rust中，一块内存（存储int, float, string,对象...）同一时刻只能拥有一个所有者（Owner），这个所有者对这块内存拥有所有权（Ownership）。只有所有者可以读写该内存（除非被借用Borrow），一旦所有者被释放（如离开作用域），那么对应的这块内存也同时被释放。
Rust通过**let**语句完成所有权的绑定：
```rust
fn main() {
	let x: Vec<i32> = vec!(1i32, 2, 3);
}
```
### 移动语义
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

### move关键字
....

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
