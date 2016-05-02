# RustPrimer
The Rust primer for beginners.
给初学者的Rust中文教程。

## 在线阅读地址

[点我阅读](https://wayslog.gitbooks.io/rustprimer/content/)

也可复制以下链接：

```
https://wayslog.gitbooks.io/rustprimer/content/
```

## 社区

### QQ群

群号：
```
    Rust语言中文社区 144605258 （已满）
    Rust编程语言社区2群 303838735
```

*进群必须附带您的github地址，否则不予通过*

### 社区

chat: https://chat.rust-china.org/ (使用github验证登录)

blog: https://rust-china.org/

wiki: https://wiki.rust-china.org/

## 大纲

* [初识Rust](1st-glance/README.md)「daogangtang, tiansiyuan」
* [安装Rust](install/preface.md)「marvin-min」
  * [Linux](install/install_rust_on_linux.md)
  * [Mac](install/install_rust_on_mac_os.md)
  * [Windows](install/install_rust_on_windows.md)
  * [版本管理工具: multirust](install/multirust.md)
* [编辑器](editors/preface.md)
  * [前期准备](editors/before.md)「wayslog」
  * [vim](editors/vim.md)「wayslog」
  * [emacs](editors/emacs.md)「tiansiyuan」
  * [vscode](editors/vscode.md)「daogangtang」
  * [atom](editors/atom.md)「wayslog」
  * [sublime](editors/sublime.md)「domty」
  * [visual studio](editors/visualstudio.md)「marvinguo」
  * [spacemacs](editors/spacemacs.md)「wayslog」
* [Rust快速入门](quickstart/quickstart.md)「Naupio」
  * [Rust旅程](quickstart/rust-travel.md)
  * [基础类型](quickstart/primitive-type.md)
  * [向量与字符串](quickstart/vector-string.md)
  * [结构体与枚举](quickstart/struct-enum.md)
  * [操作流](quickstart/control-stream.md)
  * [函数与方法](quickstart/function-method.md)
  * [特性](quickstart/trait.md)
  * [注释与文档](quickstart/comments-document.md)
  * [输入输出流](quickstart/io-stream.md)
* [Cargo项目管理器](cargo-projects-manager/cargo-projects-manager.md)「fuyingfuying」
* [基本程序结构](flow/preface.md)「daogangtang」
  * [注释](flow/comment.md)
  * [条件](flow/condition.md)
  * [循环](flow/repeatition.md)
* [类型、运算符和字符串](type/preface.md)「wayslog」
  * [基础类型](type/types.md)
  * [复合类型](type/compound-types.md)
  * [字符串类](type/strings.md)
  * [基础运算符和字符串格式化](type/operator-and-format.md)
* [函数](function/overview.md)「qdao」
  * [函数参数](function/arguement.md)
  * [函数返回值](function/return_value.md)
  * [语句和表达式](function/statement_expression.md)
  * [高阶函数](function/high_order_function.md)
* [模式匹配](match/overview.md)「wayslog」
  * [match关键字](match/match.md)
  * [模式](match/pattern.md)
* [Trait （特征）](trait/overview.md)「JohnSmithX」
  * [trait关键字](trait/trait.md)
  * [trait对象](trait/trait-object.md)
* [泛型](generics/generics.md)「stormgbs」
* [可变性、所有权、租借和生命期](ownership-system/ownership_system.md)「stormgbs」
  * [所有权](ownership-system/ownership.md)
  * [引用和借用](ownership-system/borrowing_references.md)
  * [生命周期](ownership-system/lifetimes.md)
* [闭包](closure/overview.md)「qdao」
  * [闭包的语法](closure/syntax.md)
  * [闭包的实现](closure/implementation.md)
  * [闭包作为参数和返回值](closure/as_argument_return_value.md)
* [集合类型](collections/overview.md)「wayslog」
  * [动态数组](collections/vec.md)
  * [哈希表](collections/hashmap.md)
* [迭代器](iterator/overview.md)「wayslog」
  * [迭代器、适配器、消费者](iterator/iterator.md)
* [模块和包系统、Prelude](modules/preface.md)「daogangtang」
  * [模块（module）和包（crate）](modules/module.md)
  * [Prelude](modules/prelude.md)
* [Option、Result与错误处理](error-handling/option-result.md)「JohnSmithX」
* [宏系统](macro/macro.md)「tennix」
* [堆、栈与Box](heap-stack/heap-stack.md)「tennix」
* [几种智能指针](rcarc/preface.md)「daogangtang」
  * [Rc, Arc](rcarc/rcarc.md)
  * [Mutex, RwLock](rcarc/mutex.md)
  * [Cell, RefCell](rcarc/cell.md)
* [类型系统中的几个常见 Trait](intoborrow/preface.md) 「daogangtang」
  * [Into/From 及其在 String 和 &str 互转上的应用](intoborrow/into.md)
  * [AsRef, AsMut](intoborrow/asref.md)
  * [Borrow, BorrowMut, ToOwned](intoborrow/borrow.md)
  * [Deref 与 Deref coercions](intoborrow/deref.md)
  * [Cow 及其在 String 和 &str 上的应用](intoborrow/cow.md)
* [Send 和 Sync](marker/sendsync.md)「daogangtang」
* [并发，并行，多线程编程](concurrency-parallel-threads/preface.md)「anzhihun」
  * [线程](concurrency-parallel-threads/thread.md)
  * [消息传递](concurrency-parallel-threads/message-passing.md)
  * [共享内存](concurrency-parallel-threads/share-memory.md)
  * [同步](concurrency-parallel-threads/synchronize.md)
  * [并行](concurrency-parallel-threads/parallel.md)
* [Unsafe、原始指针](unsafe-rawpointer/preface.md)「JohnSmithX」
  * [Unsafe](unsafe-rawpointer/unsafe.md)
  * [原始指针](unsafe-rawpointer/raw-pointer.md)
* [FFI](ffi/preface.md)「42」
  * [rust调用ffi函数](ffi/calling-ffi-functions.md)
  * [将rust编译成库](ffi/compiling-rust-to-lib.md)
* [运算符重载](operator-overloading/operator.md)「wayslog」
* [属性和编译器参数](attr-and-compiler-args/preface.md)「elton」
  * [属性](attr-and-compiler-args/attributes.md)
  * [编译器参数](attr-and-compiler-args/rustc-options.md)
* [Cargo参数配置](cargo-detailed-cfg/cargo-detailed-cfg.md)「fuyingfuying」
* [测试与评测](testing/preface.md)「daogangtang」
  * [测试 (testing/threearchtest.md)
  * [评测 (testing/bench.md)
* [代码风格](coding-style/style.md)「tiansiyuan」
* [Any与反射](any/any.md)「wayslog」
* [安全（safe）](safe/safe.md)「daogangtang」
* [常用数据结构实现](data-structure/preface.md)「Naupio」
  * [栈结构](data-structure/stack.md)
  * [队列](data-structure/queue.md)
  * [优先队列](data-structure/priority_queue.md)
  * [二叉树](data-structure/binary_tree.md)
  * [链表](data-structure/linked_list.md)
  * [图结构](data-structure/graph.md)
* [标准库介绍](std/overview.md)「wayslog」
  * [系统命令:调用grep](std/process.md)
  * [目录操作:简单grep](std/fs-and-path.md)
  * [网络模块:W回音](std/net.md)
* [实战篇](action/preface.md)「wangyu190810」
  * [实战：Json处理](action/json_data/readme.md)
  * [实战：Web 应用开发入门](action/mysite/readme.md)
  * [实战：使用Postgresql数据库](action/db/readme.md)

## 版权规定

本书使用 `CC BY-SA 3.0` 协议，转载请注明地址。

## gitbook生成

直接用：

```
gitbook serve
```

即可

## ChangeLog

1. 2016年3月31日，初稿完成。发布 v1.0 版。
