# 编译器参数

本章将介绍Rust编译器的参数。

Rust编译器程序的名字是`rustc`，使用它的方法很简单：

```bash
$ rustc [OPTIONS] INPUT
```

其中，`[OPTIONS]`表示编译参数，而`INPUT`则表示输入文件。而编译参数有以下可选：

* `-h, --help` - 输出帮助信息到标准输出；

* `--cfg SPEC` - 传入自定义的条件编译参数，使用方法如

  ```rust
  fn main() {
      if cfg!(hello) {
          println!("world!");
      }
  }
  ```

  如上例所示，若`cfg!(hello)`成立，则运行程序就会输出`"world"`到标准输出。我们把这个文件保存为`hello.rs`然后编译它

  ```bash
  $ rustc --cfg hello hello.rs
  ```

  运行它就会看到屏幕中输出了`world!`。

* `-L [KIND=]PATH` - 往链接路径中加入一个文件夹，并且可以指定这个路径的类型（Kind），这些类型包括
  - `dependency` - 在这个路径下找依赖的文件，比如说`mod`；
  - `crate` - 只在这个路径下找`extern crate`中定义的库；
  - `native` - 只在这个路径下找Native库；
  - `framework` - 只在OS X下有用，只在这个路径下找Framework；
  - `all` - 默认选项。

* `-l [KIND=]NAME` - 链接一个库，这个库可以指定类型（Kind）
  - `static` - 静态库；
  - `dylib` - 动态库；
  - `framework` - OS X的Framework。

  如果不传，默认为`dylib`。

  此处举一个例子如何手动链接一个库，我们先创建一个文件叫`myhello.rs`，在里面写一个函数

  ```rust
  // myhello.rs

  /// 这个函数仅仅向标签输出打印 Hello World!
  /// 不要忘记要把它标记为 pub 哦。
  pub fn print_hello() {
      println!("Hello World!");
  }
  ```

  然后把这个文件编译成一个静态库，`libmyhello.a`

  ```bash
  $ rustc --crate-type staticlib myhello.rs
  ```

  然后再创建一个`main.rs`，链接这个库并打印出"Hello World!"

  ```rust
  // main.rs

  // 指定链接库 myhello
  extern crate myhello;

  fn main() {
      // 调用库函数
      myhello::print_hello();
  }
  ```

  编译`main.rs`

  ```bash
  $ rustc -L. -lmyhello main.rs
  ```

  运行`main`，就会看到屏幕输出"Hello World!"啦。

* `--crate-type` - 指定编译输出类型，它的参数包括
  - `bin` - 二进行可执行文件
  - `lib` - 编译为库
  - `rlib` - Rust库
  - `dylib` - 动态链接库
  - `staticlib` - 静态链接库

* `--crate-name` - 指定这个Crate的名字，默认是文件名，如`main.rs`编译成可执行文件时默认是`main`，但你可以指定它为`foo`

  ```bash
  $ rustc --crate-name foo main.rs
  ```

  则会输出`foo`可执行文件。

* `--emit` - 指定编译器的输出。编译器默认是输出一个可执行文件或库文件，但你可以选择输出一些其它的东西用于Debug

  - `asm` - 输出汇编
  - `llvm-bc` - [LLVM Bitcode](http://llvm.org/docs/BitCodeFormat.html)；
  - `llvm-ir` - [LLVM IR](http://llvm.org/docs/LangRef.html)，即LLVM中间码（LLVM Intermediate Representation）；
  - `obj` - Object File（就是`*.o`文件）；
  - `link` - 这个是要结合其它`--emit`参数使用，会执行Linker再输出结果；
  - `dep-info` - 文件依赖关系（Debug用，类似于Makefile一样的依赖）。

  以上参数可以同时使用，使用逗号分割，如

  ```bash
  $ rustc --emit asm,llvm-ir,obj main.rs
  ```

  同时，在最后可以加一个`=PATH`来指定输出到一个特定文件，如

  ```bash
  $ rustc --emit asm=output.S,llvm-ir=output.ir main.rs
  ```

  这样会把汇编生成到`output.S`文件中，把LLVM中间码输出到`output.ir`中。

* `--print` - 打印一些信息，参数有
  - `crate-name` - 编译目标名；
  - `file-names` - 编译的文件名；
  - `sysroot` - 打印Rust工具链的根目录地址。

* `-g` - 在目标文件中保存符号，这个参数等同于`-C debuginfo=2`。

* `-O` - 开启优化，这个参数等同于`-C opt-level=2`。

* `-o FILENAME` - 指定输出文件名，同样适用于`--emit`的输出。

* `--out-dir DIR` - 指定输出的文件夹，默认是当前文件夹，且会忽略`-o`配置。

* `--explain OPT` - 解释某一个编译错误，比如

  若你写了一个`main.rs`，使用了一个未定义变量`f`

  ```rust
  fn main() {
      f
  }
  ```

  编译它时编译器会报错：

  ```
  main.rs:2:5: 2:6 error: unresolved name `f` [E0425]
  main.rs:2     f
                ^
  main.rs:2:5: 2:6 help: run `rustc --explain E0425` to see a detailed explanation
  error: aborting due to previous error
  ```

  虽然错误已经很明显，但是你也可以让编译器解释一下，什么是`E0425`错误：

  ```bash
  $ rustc --explain E0425
  // 编译器打印的说明
  ```

* `--test` - 编译成一个单元测试可执行文件

* `--target TRIPLE` - 指定目标平台，基本格式是`cpu-manufacturer-kernel[-os]`，例如

  ```bash
  ## 64位OS X
  $ rustc --target x86_64-apple-darwin
  ```

* `-W help` - 打印Linter的所有可配置选项和默认值。

* `-W OPT, --warn OPT` - 设置某一个Linter选项为Warning。
* `-A OPT, --allow OPT` - 设置某一个Linter选项为Allow。
* `-D OPT, --deny OPT` - 设置某一个Linter选项为Deny。
* `-F OPT, --forbit OPT` - 设置某一个Linter选项为Forbit。

* `-C FLAG[=VAL], --codegen FLAG[=VAL]` - 目标代码生成的的相关参数，可以用`-C help`来查看配置，值得关注的几个是
  - `linker=val` - 指定链接器；
  - `linker-args=val` - 指定链接器的参数；
  - `prefer-dynamic` - 默认Rust编译是静态链接，选择这个配置将改为动态链接；
  - `debug-info=level` - Debug信息级数，`0` = 不生成，`1` = 只生成文件行号表，`2` = 全部生成；
  - `opt-level=val` - 优化级数，可选`0-3`；
  - `debug_assertion` - 显式开启`cfg(debug_assertion)`条件。

* `-V, --version` - 打印编译器版本号。

* `-v, --verbose` - 开启啰嗦模式（打印编译器执行的日志）。

* `--extern NAME=PATH` - 用来指定外部的Rust库（`*.rlib`）名字和路径，名字应该与`extern crate`中指定的一样。

* `--sysroot PATH` - 指定工具链根目录。

* `-Z flag` - 编译器Debug用的参数，可以用`-Z help`来查看可用参数。

* `--color auto|always|never` - 输出时对日志加颜色
  - `auto` - 自动选择加还是不加，如果输出目标是虚拟终端（TTY）的话就加，否则就不加；
  - `always` - 给我加！
  - `never` - 你敢加？
