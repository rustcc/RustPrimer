## A little about Rust
Rust is a system-level programming language designed to ensure memory and thread safety and to prevent segment errors. 
As a system-level programming language, its basic philosophy is "zero overhead abstraction". In theory, its speed is on par with C / C + + .

Rust can be classified as a generic, multi-paradigm, compiled programming language, similar to C or C ++. Unlike the two programming languages, Rust is thread-safe!

The goal of the Rust programming language is to create a highly secure and concurrent software system. It emphasizes security, concurrency, and memory control. 

Although Rust borrows C and C ++ syntax, it does not allow null pointers and hanging pointers, both of which are rooted in C and C ++. This means that Rust avoids system crashes, memory leaks, and insecure code associated with the former.

Rust has general control structures such as if else and loop statements for and while. Like C and C ++ style programming languages, the code snippet is placed in curly braces.

Rust uses implementation, trait, and structured types rather than classes. This is quite different from the inherited OO (object oriented) languages such as C ++ and Java. 

Rust does memory safety without the overhead of implementing an automated garbage collector. This is achieved through ownership / borrowing mechanisms, lifecycle, and type systems.

Here is an example of a code snippet, the classic Hello World application:

``` rust
fn main() {
  println!("hello, world");
}
```

The popular programming languages ​​that affect Rust include C, C ++, C #, Erlang, Haskell, OCaml, Ruby, Scheme, and Swift. Rust also affected C # 7, Elm, Idris, Swift.

Rust provides the installer, you only need to download from the official website and run the installer on the appropriate operating system. The installer supports 32-bit and 64-bit CPU architectures on Windows, Mac, and Linux (via scripts) for Apache License 2.0 or MIT Licenses.

## History 
Rust started as a personal project from the first Mozilla employee Graydon Hoare. From 2009 onwards - with the support of the Mozilla Institute - the 2010 project was announced. From 2010 to 2011 to achieve the bootstrap. Since then, Rust has undergone tremendous design changes and repetitions (extremely difficult), and finally released version 1.0 on May 15, 2015. 

In this research and development process, Rust established a strong and active community, ___***forming a complete set of stable and stable project contribution mechanism (which is really terrible).***___ Rust is now maintained by the Rust project developer community (https://github.com/rust-lang/rust).

Since the release of May 2001, a large number of excellent projects have emerged (which can be found on github for Rust), and large companies are increasingly actively involved in Rust's application development and feedback to open source communities.

This book (RustPrimer) aims to provide a correct, up-to-date, easy-to-understand Chinese tutorial for Chinese Rustaceans beginners. ___***This book will always be perfect follow up, never stop.***___

This book is the result of the joint efforts of the entire Rust Chinese community. Among them, Rustaceans who are involved in the preparation and revision of this book (in alphabetical order):

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
- [Archae0pteryx - us translation](https://github.com/archae0pteryx)

With this, their hard work and selfless dedication, we'd like to show our respect and say thanks!

I wish you a happy programming with Rust!
