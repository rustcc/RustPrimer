# Rust for Windows

Rust 支持主流的操作系统，Linux,Mac和 Windows。

Rust在Windows上的安装和你在windows上安装其它软件一样。

### 1、下载安装包：

  [下载地址](https://www.rust-lang.org/zh-CN/other-installers.html)

  Rust提供了多个版本和多个平台的安装包，下载对应的即可，此处我们以[1.6.0](https://static.rust-lang.org/dist/rust-1.6.0-x86_64-pc-windows-gnu.msi)的稳定版为例。

### 2、安装：
双击下载到的安装包，如下图所示：

![Mac-os-inofrmatoin](../images/install-on-windows-1st.png)

默认，rust将安装到所有用户下，选择“Advanced”，可以指定安装用户和安装路径。然后点击"install"等待几分钟即可（中间可能会有安全提示，点击允许即可，如果你装了360之类的，需要小心360阻止写入注册表）。

### 3.验证安装：

安装完成后，运行windows命令行，然后输入:

`rustc --version`

看到 以 **rustc 1.6.0** 开头，说明你安装成功了。

**注意**

中国科学技术大学镜像源包含 [rust-static](http://mirrors.ustc.edu.cn/rust-static/)，梯子暂时出问题的同学可以尝试从这里下载编译器；除此之外。还有 Crates 源，详见[这里的说明](https://servers.ustclug.org/2016/01/mirrors-add-rust-crates/)。
