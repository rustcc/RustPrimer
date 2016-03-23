# Sublime
SublimeText是一款非常有名的文本编辑器，其本身也具备强大的插件机制。通过配置各种插件可以在令您在使用SublimeText编辑Rust代码时提供更加良好的支持。
本文主要展示在已经预装Rust的Windows环境下的安装，如果您还没有安装Rust，请先参照本书的[安装章节](../02-install/02-03-install_rust_on_windows.md)安装Rust。

## 安装

### Sublime Text3安装

请在[Sublime Text3官网](http://www.sublimetext.com/3)的官网上选择适合您当前机器版本的Sublime Text版本进行下载和安装。

### Rust的安装

请在Rust官网的[下载页面](https://www.rust-lang.org/downloads.html)上下载Rust的源代码压缩包并在本地路径下解压缩安装，请牢记您的Rust源码包路径，因为我们会在下面的配置环境用到这个路径。如果国内的用户下载rust源码包的速度过慢，可以考虑使用中国科学技术大学的[镜像](http://mirrors.ustc.edu.cn/)下载Rust的源码包。

### 下载Rust代码提示插件racer

racer是一个由rust编写的代码提示插件，在稍后的配置过程中我们会将它配置到Sublime编辑器中。它的代码目前托管在[gitHub](https://github.com/phildawes/racer
)上,您可以尝试将它clone到本地或者直接下载zip压缩包在本地解压。

## 配置

### 编译racer插件

首先请在您之前下载的racer项目的根路径下使用cargo构建出其可执行文件racer.exe。
比如我的racer项目路径是`E:\soft\racer-master`,请在这个路径下执行cargo构建命令

```shell

    cargo build --release

```

构建成功后的racer.exe文件将保存在`.\target\release\racer.exe`,比如我的racer.exe本地路径就是`E:\soft\racer-master\target\release\racer.exe`。
国内链接cargo地址的速度较慢，如果构建较慢请耐心等待。

### Sublime Text3相关插件安装

#### 安装Package Control

Sublime Text3在安装各种插件前需要先安装Package Control，如果您的编辑器已经安装请跳过本段直接安装Rust的代码高亮和自动补全插件。

您可以查看[Package Control官网](https://packagecontrol.io/installation)学习如何安装。
或者在Sublime Text3中使用``+ctrl`启动控制台，然后复制粘贴以下代码并回车进行安装。

```shell

import urllib.request,os,hashlib; h = '2915d1851351e5ee549c20394736b442' + '8bc59f460fa1548d1514676163dafc88'; pf = 'Package Control.sublime-package'; ipp = sublime.installed_packages_path(); urllib.request.install_opener( urllib.request.build_opener( urllib.request.ProxyHandler()) ); by = urllib.request.urlopen( 'http://packagecontrol.io/' + pf.replace(' ', '%20')).read(); dh = hashlib.sha256(by).hexdigest(); print('Error validating download (got %s instead of %s), please try manual install' % (dh, h)) if dh != h else open(os.path.join( ipp, pf), 'wb' ).write(by)

```


#### Rust相关插件

