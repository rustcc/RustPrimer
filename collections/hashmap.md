# 哈希表 HashMap

和动态数组`Vec`一样，哈希表(HashMap)也是Rust内置的集合类型之一，同属`std::collections`模块下。

它提供了一个平均复杂度为`O(1)`的查询方法，是实现快速搜索必备的类型之一。

这里呢，主要给大家介绍一下HashMap的几种典型用法。

## HashMap的要求

顾名思义, HashMap 要求一个可哈希（实现 Hash trait）的Key类型，和一个编译时知道大小的Value类型。
同时，Rust还要求你的Key类型必须是可比较的，在Rust中，你可以为你的类型轻易的加上编译器属性：

```rust
#[derive(PartialEq, Eq, Hash)]
```

这样，即可将你的类型转换成一个可以作为Hash的Key的类型。
但是，如果你想要自己实现`Hash`这个trait的话，你需要谨记两点：

* 1. 如果 Key1==Key2 ,那么一定有 Hash(Key1) == Hash(Key2)
* 2. 你的Hash函数本身不能改变你的Key值，否则将会引发一个逻辑错误（很难排查，遇到就完的那种）

什么？你看到 `std::hash::Hash` 这个 trait 中的函数没有`&mut self`的啊！但是，你不要忘了Rust中还有`Cell`和`RefCell`这种存在，他们提供了不可变对象的内部可变性，具体怎么变呢，请参照第20章。

另外，要保证你写的Hash函数不会被很轻易的碰撞，即 `Key1! = Key2`，但 `Hash(Key1)==Hash(Key2)`，碰撞的严重了，HashMap甚至有可能退化成链表！

这里笔者提议，别费劲，就按最简单的来就好。

## 增删改查

对于这种实用的类型，我们推荐用一个例子来解释：

```rust
use std::collections::HashMap;

// 声明
let mut come_from = HashMap::new();
// 插入
come_from.insert("WaySLOG", "HeBei");
come_from.insert("Marisa", "U.S.");
come_from.insert("Mike", "HuoGuo");

// 查找key
if !come_from.contains_key("elton") {
    println!("Oh, 我们查到了{}个人，但是可怜的Elton猫还是无家可归", come_from.len());
}

// 根据key删除元素
come_from.remove("Mike");
println!("Mike猫的家乡不是火锅！不是火锅！不是火锅！虽然好吃！");

// 利用get的返回判断元素是否存在
let who = ["MoGu", "Marisa"];
for person in &who {
    match come_from.get(person) {
        Some(location) => println!("{} 来自: {}", person, location),
        None => println!("{} 也无家可归啊.", person),
    }
}

// 遍历输出
println!("那么，所有人呢？");
for (name, location) in &come_from {
    println!("{}来自: {}", name, location);
}
```

这段代码输出：

```
Oh, 我们查到了3个人，但是可怜的Elton猫还是无家可归
Mike猫的家乡不是火锅！不是火锅！不是火锅！虽然好吃！
MoGu 也无家可归啊.
Marisa 来自: U.S.
那么，所有人呢？
Marisa来自: U.S.
WaySLOG来自: HeBei
```

## entry

我们在编程的过程中，经常遇到这样的场景，统计一个字符串中所有的字符总共出现过几次。借助各种语言内置的Map类型我们总能完成这件事，但是完成的几乎都并不令人满意。很多人讨厌的一点是：为什么我要判断这个字符在字典中有没有出现，就要写一个大大的if条件！烦不烦？烦！于是，现代化的编程语言开始集成了类似Python里`setdefault`类似的特性（方法），下面是一段Python代码：

```python
val = {}
for c in "abcdefasdasdawe":
    val[c] = 1 + val.setdefault(c, 0)
print val
```

唔，总感觉怪怪的。那么Rust是怎么解决这个问题的呢？
以下内容摘自标注库api注释：

```rust
use std::collections::HashMap;

let mut letters = HashMap::new();

for ch in "a short treatise on fungi".chars() {
    let counter = letters.entry(ch).or_insert(0);
    *counter += 1;
}

assert_eq!(letters[&'s'], 2);
assert_eq!(letters[&'t'], 3);
assert_eq!(letters[&'u'], 1);
assert_eq!(letters.get(&'y'), None);
```

Rust为我们提供了一个名叫 `entry` 的api，它很有意思，和Python相比，我们不需要在一次迭代的时候二次访问原map，只需要借用 entry 出来的Entry类型（这个类型持有原有HashMap的引用）即可对原数据进行修改。就语法来说，毫无疑问Rust在这个方面更加直观和具体。
