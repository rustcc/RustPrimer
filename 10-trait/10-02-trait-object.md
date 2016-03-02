# 10.2 trait对象

trait对象在**Rust**中可以是指任何实现了该trait的类型对象。

```rust
trait Foo { fn method(&self) -> String; }

impl Foo for u8 { fn method(&self) -> String { format!("u8: {}", *self) } }
impl Foo for String { fn method(&self) -> String { format!("string: {}", *self) } }

fn do_something(x: &Foo) {
    x.method();
}

fn main() {
    let x = "Hello".to_string();
    do_something(&x);
    let y = 8u8;
    do_something(&y);
}
```

`x: &Foo`其中`x`是一个`Foo`trait对象的指针，用指针的目的是因为，`x`可以是任意实现`Foo`的类型实例，内存大小并不确定，但指针的大小是固定的。

## trait对象的实现

在`std::raw`模块中trait对象的实现：

```rust
pub struct TraitObject {
    pub data: *mut (),
    pub vtable: *mut (),
}
```

其中`data`是一个指向实际类型实例的指针， `vtable`是一个指向实际类型对于该trait的实现的虚函数表：

`Foo`的虚函数表类型：

```rust
struct FooVtable {
    destructor: fn(*mut ()),
    size: usize,
    align: usize,
    method: fn(*const ()) -> String,
}
```

之前的代码可以解读为：

```rust
// u8:
// 这个函数只会被指向u8的指针调用
fn call_method_on_u8(x: *const ()) -> String {
    let byte: &u8 = unsafe { &*(x as *const u8) };

    byte.method()
}

static Foo_for_u8_vtable: FooVtable = FooVtable {
    destructor: /* compiler magic */,
    size: 1,
    align: 1,

    method: call_method_on_u8 as fn(*const ()) -> String,
};


// String:
// 这个函数只会被指向String的指针调用
fn call_method_on_String(x: *const ()) -> String {
    let string: &String = unsafe { &*(x as *const String) };

    string.method()
}

static Foo_for_String_vtable: FooVtable = FooVtable {
    destructor: /* compiler magic */,
    size: 24,
    align: 8,

    method: call_method_on_String as fn(*const ()) -> String,
};


let a: String = "foo".to_string();
let x: u8 = 1;

// let b: &Foo = &a;
let b = TraitObject {
    // data存储实际值的引用
    data: &a,
    // vtable存储实际类型实现Foo的方法
    vtable: &Foo_for_String_vtable
};

// let y: &Foo = x;
let y = TraitObject {
    data: &x,
    vtable: &Foo_for_u8_vtable
};

// b.method();
(b.vtable.method)(b.data);

// y.method();
(y.vtable.method)(y.data);
```

## 对象安全

并不是所有的trait都能作为trait对象使用的，比如：

```rust
let v = vec![1, 2, 3];
let o = &v as &Clone;
```

会有一个错误：

```
error: cannot convert to a trait object because trait `core::clone::Clone` is not object-safe [E0038]
let o = &v as &Clone;
        ^~
note: the trait cannot require that `Self : Sized`
let o = &v as &Clone;
        ^~
```
让我来分析一下错误的原因：

```rust
pub trait Clone: Sized {
    fn clone(&self) -> Self;

    fn clone_from(&mut self, source: &Self) { ... }
}
```

虽然`Clone`本身集成了`Sized`这个trait，但是它的方法`fn clone(&self) -> Self`和`fn clone_from(&mut self, source: &Self) { ... }`含有`Self`类型，而在使用trait对象方法的时候**Rust**是动态派发的，我们根本不知道这个trait对象的实际类型，它可以是任何一个实现了该trait的类型的值，所以`Self`在这里的大小不是`Self: Sized`的，这样的情况在**Rust**中被称为`object-unsafe`或者`not object-safe`，这样的trait是不能成为trait对象的。

总结：

* trait的方法要求`Self: Sized`，就是方法的参数和返回值必须要有确定的size
* 或者trait的方法没有任何类型参数和不使用`Self`

参考[stackoverflow](http://stackoverflow.com/questions/29985153/trait-object-is-not-object-safe-error)
