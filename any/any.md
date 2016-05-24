# Any和反射

熟悉Java的同学肯定对Java的反射能力记忆犹新，同样的，Rust也提供了运行时反射的能力。但是，这里有点小小的不同，因为 Rust 不带 VM 不带 Runtime ,因此，其提供的反射更像是一种编译时反射。

因为，Rust只能对 `'static` 生命周期的变量（常量）进行反射！

## 举个例子

我们会有这样的需求，去某些路径里加载配置文件。我们可能提供一个配置文件路径，好吧，这是个字符串(`String`)。但是，当我想要传入多个配置文件的路径的时候怎们办？理所应当的，我们传入了一个数组。

这下可坏了……Rust不支持重载啊！于是有人就很单纯的写了两个函数～～！

其实不用……我们只需要这么写……

```rust
use std::any::Any;
use std::fmt::Debug ;

fn load_config<T:Any+Debug>(value: &T) -> Vec<String>{
    let mut cfgs: Vec<String>= vec![];
    let value = value as &Any;
    match value.downcast_ref::<String>() {
        Some(cfp) => cfgs.push(cfp.clone()),
        None => (),
    };

    match value.downcast_ref::<Vec<String>>() {
        Some(v) => cfgs.extend_from_slice(&v),
        None =>(),
    }

    if cfgs.len() == 0 {
        panic!("No Config File");
    }
    cfgs
}

fn main() {
    let cfp = "/etc/wayslog.conf".to_string();
    assert_eq!(load_config(&cfp), vec!["/etc/wayslog.conf".to_string()]);
    let cfps = vec!["/etc/wayslog.conf".to_string(),
                    "/etc/wayslog_sec.conf".to_string()];
    assert_eq!(load_config(&cfps),
               vec!["/etc/wayslog.conf".to_string(),
                    "/etc/wayslog_sec.conf".to_string()]);
}
```

我们来重点分析一下中间这个函数：

```rust
fn load_config<T:Any+Debug>(value: &T) -> Vec<String>{..}
```

首先，这个函数接收一个泛型`T`类型，`T`必须实现了`Any`和`Debug`。

这里可能有同学疑问了，你不是说只能反射 `'static` 生命周期的变量么？我们来看一下`Any`限制：

```rust
pub trait Any: 'static + Reflect {
    fn get_type_id(&self) -> TypeId;
}
```

看，`Any`在定义的时候就规定了其生命周期，而`Reflect`是一个Marker，默认所有的Rust类型都会实现他！注意，这里不是所有原生类型，而是所有类型。

好的，继续，由于我们无法判断出传入的参数类型，因此，只能从运行时候反射类型。

```rust
let value = value as &Any;
```

首先，我们需要将传入的类型转化成一个 `trait Object`, 当然了，你高兴的话用 `UFCS` 也是可以做的，参照本章最后的附录。

这样，value 就可以被堪称一个 Any 了。然后，我们通过 `downcast_ref` 来进行类型推断。如果类型推断成功，则 value 就会被转换成原来的类型。

有的同学看到这里有点懵，为什么你都转换成 Any 了还要转回来？

其实，转换成 Any 是为了有机会获取到他的类型信息，转换回来，则是为了去使用这个值本身。

最后，我们对不同的类型处以不同的处理逻辑。最终，一个反射函数就完成了。

## 说说注意的地方

需要注意的是，Rust本身提供的反射能力并不是很强大。相对而言只能作为一个辅助的手段。并且，其只能对`'static`周期进行反射的限制，的确限制了其发挥。还有一点需要注意的是，Rust的反射只能被用作类型推断，绝对不能被用作接口断言！

啥，你问原因？因为写不出来啊……
