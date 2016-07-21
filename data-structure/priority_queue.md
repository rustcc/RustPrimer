# 优先队列

## 简介
普通的队列是一种先进先出的数据结构，元素在队列尾追加，而从队列头删除。在优先队列中，元素被赋予优先级。当访问元素时，具有最高优先级的元素最先删除。优先队列具有最高级先出 （largest-in，first-out）的行为特征。

>优先队列是0个或多个元素的集合，每个元素都有一个优先权或值，对优先队列执行的操作有：

1. 查找;
2. 插入一个新元素;
3. 删除。

在最小优先队列(min priority queue)中，查找操作用来搜索优先权最小的元素，删除操作用来删除该元素；对于最大优先队列(max priority queue)，查找操作用来搜索优先权最大的元素，删除操作用来删除该元素。优先权队列中的元素可以有相同的优先权，查找与删除操作可根据任意优先权进行。

## 优先队列的实现：

首先定义 PriorityQueue 结构体

```rust
#[derive(Debug)]
struct PriorityQueue<T> where T: PartialOrd + Clone {
    pq: Vec<T>
}
```

第二行的`where T: PartialOrd + Clone`指的是 PriorityQueue 存储的泛型 T 是满足 `PartialOrd` 和 `Clone` trait 约束的，意味着泛型 T 是可排序和克隆的。

后面是一些基本的方法实现，比较简单，就直接看代码吧。这个优先队列是基于Vec实现的，有O(1)的插入和O(n)的最大/最小值出列。

```rust
impl<T> PriorityQueue<T> where T: PartialOrd + Clone {
    fn new() -> PriorityQueue<T> {
        PriorityQueue { pq: Vec::new() }
    }

    fn len(&self) -> usize {
        self.pq.len()
    }

    fn is_empty(&self) -> bool {
        self.pq.len() == 0
    }

    fn insert(&mut self, value: T) {
        self.pq.push(value);
    }

    fn max(&mut self) -> Option<T> {
        if self.is_empty() { return None }
        let max = self.max_index();
        Some(self.pq[max].clone())
    }

    fn min(&mut self) -> Option<T> {
        if self.is_empty() { return None }
        let min = self.min_index();
        Some(self.pq[min].clone())
    }

    fn delete_max(&mut self) -> Option<T> {
        if self.is_empty() { return None; }
        let max = self.max_index();
        Some(self.pq.remove(max).clone())
    }

    fn delete_min(&mut self) -> Option<T> {
        if self.is_empty() { return None; }
        let min = self.min_index();
        Some(self.pq.remove(min).clone())
    }

    fn max_index(&self) -> usize {
        let mut max = 0;
        for i in 1..self.pq.len() - 1 {
            if self.pq[max] < self.pq[i] {
                max = i;
            }
        }
        max
    }

    fn min_index(&self) -> usize {
        let mut min = 0;
        for i in 0..self.pq.len() - 1 {
            if self.pq[i] < self.pq[i + 1] {
                min = i;
            }
        }
        min
    }
}
```

## 测试代码：

```rust
fn test_keep_min() {
    let mut pq = PriorityQueue::new();
    pq.insert(3);
    pq.insert(2);
    pq.insert(1);
    pq.insert(4);
    assert!(pq.min().unwrap() == 1);
}

fn test_keep_max() {
    let mut pq = PriorityQueue::new();
    pq.insert(2);
    pq.insert(4);
    pq.insert(1);
    pq.insert(3);
    assert!(pq.max().unwrap() == 4);
}

fn test_is_empty() {
    let mut pq = PriorityQueue::new();
    assert!(pq.is_empty());
    pq.insert(1);
    assert!(!pq.is_empty());
}

fn test_len() {
    let mut pq = PriorityQueue::new();
    assert!(pq.len() == 0);
    pq.insert(2);
    pq.insert(4);
    pq.insert(1);
    assert!(pq.len() == 3);
}

fn test_delete_min() {
    let mut pq = PriorityQueue::new();
    pq.insert(3);
    pq.insert(2);
    pq.insert(1);
    pq.insert(4);
    assert!(pq.len() == 4);
    assert!(pq.delete_min().unwrap() == 1);
    assert!(pq.len() == 3);
}

fn test_delete_max() {
    let mut pq = PriorityQueue::new();
    pq.insert(2);
    pq.insert(10);
    pq.insert(1);
    pq.insert(6);
    pq.insert(3);
    assert!(pq.len() == 5);
    assert!(pq.delete_max().unwrap() == 10);
    assert!(pq.len() == 4);
}

fn main() {
    test_len();
    test_delete_max();
    test_delete_min();
    test_is_empty();
    test_keep_max();
    test_keep_min();
}
```

## 练习
基于二叉堆实现一个优先队列，以达到O(1)的出列和O(log n)的入列
