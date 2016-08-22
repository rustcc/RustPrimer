## 同步

同步指的是线程之间的协作配合，以共同完成某个任务。在整个过程中，需要注意两个关键点：一是共享资源的访问， 二是访问资源的顺序。通过前面的介绍，我们已经知道了如何让多个线程访问共享资源，但并没介绍如何控制访问顺序，才不会出现错误。如果两个线程同时访问同一内存地址的数据，一个写，一个读，如果不加控制，写线程只写了一半，读线程就开始读，必然读到的数据是错误的，不可用的，从而造成程序错误，这就造成了并发安全问题，为此我们必须要有一套控制机制来避免这样的事情发生。就好比两个人喝一瓶可乐，只有一根吸管，那肯定也得商量出一个规则，才能相安无事地都喝到可乐。本节就将具体介绍在Rust中，我们要怎么做，才能解决这个问题。

继续上面喝可乐的例子，一人一口的方式，就是一种解决方案，只要不是太笨，几乎都能想到这个方案。具体实施时，A在喝的时候，B一直在旁边盯着，要是A喝完一口，B马上拿过来喝，此时A肯定也是在旁边盯着。在现实生活中，这样的示例比比皆是。细想一下，貌似同步中都可能涉及到等待。诸葛先生在万事具备，只欠东风时，也只能等，因为条件不成熟啊。依照这个逻辑，在操作系统和各大编程语言中，几乎都支持当前线程等待，当然Rust也不例外。

### 等待
Rust中线程等待和其他语言在机制上并无差异，大致有下面几种：

* 等待一段时间后，再接着继续执行。看起来就像一个人工作累了，休息一会再工作。通过调用相关的API可以让当前线程暂停执行进入睡眠状态，此时调度器不会调度它执行，等过一段时间后，线程自动进入就绪状态，可以被调度执行，继续从之前睡眠时的地方执行。对应的API有`std::thread::sleep`，`std::thread::sleep_ms`，`std::thread::park_timeout`，`std::thread::park_timeout_ms`，还有一些类似的其他API，由于太多，详细信息就请参见官网[`std::thread`](https://doc.rust-lang.org/stable/std/thread/index.html)。
* 这一种方式有点特殊，时间非常短，就一个时间片，当前线程自己主动放弃当前时间片的调度，让调度器重新选择线程来执行，这样就把运行机会给了别的线程，但是要注意的是，如果别的线程没有更好的理由执行，当然最后执行机会还是它的。在实际的应用业务中，比如生产者制造出一个产品后，可以放弃一个时间片，让消费者获得执行机会，从而快速地消费才生产的产品。这样的控制粒度非常小，需要合理使用，如果需要连续放弃多个时间片，可以借用循环实现。对应的API是`std::thread::yield_now`，详细信息参见官网[`std::thread`](https://doc.rust-lang.org/stable/std/thread/index.html)。
* 1和2的等待都无须其他线程的协助，即可在一段时间后继续执行。最后我们还遇到一种等待，是需要其他线程参与，才能把等待的线程叫醒，否则，线程会一直等待下去。好比一个女人，要是没有遇到一个男人，就永远不可能摆脱单身的状态。相关的API包括`std::thread::JoinHandle::join`，`std::thread::park`，`std::sync::Mutex::lock`等，还有一些同步相关的类的API也会导致线程等待。详细信息参见官网[`std::thread`](https://doc.rust-lang.org/stable/std/thread/index.html)和[`std::sync`](https://doc.rust-lang.org/stable/std/sync/index.html)。

第一种和第三种等待方式，其实我们在上面的介绍中，都已经遇到过了，它们也是使用的最多的两种方式。在此，也可以回过头去看看前面的使用方式和使用效果，结合自己的理解，做一些简单的练习。

毫无疑问，第三种方式稍显复杂，要将等待的线程叫醒，必然基于一定的规则，比如早上7点必须起床，那么就定一个早上7点的闹钟，到时间了就响，没到时间别响。不管基于什么规则，要触发叫醒这个事件，就肯定是某个条件已经达成了。基于这样的逻辑，在操作系统和编程语言中，引入了一种叫着**条件变量**的东西。可以模拟现实生活中的闹钟的行为，条件达成就通知等待条件的线程。Rust的条件变量就是`std::sync::Condvar`，详情参见官网[条件变量](https://doc.rust-lang.org/std/sync/struct.Condvar.html)。但是通知也并不只是条件变量的专利，还有其他的方式也可以触发通知，下面我们就来瞧一瞧。

### 通知
看是简单的通知，在编程时也需要注意以下几点：

* 通知必然是因为有等待，所以通知和等待几乎都是成对出现的，比如`std::sync::Condvar::wait`和`std::sync::Condvar::notify_one`，`std::sync::Condvar::notify_all`。
* 等待所使用的对象，与通知使用的对象是同一个对象，从而该对象需要在多个线程之间共享，参见下面的例子。
* 除了`Condvar`之外，其实*锁*也是具有自动通知功能的，当持有锁的线程释放锁的时候，等待锁的线程就会自动被唤醒，以抢占锁。关于锁的介绍，在下面有详解。
* 通过条件变量和锁，还可以构建更加复杂的自动通知方式，比如`std::sync::Barrier`。
* 通知也可以是1:1的，也可以是1:N的，`Condvar`可以控制通知一个还是N个，而锁则不能控制，只要释放锁，所有等待锁的其他线程都会同时醒来，而不是只有最先等待的线程。

下面我们分析一个简单的例子：

```rust
use std::sync::{Arc, Mutex, Condvar};
use std::thread;

fn main() {

	let pair = Arc::new((Mutex::new(false), Condvar::new()));
	let pair2 = pair.clone();

	// 创建一个新线程
	thread::spawn(move|| {
	    let &(ref lock, ref cvar) = &*pair2;
	    let mut started = lock.lock().unwrap();
	    *started = true;
	    cvar.notify_one();
	    println!("notify main thread");
	});

	// 等待新线程先运行
	let &(ref lock, ref cvar) = &*pair;
	let mut started = lock.lock().unwrap();
	while !*started {
		println!("before wait");
	    started = cvar.wait(started).unwrap();
	    println!("after wait");
	}
}
```

运行结果：

```
before wait
notify main thread
after wait
```

这个例子展示了如何通过条件变量和锁来控制新建线程和主线程的同步，让主线程等待新建线程执行后，才能继续执行。从结果来看，功能上是实现了。对于上面这个例子，还有下面几点需要说明：

* `Mutex`是Rust中的一种锁。
* `Condvar`需要和`Mutex`一同使用，因为有`Mutex`保护，`Condvar`并发才是安全的。
* `Mutex::lock`方法返回的是一个`MutexGuard`，在离开作用域的时候，自动销毁，从而自动释放锁，从而避免锁没有释放的问题。
* `Condvar`在等待时，时会释放锁的，被通知唤醒时，会重新获得锁，从而保证并发安全。

到此，你应该对锁比较感兴趣了，为什么需要锁？锁存在的目的就是为了保证资源在同一个时间，能有序地被访问，而不会出现异常数据。但其实要做到这一点，也并不是只有锁，包括锁在内，主要涉及两种基本方式：

### 原子类型
原子类型是最简单的控制共享资源访问的一种机制，相比较于后面将介绍的锁而言，原子类型不需要开发者处理加锁和释放锁的问题，同时支持修改，读取等操作，还具备较高的并发性能，从硬件到操作系统，到各个语言，基本都支持。在标准库`std::sync::atomic`中，你将在里面看到Rust现有的原子类型，包括`AtomicBool`，`AtomicIsize`，`AtomicPtr`，`AtomicUsize`。这4个原子类型基本能满足百分之九十的共享资源安全访问的需要。下面我们就用原子类型，结合共享内存的知识，来展示一下一个线程修改，一个线程读取的情况：

```rust
use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
	let var : Arc<AtomicUsize> = Arc::new(AtomicUsize::new(5));
	let share_var = var.clone();

	// 创建一个新线程
	let new_thread = thread::spawn(move|| {
		println!("share value in new thread: {}", share_var.load(Ordering::SeqCst));
		// 修改值
		share_var.store(9, Ordering::SeqCst);
	});

	// 等待新建线程先执行
	new_thread.join().unwrap();
	println!("share value in main thread: {}", var.load(Ordering::SeqCst));
}
```

运行结果：

```
share value in new thread: 5
share value in main thread: 9
```

结果表明新建线程成功的修改了值，并在主线程中获取到了最新值，你也可以尝试使用其他的原子类型。此处我们可以思考一下，如果我们用`Arc::new(*mut Box<u32>)`是否也可以做到？ 为什么？ 思考后，大家将体会到Rust在多线程安全方面做的有多么的好。除了原子类型，我们还可以使用锁来实现同样的功能。

### 锁
在多线程中共享资源，除了原子类型之外，还可以考虑用锁来实现。在操作之前必须先获得锁，一把锁同时只能给一个线程，这样能保证同一时间只有一个线程能操作共享资源，操作完成后，再释放锁给等待的其他线程。在Rust中`std::sync::Mutex`就是一种锁。下面我们用`Mutex`来实现一下上面的原子类型的例子：

```rust
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
	let var : Arc<Mutex<u32>> = Arc::new(Mutex::new(5));
	let share_var = var.clone();

	// 创建一个新线程
	let new_thread = thread::spawn(move|| {
		let mut val = share_var.lock().unwrap();
		println!("share value in new thread: {}", *val);
		// 修改值
		*val = 9;
	});

	// 等待新建线程先执行
	new_thread.join().unwrap();
	println!("share value in main thread: {}", *(var.lock().unwrap()));
}
```

运行结果：

```
share value in new thread: 5
share value in main thread: 9
```

结果都一样，看来用`Mutex`也能实现，但如果从效率上比较，原子类型会更胜一筹。暂且不论这点，我们从代码里面看到，虽然有`lock`，但是并么有看到有类似于`unlock`的代码出现，并不是不需要释放锁，而是Rust为了提高安全性，已然在`val`销毁的时候，自动释放锁了。同时我们发现，为了修改共享的值，开发者必须要调用`lock`才行，这样就又解决了一个安全问题。不得不再次赞叹一下Rust在多线程方面的安全性做得真是太好了。如果是其他语言，我们要做到安全，必然得自己来实现这些。

为了保障锁使用的安全性问题，Rust做了很多工作，但从效率来看还不如原子类型，那么锁是否就没有存在的价值了？显然事实不可能是这样的，既然存在，那必然有其价值。它能解决原子类型锁不能解决的那百分之十的问题。我们再来看一下之前的一个例子：

```rust
use std::sync::{Arc, Mutex, Condvar};
use std::thread;

fn main() {

	let pair = Arc::new((Mutex::new(false), Condvar::new()));
	let pair2 = pair.clone();

	// 创建一个新线程
	thread::spawn(move|| {
	    let &(ref lock, ref cvar) = &*pair2;
	    let mut started = lock.lock().unwrap();
	    *started = true;
	    cvar.notify_one();
	    println!("notify main thread");
	});

	// 等待新线程先运行
	let &(ref lock, ref cvar) = &*pair;
	let mut started = lock.lock().unwrap();
	while !*started {
		println!("before wait");
	    started = cvar.wait(started).unwrap();
	    println!("after wait");
	}
}
```

代码中的`Condvar`就是条件变量，它提供了`wait`方法可以主动让当前线程等待，同时提供了`notify_one`方法，让其他线程唤醒正在等待的线程。这样就能完美实现顺序控制了。看起来好像条件变量把事都做完了，要`Mutex`干嘛呢？为了防止多个线程同时执行条件变量的`wait`操作，因为条件变量本身也是需要被保护的，这就是锁能做，而原子类型做不到的地方。

在Rust中，`Mutex`是一种独占锁，同一时间只有一个线程能持有这个锁。这种锁会导致所有线程串行起来，这样虽然保证了安全，但效率并不高。对于写少读多的情况来说，如果在没有写的情况下，都是读取，那么应该是可以并发执行的，为了达到这个目的，几乎所有的编程语言都提供了一种叫读写锁的机制，Rust中也存在，叫[`std::sync::RwLock`](https://doc.rust-lang.org/std/sync/struct.RwLock.html)，在使用上同`Mutex`差不多，在此就留给大家自行练习了。

同步是多线程编程的永恒主题，Rust已经为我们提供了良好的编程范式，并强加检查，即使你之前没有怎么接触过，用Rust也能编写出非常安全的多线程程序。
