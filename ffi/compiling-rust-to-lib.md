# 将Rust编译成库
上一章讲述了如何从rust中调用c库，这一章我们讲如何把rust编译成库让别的语言通过cffi调用。

## 调用约定和mangle
正如上一章讲述的，为了能让rust的函数通过ffi被调用，需要加上`extern "C"`对函数进行修饰。

但由于rust支持重载，所以函数名会被编译器进行混淆，就像c++一样。因此当你的函数被编译完毕后，函数名会带上一串表明函数签名的字符串。

比如：`fn test() {}`会变成`_ZN4test20hf06ae59e934e5641haaE`.
这样的函数名为ffi调用带来了困难，因此，rust提供了`#[no_mangle]`属性为函数修饰。
对于带有`#[no_mangle]`属性的函数，rust编译器不会为它进行函数名混淆。如：

```rust
#[no_mangle]
extern "C" fn test() {}
```

在nm中观察到为

```
...
00000000001a7820 T test
...
```

至此，`test`函数将能够被正常的由`cffi`调用。
## 指定`crate`类型
`rustc`默认编译产生`rust`自用的`rlib`格式库，要让`rustc`产生动态链接库或者静态链接库，需要显式指定。

1. 方法1: 在文件中指定。
   在文件头加上`#![crate_type = "foo"]`, 其中`foo`的可选类型有`bin`, `lib`, `rlib`, `dylib`, `staticlib`.分别对应可执行文件，
   默认(将由`rustc`自己决定), `rlib`格式，动态链接库，静态链接库。
2. 方法2: 编译时给rustc 传`--crate-type`参数。参数内容同上。
3. 方法3: 使用cargo，指定`crate-type = ["foo"] `, `foo`可选类型同1

## 小技巧: `Any`

由于在跨越`ffi`过程中，`rust`类型信息会丢失，比如当用`rust`提供一个`OpaqueStruct`给别的语言时：

```rust
use std::mem::transmute;

#[derive(Debug)]
struct Foo<T> {
  t: T
}

#[no_mangle]
extern "C" fn new_foo_vec() -> *const c_void {
    Box::into_raw(Box::new(Foo {t: vec![1,2,3]})) as *const c_void
}

#[no_mangle]
extern "C" fn new_foo_int() -> *const c_void {
    Box::into_raw(Box::new(Foo {t: 1})) as *const c_void
}

fn push_foo_element(t: &mut Foo<Vec<i32>>) {
    t.t.push(1);
}

#[no_mangle]
extern "C" fn push_foo_element_c(foo: *mut c_void){
    let foo2 = unsafe {
        &mut *(foo as *mut Foo<Vec<i32>>) // 这么确定是Foo<Vec<i32>>? 万一foo是Foo<i32>怎么办？
    };
    push_foo_element(foo3);
}
```

以上代码中完全不知道`foo`是一个什么东西。安全也无从说起了，只能靠文档。
因此在`ffi`调用时往往会丧失掉`rust`类型系统带来的方便和安全。在这里提供一个小技巧:使用`Box<Box<Any>>`来包装你的类型。

`rust`的`Any`类型为`rust`带来了运行时反射的能力，使用`Any`跨越`ffi`边界将极大提高程序安全性。

```rust
use std::any::Any;

#[derive(Debug)]
struct Foo<T> {
  t: T
}

#[no_mangle]
extern "C" fn new_foo_vec() -> *const c_void {
    Box::into_raw(Box::new(Box::new(Foo {t: vec![1,2,3]}) as Box<Any>)) as *const c_void
}

#[no_mangle]
extern "C" fn new_foo_int() -> *const c_void {
    Box::into_raw(Box::new(Box::new(Foo {t: 1}) as Box<Any>)) as *const c_void
}

fn push_foo_element(t: &mut Foo<Vec<i32>>) {
    t.t.push(1);
}

#[no_mangle]
extern "C" fn push_foo_element_c(foo: *mut c_void){
    let foo2 = unsafe {
        &mut *(foo as *mut Box<Any>)
    };
    let foo3: Option<&mut Foo<Vec<i32>>> = foo2.downcast_mut(); // 如果foo2不是*const Box<Foo<Vec<i32>>>, 则foo3将会是None
    if let Some(value) = foo3 {
      push_foo_element(value);
    }
}
```

这样一来，就非常不容易出错了。
