# 栈

## 栈简介

- 栈作为一种数据结构，是一种只能在**一端**进行**插入**和**删除**操作的特殊线性表。

- 它按照**先进后出**的原则存储数据，先进入的数据被压入栈底，最后的数据在栈顶，需要读数据的时候从栈顶开始弹出数据（最后一个数据被第一个读出来）。

>栈（stack）又名堆栈，它是一种运算受限的线性表。其限制是仅允许在表的一端进行插入和删除运算。这一端被称为栈顶，相对地，把另一端称为栈底。向一个栈插入新元素又称作进栈、入栈或压栈，它是把新元素放到栈顶元素的上面，使之成为新的栈顶元素；从一个栈删除元素又称作出栈或退栈，它是把栈顶元素删除掉，使其相邻的元素成为新的栈顶元素。

## 栈的实现步骤：

- 定义一个栈结构`Stack`
- 定义组成栈结构的栈点`StackNode`
- 实现栈的初始化函数`new( )`
- 实现进栈函数`push( )`
- 实现退栈函数`pop( )`

## 定义一个栈结构`Stack`

```rust
#[derive(Debug)]
struct Stack<T> {
    top: Option<Box<StackNode<T>>>,
}
```

让我们一步步来分析

- 第一行的`#[derive(Debug)]`是为了让`Stack`结构体可以打印调试。
- 第二行是定义了一个`Stack`结构体，这个结构体包含一个泛型参数`T`。
- 第三行比较复杂，在定义`StackNode`的时候介绍

## 定义组成栈结构的栈点`StackNode`

```rust
#[derive(Clone,Debug)]
struct StackNode<T> {
    val: T,
    next: Option<Box<StackNode<T>>>,
}
```

在这段代码的第三行， 我们定义了一个`val`保存`StackNode`的值。

>现在我们重点来看看第四行：
我们**从里到外**拆分来看看，首先是`Box<StackNode<T>`，这里的`Box`是 Rust 用来显式分配堆内存的类型：

> `pub struct Box<T> where T: ?Sized(_);`  
[详细文档请参考Rust的标准库](http://doc.rust-lang.org/nightly/std/boxed/struct.Box.html)

> 在 Rust 里面用强大的类型系统做了统一的抽象。在这里相当于在堆空间里申请了一块内存保存`StackNode<T>`。  

> **为什么要这么做了？如果不用Box封装会怎么样呢？**  

> 如果不用 Box 封装，rustc 编译器会报错，在 Rust 里面，rustc 默认使用栈空间，但是这里的`StackNode`定义的时候使用了递归的数据结构，next 属性的类型是 `StackNode<T>`，而这个类型是无法确定大小的，所有这种无法确定大小的类型，都不能保存在栈空间。所以需要使用`Box`来封装。这样的话`next`的类型就是一个指向某一块堆空间的指针，而指针是可以确定大小的，因此能够保存在栈空间。  

> **那么为什么还需要使用`Option`来封装呢？**  

> `Option`是 Rust 里面的一个抽象类型，定义如下：  
>

```rust
pub enum Option<T> {
    None,
    Some(T),
}
```

Option 里面包括元素，None 和 Some(T) ，这样就很轻松的描述了 next 指向栈尾的元素的时候，都是在 Option 类型下，方便了功能实现，也方便了错误处理。Option 还有很多强大的功能，读者可以参考下面几个连接：

[Option标准库文档](http://doc.rust-lang.org/nightly/std/option/enum.Option.html)

[Error Handling in Rust](http://blog.burntsushi.net/rust-error-handling/)

[rustbyexample 的 Options with Results部分](http://rustbyexample.com/error/option_with_result.html)

## 实现 `new( ) push( ) pop( )`
接下来是实现 stack 的主要功能了。

```rust
impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack{ top: None }
    }

    fn push(&mut self, val: T) {
        let mut node = StackNode::new(val);
        let next = self.top.take();
        node.next = next;
        self.top = Some(Box::new(node));
    }

    fn pop(&mut self) -> Option<T> {
        let val = self.top.take();
        match val {
            None => None,
            Some(mut x) => {
                self.top = x.next.take();
                Some(x.val)
            },
        }
    }
}
```

- `new( )`比较简单，Stack 初始化的时候为空，栈顶元素 `top` 就没有任何值，所以 `top` 为 `None`。

- `push( )`的主要功能是往栈里面推入元素，把新的 StackNode 指向 Stack 里面旧的值，同时更新 Stack 栈顶指向新进来的值。
> 这里有个需要注意的地方是第8行代码里面，`let next = self.top.take();`，使用了 Option 类型的 take 方法：  
`fn take(&mut self) -> Option<T>`
它会把 Option 类型的值取走，并把它的元素改为 None

- `pop( )`的功能是取出栈顶的元素，如果栈顶为 None 则返回 None。

## 完整代码（包含简单的测试）

```rust
#[derive(Debug)]
struct Stack<T> {
    top: Option<Box<StackNode<T>>>,
}

#[derive(Clone,Debug)]
struct StackNode<T> {
    val: T,
    next: Option<Box<StackNode<T>>>,
}

impl <T> StackNode<T> {
    fn new(val: T) -> StackNode<T> {
        StackNode { val: val, next: None }
    }
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack{ top: None }
    }

    fn push(&mut self, val: T) {
        let mut node = StackNode::new(val);
        let next = self.top.take();
        node.next = next;
        self.top = Some(Box::new(node));
    }

    fn pop(&mut self) -> Option<T> {
        let val = self.top.take();
        match val {
            None => None,
            Some(mut x) => {
                self.top = x.next.take();
                Some(x.val)
            },
        }
    }
}

fn main() {
    #[derive(PartialEq,Eq,Debug)]
    struct TestStruct {
        a: i32,
    }

    let a = TestStruct{ a: 5 };
    let b = TestStruct{ a: 9 };

    let mut s = Stack::<&TestStruct>::new();
    assert_eq!(s.pop(), None);

    s.push(&a);
    s.push(&b);
    println!("{:?}", s);

    assert_eq!(s.pop(), Some(&b));
    assert_eq!(s.pop(), Some(&a));
    assert_eq!(s.pop(), None);
}
```
