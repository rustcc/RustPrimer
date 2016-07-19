## 共享内存
在消息传递之外，还存在一种广为人知的并发模型，那就是共享内存。其实如果不能共享内存，消息传递也是不能在不同的线程间传递消息，也谈不上在不同的线程间等待和通知了。共享内存是这一切得以发生的基础。如果查看源码，你会发现消息传递的内部实现就是借用了共享内存机制。相对于消息传递而言，共享内存会有更多的竞争，但是不用进行多次拷贝，在某些情况下，也需要考虑使用这种方式来处理。在Rust中，能共享内存的情况，主要体现在下面两个方面：

### static
Rust语言中也存在static变量，其生命周期是整个应用程序，并且在内存中某个固定地址处只存在一份实例。所有线程都能够访问到它。这种方式也是最简单和直接的共享方式。几乎大多数语言都存在这种机制。下面简单看一下Rust中多个线程访问static变量的用法：

```rust
use std::thread;

static VAR: i32 = 5;

fn main() {
	// 创建一个新线程
	let new_thread = thread::spawn(move|| {
	    println!("static value in new thread: {}", VAR);
	});

	// 等待新线程先运行
	new_thread.join().unwrap();
	println!("static value in main thread: {}", VAR);
}
```

运行结果：

```
static value in new thread: 5
static value in main thread: 5
```

`VAR`这个`static`变量在各线程中可以直接使用，非常方便。当然上面只是读取，那么要修改也是很简单的：

```rust
use std::thread;

static mut VAR: i32 = 5;

fn main() {
	// 创建一个新线程
	let new_thread = thread::spawn(move|| {
	    unsafe {
	    	println!("static value in new thread: {}", VAR);
	    	VAR = VAR + 1;
	    }
	});

	// 等待新线程先运行
	new_thread.join().unwrap();
	unsafe {
		println!("static value in main thread: {}", VAR);
	}
}
```

运行结果：

```
static value in new thread: 5
static value in main thread: 6
```

从结果来看`VAR`的值变了，从代码上来看，除了在`VAR`变量前面加了`mut`关键字外，更加明显的是在使用`VAR`的地方都添加了`unsafe`代码块。为什么？所有的线程都能访问`VAR`，且它是可以被修改的，自然就是不安全的。上面的代码比较简单，同一时间只会有一个线程读写`VAR`，不会有什么问题，所以用`unsafe`来标记就可以。如果是更多的线程，还是请使用接下来要介绍的同步机制来处理。

static如此，那const呢？ const会在编译时内联到代码中，所以不会存在某个固定的内存地址上，也不存在可以修改的情况，并不是内存共享的。

### 堆
由于现代操作系统的设计，线程寄生于进程，可以共享进程的资源，如果要在各个线程中共享一个变量，那么除了上面的static，还有就是把变量保存在堆上了。当然Rust也不例外，遵从这一设计。只是我们知道Rust在安全性上肯定又会做一些考量，从而在语言设计和使用上稍有不同。

为了在堆上分配空间，Rust提供了`std::boxed::Box`，由于堆的特点，存活时间比较长，所以除了我们这个地方介绍的线程间共享外，还有其他的用处，此处不详细说明，若不甚了解，请学习或回顾**堆、栈与Box**章节的介绍。下面我们来看一下如何在多个线程间访问`Box`创建的变量：

```rust
use std::thread;
use std::sync::Arc;

fn main() {
	let var : Arc<i32> = Arc::new(5);
	let share_var = var.clone();

	// 创建一个新线程
	let new_thread = thread::spawn(move|| {
		println!("share value in new thread: {}, address: {:p}", share_var, &*share_var);
	});

	// 等待新建线程先执行
	new_thread.join().unwrap();
	println!("share value in main thread: {}, address: {:p}", var, &*var);
}
```

运行结果：

```
share value in new thread: 5, address: 0x2825070
share value in main thread: 5, address: 0x2825070
```

你可能会觉得很奇怪，上面怎么没有看到Box创建的变量啊，这明明就是`Arc`的使用呀？`Box`创建的变量要想在多个线程中安全使用，我们还需要实现很多功能才行，需要是`Sync`，而`Arc`正是利用`Box`来实现的一个通过引用计数来共享状态的包裹类。下面引用一段`Arc::new`的源码即可看出它是通过`Box`来实现的：

```rust
pub fn new(data: T) -> Arc<T> {
    // Start the weak pointer count as 1 which is the weak pointer that's
    // held by all the strong pointers (kinda), see std/rc.rs for more info
    let x: Box<_> = box ArcInner {
        strong: atomic::AtomicUsize::new(1),
        weak: atomic::AtomicUsize::new(1),
        data: data,
    };
    Arc { _ptr: unsafe { NonZero::new(Box::into_raw(x)) } }
}
```

通过上面的运行结果，我们也可以发现新建线程和主线程中打印的`address`是一样的，说明状态确实是在同一个内存地址处。

如果`Box`在堆上分配的资源仅在一个线程中使用，那么释放时，就非常简单，使用完，及时释放即可。如果是要在多个线程中使用，就需要面临两个关键问题：

1. 资源何时释放？
2. 线程如何安全的并发修改和读取？

由于上面两个问题的存在，这就是为什么我们不能直接用`Box`变量在线程中共享的原因，可以看出来，共享内存比消息传递机制似乎要复杂许多。Rust用了引用计数的方式来解决第一个问题，在标准库中提供了两个包裹类，除了上面一个用于多线程的`std::sync::Arc`之外，还有一个不能用于多线程的`std::rc::Rc`。在使用时，可以根据需要进行选择。如果你一不小心把`std::rc::Rc`用于多线程中，编译器会毫不客气地纠正你的。

关于上面的第二个问题，Rust语言及标准库提供了一系列的同步手段来解决。下面的章节我们将详细讲解这些方式和用法。
