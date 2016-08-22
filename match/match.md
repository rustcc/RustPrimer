# match关键字
模式匹配，多出现在函数式编程语言之中，为其复杂的类型系统提供一个简单轻松的解构能力。比如从enum等数据结构中取出数据等等，但是在书写上，相对比较复杂。我们来看一个例子:

```rust
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        },
        _ => println!("West"),
    };
}
```

这是一个没什么实际意义的程序，但是能清楚的表达出match的用法。看到这里，你肯定能想起一个常见的控制语句——`switch`。没错，match可以起到和switch相同的作用。不过有几点需要注意：

1. match所罗列的匹配，必须穷举出其所有可能。当然，你也可以用 **_** 这个符号来代表其余的所有可能性情况，就类似于switch中的`default`语句。
2. match的每一个分支都必须是一个表达式，并且，除非一个分支一定会触发panic，这些分支的所有表达式的最终返回值类型必须相同。

关于第二点，有的同学可能不明白。这么说吧，你可以把match整体视为一个表达式，既然是一个表达式，那么就一定能求得它的结果。因此，这个结果当然就可以被赋予一个变量咯。
看代码：

```rust
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    // let d_panic = Direction::South;
    let d_west = Direction::West;
    let d_str = match d_west {
        Direction::East => "East",
        Direction::North | Direction::South => {
            panic!("South or North");
        },
        _ => "West",
    };

    println!("{}", d_str);
}
```

## 解构初窥

match还有一个非常重要的作用就是对现有的数据结构进行解构，轻易的可以拿出其中的数据部分来。
比如，以下是比较常见的例子：

```rust
enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

fn main() {
    let action = Action::Say("Hello Rust".to_string());
    match action {
        Action::Say(s) => {
            println!("{}", s);
        },
        Action::MoveTo(x, y) => {
            println!("point from (0, 0) move to ({}, {})", x, y);
        },
        Action::ChangeColorRGB(r, g, _) => {
            println!("change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                r, g,
            );
        }
    }
}
```

有人说了，从这来看也并不觉得match有多神奇啊！别急，请看下一小节——>[模式](pattern.md)
