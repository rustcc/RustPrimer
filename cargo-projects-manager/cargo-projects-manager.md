# cargo简介

曾几何时，对于使用惯了`C/C++`语言的猿们来说，项目代码的组织与管理绝对是一场噩梦。为了解决`C/C++`项目的管理问题，猿神们想尽了各种办法，开发出了各种五花八门的项目管理工具，从一开始的`automake`到后来的`cmake`、`qmake`等等，但结果并不如人意，往往是解决了一些问题，却引入了更多的问题，`C/C++`猿们经常会陷入在掌握语言本身的同时，还要掌握复杂的构建工具语法的窘境。无独有偶，`java`的项目代码组织与管理工具`ant`和`maven`也存在同样的问题。复杂的项目管理配置参数，往往让猿们不知所措。

作为一门现代语言，`rust`自然要摒弃石器时代项目代码管理的方法和手段。`rust`项目组为各位猿提供了超级大杀器`cargo`，以解决项目代码管理所带来的干扰和困惑。用过`node.js`的猿们，应该对`node.js`中的神器`npm`、`grunt`、`gulp`等工具印象深刻。作为新一代静态语言中的翘楚，`rust`官方参考了现有语言管理工具的优点，于是就产生了`cargo`。

言而总之，作为`rust`的代码组织管理工具，`cargo`提供了一系列的工具，从项目的建立、构建到测试、运行直至部署，为`rust`项目的管理提供尽可能完整的手段。同时，与`rust`语言及其编译器`rustc`本身的各种特性紧密结合，可以说既是语言本身的知心爱人，又是`rust`猿们的贴心小棉袄，谁用谁知道。
废话就不多说了，直接上例子和各种高清无马图。

# cargo入门
首先，当然还是废话，要使用cargo，自然首先要安装cargo。安装cargo有三种方法，前两种方法请参见rust的安装方法，因为cargo工具是官方正统出身，当然包含在官方的分发包中。第三种方法即从[`cargo`](https://github.com/rust-lang/cargo)项目的源码仓库进行构建。Oh，My God。的确是废话。

好了，假设各位已经安装好了cargo，大家和我一起学一下起手式。当然了，猿的世界，起手式一般都千篇一律——那就是`hello world`大法。
在终端中输入

```bash
$ cargo new hello_world --bin
```

上述命令使用**cargo new**在当前目录下新建了基于cargo项目管理的rust项目，项目名称为hello_world，--bin表示该项目将生成可执行文件。具体生成的项目目录结构如下：

```bash
$ cd hello_world
$ tree .
.
├── Cargo.toml
└── src
    └── main.rs

1 directory, 2 files
```

大家可以在终端中输入上述命令，敲出回车键之后即可看到上述结果，或者直接去编辑器或文件管理器中去观察即可。
打开main.rs文件，可以看到，cargo new命令为我们自动生成了hello_world运行所必须的所有代码：

```rust
fn main() {
    println!("Hello, world!");
}
```

好了，心急的猿们可能已经迫不及待的脱裤子了，好吧，我们先来构建并看看cargo有多神奇，在终端中输入：

```bash
$ cargo build
```

稍等片刻，cargo会自动为我们构建好高清应用所需的一切，对于这个起手式来说，缓冲不会超过5秒，12秒88的选手要憋住了。

```bash
$ cargo run
    Running `target/debug/hello_world`
Hello, world!

```

看到了什么，看到了什么，吓尿了有木有，吓尿了有木有。好了，cargo就是这么简单。

当然了，说cargo美，并不仅仅是简单这么简单，cargo虽然简单，但是很强大。有多么强大？？可以说，基本上rust开发管理中所需的手段，cargo都有。很小很强大，既强又有节操，不带马，学习曲线几乎为零。

# 基于cargo的rust项目组织结构
这次不说废话了，先上高清无马图：

![cargo项目组织结构](../images/project-structure.png)

对上述cargo默认的项目结构解释如下：

##### `cargo.toml`和`cargo.lock`文件总是位于项目根目录下。
##### 源代码位于`src`目录下。
##### 默认的库入口文件是`src/lib.rs`。
##### 默认的可执行程序入口文件是`src/main.rs`。
##### 其他可选的可执行文件位于`src/bin/*.rs`(这里每一个rs文件均对应一个可执行文件)。
##### 外部测试源代码文件位于`tests`目录下。
##### 示例程序源代码文件位于`examples`。
##### 基准测试源代码文件位于`benches`目录下。

好了，大家一定谨记这些默认规则，最好按照这种模式来组织自己的rust项目。

# cargo.toml和cargo.lock
`cargo.toml`和`cargo.lock`是cargo项目代码管理的核心两个文件，cargo工具的所有活动均基于这两个文件。

`cargo.toml`是cargo特有的项目数据描述文件，对于猿们而言，`cargo.toml`文件存储了项目的所有信息，它直接面向rust猿，猿们如果想让自己的rust项目能够按照期望的方式进行构建、测试和运行，那么，必须按照合理的方式构建'cargo.toml'。

而`cargo.lock`文件则不直接面向猿，猿们也不需要直接去修改这个文件。lock文件是cargo工具根据同一项目的toml文件生成的项目依赖详细清单文件，所以我们一般不用不管他，只需要对着`cargo.toml`文件撸就行了。

```toml
[package]
name = "hello_world"
version = "0.1.0"
authors = ["fuying"]

[dependencies]
```

toml文件是由诸如[package]或[dependencies]这样的段落组成，每一个段落又由多个字段组成，这些段落和字段就描述了项目组织的基本信息，例如上述toml文件中的[package]段落描述了`hello_world`项目本身的一些信息，包括项目名称（对应于name字段）、项目版本（对应于version字段）、作者列表（对应于authors字段）等；[dependencies]段落描述了`hello_world`项目的依赖项目有哪些。

下面我们来看看toml描述文件中常用段落和字段的意义。

# package段落
[package]段落描述了软件开发者对本项目的各种元数据描述信息，例如[name]字段定义了项目的名称，[version]字段定义了项目的当前版本，[authors]定义了该项目的所有作者，当然，[package]段落不仅仅包含这些字段，[package]段落的其他可选字段详见cargo参数配置章节。

# 定义项目依赖
使用cargo工具的最大优势就在于，能够对该项目的各种依赖项进行方便、统一和灵活的管理。这也是使用cargo对rust 的项目进行管理的重要目标之一。在cargo的toml文件描述中，主要通过各种依赖段落来描述该项目的各种依赖项。toml中常用的依赖段落包括一下几种：
- 基于rust官方仓库crates.io，通过版本说明来描述：
- 基于项目源代码的git仓库地址，通过URL来描述：
- 基于本地项目的绝对路径或者相对路径，通过类Unix模式的路径来描述：
这三种形式具体写法如下：

```toml
[dependencies]
typemap = "0.3"
plugin = "0.2*"
hammer = { version = "0.5.0"}
color = { git = "https://github.com/bjz/color-rs" }
geometry = { path = "crates/geometry" }
```

上述例子中，2-4行为方法一的写法，第5行为方法二的写法，第6行为方法三的写法。
这三种写法各有用处，如果项目需要使用crates.io官方仓库来管理项目依赖项，推荐使用第一种方法。如果项目开发者更倾向于使用git仓库中最新的源码，可以使用方法二。方法二也经常用于当官方仓库的依赖项编译不通过时的备选方案。方法三主要用于源代码位于本地的依赖项。

# 定义集成测试用例

cargo另一个重要的功能，即将软件开发过程中必要且非常重要的测试环节进行集成，并通过代码属性声明或者toml文件描述来对测试进行管理。其中，单元测试主要通过在项目代码的测试代码部分前用`#[test]`属性来描述，而集成测试，则一般都会通过toml文件中的[[test]]段落进行描述。
例如，假设集成测试文件均位于tests文件夹下，则toml可以这样来写：

```toml
[[test]]
name = "testinit"
path = "tests/testinit.rs"

[[test]]
name = "testtime"
path = "tests/testtime.rs"
```

上述例子中，name字段定义了集成测试的名称，path字段定义了集成测试文件相对于本toml文件的路径。
看看，定义集成测试就是如此简单。
需要注意的是:

- 如果没有在Cargo.toml里定义集成测试的入口，那么tests目录(不包括子目录)下的每个rs文件被当作集成测试入口.
- 如果在Cargo.toml里定义了集成测试入口，那么定义的那些rs就是入口，不再默认指定任何集成测试入口.

# 定义项目示例和可执行程序
**上面我们介绍了cargo项目管理中常用的三个功能，还有两个经常使用的功能：example用例的描述以及bin用例的描述。其描述方法和test用例描述方法类似。不过，这时候段落名称'[[test]]'分别替换为：'[[example]]'或者'[[bin]]'。例如：**

```toml
[[example]]
name = "timeout"
path = "examples/timeout.rs"

[[bin]]
name = "bin1"
path = "bin/bin1.rs"

```

对于'[[example]]'和'[[bin]]'段落中声明的examples和bins，需要通过'cargo run --example NAME'或者'cargo run --bin NAME'来运行，其中NAME对应于你在name字段中定义的名称。

# 构建、清理、更新以及安装
领会了toml描述文件的写法，是一个重要的方面。另一个重要的方面，就是cargo工具本身为我们程序猿提供的各种好用的工具。如果大家感兴趣，自己在终端中输入'cargo --help'查看即可。其中开发时最常用的命令就是'cargo build'，用于构建项目。此外，'cargo clean'命令可以清理target文件夹中的所有内容；'cargo update'根据toml描述文件重新检索并更新各种依赖项的信息，并写入lock文件，例如依赖项版本的更新变化等等；'cargo install'可用于实际的生产部署。这些命令在实际的开发部署中均是非常有用的。

**cargo更多详细用法请参见['28. cargo参数配置'](../cargo-detailed-cfg/cargo-detailed-cfg.md)**
