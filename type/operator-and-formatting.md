# 操作符和格式化字符串

现在的Rust资料，无论是Book还是RustByExample都没有统一而完全的介绍Rust的操作符。一个很重要的原因就是，Rust的操作符号和C++大部分都是一模一样的。

## 一元操作符

顾名思义，一元操作符是专门对一个Rust元素进行操纵的操作符，主要包括以下几个:

* `-`: 取负，专门用于数值类型。
* `*`: 解引用。这是一个很有用的符号，和`Deref`（`DerefMut`）这个trait关联密切。
* `!`: 取反。取反操作相信大家都比较熟悉了，不多说了。有意思的是，当这个操作符对数字类型使用的时候，会将其每一位都置反！也就是说，你对一个`1u8`进行`!`的话你将会得到一个`254u8`。
* `&`和`&mut`: 租借，borrow。向一个owner租借其使用权，分别是租借一个只读使用权和读写使用权。

## 二元操作符

### 算数操作符

算数运算符都有对应的trait的，他们都在`std::ops`下：

* `+`: 加法。实现了`std::ops::Add`。
* `-`: 减法。实现了`std::ops::Sub`。
* `*`: 乘法。实现了`std::ops::Mul`。
* `/`: 除法。实现了`std::ops::Div`。
* `%`: 取余。实现了`std::ops::Rem`。

### 位运算符

和算数运算符差不多的是，位运算也有对应的trait。

* `&`: 与操作。实现了`std::ops::BitAnd`。
* `|`: 或操作。实现了`std::ops::BitOr`。
* `^`: 异或。实现了`std::ops::BitXor`。
* `<<`: 左移运算符。实现了`std::ops::Shl`。
* `>>`: 右移运算符。实现了`std::ops::Shr`。

### 惰性boolean运算符

逻辑运算符有三个，分别是`&&`、`||`、`!`。其中前两个叫做惰性boolean运算符，之所以叫这个名字。是因为在Rust里也会出现其他类C语言的逻辑短路问题。所以取了这么一个高大上然并卵的名字。
其作用和C语言里的一毛一样啊！哦，对了，有点不同的是Rust里这个运算符只能用在bool类型变量上。什么 `1 && 1` 之类的表达式给我死开。

### 比较运算符

比较运算符其实也是某些trait的语法糖啦，不同的是比较运算符所实现的trait只有两个`std::cmp::PartialEq`和`std::cmp::PartialOrd`

其中， `==`和`!=`实现的是`PartialEq`。
而，`<`、`>`、`>=`、`<=`实现的是`PartialOrd`。

边看本节边翻开标准库（好习惯，鼓励）的同学一定会惊奇的发现，不对啊，`std::cmp`这个mod下明明有四个trait，而且从肉眼上来看更符合逻辑的`Ord`和`Eq`岂不是更好？其实，Rust对于这四个trait的处理是很明确的。分歧主要存在于浮点类型。
熟悉IEEE的同学一定知道浮点数有一个特殊的值叫`NaN`，这个值表示未定义的一个浮点数。在Rust中可以用`0.0f32 / 0.0f32`来求得其值。那么问题来了，这个数他是一个确定的值，但是它表示的是一个不确定的数！那么 `NaN != NaN` 的结果是啥？标准告诉我们，是 `true` 。但是这么写又不符合`Eq`的定义里`total equal`(每一位一样两个数就一样)的定义。因此有了`PartialEq`这么一个定义，我们只支持部分相等好吧，NaN这个情况我就给它特指了。

为了普适的情况，Rust的编译器选择了`PartialOrd`和`PartialEq`来作为其默认的比较符号的trait。我们也就和中央保持一致就好。

## 类型转换运算符

其实这个并不算运算符，因为他是个单词`as`。

这个就是C语言中各位熟悉的显式类型转换了。

show u the code:

```rust
fn avg(vals: &[f64]) -> f64 {
    let sum: f64 = sum(vals);
    let num: f64 = len(vals) as f64;
    sum / num
}
```

## 重载运算符

上面说了很多trait。有人会问了，你说这么多干啥？

答，为了运算符重载！

Rust是支持运算符重载的（某咖啡语言哭晕在厕所）。

关于这部分呢，在本书的第30节会有很详细的叙述，因此在这里我就不铺开讲了，上个栗子给大家，仅作参考：

```rust
use std::ops::{Add, Sub};

#[derive(Copy, Clone)]
struct A(i32);

impl Add for A {
    type Output = A;
    fn add(self, rhs: A) -> A {
        A(self.0 - rhs.0)
    }
}

impl Sub for A {
    type Output = A;
    fn sub(self, rhs: A) -> A{
        A(self.0 + rhs.0)
    }
}

fn main() {
    let a1 = A(10i32);
    let a2 = A(5i32);
    let a3 = a1 + a2;
    println!("{}", (a3).0);
    let a4 = a1 - a2;
    println!("{}", (a4).0);
}

```

output:

```
5
15
```

# 格式化字符串

说起格式化字符串，Rust采取了一种类似Python里面format的用法，其核心组成是五个宏和两个trait:`format!`、`format_arg!`、`print!`、`println!`、`write!`;`Debug`、`Display`。

相信你们在写Rust版本的Hello World的时候用到了`print!`或者`println!`这两个宏，但是其实最核心的是`format!`，前两个宏只不过将`format!`的结果输出到了console而已。

那么，我们来探究一下`format!`这个神奇的宏吧。

在这里呢，列举`format!`的定义是没卵用的，因为太复杂。我只为大家介绍几种典型用法。学会了基本上就能覆盖你平时80%的需求。

首先我们来分析一下format的一个典型调用

```rust
fn main() {
    let s = format!("{1}是个有着{0:>0width$}KG重，{height:?}cm高的大胖子",
                    81, "wayslog", width=4, height=178);
    // 我被逼的牺牲了自己了……
    print!("{}", s);
}
```

我们可以看到，`format!`宏调用的时候参数可以是任意类型，而且是可以position参数和key-value参数混合使用的。但是要注意的一点是，key-value的值只能出现在position值之后并且不占position。例如例子里你用`3$`引用到的绝对不是`width`，而是会报错。
这里面关于参数稍微有一个规则就是，参数类型必须要实现 `std::fmt` mod 下的某些trait。比如我们看到原生类型大部分都实现了`Display`和`Debug`这两个宏，其中整数类型还会额外实现一个`Binary`，等等。

当然了，我们可以通过 `{:type}`的方式去调用这些参数。

比如这样：

```rust
format!("{:b}", 2);
// 调用 `Binary` trait
// Get : 10
format!("{:?}", "Hello");
// 调用 `Debug`
// Get : "Hello"
```

另外请记住：type这个地方为空的话默认调用的是`Display`这个trait。

关于`:`号后面的东西其实还有更多式子，我们从上面的`{0:>0width$}`来分析它。

首先`>`是一个语义，它表示的是生成的字符串向右对齐，于是我们得到了 `0081`这个值。与之相对的还有`<`(向左对齐)和`^`(居中)。

再接下来`0`是一种特殊的填充语法，他表示用0补齐数字的空位，要注意的是，当0作用于负数的时候，比如上面例子中wayslog的体重是-81，那么你最终将得到`-0081`;当然了，什么都不写表示用空格填充啦;在这一位上，还会出现`+`、`#`的语法，使用比较诡异，一般情况下用不上。

最后是一个组合式子`width$`，这里呢，大家很快就能认出来是表示后面key-value值对中的`width=4`。你们没猜错，这个值表示格式化完成后字符串的长度。它可以是一个精确的长度数值，也可以是一个以`$`为结尾的字符串，`$`前面的部分可以写一个key或者一个postion。

最后，你需要额外记住的是，在width和type之间会有一个叫精度的区域（可以省略不写如例子），他们的表示通常是以`.`开始的，比如`.4`表示小数点后四位精度。最让人遭心的是，你仍然可以在这个位置引用参数，只需要和上面width一样，用`.N$`来表示一个position的参数，但是就是不能引用key-value类型的。这一位有一个特殊用法，那就是`.*`，它不表示一个值，而是表示两个值！第一个值表示精确的位数，第二个值表示这个值本身。这是一种很尴尬的用法，而且极度容易匹配到其他参数。因此，我建议在各位能力或者时间不欠缺的时候尽量把格式化表达式用标准的形式写的清楚明白。尤其在面对一个复杂的格式化字符串的时候。

好了好了，说了这么多，估计你也头昏脑涨的了吧，下面来跟我写一下format宏的完整用法。仔细体会并提炼每一个词的意思和位置。

```
format_string := <text> [ format <text> ] *
format := '{' [ argument ] [ ':' format_spec ] '}'
argument := integer | identifier

format_spec := [[fill]align][sign]['#'][0][width]['.' precision][type]
fill := character
align := '<' | '^' | '>'
sign := '+' | '-'
width := count
precision := count | '*'
type := identifier | ''
count := parameter | integer
parameter := integer '$'
```

最后，留个作业吧。
给出参数列表如下：
`(500.0, 12, "ELTON", "QB", 4, CaiNiao="Mike")`

请写出能最后输出一下句子并且将参数*都*被用过*至少一遍*的格式化字符串，并自己去play实验一下。

```
rust.cc社区的唐Mike眼睛度数足有0500.0度却还是每天辛苦码代码才能赚到100个QB。
但是ELTON却只需睡  12  个小时就可以迎娶白富美了。
```
