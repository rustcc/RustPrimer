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

按github上的markdown规范来。

## 写作要求

理论与实践结合，多举例，把事情讲清楚。必要时逐行分析，不怕啰嗦。要求描述精确，给出的示例尽量完整，能复制到文件中直接编译。所有给的例子，都要求能正常编译通过。举例的代码风格要符合官方规范，尽量消除警告。

## 时间规划

初步规划，在2016年2月29日之前，我们能完成第一版 v1.0。


## 大纲

1. 初识Rust
2. 安装Rust「marvin-min 160105」
  1. [Linux](./02-install/02-01-install_rust_on_linux.md)
  2. [Mac](./02-install/02-02-install_rust_on_mac_os.md)
  3. [Windows](./02-install/02-03-install_rust_on_windows.md)
3. 编辑器
  1. [前期准备](./03-editors/03-01-before.md)「wayslog 160105」
  1. [vim](./03-editors/03-02-vim.md)「wayslog 160105」
  2. [emacs](./03-editors/03-03-emacs.md)「tiansiyuan 160120」
  3. [vscode](./03-editors/03-04-emacs.md)「daogangtang 160105」
  4. [atom](./03-editors/03-05-atom.md)「wayslog 160105」
  5. sublime
  6. visual studio
  7. eclipse
  8. Intellij IDEA
  9. [spacemacs](./03-editors/03-10-spacemacs.md)「wayslog 160105」
4. Rust一小时快速入门「ee0703 160120」
5. Cargo项目管理器、crate「fuyingfuying 160131」
6. 基本程序结构「daogangtang 160131」
  1. [注释](./06-flow/06-01-comment.md)
  2. [条件](./06-flow/06-02-condition.md)
  3. [循环](./06-flow/06-03-repeatition.md)
7. 基础类型和运算符「wayslog 160125」
  1. [基础类型](7-primitive-type/07-01-types.md)
  3. [字符串类型](7-primitive-type/07-02-strings.md)
  4. [基本操作符和字符串的格式化](7-primitive-type/07-03-operator-and-format.md)
8. 函数「qdao 160120」
9. 模式匹配「andelf 160120」
10. Trait （特征）「JohnSmithX 160130」
11. 泛型「stormgbs 160120」
12. 集合类型（Collections）「cai-lw 160205」
13. 可变性、所有权、租借和生命期「stormgbs 160105」
14. 闭包「qdao 160120」
15. 迭代器「andelf 160120」
16. 模块和包系统、Prelude「jessiex 160120」
17. Option、Result与错误处理「JohnSmithX 160130」
18. 宏系统
19. 堆、栈与Box
20. 常用数据结构实现
21. Rc, Arc, Cell, RefCell, UnsafeCell「daogangtang 160120」
22. Marker: Send, Sync, Copy, Clone「daogangtang 160125」
23. 通用引用类型 Borrow, AsRef, Into「daogangtang 160130」
24. 多线程编程「anzhihun 160120」
25. 并发「anzhihun 160120」
26. 并行「anzhihun 160120」
27. 安全
28. Unsafe、原始指针「JohnSmithX 160130」
29. FFI
30. 运算符重载
31. Any与反射
32. 属性和编译器参数「elton 160215」
33. Cargo参数配置「fuyingfuying 160131」
34. 测试与评测
35. 代码风格「tiansiyuan」
36. 标准库介绍
  1. 操作系统与文件处理
  2. 时间日期处理
  3. 网络编程
37. 实战篇
  1. 实战：Json处理
  2. 实战：Web 应用开发入门
  3. 实战：使用Postgresql数据库
