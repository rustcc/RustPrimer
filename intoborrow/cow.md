# Cow

直译为奶牛！开玩笑。
`Cow` 是一个枚举类型，通过 `use std::borrow::Cow;` 引入。它的定义是 `Clone-on-write`，即写时克隆。本质上是一个智能指针。

它有两个可选值：
- `Borrowed`，用于包裹对象的引用（通用引用）；
- `Owned`，用于包裹对象的所有者；

`Cow` 提供

1. 对此对象的不可变访问（比如可直接调用此对象原有的不可变方法）；
2. 如果遇到需要修改此对象，或者需要获得此对象的所有权的情况，`Cow` 提供方法做克隆处理，并避免多次重复克隆。

`Cow` 的设计目的是提高性能（减少复制）同时增加灵活性，因为大部分情况下，业务场景都是读多写少。利用 `Cow`，可以用统一，规范的形式实现，需要写的时候才做一次对象复制。这样就可能会大大减少复制的次数。

它有以下几个要点需要掌握：

1. `Cow<T>` 能直接调用 `T` 的不可变方法，因为 `Cow` 这个枚举，实现了 `Deref`；
2. 在需要写 `T` 的时候，可以使用 `.to_mut()` 方法得到一个具有所有权的值的可变借用；
    1. 注意，调用 `.to_mut()` 不一定会产生克隆；
    2. 在已经具有所有权的情况下，调用 `.to_mut()` 有效，但是不会产生新的克隆；
    3. 多次调用 `.to_mut()` 只会产生一次克隆。
3. 在需要写 `T` 的时候，可以使用 `.into_owned()` 创建新的拥有所有权的对象，这个过程往往意味着内存拷贝并创建新对象；
    1. 如果之前 `Cow` 中的值是借用状态，调用此操作将执行克隆；
    2. 本方法，参数是`self`类型，它会“吃掉”原先的那个对象，调用之后原先的对象的生命周期就截止了，在 `Cow` 上不能调用多次；


## 举例

`.to_mut()` 举例

```rust
use std::borrow::Cow;

let mut cow: Cow<[_]> = Cow::Owned(vec![1, 2, 3]);

let hello = cow.to_mut();

assert_eq!(hello, &[1, 2, 3]);
```

`.into_owned()` 举例

```rust
use std::borrow::Cow;

let cow: Cow<[_]> = Cow::Owned(vec![1, 2, 3]);

let hello = cow.into_owned();

assert_eq!(vec![1, 2, 3], hello);
```

综合举例

```rust
use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>) {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // clones into a vector the first time (if not already owned)
            input.to_mut()[i] = -v;
        }
    }
}
```

## `Cow` 在函数返回值上的应用实例

题目：写一个函数，过滤掉输入的字符串中的所有空格字符，并返回过滤后的字符串。

对这个简单的问题，不用思考，我们都可以很快写出代码：

```rust
fn remove_spaces(input: &str) -> String {
   let mut buf = String::with_capacity(input.len());

   for c in input.chars() {
      if c != ' ' {
         buf.push(c);
      }
   }

   buf
}
```

设计函数输入参数的时候，我们会停顿一下，这里，用 `&str` 好呢，还是 `String` 好呢？思考一番，从性能上考虑，有如下结论：

1. 如果使用 `String`， 则外部在调用此函数的时候，
    1. 如果外部的字符串是 `&str`，那么，它需要做一次克隆，才能调用此函数；
    2. 如果外部的字符串是 `String`，那么，它不需要做克隆，就可以调用此函数。但是，一旦调用后，外部那个字符串的所有权就被 `move` 到此函数中了，外部的后续代码将无法再使用原字符串。
2. 如果使用 `&str`，则不存在上述两个问题。但可能会遇到生命周期的问题，需要注意。

继续分析上面的例子，我们发现，在函数体内，做了一次新字符串对象的生成和拷贝。

让我们来仔细分析一下业务需求。最坏的情况下，如果字符串中没有空白字符，那最好是直接原样返回。这种情况做这样一次对象的拷贝，完全就是浪费了。

于是我们心想改进这个算法。很快，又遇到了另一个问题，返回值是 `String` 的嘛，我不论怎样，要把 `&str` 转换成 `String` 返回，始终都要经历一次复制。于是我们快要放弃了。

好吧，`Cow`君这时出马了。奶牛君很快写出了如下代码：

```rust
use std::borrow::Cow;

fn remove_spaces<'a>(input: &'a str) -> Cow<'a, str> {
    if input.contains(' ') {
        let mut buf = String::with_capacity(input.len());

        for c in input.chars() {
            if c != ' ' {
                buf.push(c);
            }
        }

        return Cow::Owned(buf);
    }

    return Cow::Borrowed(input);
}

```

完美解决了业务逻辑与返回值类型冲突的问题。本例可细细品味。

外部程序，拿到这个 `Cow` 返回值后，按照我们上文描述的 `Cow` 的特性使用就好了。
