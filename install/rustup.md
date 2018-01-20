# Rust 版本管理工具: rustup

rustup 是rust官方的版本管理工具。应当作为安装 Rust 的首选。

项目主页是: <https://github.com/rust-lang-nursery/rustup.rs>

## Features

* 管理安装多个官方版本的 Rust 二进制程序。
* 配置基于目录的 Rust 工具链。
* 安装和更新来自 Rust 的发布通道: nightly, beta 和 stable。
* 接收来自发布通道更新的通知。
* 从官方安装历史版本的 nightly 工具链。
* 通过指定 stable 版本来安装。
* 安装额外的 std 用于交叉编译。
* 安装自定义的工具链。
* 独立每个安装的 Cargo metadata。
* 校验下载的 hash 值。
* 校验签名 (如果 GPG 存在)。
* 断点续传。
* 只依赖 bash, curl 和常见 unix 工具。
* 支持 Linux, OS X, Windows(via MSYS2)。

## 安装

### Windows

在[rustup的主页](http://www.rustup.rs)下载并运行[rustup-init.exe](https://win.rustup.rs/),并按照提示选择选项。

```
Welcome to Rust!

This will download and install the official compiler for the Rust programming
language, and its package manager, Cargo.

It will add the cargo, rustc, rustup and other commands to Cargo's bin
directory, located at:

  C:\Users\Liqueur Librazy\.cargo\bin

This path will then be added to your PATH environment variable by modifying the
HKEY_CURRENT_USER/Environment/PATH registry key.

You can uninstall at any time with rustup self uninstall and these changes will
be reverted.

Current installation options:

   default host triple: x86_64-pc-windows-msvc
     default toolchain: stable
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
```

三个选项分别是

1) 开始安装（默认选项）
2) 自定义安装
3) 取消

其中自定义安装可以更改默认架构与工具链、是否添加 PATH。例如想要选择 nightly 工具链可以进行以下自定义

```
I'm going to ask you the value of each these installation options.
You may simply press the Enter key to leave unchanged.

Default host triple?


Default toolchain? (stable/beta/nightly)
nightly

Modify PATH variable? (y/n)

```

设置完毕后，选择 1 以开始安装。

### Linux & macOS

运行以下命令

```
curl https://sh.rustup.rs -sSf | sh
```

这个命令将会编译和安装 rustup, 安装过程中可能会提示你输入 sudo 的密码。 然后, 他会下载和安装 stable 版本的工具链, 当执行 rustc, rustdoc 和 cargo 时, 将会配置他为默认工具链。

`Unix` 上安装后工具链会被安装到 `$HOME/.cargo/bin` 目录。

`.cargo/bin` 目录会被添加到系统的 `$PATH` 环境变量,重新登录后即可使用 `rustc`，`cargo` 等命令。

## 卸载

```
rustup self uninstall
```

## 用法

安装后会得到一个 rustup 命令, 多使用命令自带的帮助提示, 可以快速定位你需要功能。

### 帮助

运行 ` rustup -h` 你将会得到如下提示:

```
❯ rustup -h
rustup 1.5.0 (92d0d1e9e 2017-06-24)
The Rust toolchain installer

USAGE:
    rustup.exe [FLAGS] [SUBCOMMAND]

FLAGS:
    -v, --verbose    Enable verbose output
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    show           Show the active and installed toolchains
    update         Update Rust toolchains and rustup
    default        Set the default toolchain
    toolchain      Modify or query the installed toolchains
    target         Modify a toolchain's supported targets
    component      Modify a toolchain's installed components
    override       Modify directory toolchain overrides
    run            Run a command with an environment configured for a given toolchain
    which          Display which binary will be run for a given command
    doc            Open the documentation for the current toolchain
    self           Modify the rustup installation
    set            Alter rustup settings
    completions    Generate completion scripts for your shell
    help           Prints this message or the help of the given subcommand(s)

DISCUSSION:
    rustup installs The Rust Programming Language from the official
    release channels, enabling you to easily switch between stable,
    beta, and nightly compilers and keep them updated. It makes
    cross-compiling simpler with binary builds of the standard library
    for common platforms.

    If you are new to Rust consider running `rustup doc --book` to
    learn Rust.

```

根据提示, 使用 `rust help <command>` 来查看子命令的帮助。

`rustup doc --book` 会打开英文版的 [The Rust Programming Language](https://doc.rust-lang.org/book/)。

### 常用命令

`rustup default <toolchain>` 配置默认工具链。

`rustup show` 显示当前安装的工具链信息。

`rustup update` 检查安装更新。

`rustup toolchain [SUBCOMMAND]` 配置工具链

> * `rustup toolchain install <toolchain>` 安装工具链。
> * `rustup toolchain uninstall <toolchain>` 卸载工具链。
> * `rustup toolchain link <toolchain-name> "<toolchain-path>"` 设置[自定义工具链](https://github.com/rust-lang-nursery/rustup.rs#working-with-custom-toolchains-and-local-builds)。
> 
> 其中标准的 `<toolchain>`具有如下的形式
> ```
> `<channel>[-<date>][-<host>]`
> <channel>       = stable|beta|nightly|<version>
> <date>          = YYYY-MM-DD
> <host>          = <target-triple>
> ```
> 如 `stable-x86_64-pc-windows-msvc` `nightly-2017-7-25` `1.18.0` 等都是合法的toolchain名称。

`rustup override [SUBCOMMAND]` 配置一个目录以及其子目录的默认工具链

> 使用 `--path <path>` 指定目录或在某个目录下运行以下命令
> 
> * `rustup override set <toolchain>` 设置该目录以及其子目录的默认工具链。
> * `rustup override unset` 取消目录以及其子目录的默认工具链。
> 
> 使用 `rustup override list` 查看已设置的默认工具链。

`rustup target [SUBCOMMAND]` 配置工具链的可用目标

> * `rustup target add <target>` 安装目标。
> * `rustup target remove <target>` 卸载目标。
> * `rustup target add --toolchain <toolchain> <target>` 为特定工具链安装目标。

`rustup component` 配置 rustup 安装的组件

> * `rustup component add <component>` 安装组件
> * `rustup component remove <component>` 卸载组件
> * `rustup component list` 列出可用组件
>
> 常用组件：
> * Rust 源代码 `rustup component add rust-src`
> * Rust Langular Server (RLS) `rustup component add rls`
