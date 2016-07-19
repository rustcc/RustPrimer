# unsafe

**Rust**的内存安全依赖于强大的类型系统和编译时检测，不过它并不能适应所有的场景。
首先，所有的编程语言都需要跟外部的“不安全”接口打交道，调用外部库等，在“安全”的Rust下是无法实现的; 其次，“安全”的Rust无法高效表示复杂的数据结构，特别是数据结构内部有各种指针互相引用的时候；再次，
事实上还存在着一些操作，这些操作是安全的，但不能通过编译器的验证。

因此在安全的Rust背后，还需要`unsafe`的支持。

`unsafe`块能允许程序员做的额外事情有：

* 解引用一个裸指针`*const T`和`*mut T`
 
```rust
let x = 5;
let raw = &x as *const i32;
let points_at = unsafe { *raw };
println!("raw points at {}", points_at);
```

* 读写一个可变的静态变量`static mut`

```rust
static mut N: i32 = 5;
unsafe {
    N += 1;
    println!("N: {}", N);
}
```

* 调用一个不安全函数

```rust
unsafe fn foo() {
	//实现
}
fn main() {
	unsafe {
    	foo();
    }
}
```

## 使用`unsafe`

`unsafe fn`不安全函数标示如果调用它可能会违反**Rust**的内存安全语意：

```rust
unsafe fn danger_will_robinson() {
    // 实现
}
```

`unsafe block`不安全块可以在其中调用不安全的代码：

```rust
unsafe {
    // 实现
}
```

`unsafe trait`不安全trait及它们的实现，所有实现它们的具体类型有可能是不安全的:

```rust
unsafe trait Scary { }
unsafe impl Scary for i32 {}
```

## safe != no bug

对于**Rust**来说禁止你做任何不安全的事是它的本职，不过有些是编写代码时的`bug`，它们并不属于“内存安全”的范畴：

* 死锁
* 内存或其他资源溢出
* 退出未调用析构函数
* 整型溢出

使用`unsafe`时需要注意一些特殊情形：

* 数据竞争
* 解引用空裸指针和悬垂裸指针
* 读取未初始化的内存
* 使用裸指针打破指针重叠规则
* `&mut T`和`&T`遵循LLVM范围的`noalias`模型，除了如果`&T`包含一个`UnsafeCell<U>`的话。不安全代码必须不能违反这些重叠（aliasing）保证
* 不使用`UnsafeCell<U>`改变一个不可变值/引用
* 通过编译器固有功能调用未定义行为：
	* 使用`std::ptr::offset`（offset功能）来索引超过对象边界的值，除了允许的末位超出一个字节
	* 在重叠（overlapping）缓冲区上使用`std::ptr::copy_nonoverlapping_memory`（memcpy32/memcpy64功能）
* 原生类型的无效值，即使是在私有字段/本地变量中：
	* 空/悬垂引用或装箱
	* `bool`中一个不是`false`（0）或`true`（1）的值
	* `enum`中一个并不包含在类型定义中判别式
	* `char`中一个代理字（surrogate）或超过char::MAX的值
	* `str`中非UTF-8字节序列
* 在外部代码中使用Rust或在Rust中使用外部语言



