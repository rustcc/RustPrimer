# RustPrimer
The Rust primer for beginners.
给初学者的Rust中文教程。

## 协作规则

1. 每个参与撰写的成员fork本项目，通过提pr的形式来协作书写；
2. 本书源码使用markdown语法格式书写；
3. 原则上每章（如有小节则每小节）对应独立的md文件；文件命名规则：章节号加名字，如第三章第二节03-02-emacs.md；
4. 在本项目的issue区提issue来认领各自负责的章节，项目评审组审核后会在大纲每个章节标题后面，标注已由谁认领，预计什么时间完成；
5. 本项目会组织评审委员会来负责审校内容质量，审核通过后，才会合并pr。

## 格式排版

1. 按github上的markdown规范来；
2. 中英文之间使用一个空格隔开。

## 写作要求

理论与实践结合，多举例，把事情讲清楚。必要时逐行分析，不怕啰嗦。要求描述精确，给出的示例尽量完整，能复制到文件中直接编译。所有给的例子，都要求能正常编译通过。举例的代码风格要符合官方规范，尽量消除警告。


## 大纲

1. [初识Rust](./01-1st-glance/README.md)「daogangtang, tiansiyuan」
2. [安装Rust](./02-install/02-00-preface.md)「marvin-min」
  1. [Linux](./02-install/02-01-install_rust_on_linux.md)
  2. [Mac](./02-install/02-02-install_rust_on_mac_os.md)
  3. [Windows](./02-install/02-03-install_rust_on_windows.md)
3. [编辑器](./03-editors/03-00-preface.md)
  1. [前期准备](./03-editors/03-01-before.md)「wayslog」
  1. [vim](./03-editors/03-02-vim.md)「wayslog」
  2. [emacs](./03-editors/03-03-emacs.md)「tiansiyuan」
  3. [vscode](./03-editors/03-04-vscode.md)「daogangtang」
  4. [atom](./03-editors/03-05-atom.md)「wayslog」
  6. [sublime](./03-editors/03-06-sublime.md)「domty」
  7. [visual studio](./03-editors/03-07-visualstudio.md)「marvinguo」
  9. [spacemacs](./03-editors/03-10-spacemacs.md)「wayslog」
4. [Rust一小时快速入门](./04-quickstart/04-00-intro.md)「ee0703」
  1. [第一个Rust程序](./04-quickstart/04-01-hello-world.md)
  2. [简单的数学运算](./04-quickstart/04-02-basic-math.md)
  3. [快速上手](./04-quickstart/04-03-cheet-sheet.md)
5. [Cargo项目管理器、crate](./05-cargo-projects-manager/05-cargo-projects-manager.md)「fuyingfuying」
6. [基本程序结构](./06-flow/06-00-preface.md)「daogangtang」
  1. [注释](./06-flow/06-01-comment.md)
  2. [条件](./06-flow/06-02-condition.md)
  3. [循环](./06-flow/06-03-repeatition.md)
7. [类型、运算符和字符串](07-type/07-00-preface.md)「wayslog」
  1. [基础类型](07-type/07-01-types.md)
  2. [复合类型](07-type/07-02-compound-types.md)
  2. [字符串类](07-type/07-03-strings.md)
  4. [基础运算符和字符串格式化](07-type/07-04-operator-and-format.md)
8. [函数](./08-function/08-00-overview.md)「qdao」
  1. [函数参数](./08-function/08-01-arguement.md)
  2. [函数返回值](./08-function/08-02-return_value.md)
  3. [语句和表达式](08-function/08-03-statement_expression.md)
  4. [高阶函数](08-function/08-04-high_order_function.md)
9. [模式匹配](09-match/09-00-overview.md)「wayslog」
  1. [match关键字](09-match/09-01-match.md)
  2. [模式](09-match/09-02-pattern.md)
10. [Trait （特征）](10-trait/10-00-overview.md)「JohnSmithX」
  1. [trait关键字](10-trait/10-01-trait.md)
  2. [trait对象](10-trait/10-02-trait-object.md)
11. [泛型](11-generics/11-01-generics.md)「stormgbs」
12. [可变性、所有权、租借和生命期](12-ownership-system/12-00-ownership_system.md)「stormgbs」
  1. [所有权](12-ownership-system/12-01-ownership.md)
  2. [引用和借用](12-ownership-system/12-02-borrowing_references.md)
  3. [生命周期](12-ownership-system/12-03-lifetimes.md)
13. [闭包](13-closure/13-00-overview.md)「qdao」
  1. [闭包的语法](13-closure/13-01-syntax.md)
  2. [闭包的实现](13-closure/13-02-implementation.md)
  3. [闭包作为参数和返回值](13-closure/13-03-as_argument_return_value.md)
14. [集合类型(Collections)](14-collections/14-00-overview.md)「wayslog」
  1. [动态数组](14-collections/14-01-vec.md)
  2. [哈希表](14-collections/14-02-hashmap.md)
15. [迭代器](15-iterator/15-00-overview.md)「wayslog」
  1. [迭代器、适配器、消费者](15-iterator/15-01-iterator.md)
16. [模块和包系统、Prelude](16-modules/16-00-preface.md)「daogangtang」
  1. [模块（module）和包（crate）](16-modules/16-01-module.md)
  2. [Prelude](16-modules/16-02-prelude.md)
17. [Option、Result与错误处理](17-error-handling/17-01-option-result.md)「JohnSmithX」
18. [宏系统](18-macro/18-01-macro.md)「tennix」
19. [堆、栈与Box](./19-heap-stack/heap-stack.md)「tennix」
20. [几种智能指针](./20-rcarc/20-00-preface.md)「daogangtang」
  1. [Rc, Arc](./20-rcarc/20-01-rcarc.md)
  2. [Mutex, RwLock](./20-rcarc/20-02-mutex.md)
  3. [Cell, RefCell](./20-rcarc/20-03-cell.md)
21. [类型系统中的几个常见 Trait](./21-intoborrow/21-00-preface.md) 「daogangtang」
  1. [Into/From 及其在 String 和 &str 互转上的应用](./21-intoborrow/21-01-into.md)
  2. [AsRef, AsMut](./21-intoborrow/21-02-asref.md)
  3. [Borrow, BorrowMut, ToOwned](./21-intoborrow/21-03-borrow.md)
  4. [Deref 与 Deref coercions](./21-intoborrow/21-04-deref.md)
  5. [Cow 及其在 String 和 &str 上的应用](./21-intoborrow/21-05-cow.md)
22. [Send 和 Sync](./22-marker/21-01-sendsync.md)「daogangtang」
23. [并发，并行，多线程编程](./23-concurrency-parallel-threads/23-00-preface.md)「anzhihun」
  1. [线程](./23-concurrency-parallel-threads/24-01-thread.md)
  2. [消息传递](./23-concurrency-parallel-threads/24-02-message-passing.md)
  3. [共享内存](./23-concurrency-parallel-threads/24-03-share-memory.md)
  4. [同步](./23-concurrency-parallel-threads/24-04-synchronize.md)
  5. [并行](./23-concurrency-parallel-threads/24-05-parallel.md)
24. [Unsafe、原始指针](24-unsafety-rawpointer/24-00-preface.md)「JohnSmithX」
  1. [Unsafe](24-unsafety-rawpointer/24-01-unsafety.md)
  2. [原始指针](24-unsafety-rawpointer/24-02-raw-pointer.md)
25. [FFI](25-ffi/25-00-preface.md)「42」
  1. [rust调用ffi函数](25-ffi/25-01-calling-ffi-functions.md)
  2. [将rust编译成库](25-ffi/25-02-compiling-rust-to-lib.md)
26. [运算符重载](26-operator-overload/26-01-operator.md)「wayslog」
27. [属性和编译器参数](27-attr-and-compiler-args/27-00-preface.md)「elton」
  1. [属性](27-attr-and-compiler-args/27-01-attributes.md)
  2. [编译器参数](27-attr-and-compiler-args/27-02-rustc-options.md)
28. [Cargo参数配置](28-cargo-detailed-cfg/28-01-cargo-detailed-cfg.md)「fuyingfuying」
29. [测试与评测](29-testing/29-00-preface.md)「daogangtang」
  1. [测试 (testing)](29-testing/29-01-threearchtest.md)
  2. [评测 (benchmark)](29-testing/29-02-bench.md)
30. [代码风格](30-coding-style/30-01-style.md)「tiansiyuan」
31. [Any与反射](31-any/31-01-any.md)「wayslog」
32. [安全（safety）](32-safety/32-01-safety.md)「daogangtang」
33. [常用数据结构实现](33-data-struct/33-00-preface.md)「Naupio」
  1. [栈结构](33-data-struct/33-01-stack.md)
  2. [队列](33-data-struct/33-02-queue.md)
  3. [优先队列](33-data-struct/33-03-priority_queue.md)
  4. [二叉树](33-data-struct/33-04-binary_tree.md)
  5. [链表](33-data-struct/33-05-linked_list.md)
  6. [图结构](33-data-struct/33-06-graph.md)
34. [标准库介绍](34-std/34-00-overview.md)「wayslog」
  1. [系统命令:调用grep](34-std/34-01-process.md)
  2. [目录操作:简单grep](34-std/34-02-fs-and-path.md)
  3. [网络模块:W回音](34-std/34-03-net.md)
35. [实战篇](35-action/35-00-preface.md)「wangyu190810」
  1. [实战：Json处理](35-action/json_data/readme.md)
  2. [实战：Web 应用开发入门](35-action/mysite/readme.md)
  3. [实战：使用Postgresql数据库](35-action/db/readme.md)

## 版权规定

本书使用 `CC BY-NC-SA 4.0` 协议，转载请注明地址。

## gitbook生成

直接用：

```
gitbook serve
```

即可

## ChangeLog

1. 2016年3月31日，初稿完成。发布 v1.0 版。
