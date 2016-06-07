# 队列

## 队列简介
队列是一种特殊的线性表，特殊之处在于它只允许在表的前端（front）进行删除操作，而在表的后端（rear）进行插入操作，和栈一样，队列是一种操作受限制的线性表。进行插入操作的端称为队尾，进行删除操作的端称为队头。队列中没有元素时，称为空队列。

>在队列的形成过程中，可以利用线性链表的原理，来生成一个队列。基于链表的队列，要动态创建和删除节点，效率较低，但是可以动态增长。队列采用的 **FIFO(first in first out)**，新元素（等待进入队列的元素）总是被插入到链表的尾部，而读取的时候总是从链表的头部开始读取。每次读取一个元素，释放一个元素。所谓的动态创建，动态释放。因而也不存在溢出等问题。由于链表由结构体间接而成，遍历也方便。

## 队列实现
下面看一下我们使用 Vec 来实现的简单 Queue：

主要实现的`new( ), push( ), pop( )`三个方法

```rust
#[derive(Debug)]
struct Queue<T> {
    qdata: Vec<T>,
}

impl <T> Queue<T> {
    fn new() -> Self {
        Queue{qdata: Vec::new()}
    }

    fn push(&mut self, item:T) {
        self.qdata.push(item);
    }

    fn pop(&mut self) -> T{
        self.qdata.remove(0)
    }
}

fn main() {
    let mut q = Queue::new();
    q.push(1);
    q.push(2);
    println!("{:?}", q);
    q.pop();
    println!("{:?}", q);
    q.pop();
}
```

## 练习
看起来比我们在上一节实现的Stack简单多了。不过这个Queue实现是有Bug的。

练习：在这个代码的上找到 Bug，并修改。

提示：`pop( )`方法有 Bug，请参考 Stack 小节的实现，利用 Option 来处理。
