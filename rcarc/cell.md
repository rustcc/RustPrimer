# Cell, RefCell

前面我们提到，Rust 通过其所有权机制，严格控制拥有和借用关系，来保证程序的安全，并且这种安全是在编译期可计算、可预测的。但是这种严格的控制，有时也会带来灵活性的丧失，有的场景下甚至还满足不了需求。

因此，Rust 标准库中，设计了这样一个系统的组件：`Cell`, `RefCell`，它们弥补了 Rust 所有权机制在灵活性上和某些场景下的不足。同时，又没有打破 Rust 的核心设计。它们的出现，使得 Rust 革命性的语言理论设计更加完整，更加实用。

具体是因为，它们提供了 `内部可变性`（相对于标准的 `继承可变性` 来讲的）。

通常，我们要修改一个对象，必须

1. 成为它的拥有者，并且声明 `mut`；
2. 或 以 `&mut` 的形式，借用；

而通过 `Cell`, `RefCell`，我们可以在需要的时候，就可以修改里面的对象。而不受编译期静态借用规则束缚。

## `Cell`

`Cell` 有如下特点：

1. `Cell<T>` 只能用于 `T` 实现了 `Copy` 的情况；

### `.get()`

`.get()` 方法，返回内部值的一个拷贝。比如：

```rust
use std::cell::Cell;

let c = Cell::new(5);

let five = c.get();
```

### `.set()`

`.set()` 方法，更新值。

```rust
use std::cell::Cell;

let c = Cell::new(5);

c.set(10);
```


## `RefCell`

相对于 `Cell` 只能包裹实现了 `Copy` 的类型，`RefCell` 用于更普遍的情况（其它情况都用 `RefCell`）。

相对于标准情况的 `静态借用`，`RefCell` 实现了 `运行时借用`，这个借用是临时的。这意味着，编译器对 `RefCell` 中的内容，不会做静态借用检查，也意味着，出了什么问题，用户自己负责。

`RefCell` 的特点：

1. 在不确定一个对象是否实现了 `Copy` 时，直接选 `RefCell`；
2. 如果被包裹对象，同时被可变借用了两次，则会导致线程崩溃。所以需要用户自行判断；
3. `RefCell` 只能用于线程内部，不能跨线程；
4. `RefCell` 常常与 `Rc` 配合使用（都是单线程内部使用）；

我们来看实例：

```rust
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));
    shared_map.borrow_mut().insert("africa", 92388);
    shared_map.borrow_mut().insert("kyoto", 11837);
    shared_map.borrow_mut().insert("piccadilly", 11826);
    shared_map.borrow_mut().insert("marbles", 38);
}
```

从上例可看出，用了 `RefCell` 后，外面是 `不可变引用` 的情况，一样地可以修改被包裹的对象。

常用方法
### `.borrow()`
不可变借用被包裹值。同时可存在多个不可变借用。

比如：

```rust
use std::cell::RefCell;

let c = RefCell::new(5);

let borrowed_five = c.borrow();
let borrowed_five2 = c.borrow();
```

下面的例子会崩溃：

```rust
use std::cell::RefCell;
use std::thread;

let result = thread::spawn(move || {
   let c = RefCell::new(5);
   let m = c.borrow_mut();

   let b = c.borrow(); // this causes a panic
}).join();

assert!(result.is_err());
```

### `.borrow_mut()`

可变借用被包裹值。同时只能有一个可变借用。

比如：

```rust
use std::cell::RefCell;

let c = RefCell::new(5);

let borrowed_five = c.borrow_mut();
```

下面的例子会崩溃：

```rust
use std::cell::RefCell;
use std::thread;

let result = thread::spawn(move || {
   let c = RefCell::new(5);
   let m = c.borrow();

   let b = c.borrow_mut(); // this causes a panic
}).join();

assert!(result.is_err());
```

### `.into_inner()`

取出包裹值。

```rust
use std::cell::RefCell;

let c = RefCell::new(5);

let five = c.into_inner();
```

## 一个综合示例

下面这个示例，表述的是如何实现两个对象的循环引用。综合演示了 `Rc`, `Weak`, `RefCell` 的用法

```rust

use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

struct Owner {
    name: String,
    gadgets: RefCell<Vec<Weak<Gadget>>>,
    // 其他字段
}

struct Gadget {
    id: i32,
    owner: Rc<Owner>,
    // 其他字段
}

fn main() {
    // 创建一个可计数的Owner。
    // 注意我们将gadgets赋给了Owner。
    // 也就是在这个结构体里， gadget_owner包含gadets
    let gadget_owner : Rc<Owner> = Rc::new(
        Owner {
            name: "Gadget Man".to_string(),
            gadgets: RefCell::new(Vec::new()),
        }
    );

    // 首先，我们创建两个gadget，他们分别持有 gadget_owner 的一个引用。
    let gadget1 = Rc::new(Gadget{id: 1, owner: gadget_owner.clone()});
    let gadget2 = Rc::new(Gadget{id: 2, owner: gadget_owner.clone()});

    // 我们将从gadget_owner的gadgets字段中持有其可变引用
    // 然后将两个gadget的Weak引用传给owner。
    gadget_owner.gadgets.borrow_mut().push(Rc::downgrade(&gadget1));
    gadget_owner.gadgets.borrow_mut().push(Rc::downgrade(&gadget2));

    // 遍历 gadget_owner的gadgets字段
    for gadget_opt in gadget_owner.gadgets.borrow().iter() {

        // gadget_opt 是一个 Weak<Gadget> 。 因为 weak 指针不能保证他所引用的对象
        // 仍然存在。所以我们需要显式的调用 upgrade() 来通过其返回值(Option<_>)来判
        // 断其所指向的对象是否存在。
        // 当然，这个Option为None的时候这个引用原对象就不存在了。
        let gadget = gadget_opt.upgrade().unwrap();
        println!("Gadget {} owned by {}", gadget.id, gadget.owner.name);
    }

    // 在main函数的最后， gadget_owner, gadget1和daget2都被销毁。
    // 具体是，因为这几个结构体之间没有了强引用（`Rc<T>`），所以，当他们销毁的时候。
    // 首先 gadget1和gadget2被销毁。
    // 然后因为gadget_owner的引用数量为0，所以这个对象可以被销毁了。
    // 循环引用问题也就避免了
}
```
