# 将rust编译成动态链接库
上一章讲述了如何从rust中调用c库，这一章我们讲如何让rust能被别的语言通过cffi调用。

## 调用约定和mangle
正如上一章讲述的，为了能让rust的函数通过ffi被调用，需要加上`extern "C"`对函数进行修饰。
由于rust支持重载，所以函数名会被编译器进行混淆，就像c++一样。因此当你的函数被编译完毕后，函数名会带上一串表明函数签名的字符串，
比如：`fn test() {}`会变成`_ZN4test20hf06ae59e934e5641haaE`.
这样的函数名这为ffi调用带来了困难，因此，rust提供了`#[no_mangle]`属性为函数修饰。
对于带有`#[no_mangle]`属性的函数，rust编译器不会为它进行函数名混淆。
对于
```
#[no_mangle]
extern "C" fn test() {}
```
在nm中观察到为
```
...
00000000001a7820 T test
...
```
至此，test函数将能够被正常的由cffi调用。
## 指定crate类型
rustc默认编译产生rust自用的rlib格式库，要让rustc产生动态链接库或者静态链接库，需要显示指定。
1. 方法1: 在文件中指定。
   在文件头加上`#![crate_type = "foo"]`, 其中foo的可选类型有`bin`, `lib`, `rlib`, `dylib`, `staticlib`.分别对应可执行文件，
   默认(将由rustc自己决定), rlib格式，动态链接库，静态链接库。
2. 方法2: 编译时给rustc 传`--crate-type`参数。参数内容同上。
3. 方法3: 使用cargo，指定`crate-type = ["foo"] `, foo可选类型同1
## 小技巧: Any
由于在跨越ffi过程中，rust类型信息会丢失，比如当用rust提供一个OpaqueStruct给别的语言时：
```
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
    t.push(1);
}

#[no_mangle]
extern "C" fn push_foo_element_c(foo: *const c_void){
    let foo2 = unsafe {
        &mut *(foo as *const Foo<Vec<i32>>) // 这么确定是Foo<Vec<i32>>? 万一foo是Foo<i32>怎么办？
    };
    push_foo_element(foo3);
}
```
以上代码中完全不知道foo是一个什么东西。安全也无从说起了，只能靠文档。
因此在ffi调用时往往会丧失掉rust类型系统带来的方便和安全。在这里提供一个小技巧:使用Box<Box<Any>>来包装你的类型。

rust的Any类型为rust带来了运行时反射的能力，使用Any跨越ffi边界将极大提高程序安全性。
```
use std::any::Any

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
    t.push(1);
}

#[no_mangle]
extern "C" fn push_foo_element_c(foo: *const c_void){
    let foo2 = unsafe {
        &mut *(foo as *const Box<Any>)
    };
    let foo3: Opetion<&mut Foo<Vec<i32>>> = foo2.downcast_mut(); // 如果foo2不是*const Box<Foo<Vec<i32>>>, 则foo3将会是None
    if let Some(value) = foo3 {
      push_foo_element(value);
    }
}
```
这样一来，就非常不容易出错了。
