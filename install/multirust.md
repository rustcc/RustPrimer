# Rust 版本管理工具: multirust

multirust 是一个简单的 Rust 版本管理工具.

项目主页是: <https://github.com/brson/multirust>

## Features

* 管理安装多个官方版本的 Rust 二进制程序.
* 配置基于目录的 Rust 工具链.
* 安装和更新来自 Rust 的发布通道: nightly, beta 和 stable.
* 接收来自发布通道更新的通知.
* 从官方安装历史版本的 nightly 工具链.
* 通过指定 stable 版本来安装.
* 安装额外的 std 用于交叉编译.
* 安装自定义的工具链.
* 独立每个安装的 Cargo metadata.
* 校验下载的 hash 值.
* 校验签名 (如果 GPG 存在).
* 断点续传.
* 只依赖 bash, curl 和常见 unix 工具.
* 支持 Linux, OS X, Windows(via MSYS2).

## 安装

```
curl -sf https://raw.githubusercontent.com/brson/multirust/master/blastoff.sh | sh
```

这个命令将会编译和安装 multirust, 安装过程中可能会提示你输入 sudo 的密码. 然后, 他会下载和安装 stable 版本的工具链, 当执行 rustc, rustdoc 和 cargo 时, 将会配置他为默认工具链.

`*nix` 上安装后工具链会被安装到 `~/.multirust/toolchains`.

## 卸载

```
curl -sf https://raw.githubusercontent.com/brson/multirust/master/blastoff.sh | sh -s -- --uninstall
```

## 用法

安装后会得到一个 multirust 命令, 多使用命令自带的帮助提示, 可以快速定位你需要功能.

### 帮助

运行 `multirust -h` 你将会得到如下提示:

```
❯ multirust -h
# Usage: multirust <command> [--verbose] [--version]
#
# Commands:
#
#     default          Set the default toolchain
#     override         Set the toolchain override for the current directory tree
#     update           Install or update a given toolchain (for example, "stable", "beta", "nightly")
#     show-override    Show information about the current override
#     show-default     Show information about the current default
#     list-overrides   List all overrides
#     list-toolchains  List all installed toolchains
#     remove-override  Remove an override, for current directory unless specified
#     remove-toolchain Uninstall a toolchain
#     list-available-targets
#                      List targets available to install
#     add-target       Add additional compilation targets to an existing toolchain
#     run              Run a command in an environment configured for a toolchain
#     delete-data      Delete all user metadata, including installed toolchains
#     upgrade-data     Upgrade the ~/.multirust directory from previous versions
#     doc              Open the documentation for the currently active toolchain
#     which            Report location of the currently active Rust tool.
#     help             Show help for this command or subcommands
#
# Use `multirust help <command>` for detailed help.
#
```

根据提示, 使用 `multirust help <command>` 来查看子命令的帮助, 基本看这些帮助文档就足够了.

下面着重介绍几个常用的命令.

`multirust default beta|stable|nightly` 配置默认的工具链.

`multirust override beta|stable|nightly` 配置一个目录以及其子目录的工具链.

`multirust show-default` 显示当前默认的工具链信息.

`multirust show-override` 显示当前目录的 override 的工具链信息.

`multirust list-toolchains` 显示所有安装的工具链.

`multirust list-overrides` 显示所有的 override 的工具链.

`multirust update` 更新所有的发布通道的工具链.
