# 函数
  尽管rust是一门多范式的编程语言，但rust的编程风格是更偏向于函数式的，函数在rust中是“一等公民”——first-class type。这意味着，函数是可以作为数据在程序中进行传递，如：作为函数的参数。跟C、C++一样，rust程序也有一个唯一的程序入口-main函数。rust的main函数形式如下：
  
  ```rust
fn main() {
  //statements
}
  ```
  
  rust使用 `fn` 关键字来声明和定义函数，`fn` 关键字隔一个空格后跟函数名，函数名后跟着一个括号，函数参数定义在括号内。rust使用`snake_case`风格来命名函数，即所有字母小写并使用下划线类分隔单词，如：`foo_bar`。如果函数有返回值，则在括号后面加上箭头 __->__ ，在箭头后加上返回值的类型。

  这一章我们将学习以下与函数相关的知识：
  1. [函数参数](arguement.md)
  2. [函数返回值](return_value.md)
  3. [语句和表达式](statement_expression.md)
  4. [高阶函数](higher_order_function.md)

> ### 注：本章所有例子均在rustc1.4下编译通过，且例子中说明的所有的编译错误都是rustc1.4版本给出的。
