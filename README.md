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

## 时间规划

初步规划，在2016年2月29日之前，我们能完成第一版 v1.0。


## 大纲

1. [初识Rust](./01-1st-glance/README.md)
2. 安装Rust「marvin-min 160105」
  1. [Linux](./02-install/02-01-install_rust_on_linux.md)
  2. [Mac](./02-install/02-02-install_rust_on_mac_os.md)
  3. [Windows](./02-install/02-03-install_rust_on_windows.md)
3. 编辑器
  1. [前期准备](./03-editors/03-01-before.md)「wayslog 160105」
  1. [vim](./03-editors/03-02-vim.md)「wayslog 160105」
  2. [emacs](./03-editors/03-03-emacs.md)「tiansiyuan 160120」
  3. [vscode](./03-editors/03-04-vscode.md)「daogangtang 160105」
  4. [atom](./03-editors/03-05-atom.md)「wayslog 160105」
  5. sublime
  6. [visual studio](./03-editors/03-07-visualstudio.md)「marvinguo 」
  7. eclipse
  8. Intellij IDEA
  9. [spacemacs](./03-editors/03-10-spacemacs.md)「wayslog 160105」
4. [Rust一小时快速入门](./04-quickstart/04-00-intro.md)「ee0703 160120」
  1. [第一个Rust程序](./04-quickstart/04-01-hello-world.md)
  2. [简单的数学运算](./04-quickstart/04-02-basic-math.md)
  3. [快速上手](./04-quickstart/04-03-cheet-sheet.md)
5. [Cargo项目管理器、crate](./05-cargo-projects-manager/05-cargo-projects-manager.md)「fuyingfuying 160131」
6. 基本程序结构「daogangtang 160131」
  1. [注释](./06-flow/06-01-comment.md)
  2. [条件](./06-flow/06-02-condition.md)
  3. [循环](./06-flow/06-03-repeatition.md)
7. 基础类型和运算符「wayslog 160125」
  1. [基础类型](07-primitive-type/07-01-types.md)
  2. [字符串类](07-primitive-type/07-02-strings.md)
  4. [基础运算符和字符串格式化](07-primitive-type/07-03-operator-and-format.md)
8. [函数](./08-function/08-00-overview.md)「qdao 160120」
  1. [函数参数](./08-function/08-01-arguement.md)
  2. [函数返回值](./08-function/08-02-return_value.md)
  3. [语句和表达式](08-function/08-03-statement_expression.md)
  4. [高阶函数](08-function/08-04-high_order_function.md)
9. [模式匹配](09-match/09-00-overview.md)「wayslog」
  1. [match关键字](09-match/09-01-match.md)
  2. [模式](09-match/09-02-pattern.md)
10. [Trait （特征）](10-trait/10-00-overview.md)「JohnSmithX 160130」
  1. [trait关键字](10-trait/10-01-trait.md)
  2. [trait对象](10-trait/10-02-trait-object.md)
11. [泛型](11-generics/11-01-generics.md)「stormgbs 160120」
12. 集合类型（Collections）「wayslog」
13. [可变性、所有权、租借和生命期](13-ownership-system/13-00-ownership_system.md)「stormgbs 160105」
  1. [所有权](13-ownership-system/13-01-ownership.md)
  2. [引用和借用](13-ownership-system/13-02-borrowing_references.md)
  3. [生命周期](13-ownership-system/13-03-lifetimes.md)
14. [闭包](14-closure/14-00-overview.md)「qdao 160120」
  1. [闭包的语法](14-closure/14-01-syntax.md)
  2. [闭包的实现](14-closure/14-02-implementation.md)
  3. [闭包作为参数和返回值](14-closure/14-03-as_argument_return_value.md)
15. 迭代器「wayslog」
16. 模块和包系统、Prelude「daogangtang」
  1. [模块（module）和包（crate）](16-modules/16-01-module.md)
  2. [Prelude](16-modules/16-02-prelude.md)
17. [Option、Result与错误处理](17-error-handling/17-01-option-result.md)「JohnSmithX 160130」
18. [宏系统](18-macro/18-01-macro.md)「tennix 160222」
19. [堆、栈与Box](./19-heap-stack/heap-stack.md)「tennix 160222」
20. Rc, Arc, Mutex, RwLock, Cell, RefCell「daogangtang 160120」
  1. [Rc, Arc](./20-rcarc/20-01-rcarc.md)
  2. [Mutex, RwLock](./20-rcarc/20-02-mutex.md)
  3. [Cell, RefCell](./20-rcarc/20-03-cell.md)
21. 类型系统相关的Trait 「daogangtang 160130」
  1. [Into/From 及其在 String 和 &str 互转上的应用](./21-intoborrow/21-01-into.md)
  2. [AsRef, AsMut](./21-intoborrow/21-02-asref.md)
  3. [Borrow, BorrowMut, ToOwned](./21-intoborrow/21-03-borrow.md)
  4. [Deref 与 Deref coercions](./21-intoborrow/21-04-deref.md)
  5. [Cow 及其在 String 和 &str 上的应用](./21-intoborrow/21-05-cow.md)
22. Marker: Send, Sync, Copy, Sized, PhantomData「daogangtang」
23. 并发，并行，多线程编程「anzhihun 160120」
  1. [线程](./23-concurrency-parallel-threads/24-01-thread.md)
  2. [消息传递](./23-concurrency-parallel-threads/24-02-message-passing.md)
  3. [共享内存](./23-concurrency-parallel-threads/24-03-share-memory.md)
  4. [同步](./23-concurrency-parallel-threads/24-04-synchronize.md)
  5. [并行](./23-concurrency-parallel-threads/24-05-parallel.md)
24. Unsafe、原始指针「JohnSmithX 160130」
  1. [Unsafe](24-unsafety-rawpointer/24-01-unsafety.md)
  2. [原始指针](24-unsafety-rawpointer/24-02-raw-pointer.md)
25. FFI「42 160222」
26. [运算符重载](26-operator-overload/26-01-operator.md)「wayslog 160221」
27. [属性和编译器参数](27-attr-and-compiler-args/27-00-preface.md)「elton 160215」
  1. [属性](27-attr-and-compiler-args/27-01-attributes.md)
  2. [编译器参数](27-attr-and-compiler-args/27-02-rustc-options.md)
28. [Cargo参数配置](28-cargo-detailed-cfg/28-01-cargo-detailed-cfg.md)「fuyingfuying 160131」
29. 测试与评测「daogangtang 160222」
  1. [测试 (testing)](29-testing/29-01-threearchtest.md)
  2. 评测 (benchmark)
30. [代码风格](30-coding-style/30-01-style.md)「tiansiyuan」
31. Any与反射「wayslog」
32. 安全（safety）「daogangtang」
33. 常用数据结构实现「Naupio」
  1. [栈结构](33-data-struct/33-01-stack.md)
  2. [队列](33-data-struct/33-02-queue.md)
  3. [优先队列](33-data-struct/33-03-priority_queue.md)
  4. TODO...
34. 标准库介绍「wayslog」
  1. 操作系统与文件处理
  2. 时间日期处理
  3. 网络编程
35. 实战篇「wangyu190810 已完成」
  1. [实战：Json处理](35-action/json_data/readme.md)
  2. [实战：Web 应用开发入门](35-action/mysite/readme.md)
  3. [实战：使用Postgresql数据库](35-action/db/readme.md)
