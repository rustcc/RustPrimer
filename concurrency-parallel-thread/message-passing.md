## 消息传递
稍加考虑，上一节的练习题其实是不完整的，它只是评分系统中的一环，一个评分系统是需要先把信息从数据库或文件中读取出来，然后才是评分，最后还需要把评分结果再保存到数据库或文件中去。如果一步一步串行地做这三个步骤，是完全没有问题的。那么我们是否可以用三个线程来分别做这三个步骤呢？上一节练习题我们已经用了一个线程来实现评分，那么我们是否也可以再用一个线程来读取成绩，再用另个线程来实现保存呢？ 如果能这样的话，那么我们就可以利用上多核多cpu的优势，加快整个评分的效率。既然在此提出这个问题，答案就很明显了。问题在于我们要怎么在Rust中来实现，关键在于三个线程怎么交换信息，以达到串行的逻辑处理顺序？

为了解决这个问题，下面将介绍一种Rust在标准库中支持的消息传递技术。**消息传递**是并发模型里面大家比较推崇的模式，不仅仅是因为使用起来比较简单，关键还在于它可以减少数据竞争，提高并发效率，为此值得深入学习。Rust是通过一个叫做通道(`channel`)的东西来实现这种模式的，下面直接进入主题。

### 初试通道(channel)
Rust的通道(`channel`)可以把一个线程的消息(数据)传递到另一个线程，从而让信息在不同的线程中流动，从而实现协作。详情请参见[`std::sync::mpsc`](https://doc.rust-lang.org/std/sync/mpsc/index.html)。通道的两端分别是发送者(`Sender`)和接收者(`Receiver`)，发送者负责从一个线程发送消息，接收者则在另一个线程中接收该消息。下面我们来看一个简单的例子：

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    // 创建一个通道
    let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = 
        mpsc::channel();

    // 创建线程用于发送消息
    thread::spawn(move || {
        // 发送一个消息，此处是数字id
        tx.send(1).unwrap();
    });

    // 在主线程中接收子线程发送的消息并输出
    println!("receive {}", rx.recv().unwrap());
}
```

程序说明参见代码中的注释，程序执行结果为：

```
receive 1
```

结果表明`main`所在的主线程接收到了新建线程发送的消息，用Rust在线程间传递消息就是这么简单！

虽然简单，但使用过其他语言就会知道，通道有多种使用方式，且比较灵活，为此我们需要进一步考虑关于`Rust`的`Channel`的几个问题：

1. 通道能保证消息的顺序吗？是否先发送的消息，先接收？
2. 通道能缓存消息吗？如果能的话能缓存多少？
3. 通道的发送者和接收者支持N:1，1:N，N:M模式吗？
4. 通道能发送任何数据吗？
5. 发送后的数据，在线程中继续使用没有问题吗？

让我们带着这些问题和思考进入下一个小节，那里有相关的答案。

### 消息类型
上面的例子中，我们传递的消息类型为`i32`，除了这种类型之外，是否还可以传递更多的原始类型，或者更复杂的类型，和自定义类型？下面我们尝试发送一个更复杂的`Rc`类型的消息：

```rust
use std::fmt;
use std::sync::mpsc;
use std::thread;
use std::rc::Rc;

pub struct Student {
    id: u32
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "student {}", self.id)
    }
}

fn main() {
    // 创建一个通道
    let (tx, rx): (mpsc::Sender<Rc<Student>>, mpsc::Receiver<Rc<Student>>) = 
        mpsc::channel();

    // 创建线程用于发送消息
    thread::spawn(move || {
        // 发送一个消息，此处是数字id
        tx.send(Rc::new(Student{
            id: 1,
        })).unwrap();
    });

    // 在主线程中接收子线程发送的消息并输出
    println!("receive {}", rx.recv().unwrap());
}
```

编译代码，奇迹没有出现，编译时错误，错误提示：

```
error: the trait `core::marker::Send` is not 
implemented for the type `alloc::rc::Rc<Student>` [E0277]
note: `alloc::rc::Rc<Student>` cannot be sent between threads safely
```

看来并不是所有类型的消息都可以通过通道发送，消息类型必须实现`marker trait Send`。Rust之所以这样强制要求，主要是为了解决并发安全的问题，再一次强调，**安全**是Rust考虑的重中之重。如果一个类型是`Send`，则表明它可以在线程间安全的转移所有权(`ownership`)，当所有权从一个线程转移到另一个线程后，同一时间就只会存在一个线程能访问它，这样就避免了数据竞争，从而做到线程安全。`ownership`的强大又一次显示出来了。通过这种做法，在编译时即可要求所有的代码必须满足这一约定，这种方式方法值得借鉴，`trait`也是非常强大。

看起来问题得到了完美的解决，然而由于`Send`本身是一个不安全的`marker trait`，并没有实际的`API`，所以实现它很简单，但没有强制保障，就只能靠开发者自己约束，否则还是可能引发并发安全问题。对于这一点，也不必太过担心，因为Rust中已经存在的类，都已经实现了`Send`或`!Send`，我们只要使用就行。`Send`是一个默认应用到所有Rust已存在类的trait，所以我们用`!Send`显式标明该类没有实现`Send`。目前几乎所有的原始类型都是`Send`，例如前面例子中发送的`i32`。对于开发者而言，我们可能会更关心哪些是非`Send`，也就是实现了`!Send`，因为这会导致线程不安全。更全面的信息参见[`Send`官网API](https://doc.rust-lang.org/std/marker/trait.Send.html)。

对于不是`Send`的情况（`!Send`），大致分为两类：

1. 原始指针，包括`*mut T`和`*const T`，因为不同线程通过指针都可以访问数据，从而可能引发线程安全问题。
2. `Rc`和`Weak`也不是，因为引用计数会被共享，但是并没有做并发控制。

虽然有这些`!Send`的情况，但是逃不过编译器的火眼金睛，只要你错误地使用了消息类型，编译器都会给出类似于上面的错误提示。我们要担心的不是这些，因为错误更容易出现在新创建的自定义类，有下面两点需要注意：

1. 如果自定义类的所有字段都是`Send`，那么这个自定义类也是`Send`。
    反之，如果有一个字段是`!Send`，那么这个自定义类也是`!Send`。
    如果类的字段存在递归包含的情况，按照该原则以此类推来推论类是`Send`还是`!Send`。

2. 在为一个自定义类实现`Send`或者`!Send`时，必须确保符合它的约定。

到此，消息类型的相关知识已经介绍完了，说了这么久，也该让大家自己练习一下了：请实现一个自定义类，该类包含一个Rc字段，让这个类变成可以在通道中发送的消息类型。

### 异步通道(Channel)
在粗略地尝试通道之后，是时候更深入一下了。Rust的标准库其实提供了两种类型的通道：异步通道和同步通道。上面的例子都是使用的异步通道，为此这一小节我们优先进一步介绍异步通道，后续再介绍同步通道。异步通道指的是：不管接收者是否正在接收消息，消息发送者在发送消息时都不会阻塞。为了验证这一点，我们尝试多增加一个线程来发送消息：

```rust
use std::sync::mpsc;
use std::thread;

// 线程数量
const THREAD_COUNT :i32 = 2;

fn main() {
    // 创建一个通道
    let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();

    // 创建线程用于发送消息
    for id in 0..THREAD_COUNT {
        // 注意Sender是可以clone的，这样就可以支持多个发送者
        let thread_tx = tx.clone();
        thread::spawn(move || {
            // 发送一个消息，此处是数字id
            thread_tx.send(id + 1).unwrap();
            println!("send {}", id + 1);
        });
    }

    thread::sleep_ms(2000);
    println!("wake up");
    // 在主线程中接收子线程发送的消息并输出
    for _ in 0..THREAD_COUNT {
        println!("receive {}", rx.recv().unwrap());
    }
}
```

运行结果:

```
send 1
send 2
wake up
receive 1
receive 2
```

在代码中，我们故意让`main`所在的主线程睡眠2秒，从而让发送者所在线程优先执行，通过结果可以发现，发送者发送消息时确实没有阻塞。还记得在前面提到过很多关于通道的问题吗？从这个例子里面还发现什么没？除了不阻塞之外，我们还能发现另外的三个特征：

1.通道是可以同时支持多个发送者的，通过`clone`的方式来实现。
    这类似于`Rc`的共享机制。
    其实从`Channel`所在的库名`std::sync::mpsc`也可以知道这点。
    因为`mpsc`就是多生产者单消费者(Multiple Producers Single Consumer)的简写。
    可以有多个发送者,但只能有一个接收者，即支持的N:1模式。

2.异步通道具备消息缓存的功能，因为1和2是在没有接收之前就发了的，在此之后还能接收到这两个消息。

那么通道到底能缓存多少消息？在理论上是无穷的，尝试一下便知：

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    // 创建一个通道
    let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();

    // 创建线程用于发送消息
    let new_thread = thread::spawn(move || {
        // 发送无穷多个消息
        let mut i = 0;
        loop {
            i = i + 1;
            // add code here
            println!("send {}", i);
            match tx.send(i) {
                Ok(_) => (),
                Err(e) => {
                    println!("send error: {}, count: {}", e, i);
                    return;
                },
            }
        }
    });

    // 在主线程中接收子线程发送的消息并输出
    new_thread.join().unwrap();
    println!("receive {}", rx.recv().unwrap());
}
```

最后的结果就是耗费内存为止。

3.消息发送和接收的顺序是一致的，满足先进先出原则。

上面介绍的内容大多是关于发送者和通道的，下面开始考察一下接收端。通过上面的几个例子，细心一点的可能已经发现接收者的`recv`方法应该会阻塞当前线程，如果不阻塞，在多线程的情况下，发送的消息就不可能接收完全。所以没有发送者发送消息，那么接收者将会一直等待，这一点要谨记。在某些场景下，一直等待是符合实际需求的。但某些情况下并不需一直等待，那么就可以考虑释放通道，只要通道释放了，`recv`方法就会立即返回。

异步通道的具有良好的灵活性和扩展性，针对业务需要，可以灵活地应用于实际项目中，实在是必备良药！

### 同步通道
同步通道在使用上同异步通道一样，接收端也是一样的，唯一的区别在于发送端，我们先来看下面的例子：

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    // 创建一个同步通道
    let (tx, rx): (mpsc::SyncSender<i32>, mpsc::Receiver<i32>) = mpsc::sync_channel(0);

    // 创建线程用于发送消息
    let new_thread = thread::spawn(move || {
        // 发送一个消息，此处是数字id
        println!("before send");
        tx.send(1).unwrap();
        println!("after send");
    });

    println!("before sleep");
    thread::sleep_ms(5000);
    println!("after sleep");
    // 在主线程中接收子线程发送的消息并输出
    println!("receive {}", rx.recv().unwrap());
    new_thread.join().unwrap();
}
```

运行结果：

```
before sleep
before send
after sleep
receive 1
after send
```

除了多了一些输出代码之外，上面这段代码几乎和前面的异步通道的没有什么区别，唯一不同的在于创建同步通道的那行代码。同步通道是`sync_channel`，对应的发送者也变成了`SyncSender`。为了显示出同步通道的区别，故意添加了一些打印。和异步通道相比，存在两点不同：

1. 同步通道是需要指定缓存的消息个数的，但需要注意的是，最小可以是0，表示没有缓存。
2. 发送者是会被阻塞的。当通道的缓存队列不能再缓存消息时，发送者发送消息时，就会被阻塞。

对照上面两点和运行结果来分析，由于主线程在接收消息前先睡眠了，从而子线程这个时候会被调度执行发送消息，由于通道能缓存的消息为0，而这个时候接收者还没有接收，所以`tx.send(1).unwrap()`就会阻塞子线程，直到主线程接收消息，即执行`println!("receive {}", rx.recv().unwrap());`。运行结果印证了这点，要是没阻塞，那么在`before send`之后就应该是`after send`了。

相比较而言，异步通道更没有责任感一些，因为消息发送者一股脑的只管发送，不管接收者是否能快速处理。这样就可能出现通道里面缓存大量的消息得不到处理，从而占用大量的内存，最终导致内存耗尽。而同步通道则能避免这种问题，把接受者的压力能传递到发送者，从而一直传递下去。
