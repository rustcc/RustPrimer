# Rc 和 Arc

Rust 建立在所有权之上的这一套机制，它要求一个资源同一时刻有且只能有一个拥有所有权的绑定或 `&mut` 引用，这在大部分的情况下保证了内存的安全。但是这样的设计是相当严格的，在另外一些情况下，它限制了程序的书写，无法实现某些功能。因此，Rust 在 std 库中提供了额外的措施来补充所有权机制，以应对更广泛的场景。

默认 Rust 中，对一个资源，同一时刻，有且只有一个所有权拥有者。`Rc` 和 `Arc` 使用引用计数的方法，让程序在同一时刻，实现同一资源的多个所有权拥有者，多个拥有者共享资源。

## Rc
`Rc` 用于同一线程内部，通过 `use std::rc::Rc` 来引入。它有以下几个特点：

1. 用 `Rc` 包装起来的类型对象，是 `immutable` 的，即 不可变的。即你无法修改 `Rc<T>` 中的 `T` 对象，只能读；
2. 一旦最后一个拥有者消失，则资源会被自动回收，这个生命周期是在编译期就确定下来的；
3. `Rc` 只能用于同一线程内部，不能用于线程之间的对象共享（不能跨线程传递）；
4. `Rc` 实际上是一个指针，它不影响包裹对象的方法调用形式（即不存在先解开包裹再调用值这一说）。

例子：

```rust
use std::rc::Rc;

let five = Rc::new(5);
let five2 = five.clone();
let five3 = five.clone();

```

## Rc Weak

`Weak` 通过 `use std::rc::Weak` 来引入。

`Rc` 是一个引用计数指针，而 `Weak` 是一个指针，但不增加引用计数，是 `Rc` 的 weak 版。它有以下几个特点：

1. 可访问，但不拥有。不增加引用计数，因此，不会对资源回收管理造成影响；
2. 可由 `Rc<T>` 调用 `downgrade` 方法而转换成 `Weak<T>`；
3. `Weak<T>` 可以使用 `upgrade` 方法转换成 `Option<Rc<T>>`，如果资源已经被释放，则 Option 值为 `None`；
4. 常用于解决循环引用的问题。

例子：

```rust
use std::rc::Rc;

let five = Rc::new(5);

let weak_five = Rc::downgrade(&five);

let strong_five: Option<Rc<_>> = weak_five.upgrade();
```

## Arc

`Arc` 是原子引用计数，是 `Rc` 的多线程版本。`Arc` 通过 `std::sync::Arc` 引入。

它的特点：

1. `Arc` 可跨线程传递，用于跨线程共享一个对象；
2. 用 `Arc` 包裹起来的类型对象，对可变性没有要求；
3. 一旦最后一个拥有者消失，则资源会被自动回收，这个生命周期是在编译期就确定下来的；
4. `Arc` 实际上是一个指针，它不影响包裹对象的方法调用形式（即不存在先解开包裹再调用值这一说）；
5. `Arc` 对于多线程的共享状态**几乎是必须的**（减少复制，提高性能）。

示例：

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers);

    for _ in 0..10 {
        let child_numbers = shared_numbers.clone();

        thread::spawn(move || {
            let local_numbers = &child_numbers[..];

            // Work with the local numbers
        });
    }
}
```

### Arc Weak

与 `Rc` 类似，`Arc` 也有一个对应的 `Weak` 类型，从 `std::sync::Weak` 引入。

意义与用法与 `Rc Weak` 基本一致，不同的点是这是多线程的版本。故不再赘述。



## 一个例子

下面这个例子，表述的是如何实现多个对象同时引用另外一个对象。

```rust
use std::rc::Rc;

struct Owner {
    name: String
}

struct Gadget {
    id: i32,
    owner: Rc<Owner>
}

fn main() {
    // Create a reference counted Owner.
    let gadget_owner : Rc<Owner> = Rc::new(
        Owner { name: String::from("Gadget Man") }
    );

    // Create Gadgets belonging to gadget_owner. To increment the reference
    // count we clone the `Rc<T>` object.
    let gadget1 = Gadget { id: 1, owner: gadget_owner.clone() };
    let gadget2 = Gadget { id: 2, owner: gadget_owner.clone() };

    drop(gadget_owner);

    // Despite dropping gadget_owner, we're still able to print out the name
    // of the Owner of the Gadgets. This is because we've only dropped the
    // reference count object, not the Owner it wraps. As long as there are
    // other `Rc<T>` objects pointing at the same Owner, it will remain
    // allocated. Notice that the `Rc<T>` wrapper around Gadget.owner gets
    // automatically dereferenced for us.
    println!("Gadget {} owned by {}", gadget1.id, gadget1.owner.name);
    println!("Gadget {} owned by {}", gadget2.id, gadget2.owner.name);

    // At the end of the method, gadget1 and gadget2 get destroyed, and with
    // them the last counted references to our Owner. Gadget Man now gets
    // destroyed as well.
}
```
