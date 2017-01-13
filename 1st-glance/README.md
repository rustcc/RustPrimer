Rust 是一门系统级编程语言，被设计为保证内存和线程安全，并防止段错误。作为系统级编程语言，它的基本理念是 “零开销抽象”。理论上来说，它的速度与 C / C++ 同级。

Rust 可以被归为通用的、多范式、编译型的编程语言，类似 C 或者 C++。与这两门编程语言不同的是，Rust 是线程安全的！

Rust 编程语言的目标是，创建一个高度安全和并发的软件系统。它强调安全性、并发和内存控制。尽管 Rust 借用了 C 和 C++ 的语法，它不允许空指针和悬挂指针，二者是 C 和 C++ 中系统崩溃、内存泄露和不安全代码的根源。

Rust 中有诸如 if else 和循环语句 for 和 while 的通用控制结构。和 C 和 C++ 风格的编程语言一样，代码段放在花括号中。

Rust 使用实现（implementation）、特征（trait）和结构化类型（structured type）而不是类（class）。这点，与基于继承的OO语言 C++, Java 有相当大的差异。而跟 Ocaml, Haskell 这类函数式语言更加接近。

Rust 做到了内存安全而无需 .NET 和 Java 编程语言中实现自动垃圾收集器的开销，这是通过所有权/借用机制、生命周期、以及类型系统来达到的。

下面是一个代码片段的例子，经典的 Hello World 应用：

``` rust
fn main() {
  println!("hello, world");
}
```

影响了 Rust 的流行的编程语言包括 C, C++, C#, Erlang, Haskell, OCaml, Ruby, Scheme 和 Swift 等等。Rust 也影响了 C# 7, Elm, Idris, Swift。

Rust 提供了安装程序，你只需要从官网下载并在相应的操作系统上运行安装程序。安装程序支持 Windows、Mac 和 Linux（通过脚本）上的32位和64位 CPU 体系架构，适用 Apache License 2.0 或者 MIT Licenses。

Rust 运行在以下操作系统上：Linux, OS X, Windows, FreeBSD, Android, iOS。

简单提一下 Rust 的历史。Rust 最早是 Mozilla 雇员 Graydon Hoare 的一个个人项目，从 2009 年开始，得到了 Mozilla 研究院的支助，2010 年项目对外公布。2010 ～2011 年间实现的自举。从此以后，Rust 经历了巨大的设计变化和反复（历程极其艰辛），终于在 2015 年 5 月 15日发布了 1.0 版。在这个研发过程中，Rust 建立了一个强大活跃的社区，形成了一整套完善稳定的项目贡献机制（这是真正的可怕之处）。Rust 现在由 Rust 项目开发者社区（https://github.com/rust-lang/rust ）维护。

自 15 年 5 月 1.0 发布以来，涌现了大量优秀的项目（可以 github 上搜索 Rust 查找），大公司也逐渐积极参与 Rust 的应用开发，以及回馈开源社区。

本书（RustPrimer）旨在为中文 Rustaceans 初学者提供一个正确、最新、易懂的中文教程。本书会一直完善跟进，永不停歇。

本书是整个 Rust 中文社区共同努力的结果。其中，参与本书书写及校订的 Rustacean 有（排名不分先后）：

- [daogangtang（Mike猫）](https://github.com/daogangtang)
- [wayslog（猫猫反抗团团长）](https://github.com/wayslog)
- [marvin-min](https://github.com/marvin-min)
- [tiansiyuan](https://github.com/tiansiyuan)
- [marvinguo](https://github.com/marvinguo)
- ee0703
- fuyingfuying
- qdao
- JohnSmithX
- [stormgbs (AX) ](https://github.com/stormgbs)
- tennix
- anzhihun
- zonyitoo（Elton, e猫）
- 42
- [Naupio（N猫）](https://github.com/Naupio)
- F001（失落的神喵）
- wangyu190810
- domty
- [MarisaKirisame（帅气可爱魔理沙）](https://github.com/MarisaKirisame)
- [Liqueur Librazy](https://github.com/Librazy)
- [Knight42](https://github.com/knight42)
- [Ryan Kung](https://github.com/ryankung)
- lambdaplus
- doomsplayer
- lucklove
- veekxt
- lk-chen
- RyanKung
- arrowrowe
- marvin-min
- ghKelo
- wy193777
- domty
- xusss
- wangyu190810
- nextzhou
- zhongke
- [ryuki](https://github.com/3442853561)
- codeworm96
- anzhihun
- lidashuang
- sceext2
- loggerhead
- twq0076262
- passchaos
- yyrust
- markgeek
- ts25504
- overvenus
- Akagi201
- theJian
- jqs7
- ahjdzx
- chareice
- chenshaobo
- marvinguo
- izgzhen
- ziqin
- peng1999

等。在此，向他们的辛苦工作和无私奉献表示尊敬和感谢！

祝用 Rust 编程愉快！
