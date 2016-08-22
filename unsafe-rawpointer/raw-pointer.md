# 裸指针

**Rust**通过限制智能指针的行为保障了编译时安全，不过仍需要对指针做一些额外的操作。

`*const T`和`*mut T`在**Rust**中被称为“裸指针”。它允许别名，允许用来写共享所有权的类型，甚至是内存安全的共享内存类型如：`Rc<T>`和`Arc<T>`，但是赋予你更多权利的同时意味着你需要担当更多的责任：

* 不能保证指向有效的内存，甚至不能保证是非空的
* 没有任何自动清除，所以需要手动管理资源
* 是普通旧式类型，也就是说，它不移动所有权，因此**Rust**编译器不能保证不出像释放后使用这种bug
* 缺少任何形式的生命周期，不像`&`，因此编译器不能判断出悬垂指针
* 除了不允许直接通过`*const T`改变外，没有别名或可变性的保障

## 使用

创建一个裸指针：

```rust
let a = 1;
let b = &a as *const i32;

let mut x = 2;
let y = &mut x as *mut i32;
```

解引用需要在`unsafe`中进行：

```rust
let a = 1;
let b = &a as *const i32;
let c = unsafe { *b };
println!("{}", c);
```

`Box<T>`的`into_raw`：

```rust
let a: Box<i32> = Box::new(10);
// 我们需要先解引用a，再隐式把 & 转换成 *
let b: *const i32 = &*a;
// 使用 into_raw 方法
let c: *const i32 = Box::into_raw(a);
```

如上说所，引用和裸指针之间可以隐式转换，但隐式转换后再解引用需要使用`unsafe`：

```rust
// 显式
let a = 1;
let b: *const i32 = &a as *const i32; //或者let b = &a as *const i32；
// 隐式
let c: *const i32 = &a;
unsafe {
	println!("{}", *c);
}

```
