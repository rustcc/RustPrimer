# 调用ffi函数

> 下文提到的ffi皆指cffi。

**Rust**作为一门**系统**级语言，自带对ffi调用的支持。

## Getting Start
### 引入libc库

由于`cffi`的数据类型与`rust`不完全相同，我们需要引入`libc`库来表达对应`ffi`函数中的类型。

在`Cargo.toml`中添加以下行:

```toml
[dependencies]
libc = "0.2.9"
```

在你的rs文件中引入库:

```rust
extern crate libc
```

在以前`libc`库是和`rust`一起发布的，后来libc被移入了`crates.io`通过cargo安装。

### 声明你的`ffi`函数

就像`c语言`需要`#include`声明了对应函数的头文件一样，`rust`中调用`ffi`也需要对对应函数进行声明。

```rust
use libc::c_int;
use libc::c_void;
use libc::size_t;

#[link(name = "yourlib")]
extern {
    fn your_func(arg1: c_int, arg2: *mut c_void) -> size_t; // 声明ffi函数
    fn your_func2(arg1: c_int, arg2: *mut c_void) -> size_t;
    static ffi_global: c_int; // 声明ffi全局变量
}
```

声明一个`ffi`库需要一个标记有`#[link(name = "yourlib")]`的`extern`块。`name`为对应的库(`so`/`dll`/`dylib`/`a`)的名字。
如：如果你需要`snappy`库(`libsnappy.so`/`libsnappy.dll`/`libsnappy.dylib`/`libsnappy.a`), 则对应的`name`为`snappy`。
在一个`extern块`中你可以声明任意多的函数和变量。

### 调用ffi函数

声明完成后就可以进行调用了。
由于此函数来自外部的c库，所以rust并不能保证该函数的安全性。因此，调用任何一个`ffi`函数需要一个`unsafe`块。

```rust
let result: size_t = unsafe {
    your_func(1 as c_int, Box::into_raw(Box::new(3)) as *mut c_void)
};
```

### 封装`unsafe`，暴露安全接口

作为一个库作者，对外暴露不安全接口是一种非常不合格的做法。在做c库的`rust binding`时，我们做的最多的将是将不安全的c接口封装成一个安全接口。
通常做法是：在一个叫`ffi.rs`之类的文件中写上所有的`extern块`用以声明ffi函数。在一个叫`wrapper.rs`之类的文件中进行包装：

```rust
// ffi.rs
#[link(name = "yourlib")]
extern {
    fn your_func(arg1: c_int, arg2: *mut c_void) -> size_t;
}
```

```rust
// wrapper.rs
fn your_func_wrapper(arg1: i32, arg2: &mut i32) -> isize {
    unsafe { your_func(1 as c_int, Box::into_raw(Box::new(3)) as *mut c_void) } as isize
}
```

对外暴露(pub use) `your_func_wrapper`函数即可。

## 数据结构对应

`libc`为我们提供了很多原始数据类型，比如`c_int`, `c_float`等，但是对于自定义类型，如结构体，则需要我们自行定义。

### 结构体

`rust`中结构体默认的内存表示和c并不兼容。如果要将结构体传给ffi函数，请为`rust`的结构体打上标记：

```rust
#[repr(C)]
struct RustObject {
    a: c_int,
    // other members
}
```

此外，如果使用`#[repr(C, packed)]`将不为此结构体填充空位用以对齐。

### Union

比较遗憾的是，rust到目前为止(2016-03-31)还没有一个很好的应对c的union的方法。只能通过一些hack来实现。([对应rfc](https://github.com/rust-lang/rfcs/pull/1444))

### Enum

和`struct`一样，添加`#[repr(C)]`标记即可。

### 回调函数

和c库打交道时，我们经常会遇到一个函数接受另一个回调函数的情况。将一个`rust`函数转变成c可执行的回调函数非常简单：在函数前面加上`extern "C"`:

```rust
extern "C" fn callback(a: c_int) { // 这个函数是传给c调用的
    println!("hello {}!", a);
}

#[link(name = "yourlib")]
extern {
   fn run_callback(data: i32, cb: extern fn(i32));
}

fn main() {
    unsafe {
        run_callback(1 as i32, callback); // 打印 1
    }
}
```

对应c库代码:

```c
typedef void (*rust_callback)(int32_t);

void run_callback(int32_t data, rust_callback callback) {
    callback(data); // 调用传过来的回调函数
}
```

### 字符串

rust为了应对不同的情况，有很多种字符串类型。其中`CStr`和`CString`是专用于`ffi`交互的。

#### CStr

对于产生于c的字符串(如在c程序中使用`malloc`产生)，rust使用`CStr`来表示，和`str`类型对应，表明我们并不拥有这个字符串。

```rust
use std::ffi::CStr;
use libc::c_char;
#[link(name = "yourlib")]
extern {
    fn char_func() -> *mut c_char;
}

fn get_string() -> String {
    unsafe {
        let raw_string: *mut c_char = char_func();
        let cstr = CStr::from_ptr(my_string());
        cstr.to_string_lossy().into_owned()
    }
}
```

在这里`get_string`使用`CStr::from_ptr`从c的`char*`获取一个字符串，并且转化成了一个String.

* 注意to_string_lossy()的使用：因为在rust中一切字符都是采用utf8表示的而c不是，
  因此如果要将c的字符串转换到rust字符串的话，需要检查是否都为有效`utf-8`字节。`to_string_lossy`将返回一个`Cow<str>`类型，
  即如果c字符串都为有效`utf-8`字节，则将其0开销地转换成一个`&str`类型，若不是，rust会将其拷贝一份并且将非法字节用`U+FFFD`填充。

#### CString

和`CStr`表示从c中来，rust不拥有归属权的字符串相反，`CString`表示由rust分配，用以传给c程序的字符串。

```rust
use std::ffi::CString;
use std::os::raw::c_char;

extern {
    fn my_printer(s: *const c_char);
}

let c_to_print = CString::new("Hello, world!").unwrap();
unsafe {
    my_printer(c_to_print.as_ptr()); // 使用 as_ptr 将CString转化成char指针传给c函数
}
```

注意c字符串中并不能包含`\0`字节(因为`\0`用来表示c字符串的结束符),因此`CString::new`将返回一个`Result`，
如果输入有`\0`的话则为`Error(NulError)`。

### 不透明结构体

C库存在一种常见的情况：库作者并不想让使用者知道一个数据类型的具体内容，因此常常提供了一套工具函数，并使用`void*`或不透明结构体传入传出进行操作。
比较典型的是`ncurse`库中的`WINDOW`类型。

当参数是`void*`时，在rust中可以和c一样，使用对应类型`*mut libc::c_void`进行操作。如果参数为不透明结构体，rust中可以使用空白`enum`进行代替:

```rust
enum OpaqueStruct {}

extern "C" {
    pub fn foo(arg: *mut OpaqueStruct);
}
```

C代码：

```c
struct OpaqueStruct;
void foo(struct OpaqueStruct *arg);
```

### 空指针

另一种很常见的情况是需要一个空指针。请使用`0 as *const _` 或者 `std::ptr::null()`来生产一个空指针。

## 内存安全

由于`ffi`跨越了rust边界，rust编译器此时无法保障代码的安全性，所以在涉及ffi操作时要格外注意。

### 析构问题

在涉及ffi调用时最常见的就是析构问题：这个对象由谁来析构？是否会泄露或use after free？
有些情况下c库会把一类类型`malloc`了以后传出来，然后不再关系它的析构。因此在做ffi操作时请为这些类型实现析构(`Drop Trait`).

### 可空指针优化

当`rust`的一个`enum`为一种特殊结构：它有两种实例，一种为空，另一种只有一个数据域的时候，rustc会开启空指针优化将其优化成一个指针。
比如`Option<extern "C" fn(c_int) -> c_int>`会被优化成一个可空的函数指针。

### ownership处理

在rust中，由于编译器会自动插入析构代码到块的结束位置，在使用`owned`类型时要格外的注意。

```rust
extern {
    pub fn foo(arg: extern fn() -> *const c_char);
}

extern "C" fn danger() -> *const c_char {
    let cstring = CString::new("I'm a danger string").unwrap();
    cstring.as_ptr()
}  // 由于CString是owned类型，在这里cstring被rust free掉了。USE AFTER FREE! too young!

fn main() {
  unsafe {
        foo(danger); // boom !!
    }
}
```

由于`as_ptr`接受一个`&self`作为参数(`fn as_ptr(&self) -> *const c_char`)，`as_ptr`以后`ownership`仍然归rust所有。因此rust会在函数退出时进行析构。
正确的做法是使用`into_raw()`来代替`as_ptr()`。由于`into_raw`的签名为`fn into_raw(self) -> *mut c_char`，接受的是`self`,产生了`ownership`转移，
因此`danger`函数就不会将`cstring`析构了。

### panic

由于在`ffi`中`panic`是未定义行为，切忌在`cffi`时`panic`包括直接调用`panic!`,`unimplemented!`,以及强行`unwrap`等情况。
当你写`cffi`时，记住：你写下的每个单词都可能是发射**核弹**的密码！

## 静态库/动态库

前面提到了声明一个外部库的方式--`#[link]`标记，此标记默认为动态库。但如果是静态库，可以使用`#[link(name = "foo", kind = "static")]`来标记。
此外，对于osx的一种特殊库--`framework`, 还可以这样标记`#[link(name = "CoreFoundation", kind = "framework")]`.

## 调用约定

前面看到，声明一个被c调用的函数时，采用`extern "C" fn`的语法。此处的`"C"`即为c调用约定的意思。此外，rust还支持：

* stdcall
* aapcs
* cdecl
* fastcall
* vectorcall //这种call约定暂时需要开启abi_vectorcall feature gate.
* Rust
* rust-intrinsic
* system
* C
* win64

## bindgen

是不是觉得把一个个函数和全局变量在`extern块`中去声明，对应的数据结构去手动创建特别麻烦？没关系，`rust-bindgen`来帮你搞定。
`rust-bindgen`是一个能从对应c头文件自动生成函数声明和数据结构的工具。创建一个绑定只需要`./bindgen [options] input.h`即可。
[项目地址](https://github.com/crabtw/rust-bindgen)
