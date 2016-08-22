# 循环

- for
- while
- loop
- break 与 continue
- label


## for

for 语句用于遍历一个迭代器。

```rust
for var in iterator {
    code
}
```

Rust 迭代器返回一系列的元素，每个元素是循环中的一次重复。然后它的值与 var 绑定，它在循环体中有效。每当循环体执行完后，我们从迭代器中取出下一个值，然后我们再重复一遍。当迭代器中不再有值时，for 循环结束。

比如：

```rust
for x in 0..10 {
    println!("{}", x); // x: i32
}
```

输出

```
0
1
2
3
4
5
6
7
8
9
```

不熟悉迭代器概念的同学可能傻眼了，下面不妨用 C 形式的 for 语句做下对比：

```rust
// C 语言的 for 循环例子
for (x = 0; x < 10; x++) {
    printf( "%d\n", x );
}
```

两者输出是相同的，那么，为何 Rust 要这样来设计 for 语句呢？

1. 简化边界条件的确定，减少出错；
2. 减少运行时边界检查，提高性能。

即使对于有经验的 C 语言开发者来说，要手动控制要循环的每个元素也都是复杂并且易于出错的。

for 语句就是迭代器遍历的语法糖。

上述迭代器的形式虽好，但是好像在循环过程中，少了索引信息。Rust 考虑到了这一点，当你需要记录你已经循环了多少次了的时候，你可以使用 `.enumerate()` 函数。比如：

```rust
for (i,j) in (5..10).enumerate() {
    println!("i = {} and j = {}", i, j);
}
```

输出：

```
i = 0 and j = 5
i = 1 and j = 6
i = 2 and j = 7
i = 3 and j = 8
i = 4 and j = 9
```

再比如：

```rust
let lines = "Content of line one
Content of line two
Content of line three
Content of line four".lines();
for (linenumber, line) in lines.enumerate() {
    println!("{}: {}", linenumber, line);
}
```

输出：

```
0: Content of line one
1: Content of line two
2: Content of line three
3: Content of line four
```

关于迭代器的知识，详见 **迭代器** 章节。

## while

Rust 提供了 while 语句，条件表达式为真时，执行语句体。当你不确定应该循环多少次时可选择 while。

```rust
while expression {
    code
}
```

比如：

```rust
let mut x = 5; // mut x: i32
let mut done = false; // mut done: bool

while !done {
    x += x - 3;

    println!("{}", x);

    if x % 5 == 0 {
        done = true;
    }
}
```

## loop

有一种情况，我们经常会遇到，就是写一个无限循环：

```rust
while true {
    // do something
}
```

针对这种情况，Rust 专门优化提供了一个语句 loop。

```rust
loop {
    // do something
}
```

`loop` 与 `while true` 的主要区别在编译阶段的静态分析。

比如说，如下代码：

```rust
let mut a;
loop {
     a = 1;
     // ... break ...
}
do_something(a)
```

如果是`loop`循环，编译器会正确分析出变量`a`会被正确初始化，而如果换成`while true`，则会发生编译错误。这个微小的区别也会影响生命周期分析。

## break 和 continue

与 C 语言类似，Rust 也提供了 break 和 continue 两个关键字用来控制循环的流程。

- break 用来跳出当前层的循环；
- continue 用来执行当前层的下一次迭代。

像上面那个 while 例子：

```rust
let mut x = 5;
let mut done = false;

while !done {
    x += x - 3;

    println!("{}", x);

    if x % 5 == 0 {
        done = true;
    }
}
```

可以优化成：

```rust
let mut x = 5;

loop {
    x += x - 3;

    println!("{}", x);

    if x % 5 == 0 { break; }
}
```

这样感觉更直观一点。

下面这个例子演示 continue 的用法：

```rust
for x in 0..10 {
    if x % 2 == 0 { continue; }

    println!("{}", x);
}
```

它的作用是打印出 `0~9` 的奇数。结果如下：

```
1
3
5
7
9
```

## label

你也许会遇到这样的情形，当你有嵌套的循环而希望指定你的哪一个 break 或 continue 该起作用。就像大多数语言，默认 break 或 continue 将会作用于当前层的循环。当你想要一个 break 或 continue 作用于一个外层循环，你可以使用标签来指定你的 break 或 continue 语句作用的循环。

如下代码只会在 x 和 y 都为奇数时打印他们：

```rust
'outer: for x in 0..10 {
    'inner: for y in 0..10 {
        if x % 2 == 0 { continue 'outer; } // continues the loop over x
        if y % 2 == 0 { continue 'inner; } // continues the loop over y
        println!("x: {}, y: {}", x, y);
    }
}
```
