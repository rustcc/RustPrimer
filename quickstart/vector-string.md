# 数组、动态数组和字符串
## 数组和动态数组
### 数组 array
Rust 使用数组存储相同类型的数据集。
`[T; N]`表示一个拥有 T 类型，N 个元素的数组。数组的大小是固定。

**例子：**

```rust
fn main() {
    let mut array: [i32; 3] = [0; 3];

    array[1] = 1;
    array[2] = 2;

    assert_eq!([1, 2], &array[1..]);

    // This loop prints: 0 1 2
    for x in &array {
        println!("{} ", x);
    }
}
```

### 动态数组 Vec
动态数组是一种基于堆内存申请的连续动态数据类型，拥有 O(1) 时间复杂度的索引、压入（push）、弹出（pop)。

**例子：**

```rust
//创建空Vec
let v: Vec<i32> = Vec::new();
//使用宏创建空Vec
let v: Vec<i32> = vec![];
//创建包含5个元素的Vec
let v = vec![1, 2, 3, 4, 5];
//创建十个零
let v = vec![0; 10];
//创建可变的Vec，并压入元素3
let mut v = vec![1, 2];
v.push(3);
//创建拥有两个元素的Vec，并弹出一个元素
let mut v = vec![1, 2];
let two = v.pop();
//创建包含三个元素的可变Vec，并索引一个值和修改一个值
let mut v = vec![1, 2, 3];
let three = v[2];
v[1] = v[1] + 5;
```

## 字符串
Rust 里面有两种字符串类型。`String` 和 `str`。

### &str
`str` 类型基本上不怎么使用，通常使用 `&str` 类型，它其实是 `[u8]` 类型的切片形式 `&[u8]`。这是一种固定大小的字符串类型。
常见的的字符串字面值就是 `&'static str` 类型。这是一种带有 `'static` 生命周期的 &str 类型。

**例子：**

```rust
// 字符串字面值
let hello = "Hello, world!";

// 附带显式类型标识
let hello: &'static str = "Hello, world!";
```

### String
`String` 是一个带有的 `vec:Vec<u8>` 成员的结构体，你可以理解为 `str` 类型的动态形式。
它们的关系相当于 `[T]` 和 `Vec<T>` 的关系。
显然 `String` 类型也有压入和弹出。

**例子：**

```rust
// 创建一个空的字符串
let mut s = String::new();
// 从 `&str` 类型转化成 `String` 类型
let mut hello = String::from("Hello, ");
// 压入字符和压入字符串切片
hello.push('w');
hello.push_str("orld!");

// 弹出字符。
let mut s = String::from("foo");
assert_eq!(s.pop(), Some('o'));
assert_eq!(s.pop(), Some('o'));
assert_eq!(s.pop(), Some('f'));
assert_eq!(s.pop(), None);
```
