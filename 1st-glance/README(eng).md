Rust is a systems-level programming language designed to ensure memory and thread safety, and prevent segmentation faults. As a systems-level language, its fundamental philosophy is "zero-cost abstraction." Theoretically, its speed is comparable to C/C++.

Rust can be categorized as a general-purpose, multi-paradigm, compiled programming language, similar to C or C++. Unlike these two languages, Rust is thread-safe!

The goal of the Rust programming language is to create highly safe and concurrent software systems. It emphasizes safety, concurrency, and memory control. Although Rust borrows syntax from C and C++, it does not allow null pointers or dangling pointers, which are the root cause of system crashes, memory leaks, and unsafe code in C and C++.

Rust has general control structures such as if else and loops for and while. Like C and C++-style programming languages, code blocks are placed in curly braces.

Rust uses implementations (implementation), traits (trait), and structured types (structured type) instead of classes (class). This is a significant difference from object-oriented languages based on inheritance, such as C++, Java. It is more similar to functional languages like Ocaml and Haskell.

Rust achieves memory safety without the overhead of implementing an automatic garbage collector, as found in .NET and Java programming languages. This is achieved through ownership/borrowing mechanisms, lifetimes, and the type system.

Here is an example of a code snippet, the classic Hello World application:

```rust
fn main() {
  println!("hello, world");
}
```

Popular programming languages that influenced Rust include C, C++, C#, Erlang, Haskell, OCaml, Ruby, Scheme, and Swift, among others. Rust also influenced C# 7, Elm, Idris, and Swift.

Rust provides an installer, which you can download from the website and run on the corresponding operating system. The installer supports 32-bit and 64-bit CPU architectures on Windows, Mac, and Linux (via scripts), and is licensed under the Apache License 2.0 or MIT Licenses.

Rust runs on the following operating systems: Linux, OS X, Windows, FreeBSD, Android, and iOS.

Here is a brief overview of Rust's history. Rust was originally a personal project of Mozilla employee Graydon Hoare, starting in 2009. It received support from the Mozilla Research Lab in 2010, and the project was announced to the public in 2010. Self-hosting was achieved in 2010–2011. Since then, Rust has undergone significant design changes and iterations (a very challenging journey), and finally released version 1.0 on May 15, 2015. During this development process, Rust built a strong and active community, and formed a complete and stable project contribution mechanism (this is the real scary thing). Rust is now maintained by the Rust project developer community ([https://github.com/rust-lang/rust](https://github.com/rust-lang/rust)).

Since the release of version 1.0 in May 2015, a large number of excellent projects have emerged (you can search for Rust on GitHub), and large companies have gradually become actively involved in the development and application of Rust, and have given back to the open source community.

This book (RustPrimer) aims to provide a correct, up-to-date, and easy-to-understand Chinese tutorial for Chinese Rustaceans beginners. This book will continue to be improved and followed up, never stopping.

This book is the result of the joint efforts of the entire Rust Chinese community. The Rustaceans who participated in the writing and proofreading of this book are (in no particular order):

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

etc. Here, we express our respect and gratitude for their hard work and selfless dedication!

May you enjoy programming in Rust!