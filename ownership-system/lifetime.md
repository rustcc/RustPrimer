# 生命周期（ Lifetime ）


下面是一个资源借用的例子：

```rust
fn main() {
	let a = 100_i32;

	{
		let x = &a;
	}  // x 作用域结束
	println!("{}", x);
}
```

编译时，我们会看到一个严重的错误提示：

> error: unresolved name `x`.

错误的意思是“无法解析 `x` 标识符”，也就是找不到 `x` , 这是因为像很多编程语言一样，Rust中也存在作用域概念，当资源离开离开作用域后，资源的内存就会被释放回收，当借用/引用离开作用域后也会被销毁，所以 `x` 在离开自己的作用域后，无法在作用域之外访问。


上面的涉及到几个概念：

* **Owner**: 资源的所有者 `a`
* **Borrower**: 资源的借用者 `x`
* **Scope**: 作用域，资源被借用/引用的有效期


强调下，无论是资源的所有者还是资源的借用/引用，都存在在一个有效的存活时间或区间，这个时间区间称为**生命周期**， 也可以直接以**Scope作用域**去理解。

所以上例子代码中的生命周期/作用域图示如下：


```
            {    a    {    x    }    *    }
所有者 a:         |________________________|
借用者 x:                   |____|            x = &a
  访问 x:                             |       失败：访问 x
```

可以看到，借用者 `x` 的生命周期是资源所有者 `a` 的生命周期的**子集**。但是 `x` 的生命周期在第一个 `}` 时结束并销毁，在接下来的 `println!` 中再次访问便会发生严重的错误。

我们来修正上面的例子：

```rust
fn main() {
	let a = 100_i32;

	{
		let x = &a;
		println!("{}", x);
	}  // x 作用域结束

}
```

这里我们仅仅把 `println!` 放到了中间的 `{}`, 这样就可以在 `x`的生命周期内正常的访问 `x` ，此时的Lifetime图示如下：

```
            {    a    {    x    *    }    }
所有者 a:         |________________________|
借用者 x:                   |_________|       x = &a
  访问 x:                        |            OK：访问 x
```



## 隐式Lifetime
我们经常会遇到参数或者返回值为引用类型的函数：

```rust
fn foo(x: &str) -> &str {
	x
}
```

上面函数在实际应用中并没有太多用处，`foo` 函数仅仅接受一个 `&str ` 类型的参数（`x`为对某个`string`类型资源`Something`的借用），并返回对资源`Something`的一个新的借用。

实际上，上面函数包含该了隐性的生命周期命名，这是由编译器自动推导的，相当于：

```rust
fn foo<'a>(x: &'a str) -> &'a str {
	x
}
```

在这里，约束返回值的Lifetime必须大于或等于参数`x`的Lifetime。下面函数写法也是合法的：

```rust
fn foo<'a>(x: &'a str) -> &'a str {
	"hello, world!"
}
```

为什么呢？这是因为字符串"hello, world!"的类型是`&'static str`，我们知道`static`类型的Lifetime是整个程序的运行周期，所以她比任意传入的参数的Lifetime`'a`都要长，即`'static >= 'a`满足。


在上例中Rust可以自动推导Lifetime，所以并不需要程序员显式指定Lifetime `'a` 。

`'a`是什么呢？它是Lifetime的标识符，这里的`a`也可以用`b`、`c`、`d`、`e`、...，甚至可以用`this_is_a_long_name`等，当然实际编程中并不建议用这种冗长的标识符，这样会严重降低程序的可读性。`foo`后面的`<'a>`为Lifetime的声明，可以声明多个，如`<'a, 'b>`等等。

另外，除非编译器无法自动推导出Lifetime，否则不建议显式指定Lifetime标识符，会降低程序的可读性。

## 显式Lifetime
当输入参数为多个借用/引用时会发生什么呢？

```rust
fn foo(x: &str, y: &str) -> &str {
	if true {
		x
	} else {
		y
	}
}
```

这时候再编译，就没那么幸运了：

```
error: missing lifetime specifier [E0106]
fn foo(x: &str, y: &str) -> &str {
                            ^~~~
```

编译器告诉我们，需要我们显式指定Lifetime标识符，因为这个时候，编译器无法推导出返回值的Lifetime应该是比 `x`长，还是比`y`长。虽然我们在函数中中用了 `if true` 确认一定可以返回`x`，但是要知道，编译器是在编译时候检查，而不是运行时，所以编译期间会同时检查所有的输入参数和返回值。

修复后的代码如下：

```rust
fn foo<'a>(x: &'a str, y: &'a str) -> &'a str {
	if true {
		x
	} else {
		y
	}
}
```

## Lifetime推导

要推导Lifetime是否合法，先明确两点：

* 输出值（也称为返回值）依赖哪些输入值
* 输入值的Lifetime大于或等于输出值的Lifetime (准确来说：子集，而不是大于或等于)

**Lifetime推导公式：**
当输出值R依赖输入值X Y Z ...，当且仅当输出值的Lifetime为所有输入值的Lifetime交集的子集时，生命周期合法。

```
	Lifetime(R) ⊆ ( Lifetime(X) ∩ Lifetime(Y) ∩ Lifetime(Z) ∩ Lifetime(...) )
```

对于例子1：

```rust
fn foo<'a>(x: &'a str, y: &'a str) -> &'a str {
	if true {
		x
	} else {
		y
	}
}
```

因为返回值同时依赖输入参数`x`和`y`，所以

```
	Lifetime(返回值) ⊆ ( Lifetime(x) ∩ Lifetime(y) )

	即：

	'a ⊆ ('a ∩ 'a)  // 成立
```


#### 定义多个Lifetime标识符
那我们继续看个更复杂的例子，定义多个Lifetime标识符：

```rust
fn foo<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
	if true {
		x
	} else {
		y
	}
}
```

先看下编译，又报错了：

```
<anon>:5:3: 5:4 error: cannot infer an appropriate lifetime for automatic coercion due to conflicting requirements [E0495]
<anon>:5 		y
         		^
<anon>:1:1: 7:2 help: consider using an explicit lifetime parameter as shown: fn foo<'a>(x: &'a str, y: &'a str) -> &'a str
<anon>:1 fn bar<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
<anon>:2 	if true {
<anon>:3 		x
<anon>:4 	} else {
<anon>:5 		y
<anon>:6 	}
```

编译器说自己无法正确地推导返回值的Lifetime，读者可能会疑问，“我们不是已经指定返回值的Lifetime为`'a`了吗？"。

这儿我们同样可以通过生命周期推导公式推导：

因为返回值同时依赖`x`和`y`，所以

```
	Lifetime(返回值) ⊆ ( Lifetime(x) ∩ Lifetime(y) )

	即：

	'a ⊆ ('a ∩ 'b)  //不成立
```

很显然，上面我们根本没法保证成立。

所以，这种情况下，我们可以显式地告诉编译器`'b`比`'a`长（`'a`是`'b`的子集），只需要在定义Lifetime的时候, 在`'b`的后面加上`: 'a`, 意思是`'b`比`'a`长，`'a`是`'b`的子集:

```
fn foo<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str {
	if true {
		x
	} else {
		y
	}
}
```

这里我们根据公式继续推导：

```
	条件：Lifetime(x) ⊆ Lifetime(y)
	推导：Lifetime(返回值) ⊆ ( Lifetime(x) ∩ Lifetime(y) )

	即：

	条件： 'a ⊆ 'b
	推导：'a ⊆ ('a ∩ 'b) // 成立
```

上面是成立的，所以可以编译通过。

#### 推导总结
通过上面的学习相信大家可以很轻松完成Lifetime的推导，总之，记住两点：

1. 输出值依赖哪些输入值。
2. 推导公式。



## Lifetime in struct
上面我们更多讨论了函数中Lifetime的应用，在`struct`中Lifetime同样重要。

我们来定义一个`Person`结构体：

```rust
struct Person {
	age: &u8,
}
```

编译时我们会得到一个error：

```
<anon>:2:8: 2:12 error: missing lifetime specifier [E0106]
<anon>:2 	age: &str,
```

之所以会报错，这是因为Rust要确保`Person`的Lifetime不会比它的`age`借用长，不然会出现`Dangling Pointer`的严重内存问题。所以我们需要为`age`借用声明Lifetime：

```rust
struct Person<'a> {
	age: &'a u8,
}
```

不需要对`Person`后面的`<'a>`感到疑惑，这里的`'a`并不是指`Person`这个`struct`的Lifetime，仅仅是一个泛型参数而已，`struct`可以有多个Lifetime参数用来约束不同的`field`，实际的Lifetime应该是所有`field`Lifetime交集的子集。例如：

```
fn main() {
	let x = 20_u8;
	let stormgbs = Person {
						age: &x,
					 };
}
```

这里，生命周期/Scope的示意图如下：

```
                  {   x    stormgbs      *     }
所有者 x:              |________________________|
所有者 stormgbs:                |_______________|  'a
借用者 stormgbs.age:            |_______________|  stormgbs.age = &x
```

既然`<'a>`作为`Person`的泛型参数，所以在为`Person`实现方法时也需要加上`<'a>`，不然：

```rust
impl Person {
	fn print_age(&self) {
		println!("Person.age = {}", self.age);
	}
}
```

报错：

```
<anon>:5:6: 5:12 error: wrong number of lifetime parameters: expected 1, found 0 [E0107]
<anon>:5 impl Person {
              ^~~~~~
```

**正确的做法是**：

```rust
impl<'a> Person<'a> {
	fn print_age(&self) {
		println!("Person.age = {}", self.age);
	}
}
```

这样加上`<'a>`后就可以了。读者可能会疑问，为什么`print_age`中不需要加上`'a`？这是个好问题。因为`print_age`的输出参数为`()`，也就是可以不依赖任何输入参数, 所以编译器此时可以不必关心和推导Lifetime。即使是`fn print_age(&self, other_age: &i32) {...}`也可以编译通过。

**如果`Person`的方法存在输出值（借用）呢？**

```rust
impl<'a> Person<'a> {
	fn get_age(&self) -> &u8 {
		self.age
	}
}
```

`get_age`方法的输出值依赖一个输入值`&self`，这种情况下，Rust编译器可以自动推导为：

```
impl<'a> Person<'a> {
	fn get_age(&'a self) -> &'a u8 {
		self.age
	}
}
```

**如果输出值（借用）依赖了多个输入值呢？**


```
impl<'a, 'b> Person<'a> {
	fn get_max_age(&'a self, p: &'a Person) -> &'a u8 {
		if self.age > p.age {
			self.age
		} else {
			p.age
		}
	}
}
```

类似之前的Lifetime推导章节，当返回值（借用）依赖多个输入值时，需显示声明Lifetime。和函数Lifetime同理。



**其他**

无论在函数还是在`struct`中，甚至在`enum`中，Lifetime理论知识都是一样的。希望大家可以慢慢体会和吸收，做到举一反三。


## 总结

Rust正是通过所有权、借用以及生命周期，以高效、安全的方式近乎完美地管理了内存。没有手动管理内存的负载和安全性，也没有GC造成的程序暂停问题。



