# 迭代器

## 从for循环讲起

我们在控制语句里学习了Rust的for循环语法，我们知道，Rust的for循环实际上和C语言的循环语法是不同的。这是为什么呢？其实，for循环不过是Rust编译器提供的语法糖！

首先，我们知道Rust有一个for循环能够依次对迭代器的任意元素进行访问，即：
```
for i in 1..10 {
    println!("{}", i);
}
```
这里我们知道， (1..10) 其本身是一个迭代器，我们能对这个迭代器调用 `.next()` 方法，因此，for循环就能完整的遍历一个循环。
而对于Vec来说：

```
let values = vec![1,2,3];
for x in values {
    println!("{}", x);
}
```
在上面的语法中，我们并没有显式的将一个Vec转换成一个迭代器，那么它是如何工作的呢？现在就打开标准库翻api的同学可能发现了,Vec本身并没有实现 `Iterator` ，也就是说，你无法对Vec本身调用 `.next()` 方法。但是，我们在搜索的时候，发现了Vec实现了 `IntoIterator` 的 trait。

其实，for循环真正循环的，并不是一个迭代器(Iterator)，真正在这个语法糖里起作用的，是 `IntoIterator` 这个 trait。

因此，上面的代码可以被展开成如下的等效代码(只是示意，不保证编译成功):

```
let values = vec![1, 2, 3];

{
    let result = match IntoIterator::into_iter(values) {
        mut iter => loop {
            match iter.next() {
                Some(x) => { println!("{}", x); },
                None => break,
            }
        },
    };
    result
}

```

在这个语法里，我们首先对Vec调用 `into_iter` 来判断其是否能被转换成一个迭代器，如果能，则进行迭代。

那么，迭代器自己怎么办？

为此，Rust在标准库里提供了一个实现：

```
impl<I: Iterator> IntoIterator for I
```
也就是说，Rust为所有的迭代器默认的实现了 `IntoIterator`，这个实现很简单，就是每次返回自己就好了。

也就是说：

任意一个 `Iterator` 都可以被用在 for 循环上！

### 无限迭代器

Rust支持通过省略高位的形式生成一个无限长度的自增序列，即：

```
let inf_seq = (1..).into_iter();
```
不过不用担心这个无限增长的序列撑爆你的内存，占用你的CPU，因为适配器的惰性的特性，它本身是安全的，除非你对这个序列进行collect或者fold！
不过，我想聪明如你，不会犯这种错误吧！
因此，想要应用这个，你需要用take或者take_while来截断他，必须？ 除非你将它当作一个生成器。当然了，那就是另外一个故事了。

## 消费者与适配器

说完了for循环，我们大致弄清楚了 `Interator` 和 `IntoInterator` 之间的关系。下面我们来说一说消费者和适配器。

消费者是迭代器上一种特殊的操作，其主要作用就是将迭代器转换成其他类型的值，而非另一个迭代器。

而适配器，则是对迭代器进行遍历，并且其生成的结果是另一个迭代器，可以被链式调用直接调用下去。

由上面的推论我们可以得出: *迭代器其实也是一种适配器！*

### 消费者

就像所有人都熟知的生产者消费者模型，迭代器负责生产，而消费者则负责将生产出来的东西最终做一个转化。一个典型的消费者就是collect。前面我们写过collect的相关操作，它负责将迭代器里面的所有数据取出，例如下面的操作：

```
let v = (1..20).collect(); //编译通不过的！
```

尝试运行上面的代码，却发现编译器并不让你通过。因为你没指定类型！指定什么类型呢？原来collect只知道将迭代器收集到一个实现了 `FromIterator` 的类型中去，但是，事实上实现这个 trait 的类型有很多（Vec, HashMap等），因此，collect没有一个上下文来判断应该将v按照什么样的方式收集！！

要解决这个问题，我们有两种解决办法：

1. 显式的标明v的类型:

```
let v: Vec<_> = (1..20).collect();
```

2. 显式的制定collect调用时的类型：

```
let v = (1..20).collect::<Vec<_>>();
```

当然，一个迭代器中还存在其他的消费者，比如取第几个值所用的 `.nth()`函数，还有用来查找值的 `.find()` 函数，调用下一个值的`next()`函数等等，这里限于篇幅我们不能一一介绍。所以，下面我们只介绍另一个比较常用的消费者—— fold 。

当然了，提起Rust里的名字你可能没啥感觉，其实，fold 函数，正是大名鼎鼎的 MapReduce 中的 Reduce 函数(稍微有点区别就是这个Reduce是带初始值的)。

fold函数的形式如下：

```
fold(base, |accumulator, element| .. )
```

我们可以写成如下例子：

```
let m = (1..20).fold(1u64, |mul, x| mul*x);
```
需要注意的是，fold的输出结果的类型，最终是和base的类型是一致的（如果base的类型没指定，那么可以根据前面m的类型进行反推，除非m的类型也未指定），也就是说，一旦我们将上面代码中的 base 从 `1u64` 改成 `1`，那么这行代码最终将会因为数据溢出而崩溃！

### 适配器

我们所熟知的生产消费的模型里，生产者所生产的东西不一定都会被消费者买账，因此，需要对原有的产品进行再组装。这个再组装的过程，就是适配器。因为适配器返回的是一个新的迭代器，可以直接用链式请求一直写下去，而不至于陷入到某前端语言的回调地狱之中。

前面提到了 Reduce 函数，那么自然不得不提一下另一个配套函数 —— map :

熟悉Python语言的同学肯定知道，Python里内置了一个map函数，可以将一个迭代器的值进行变换，成为另一种。Rust中的map函数实际上也是起的同样的作用，甚至连调用方法也惊人的相似！

```
(1..20).map(|x| x+1);
```

上面的代码展示了一个“迭代器所有元素的自加一”操作，但是，如果你尝试编译这段代码，编译器会给你提示：

```
warning: unused result which must be used: iterator adaptors are lazy and
         do nothing unless consumed, #[warn(unused_must_use)] on by default
(1..20).map(|x| x + 1);
 ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
```

呀，这是啥？

因为，所有的适配器，都是惰性求值的！都是惰性求值的！都是惰性求值的！

**也就是说，除非你调用一个消费者，不然，你的操作，永远也不会被调用到！**

现在，我们知道了map，那么熟悉Python的人又说了，是不是还有filter！？答，有……用法类似，filter接受一个闭包函数，返回一个布尔值，返回true的时候表示保留，false丢弃。

```
let v: Vec<_> = (1..20).filter(|x| x%2 == 0).collect();
```

以上代码表示筛选出所有的偶数。

## 其他

上文中我们了解了迭代器、适配器、消费者的基本概念。下面将以例子来介绍Rust中的其他的适配器和消费者。

### skip和take

take(n)的作用是取前n个元素，而skip(n)正好相反，跳过前n个元素。

```
let v = vec![1, 2, 3, 4, 5, 6];
let v_take = v.iter()
    .cloned()
    .take(2)
    .collect::<Vec<_>>();
assert_eq!(v_take, vec![1, 2]);

let v_skip: Vec<_> = v.iter()
    .cloned()
    .skip(2)
    .collect();
assert_eq!(v_skip, vec![3, 4, 5, 6]);
```

### zip 和 enumerate的恩怨情仇

zip是一个适配器，他的作用就是将两个迭代器的内容压缩到一起，形成 `Iterator<Item=(ValueFromA, ValueFromB)>` 这样的新的迭代器；

```
let names = vec!["WaySLOG", "Mike", "Elton"];
let scores = vec![60, 80, 100];
let score_map: HashMap<_, _> = names.iter()
    .zip(scores.iter())
    .collect();
println!("{:?}", score_map);
```

而enumerate, 熟悉的Python的同学又叫了：Python里也有！对的，作用也是一样的，就是把迭代器的下标显示出来，即：

```
let v = vec![1u64, 2, 3, 4, 5, 6];
let val = v.iter()
    .enumerate()
    // 迭代生成标，并且每两个元素剔除一个
    .filter(|&(idx, _)| idx % 2 == 0)
    // 将下标去除,如果调用unzip获得最后结果的话，可以调用下面这句，终止链式调用
    // .unzip::<_,_, vec<_>, vec<_>>().1
    .map(|(idx, val)| val)
    // 累加 1+3+5 = 9
    .fold(0u64, |sum, acm| sum + acm);

println!("{}", val);
```

### 一系列查找函数

Rust的迭代器有一系列的查找函数，比如：

* find(): 传入一个闭包函数，从开头到结尾依次查找能令这个闭包返回true的第一个元素，返回Option<Item>
* position(): 类似find函数，不过这次输出的是Option<usize>，第几个元素。
* all(): 传入一个函数，对所有元素调用这个函数，一旦有一个返回false,则整个表达式返回false，否则返回true
* any(): 类似all()，不过这次是任何一个返回true，则整个表达式返回true，否则false
* max()和min(): 查找整个迭代器里所有元素，返回最大或最小值的元素。注意：因为第七章讲过的PartialOrder的原因，浮点数无法参被max正确的理解


以上，为常用的一些迭代器和适配器及其用法，仅作科普，对于这一章。我希望大家能够多练习去理解，而不是死记硬背。

好吧，留个习题：

## 习题

利用迭代器生成一个升序的长度为10的水仙花数序列，然后对这个序列进行逆序,并输出
