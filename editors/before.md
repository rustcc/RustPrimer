# 前期准备

## 下载 Rust 源代码（供 racer 使用）

### 从github下载

`git clone https://github.com/rust-lang/rust.git`

### 从官网下载源代码包

下载地址： `https://static.rust-lang.org/dist/rustc-nightly-src.tar.gz`

## racer
racer是一个由rust的爱好者提供的rust自动补全和语法分析工具，被用来提供基本的补全功能和定义跳转功能。其本身完全由rust写成，补全功能已经比较完善了。

我们可以通过如下的方式获取它：

### cargo自动安装
在rust 1.5版本以后，其安装包自带的cargo工具已经支持了cargo install命令，这个命令可以帮助我们通过简单的方式获取到`racer`的最新版。

在linux和unix系统中，你可以通过

```bash
sudo /usr/local/bin/cargo install --git 'https://github.com/phildawes/racer.git'
# 注：现已更新为： cargo install racer
```

来安装最新版的racer。（注：windows版的请大家补充，笔者还没有真正实践过。）

### 编译安装

事实上我更推荐有条件的用户通过这种方式安装，因为自己实战操作一遍总是有些收获的。

#### 下载源码

首先，我们需要下载racer的源码

```
git clone https://github.com/phildawes/racer.git
```

#### 进行编译

然后，进入目录然后进行编译

```
cd racer && cargo build --release
```

这样，我们会得到racer的二进制文件在 `target/release/racer`目录

#### 设置环境变量

最后，为了更好的支持racer，我们需要在[rust官网](https://www.rust-lang.org/downloads.html)下载到源码文件，请下载你对应平台的文件。然后将其解压到一个目录下。

然后，linux和unix用户请在你的`.bashrc`或者`.zshrc`的最后加入如下一行（注意用你的源码文件解压出来的路径替换掉下面的文件）：
```
export RUST_SRC_HOME=/path/to/your/rust/source/
export RUST_SRC_PATH=$RUST_SRC_HOME/src
```

而Windows用户请在你的环境变量里增加两个环境变量： `RUST_SRC_HOME` 为 `你的源码文件解压路径` ； `RUST_SRC_PATH` 为 `%RUST_SRC_HOME%\src\`

### 测试

请重新打开终端，并进入到关闭之前的路径。
执行如下代码：
linux:

```
./target/release/racer complete std::io::B
```

windows:

```
target\release\racer complete std::io::B
```

你将会看到racer的提示，这表示racer已经执行完成了。


## 安装 rustfmt

`cargo install rustfmt`


