# 函数返回值
  在rust中，任何函数都有返回类型，当函数返回时，会返回一个该类型的值。我们先来看看main函数：
  
  ```rust
  fn main() {
    //statements
  }
  ```
  
  之前有说过，函数的返回值类型是在参数列表后，加上箭头和类型来指定的。不过，一般我们看到的main函数的定义并没有这么做。这是因为main函数的返回值是`()`，在rust中，当一个函数返回`()`时，可以省略。main函数的完整形式如下：
  
  ```rust
  fn main() -> () {
    //statements
  }
  ```
  
  main函数的返回值类型是`()`，它是一个特殊的元组——没有元素的元组，称为`unit`，它表示一个函数没有任何信息需要返回。在Rust Reference的[8.1.3 Tuple types](https://doc.rust-lang.org/reference.html#tuple-types)中是的描述如下：
  > For historical reasons and convenience, the tuple type with no elements (__()__) is often called ‘unit’ or ‘the unit type’.

  `()`类型，其实类似于C/C++、Java、C#中的`void`类型。

  下面来看一个有返回值的例子：
  
  ```rust
  fn main() {
    let a = 3;
    println!("{}", inc(a));
  }

  fn inc(n: i32) -> i32 {
    n + 1
  }
  ```
  
  上面的例子中，函数`inc`有一个`i32`类型的参数和返回值，作用是将参数加1返回。需要注意的是`inc`函数中只有`n+1`一个表达式，并没有像C/C++或Java、C#等语言有显式地`return`语句类返回一个值。这是因为，与其他基于语句的语言（如C语言）不同，rust是基于表达式的语言，函数中最后一个表达式的值，默认作为返回值。当然，rust中也有语句，关于rust的语句和表达式，请看[下一节](statement_expression.md)。

## return关键字
  rust也有`return`关键字，不过一般用于提前返回。来看一个简单地例子：
  
  ```rust
fn main() {
  let a = [1,3,2,5,9,8];
  println!("There is 7 in the array: {}", find(7, &a));
  println!("There is 8 in the array: {}", find(8, &a));
}

fn find(n: i32, a: &[i32]) -> bool {
  for i in a {
    if *i == n {
      return true;
    }
  }
  false
}
  ```
  
  上例中，`find`函数，接受一个`i32`类型`n`和一个`i32`类型的切片(`slice`)`a`，返回一个`bool`值，若n是a的元素，则返回`true`，否则返回`false`。可以看到，`return`关键字，用在`for`循环的`if`表达式中，若此时a的元素与n相等，则立刻返回true，剩下的循环不必再进行，否则一直循环检测完整个切片(slice)，最后返回false。当然，return语句也可以用在最后返回，像C/C++一样使用：把`find`函数最后一句`false`改为`return false;`（注意分号不可省略）也是可以的，不过这就不是rust的编程风格了。这里需要注意的是，`for`循环中的`i`，其类型为`&i32`，需要使用解引用操作符来变换为`i32`类型。另外，切片（slice）在这里可以看作是对数组的引用，关于切片与数组的详细解释可以看[Rust Reference](https://doc.rust-lang.org/reference.html#array-and-slice-types)和[rustbyexample](http://rustbyexample.com/primitives/array.html)中的相关内容。

## 返回多个值
  rust的函数不支持多返回值，但是我们可以利用元组来返回多个值，配合rust的模式匹配，使用起来十分灵活。先看例子：
  
  ```rust
fn main() {
  let (p2,p3) = pow_2_3(789);
  println!("pow 2 of 789 is {}.", p2);
  println!("pow 3 of 789 is {}.", p3);
}

fn pow_2_3(n: i32) -> (i32, i32) {
  (n*n, n*n*n)
}
  ```
  
  可以看到，上例中，`pow_2_3`函数接收一个`i32`类型的值，返回其二次方和三次方的值，这两个值包装在一个元组中返回。在`main`函数中，`let`语句就可以使用模式匹配将函数返回的元组进行解构，将这两个返回值分别赋给`p2`和`p3`，从而可以得到`789`二次方的值和三次方的值。

## 发散函数
  发散函数（diverging function）是rust中的一个特性。发散函数不返回，它使用感叹号`!`作为返回类型表示：
  
  ```rust
fn main() {
  println!("hello");
  diverging();
  println!("world");
}

fn diverging() -> ! {
  panic!("This function will never return");
}
  ```
  
  由于发散函数不会返回，所以就算其后再有其他语句也是不会执行的。倘若其后还有其他语句，会出现如下编译警告：![error](../images/function-return-value.png)。当然了，我们要知道的是不发散的函数也是可以不返回的，比如无限循环之类的。
  发散函数一般都以`panic!`宏调用或其他调用其他发散函数结束，所以，调用发散函数会导致当前线程崩溃。[Rust Reference 6.1.3.2 Diverging functions][ref]中的描述如下：
  > We call such functions "diverging" because they never return a value to the caller. Every control path in a diverging function must end with a panic!() or a call to another diverging function on every control path. The ! annotation does not denote a type.

  [ref]:http://doc.rust-lang.org/reference.html#diverging-functions
