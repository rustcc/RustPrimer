#Rust for Mac-os

Rust 支持主流的操作系统，Linux,Mac和 windows。

Rust 提供位 mac 用户提供了两种安装方式：

### 1、直接下载安装包：

直接下载安装包的话需要检查一下你当前操作系统是64位还是32位，分别下载对应的安装包。
查看操作系统请在终端执行如下命令:

`uname -a`

![Mac-os-inofrmatoin](image/01-get-mac-os-information.png)

如上图红色部分所示，如果是 **x86_64** 则证明是64位系统，需要[下载](https://static.rust-lang.org/dist/rust-1.5.0-x86_64-apple-darwin.pkg)64位安装包；
如果是**x86-32**则需要[下载](https://static.rust-lang.org/dist/rust-1.5.0-i686-apple-darwin.pkg)32位安装包

和安装普通的软件一样，直接运行安装包即可。


*在书写本章时，最新的稳定版本为1.5.0，*
	
### 2、命令行一键安装：
Rust 提供简单的一键安装，命令如下：

`curl -sSf https://static.rust-lang.org/rustup.sh | sh`

*此过程，有可能需要你输入几次密码*

你只需打开你的命令行执行如上代码就可以了。（注意，你可能需要一个梯子，否则会遇到一些类似*Could not resolve host: static.rust-lang.org*的错误）

### 3.验证安装：
如果你完成了上面任意一个步骤，请执行如下命令：

`rustc --version`

如果看到如下信息，表明你安装成功：

`rustc 1.5.0 (3d7cd77e4 2015-12-04)`

如果提示没有 *rustc* 命令，那么请回顾你是否有某个地方操作不对，请回过头来再看一遍文档。

**注意**

除了稳定版之外，Rust 还提供了 Beta 和 Nightly 版本，下载地址如下：
https://www.rust-lang.org/downloads.html

如果你不想安装 Rust 在你的电脑上，但是你还是像尝试一下 rust，那么这里有一个在线的环境：http://play.rust-lang.org/
